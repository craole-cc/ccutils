// use crate::cli::commands::{add, list, remove, update};
use anyhow::Result;
use clap::{Parser, Subcommand};
use logline::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Default {
    /// Optional package manager to use
    #[arg(short, long)]
    pub manager: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Package name or search query (defaults to list available)
    pub query: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add/install packages
    Add {
        /// Packages to install
        packages: Vec<String>,

        /// Install from file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Update packages
    Update {
        /// Packages to update
        packages: Vec<String>,

        /// Update from file
        #[arg(short, long)]
        file: Option<String>,
    },

    /// Remove/uninstall packages
    Remove {
        /// Packages to remove
        packages: Vec<String>,
    },

    /// List packages or package managers
    List {
        #[command(subcommand)]
        command: ListCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ListCommands {
    /// List installed packages
    Installed,
    /// List available packages (default)
    Available {
        /// Search query
        query: Option<String>,
    },
    /// List outdated packages
    Outdated,
    /// List available package managers
    Managers,
}