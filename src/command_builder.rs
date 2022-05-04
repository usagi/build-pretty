use super::*;

pub struct CommandBuilder
{
 c: Command
}

impl CommandBuilder
{
 pub fn new<S: AsRef<str>>(command: S) -> Self
 {
  Self {
   c: Command::new(command.as_ref())
  }
 }

 pub fn new_with_args<S>(command: S, args: &[S]) -> Self
 where S: AsRef<str> + std::convert::AsRef<std::ffi::OsStr>
 {
  let s = Self::new(command);
  s.with_args(args)
 }

 pub fn with_args<S>(mut self, args: &[S]) -> Self
 where S: AsRef<str> + std::convert::AsRef<std::ffi::OsStr>
 {
  self.c.args(args);
  self
 }

 pub fn new_with_arg<S>(command: S, arg: S) -> Self
 where S: AsRef<str> + std::convert::AsRef<std::ffi::OsStr>
 {
  let s = Self::new(command);
  s.with_arg(arg)
 }

 pub fn with_arg<S>(mut self, arg: S) -> Self
 where S: AsRef<str> + std::convert::AsRef<std::ffi::OsStr>
 {
  self.c.arg(arg);
  self
 }
}

impl Into<Command> for CommandBuilder
{
 fn into(self) -> Command { self.c }
}
