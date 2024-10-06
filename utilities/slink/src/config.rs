use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub force: bool,
    pub debug: bool,
    pub sources: Vec<PathBuf>,
    pub link_base: PathBuf,
}

impl Config {
    pub fn new(force: bool, debug: bool, sources: Vec<PathBuf>, link_base: PathBuf) -> Self {
        Self {
            force,
            debug,
            sources,
            link_base,
        }
    }

    pub fn resolve_link_path(
        &self,
        src: &std::path::Path,
    ) -> Result<PathBuf, crate::error::SymlinkError> {
        let src_name = src
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or(crate::error::SymlinkError::NoSourceFileName)?;

        Ok(self.link_base.join(src_name))
    }
}
