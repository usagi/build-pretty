use build_pretty::{
 build_pretty,
 CommandBuilder,
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
  .enque_fn("Ofcourse Fn can be used", Box::new(|output|{ *output = "brabrabra\nmewmewmew\nnekonyankonyanko🐾".to_string(); Ok(()) }))
  .enque_command(
   "Sleep",
   CommandBuilder::new_with_arg("echo", "😴 I'm sleee....\n💤...\n🛌....pyyyyy....").into()
  );
}
