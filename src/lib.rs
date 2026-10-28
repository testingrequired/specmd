use std::fmt::Display;

use serde::{Deserialize, Serialize};

mod errors;
mod md;
mod parse;
mod span;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "tag", content = "data")]
pub enum CutBlockKind {
    Env,
    Stdin,
    Shell,
    Stdout,
    Stderr,
    Exit,
}

impl Display for CutBlockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CutBlockKind::Env => "env",
            CutBlockKind::Stdin => "stdin",
            CutBlockKind::Shell => "shell",
            CutBlockKind::Stdout => "stdout",
            CutBlockKind::Stderr => "stderr",
            CutBlockKind::Exit => "exit",
        };

        write!(f, "{s}")
    }
}

pub mod prelude {
    pub use crate::{errors::CutResult, parse::parse};
}
