use std::path::PathBuf;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bash,
    Zsh,
    Fish,
    CommandPrompt,
    PowerShell,
    Nushell,
    #[default]
    Unsupported,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Info {
    pub id: u32,
    pub name: String,
    pub path: PathBuf,
    pub conf: Vec<PathBuf>,
    pub version: Option<String>,
}

impl Info {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch(&self) -> String {
        format!(
            "Time {{\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            {:>16}: {}\n\
            }}",
            "Name",
            self.name,
            "Path",
            self.path.display(),
            "Version",
            self.version.as_deref().unwrap_or("Unknown"),
            "Configurations ",
            self.conf
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
