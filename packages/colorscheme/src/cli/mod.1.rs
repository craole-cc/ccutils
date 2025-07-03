use crate::core::CommandLocation;

pub fn init() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    eprintln!("Usage: {} <command>", args[0]);
    std::process::exit(1);
  }

  let command = &args[1];
  match CommandLocation::find(command) {
    Ok(locations) =>
      for location in locations {
        match location {
          CommandLocation::Executable(path) =>
            println!("{} is an executable at {}", command, path.display()),
          CommandLocation::ShellBuiltin => {
            println!("{} is a shell builtin", command)
          }
          CommandLocation::ShellAlias(info) => {
            println!("{}", info)
          }
          CommandLocation::ShellFunction(info) => {
            println!("{}", info)
          }
        }
      },
    Err(e) => eprintln!("Error: {}", e)
  }
}
