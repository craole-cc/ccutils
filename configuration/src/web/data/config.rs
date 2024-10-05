use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};
use crate::Project;

#[derive(Debug, Clone)]
pub struct Web {
    pub assets_path: PathBuf,
}

impl Web {
    pub fn new() -> Result<Self> {
        let project_result = Project::get().clone();
        let project = project_result.as_ref().map_err(|e| anyhow!("Failed to get project: {}", e))?;
        let assets_path = project.assets_path.join("web");
        Ok(Self { assets_path })
    }
}

impl Display for Web {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "\tWeb:")?;
        writeln!(f, "  Assets Path: {}", self.assets_path.display())
    }
}
