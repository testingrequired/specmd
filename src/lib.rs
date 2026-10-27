mod errors;
mod md;
mod parse;
mod span;

#[derive(Debug)]
pub enum CutBlockKind {
    Env,
    Stdin,
    Shell,
    Stdout,
    Stderr,
    Exit,
}

pub mod prelude {
    pub use crate::{errors::CutResult, parse::parse};
}
