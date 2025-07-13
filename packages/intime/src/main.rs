mod cli;

fn main() {
  match cli::run() {
    Ok(_) => {}
    Err(e) => {
      eprintln!("{e}");
      std::process::exit(1);
    }
  }
}
