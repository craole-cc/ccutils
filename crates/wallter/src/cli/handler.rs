use clap::{Arg, Command};

pub fn parse_args() -> Command {
  Command::new("wallter")
    .about("A wallpaper management utility")
    .arg(
      Arg::new("search")
        .short('s')
        .long("search")
        .value_name("QUERY")
        .help("Search for wallpapers using a query")
    )
    .arg(
      Arg::new("set")
        .short('w')
        .long("set")
        .value_name("URL")
        .help("Set wallpaper from a URL")
    )
}
