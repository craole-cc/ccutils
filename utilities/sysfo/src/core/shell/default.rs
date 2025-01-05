use super::process;
use std::{
    path::PathBuf,
    process::{id, Command},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub name: Name,
    pub config: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Name {
    Bash,
    Zsh,
    Fish,
    CommandPrompt,
    Powershell,
    Nushell,
    Unsupported,
}

impl Info {
    pub fn new(process: process::Info) -> Self {
        let name = Name::Bash;
        let config = vec![process.path];
        Self { name, config }
    }

    pub fn fetch(&self) -> String {
        format!(
            "Time {{\n\
            {:>16}: {:#?}\n\
            {:>16}: {:#?}\n\
            }}",
            "Id", self.name, "Name", self.config,
        )
    }
}
