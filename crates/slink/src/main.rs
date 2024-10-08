use clap::{arg, command, value_parser, ArgAction};
use directories::BaseDirs;
use glob::glob;
use slink::{process_links, Config, SymlinkError};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .about("Creates symbolic links for configuration files")
        .arg(
            arg!(-s --src <PATTERN> "Source file(s) or directory to link (supports glob patterns)")
                .value_parser(value_parser!(String))
                .required(true)
                .action(ArgAction::Append),
        )
        .arg(
            arg!(-l --link <PATH> "Base directory for links (defaults to XDG config home)")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .arg(
            arg!(-f --force "Overwrite destination if it exists without prompting")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-v --verbose "Increase verbosity (can be used multiple times)")
                .action(ArgAction::Count),
        )
        .arg(
            arg!(-q --quiet "Suppress all output except errors")
                .action(ArgAction::SetTrue)
                .conflicts_with("verbose"),
        )
        .arg(
            arg!(-d --debug "Debug mode: simulate operations and show additional info")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Set up tracing
    let filter = if matches.get_flag("quiet") {
        EnvFilter::new("error")
    } else {
        match matches.get_count("verbose") {
            0 => EnvFilter::new("info"),
            1 => EnvFilter::new("debug"),
            _ => EnvFilter::new("trace"),
        }
    };

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    let mut sources = Vec::new();
    for pattern in matches.get_many::<String>("src").unwrap() {
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => sources.push(path),
                Err(e) => {
                    tracing::error!("Error processing {}: {}", pattern, e)
                }
            }
        }
    }

    if sources.is_empty() {
        return Err(SymlinkError::NoMatchingFiles.into());
    }

    let base_dirs = BaseDirs::new().ok_or(SymlinkError::NoConfigDir)?;
    let default_link_base = base_dirs.config_dir().to_path_buf();

    let config = Config::new(
        matches.get_flag("force"),
        matches.get_flag("debug"),
        sources,
        matches
            .get_one::<std::path::PathBuf>("link")
            .cloned()
            .unwrap_or(default_link_base),
    );

    // debug!(?config, "Configuration");
    // debug!("{}", config);

    process_links(&config)?;
    Ok(())
}
