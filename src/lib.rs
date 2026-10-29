use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod errors;
mod md;
mod parse;
mod span;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "data")]
pub enum SpecMdBlockKind {
    Env,
    Stdin,
    Shell,
    Stdout,
    Stderr,
    Exit,
}

impl Display for SpecMdBlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SpecMdBlockKind::Env => "env",
            SpecMdBlockKind::Stdin => "stdin",
            SpecMdBlockKind::Shell => "shell",
            SpecMdBlockKind::Stdout => "stdout",
            SpecMdBlockKind::Stderr => "stderr",
            SpecMdBlockKind::Exit => "exit",
        };

        write!(f, "{s}")
    }
}

pub mod prelude {
    pub use crate::{errors::SpecMdResult, parse::parse};
}
