fn main() -> anyhow::Result<()> {
  println!("\nWelcome to 🦀 sysfo!");
  logline::init();
  sysfo::test();
  println!("\nGood Bye");

  Ok(())
}
