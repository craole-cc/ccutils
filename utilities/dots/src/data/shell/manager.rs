use std::path::PathBuf;

#[derive(Debug, Default)]
enum Shells {
    #[default]
    Bash,
    Zsh,
    Fish,
    Cmd,
    PowerShell,
    Nushell,
    Unknown,
}

#[derive(Debug, Default)]
pub struct Manager {
    pub name: Shells,
    pub status: Status,
    pub version: String,
    pub path: PathBuf,
    pub config: Vec<PathBuf>,
}

impl Manager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_name(&mut self, name: Shells) {
        self.name = name;
    }

    pub fn activate(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn is_avaible(&self) -> bool {
        self.available
    }

    pub fn config(mut self, config: Vec<PathBuf>) -> Self {
        self.config = config;
        self
    }

}
