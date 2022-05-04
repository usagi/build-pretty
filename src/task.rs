use super::*;

pub enum Task
{
 Command(Command),
 Fn(TaskFn)
}

impl From<Command> for Task
{
 fn from(c: Command) -> Self { Task::Command(c) }
}

impl From<TaskFn> for Task
{
 fn from(f: TaskFn) -> Self { Task::Fn(f) }
}

impl Task
{
 pub fn invoke(&mut self, task_name: &'static str, task_index: usize) -> Result<String, BuildPrettyError>
 {
  match self
  {
   Task::Command(c) => invoke_command(c, task_name, task_index),
   Task::Fn(f) => invoke_fn(f, task_name, task_index)
  }
 }
}

fn invoke_command(c: &mut Command, task_name: &'static str, task_index: usize) -> Result<String, BuildPrettyError>
{
 let o = c.output().map_err(|error_source| {
  BuildPrettyError::CommandWasFailed {
   task_name,
   task_index,
   output: None,
   error_source: Some(Box::new(error_source))
  }
 })?;

 let stdout = String::from_utf8(o.clone().stdout).map_err(|error_source| {
  BuildPrettyError::CommandWasFailed {
   task_name,
   task_index,
   output: Some(o.clone()),
   error_source: Some(Box::new(error_source))
  }
 })?;

 // let oo = o.clone();
 // let _stderr = String::from_utf8(o.stderr).map_err(|error_source| {
 //  BuildPrettyError::CommandWasFailed {
 //   task_name,
 //   task_index,
 //   output: Some(oo),
 //   error_source: Some(Box::new(error_source))
 //  }
 // })?;

 match o.status.success()
 {
  true => Ok(stdout),
  false =>
  {
   Err(BuildPrettyError::CommandWasFailed {
    task_name,
    task_index,
    output: Some(o),
    error_source: None
   })
  },
 }
}

fn invoke_fn(f: &TaskFn, task_name: &'static str, task_index: usize) -> Result<String, BuildPrettyError>
{
 let mut s = String::new();
 f(&mut s).map_err(|error_source| {
  BuildPrettyError::FnWasFailed {
   task_name,
   task_index,
   output: s.clone(),
   error_source: Some(error_source)
  }
 })?;
 Ok(s)
}
