use std::{error::Error, fmt::Display};

use crate::{SpecMdBlockKind, span::Span};

pub type SpecMdResult<T> = Result<T, SpecMdError>;

#[derive(Debug)]
pub enum ParseError {
    RequiredBlockMissing(SpecMdBlockKind),
    TooManyOfBlock(SpecMdBlockKind),
}

#[derive(Debug)]
pub enum SpecMdErrorKind {
    ParseError(ParseError),
    Aggregate(Vec<SpecMdError>),
}

#[derive(Debug)]
pub struct SpecMdError {
    pub kind: SpecMdErrorKind,
    pub span: Span,
}

impl SpecMdError {
    // Is this error an aggregate of other errors?
    pub fn is_aggregate(self) -> bool {
        matches!(self.kind, SpecMdErrorKind::Aggregate(_))
    }

    pub fn aggregated_errors(&self) -> Option<&Vec<SpecMdError>> {
        match &self.kind {
            SpecMdErrorKind::Aggregate(errs) => Some(errs),
            _ => None,
        }
    }
}

impl Error for SpecMdError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for SpecMdError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
