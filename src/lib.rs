use retrieve::mod_pub_use;
use std::process::Command;

rust_i18n::i18n!("locales");

/// This macro is the Build Script version of println!
/// Special thank to (@Ryan1729)[https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472]
#[macro_export]
macro_rules! cargo_warning_ln { ($($tokens: tt)*) => { println!("cargo:warning={}", format!($($tokens)*)); } }

/// This macro can be used to automatically set CARGO_PKG_NAME and CARGO_PKG_VERSION.
#[macro_export]
macro_rules! build_pretty {
 ($($tokens:tt)*) => {
  build_pretty::BuildPretty::new().with_cargo_pkg_name(env!("CARGO_PKG_NAME")).with_cargo_pkg_version(env!("CARGO_PKG_VERSION"))
 };
}

#[mod_pub_use(build_pretty_error, task, task_fn, build_type_string, command_builder)]
pub struct BuildPretty
{
 /// [ ( task_name, task ), ... ]
 /// Helper methods are available:
 ///  for command: `queue_command`, `queue_commands`
 ///  for fn     : `queue_fn`     , `queue_fns`
 pub tasks: Vec<(&'static str, Task)>,

 /// true  => panic! if task was failed. (default)
 /// false => run all tasks and return results.
 pub safe_panic: bool,

 /// Replacement value of "{cargo_pkg_name}" in a message.
 /// see also: `build_pretty!` macro.
 pub cargo_pkg_name: Option<&'static str>,

 /// Replacement value of "{cargo_pkg_name}" in a message.
 /// see also: `build_pretty!` macro.
 pub cargo_pkg_version: Option<&'static str>,

 /// Notice about `warnings` that are displayed on the left side by `Cargo:warning`. Set to None if not needed.
 pub message_warning_notice: Option<String>,

 /// Template for a message announcing the start of the build process. Set to None if not needed.
 pub message_process_begin: Option<String>,
 /// Template for a message announcing the end of the build process. Set to None if not needed.
 pub message_process_end:   Option<String>,

 /// Template for a message announcing the start of the task. Set to None if not needed.
 pub message_task_begin:        Option<String>,
 /// Template for a message from the task output. Set to None if not needed.
 pub message_task_body:         Option<String>,
 /// Template for a message announcing a succeeded of the task. Set to None if not needed.
 pub message_task_succeeded:    Option<String>,
 /// Template for a message announcing an error of the task. Set to None if not needed.
 pub message_task_error:        Option<String>,
 /// Template for a message announcing notice of an error of the task. Set to None if not needed.
 pub message_task_error_notice: Option<String>,

 /// Template for the safe panic message. Set to None if not needed.
 pub message_safe_panic: Option<String>
}

impl BuildPretty
{
 pub fn new() -> Self
 {
  use rust_i18n::t;
  Self {
   tasks:                     vec![],
   safe_panic:                true,
   cargo_pkg_name:            None,
   cargo_pkg_version:         None,
   message_warning_notice:    Some(t!("message_warning_notice")),
   message_process_begin:     Some(t!("message_process_begin")),
   message_process_end:       Some(t!("message_process_end")),
   message_task_begin:        Some(t!("message_task_begin")),
   message_task_body:         Some(t!("message_task_body")),
   message_task_succeeded:    Some(t!("message_task_succeeded")),
   message_task_error:        Some(t!("message_task_error")),
   message_task_error_notice: Some(t!("message_task_error_notice")),
   message_safe_panic:        Some(t!("message_safe_panic"))
  }
 }

 pub fn with_cargo_pkg_name(mut self, v: &'static str) -> Self
 {
  self.cargo_pkg_name = Some(v);
  self
 }

 pub fn with_cargo_pkg_version(mut self, v: &'static str) -> Self
 {
  self.cargo_pkg_version = Some(v);
  self
 }

 pub fn new_with_locale(locale: &str) -> Self
 {
  rust_i18n::set_locale(locale);
  Self::new()
 }

 pub fn enque_command(&mut self, task_name: &'static str, command: Command) -> &mut Self
 {
  self.tasks.push((task_name, Task::from(command)));
  self
 }

 pub fn enque_commands(&mut self, task_name_and_commands: Vec<(&'static str, Command)>) -> &mut Self
 {
  for (task_name, command) in task_name_and_commands.into_iter()
  {
   let task = Task::from(command);
   self.tasks.push((task_name, task));
  }
  self
 }

 pub fn enque_fn(&mut self, task_name: &'static str, fun: TaskFn) -> &mut Self
 {
  self.tasks.push((task_name, Task::from(fun)));
  self
 }

 pub fn enque_fns(&mut self, task_name_and_funs: Vec<(&'static str, TaskFn)>) -> &mut Self
 {
  for (task_name, fun) in task_name_and_funs.into_iter()
  {
   let task = Task::from(fun);
   self.tasks.push((task_name, task));
  }
  self
 }

 pub fn invoke(&mut self) -> Vec<Result<String, BuildPrettyError>>
 {
  let mut rr = vec![];
  let total_tasks_string = self.tasks.len().to_string();
  put_message_process(
   &self.message_process_begin,
   &total_tasks_string,
   self.cargo_pkg_name.unwrap_or_default(),
   self.cargo_pkg_version.unwrap_or_default()
  );
  put_message_process(
   &self.message_warning_notice,
   &total_tasks_string,
   self.cargo_pkg_name.unwrap_or_default(),
   self.cargo_pkg_version.unwrap_or_default()
  );
  for (task_index, (task_name, task)) in self.tasks.iter_mut().enumerate()
  {
   let task_index_string = (task_index + 1).to_string();
   put_message_task(
    &self.message_task_begin,
    task_name,
    &task_index_string,
    &total_tasks_string,
    self.cargo_pkg_name.unwrap_or_default(),
    self.cargo_pkg_version.unwrap_or_default()
   );
   let r = task.invoke(task_name, task_index);
   match r.as_ref()
   {
    Ok(o) if self.message_task_body.is_some() =>
    {
     put_message_task_body(
      &self.message_task_body,
      &o,
      task_name,
      &task_index.to_string(),
      &total_tasks_string,
      self.cargo_pkg_name.unwrap_or_default(),
      self.cargo_pkg_version.unwrap_or_default()
     )
    },
    Err(e) =>
    {
     put_message_task_body(
      &self.message_task_body,
      &e.extract_output(),
      task_name,
      &task_index_string,
      &total_tasks_string,
      self.cargo_pkg_name.unwrap_or_default(),
      self.cargo_pkg_version.unwrap_or_default()
     );
     put_message_task(
      &self.message_task_error,
      task_name,
      &task_index_string,
      &total_tasks_string,
      self.cargo_pkg_name.unwrap_or_default(),
      self.cargo_pkg_version.unwrap_or_default()
     );
     put_message_task(
      &self.message_task_error_notice,
      task_name,
      &task_index_string,
      &total_tasks_string,
      self.cargo_pkg_name.unwrap_or_default(),
      self.cargo_pkg_version.unwrap_or_default()
     );
     let m = resolve_message_safe_panic(
      &self.message_safe_panic,
      task_name,
      &task_index_string,
      &total_tasks_string,
      self.cargo_pkg_name.unwrap_or_default(),
      self.cargo_pkg_version.unwrap_or_default()
     );
     self.tasks.clear();
     panic!("{m:?}")
    },
    _ =>
    {}
   }
   put_message_task(
    &self.message_task_succeeded,
    task_name,
    &task_index_string,
    &total_tasks_string,
    self.cargo_pkg_name.unwrap_or_default(),
    self.cargo_pkg_version.unwrap_or_default()
   );
   rr.push(r)
  }
  put_message_process(
   &self.message_process_end,
   &total_tasks_string,
   self.cargo_pkg_name.unwrap_or_default(),
   self.cargo_pkg_version.unwrap_or_default()
  );
  self.tasks.clear();
  rr
 }
}

impl Drop for BuildPretty
{
 fn drop(&mut self)
 {
  if !self.tasks.is_empty()
  {
   self.invoke();
  }
 }
}

fn resolve_message_template(
 template: &String,
 task_name: &str,
 task_index: &str,
 total_tasks: &str,
 task_message: &str,
 cargo_pkg_name: &str,
 cargo_pkg_version: &str
) -> String
{
 template
  .replace("{cargo_pkg_name}", cargo_pkg_name)
  .replace("{cargo_pkg_version}", cargo_pkg_version)
  .replace("{build_type}", BUILD_TYPE_STRING)
  .replace("{task_name}", task_name)
  .replace("{task_index}", task_index)
  .replace("{total_tasks}", total_tasks)
  .replace("{task_message}", task_message)
}

fn put_message_process(template: &Option<String>, total_tasks: &str, cargo_pkg_name: &str, cargo_pkg_version: &str)
{
 if let Some(template) = template
 {
  let m = resolve_message_template(template, "", "", total_tasks, "", cargo_pkg_name, cargo_pkg_version);
  cargo_warning_ln!("{m}");
 }
}

fn put_message_task(
 template: &Option<String>,
 task_name: &str,
 task_index: &str,
 total_tasks: &str,
 cargo_pkg_name: &str,
 cargo_pkg_version: &str
)
{
 if let Some(template) = template
 {
  let m = resolve_message_template(template, task_name, task_index, total_tasks, "", cargo_pkg_name, cargo_pkg_version);
  cargo_warning_ln!("{m}");
 }
}

fn put_message_task_body(
 template: &Option<String>,
 message: &str,
 task_name: &str,
 task_index: &str,
 total_tasks: &str,
 cargo_pkg_name: &str,
 cargo_pkg_version: &str
)
{
 if let Some(template) = template
 {
  message.split('\n').into_iter().filter(|m| !m.is_empty()).for_each(|m| {
   let m = resolve_message_template(template, task_name, task_index, total_tasks, m, cargo_pkg_name, cargo_pkg_version);
   cargo_warning_ln!("{m}");
  });
 }
}

fn resolve_message_safe_panic(
 template: &Option<String>,
 task_name: &str,
 task_index: &str,
 total_tasks: &str,
 cargo_pkg_name: &str,
 cargo_pkg_version: &str
) -> String
{
 template
  .as_ref()
  .map(|template| resolve_message_template(template, task_name, task_index, total_tasks, "", cargo_pkg_name, cargo_pkg_version))
  .unwrap_or_default()
}
