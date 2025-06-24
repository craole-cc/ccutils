use std::env;

#[derive(Debug, Clone)]
pub enum Command {
  Build { crates: Vec<String> },
  Install { crates: Vec<String> },
  BuildInstall { crates: Vec<String> }
}

#[derive(Debug)]
pub struct Cli {
  pub command: Command,
  pub force: bool,
  pub verbose: bool
}

impl Cli {
  pub fn parse() -> Self {
    let args: Vec<String> = env::args().collect();
    let mut command = Command::BuildInstall { crates: vec![] };
    let mut force = false;
    let mut verbose = false;

    let mut i = 1; // Skip program name
    while i < args.len() {
      match args[i].as_str() {
        "build" => {
          //{ Collect remaining args as crate names }
          let crates = args[(i + 1)..]
            .iter()
            .filter(|arg| !arg.starts_with('-'))
            .cloned()
            .collect();
          command = Command::Build { crates };
          break;
        }
        "install" => {
          //{ Collect remaining args as crate names }
          let crates = args[(i + 1)..]
            .iter()
            .filter(|arg| !arg.starts_with('-'))
            .cloned()
            .collect();
          command = Command::Install { crates };
          break;
        }
        "build-install" => {
          //{ Collect remaining args as crate names }
          let crates = args[(i + 1)..]
            .iter()
            .filter(|arg| !arg.starts_with('-'))
            .cloned()
            .collect();
          command = Command::BuildInstall { crates };
          break;
        }
        "-f" | "--force" => force = true,
        "-v" | "--verbose" => verbose = true,
        "-h" | "--help" => {
          Self::print_help();
          std::process::exit(0);
        }
        arg if !arg.starts_with('-') => {
          //{ If no subcommand specified, treat as crates for build-install }
          let crates = args[i..]
            .iter()
            .filter(|arg| !arg.starts_with('-'))
            .cloned()
            .collect();
          command = Command::BuildInstall { crates };
          break;
        }
        _ => {
          eprintln!("Unknown argument: {}", args[i]);
          Self::print_help();
          std::process::exit(1);
        }
      }
      i += 1;
    }

    Self {
      command,
      force,
      verbose
    }
  }

  fn print_help() {
    println!("Cargo Workspace Builder");
    println!("Build and install binary crates in a Cargo workspace");
    println!();
    println!("USAGE:");
    println!("    ccutils [OPTIONS] [COMMAND] [CRATES...]");
    println!();
    println!("COMMANDS:");
    println!("    build           Build binary crates (without installing)");
    println!(
      "    install         Install binary crates (without building first)"
    );
    println!("    build-install   Build and install binary crates (default)");
    println!();
    println!("OPTIONS:");
    println!(
      "    -f, --force     Skip checking modification times and force rebuild/install"
    );
    println!("    -v, --verbose   Verbose output");
    println!("    -h, --help      Show this help message");
    println!();
    println!("EXAMPLES:");
    println!(
      "    ccutils                    # Build and install all outdated binary crates"
    );
    println!(
      "    ccutils build              # Build all outdated binary crates"
    );
    println!("    ccutils install crate1     # Install specific crate");
    println!(
      "    ccutils --force build      # Force rebuild all binary crates"
    );
    println!(
      "    ccutils --verbose crate1 crate2  # Build and install specific crates with verbose output"
    );
  }
}
