use app::*;

fn main() -> anyhow::Result<()> {
  logline::init();
  cli::init();

  let mut test_cmd = std::process::Command::new("app");
  // .args(["bat"])
  // .args(["add", "bat"])
  // .args(["add", "sharkdp.bat"])
  // .spawn()?;

  if let Err(e) = test_cmd.spawn() {
    println!("Error: {}", e);
  } else {
    println!("Command completed successfully");
  };

  Ok(())
}
