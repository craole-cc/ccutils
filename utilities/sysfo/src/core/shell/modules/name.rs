use std::{
    default,
    path::PathBuf,
    process::{id, Command},
};

use crate::core::process;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Name {
    Bash,
    Zsh,
    Fish,
    CommandPrompt,
    Powershell,
    Nushell,
    #[default]
    Unsupported,
}

// impl Name {
//     pub fn new(id: u32) -> Self

// }
