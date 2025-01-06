use super::Name;
use crate::core::process;
use std::{
    path::PathBuf,
    process::{id, Command},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub name: Name,
    pub config: Vec<PathBuf>,impl Name {
        //     pub fn new(id: u32) -> Self

        // }
}

impl Info {
    pub fn new(process: &process::Info) -> Self {
        let name = Name::Bash;
        let config = vec![PathBuf::from("/")];
        Self { name, config }
    }

    fn get
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
