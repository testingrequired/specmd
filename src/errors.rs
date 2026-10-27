use std::{error::Error, fmt::Display};

use crate::{CutBlockKind, span::Span};

pub type CutResult<T> = Result<T, CutError>;

#[derive(Debug)]
pub enum ParseError {
    RequiredBlockMissing(CutBlockKind),
    TooManyOfBlock(CutBlockKind),
}

#[derive(Debug)]
pub enum CutErrorKind {
    ParseError(ParseError),
    Aggregate(Vec<CutError>),
}

#[derive(Debug)]
pub struct CutError {
    pub kind: CutErrorKind,
    pub span: Span,
}

impl CutError {
    // Is this error an aggregate of other errors?
    pub fn is_aggregate(self) -> bool {
        matches!(self.kind, CutErrorKind::Aggregate(_))
    }

    pub fn aggregated_errors(&self) -> Option<&Vec<CutError>> {
        match &self.kind {
            CutErrorKind::Aggregate(cut_errors) => Some(cut_errors),
            _ => None,
        }
    }
}

impl Error for CutError {
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

impl Display for CutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
