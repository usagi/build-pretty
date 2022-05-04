use build_pretty::{
 build_pretty,
 CommandBuilder
};

fn main()
{
 build_pretty!()
  .enque_command(
   "Drink a cup of tea",
   CommandBuilder::new_with_arg("echo", "🍵 Green!\n☕ Black!\n🧋 Bubbles!").into()
  )
  .enque_command(
   "Eat a hotdog",
   CommandBuilder::new_with_arg("echo", "🌭 Hotdog!\n♨️ Hot?\n🐕 Dog!\n🌶️ Hot?\n🐶 Dog?").into()
  )
  .enque_command("ls -l -a", CommandBuilder::new_with_args("ls", &["-l", "-a"]).into())
  .enque_command(
   "MIGHT BE PANIC!",
   CommandBuilder::new("MIGHT BE PANIC! This test might be panic! Because this is not a message, it's running command now!🤣").into()
  );
}
