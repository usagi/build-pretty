use std::process::Output;

use thiserror::Error;

pub type BoxedDynStdError = Box<dyn std::error::Error>;

#[derive(Error, Debug)]
pub enum BuildPrettyError
{
 #[error("command was failed: task_name={task_name:?} task_index={task_index:?} output={output:?} error_source={error_source:?}")]
 CommandWasFailed
 {
  task_name:    &'static str,
  task_index:   usize,
  output:       Option<Output>,
  error_source: Option<Box<dyn std::error::Error>>
 },

 #[error("fn was failed: task_name={task_name:?} task_index={task_index:?} output={output:?} error_source={error_source:?}")]
 FnWasFailed
 {
  task_name:    &'static str,
  task_index:   usize,
  output:       String,
  error_source: Option<Box<dyn std::error::Error>>
 }
}

impl BuildPrettyError
{
 pub fn extract_output(&self) -> String
 {
  match self
  {
   Self::CommandWasFailed {
    task_name: _,
    task_index: _,
    output,
    error_source: _
   } =>
   {
    match output
    {
     Some(o) => format!("{}\n{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)),
     None => String::new()
    }
   },
   Self::FnWasFailed {
    task_name: _,
    task_index: _,
    output,
    error_source: _
   } => output.clone()
  }
 }
}
