use serde::{Deserialize, Serialize};

use crate::{
    SpecMdBlockKind,
    errors::{ParseError, SpecMdError, SpecMdErrorKind, SpecMdResult},
    md::{extract_codeblocks, extract_frontmatter},
    span::{NO_SPAN, Span},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedSpecMdFile {
    pub frontmatter: Option<Span>,
    pub shell: Span,
    pub stdout: Span,
    pub stderr: Span,
    pub exit: Span,
    pub env: Option<Span>,
    pub stdin: Option<Span>,
}

fn get_block(source: &str, kind: SpecMdBlockKind) -> SpecMdResult<Span> {
    match get_optional_block(source, kind.clone())? {
        Some(span) => Ok(span),
        None => Err(SpecMdError {
            kind: SpecMdErrorKind::ParseError(ParseError::RequiredBlockMissing(kind)),
            span: NO_SPAN,
        }),
    }
}

fn get_optional_block(source: &str, kind: SpecMdBlockKind) -> SpecMdResult<Option<Span>> {
    let blocks = extract_codeblocks(source, kind.to_string());

    let mut errs: Vec<SpecMdError> = vec![];

    if blocks.is_empty() {
        return Ok(None);
    } else if blocks.len() > 1 {
        for (_, span) in blocks.iter().skip(1) {
            errs.push(SpecMdError {
                kind: SpecMdErrorKind::ParseError(ParseError::TooManyOfBlock(kind.clone())),
                span: span.clone(),
            });
        }
    }

    if !errs.is_empty() {
        return Err(SpecMdError {
            kind: SpecMdErrorKind::Aggregate(errs),
            span: NO_SPAN,
        });
    }

    let span = &blocks.first().unwrap().0.1;

    Ok(Some(span.clone()))
}

pub fn parse(source: &str) -> SpecMdResult<ParsedSpecMdFile> {
    let mut errs: Vec<SpecMdError> = vec![];

    let frontmatter = extract_frontmatter(source).map(|x| x.1);

    // Required

    let shell = match get_block(source, SpecMdBlockKind::Shell) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            NO_SPAN
        }
    };

    let stdout = match get_block(source, SpecMdBlockKind::Stdout) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            NO_SPAN
        }
    };

    let stderr = match get_block(source, SpecMdBlockKind::Stderr) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            NO_SPAN
        }
    };

    let exit = match get_block(source, SpecMdBlockKind::Exit) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            NO_SPAN
        }
    };

    // Optional

    let env = match get_optional_block(source, SpecMdBlockKind::Env) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            None
        }
    };

    let stdin = match get_optional_block(source, SpecMdBlockKind::Stdin) {
        Ok(span) => span,
        Err(err) => {
            errs.push(err);
            None
        }
    };

    if !errs.is_empty() {
        return Err(SpecMdError {
            kind: SpecMdErrorKind::Aggregate(errs),
            span: NO_SPAN,
        });
    }

    Ok(ParsedSpecMdFile {
        frontmatter,
        shell,
        stdout,
        stderr,
        exit,
        env,
        stdin,
    })
}
