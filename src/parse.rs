use crate::{
    CutBlockKind,
    errors::{CutError, CutErrorKind, CutResult, ParseError},
    md::{extract_codeblocks, extract_frontmatter},
    span::{NO_SPAN, Span},
};

#[derive(Debug)]
pub struct CutFrontmatterFromSource {
    span: Span,
}

#[derive(Debug)]
pub struct CutBlockFromSource {
    kind: CutBlockKind,
    span: Span,
}

impl CutBlockFromSource {
    pub fn new(kind: CutBlockKind, span: &Span) -> Self {
        Self {
            kind,
            span: span.clone(),
        }
    }
}

impl CutBlockFromSource {
    pub fn get_source<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }
}

#[derive(Debug)]
pub struct ParsedCutFile {
    frontmatter: Option<CutFrontmatterFromSource>,
    shell: CutBlockFromSource,
    stdout: CutBlockFromSource,
    stderr: CutBlockFromSource,
    exit: CutBlockFromSource,
}

const ENV_BLOCK: &'static str = "env";
const SHELL_BLOCK: &'static str = "shell";
const STDIN_BLOCK: &'static str = "stdin";
const STDOUT_BLOCK: &'static str = "stdout";
const STDERR_BLOCK: &'static str = "stderr";
const EXIT_BLOCK: &'static str = "exit";

pub fn parse(source: &str) -> CutResult<ParsedCutFile> {
    let mut errs: Vec<CutError> = vec![];

    let frontmatter = extract_frontmatter(source).map(|x| CutFrontmatterFromSource { span: x.1 });

    // Required

    let shell_blocks = extract_codeblocks(source, SHELL_BLOCK);

    if shell_blocks.is_empty() {
        errs.push(CutError {
            kind: CutErrorKind::ParseError(ParseError::RequiredBlockMissing(CutBlockKind::Shell)),
            span: NO_SPAN,
        });
    } else if shell_blocks.len() > 1 {
        for (_, span) in shell_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Shell)),
                span: span.clone(),
            });
        }
    }

    let shell_block_span = &shell_blocks.first().unwrap().0.1;

    let stdout_blocks = extract_codeblocks(source, STDOUT_BLOCK);

    if stdout_blocks.is_empty() {
        errs.push(CutError {
            kind: CutErrorKind::ParseError(ParseError::RequiredBlockMissing(CutBlockKind::Stdout)),
            span: NO_SPAN,
        });
    } else if stdout_blocks.len() > 1 {
        for (_, span) in stdout_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Stdout)),
                span: span.clone(),
            });
        }
    }

    let stdout_block_span = &stdout_blocks.first().unwrap().0.1;

    let stderr_blocks = extract_codeblocks(source, STDERR_BLOCK);

    if stderr_blocks.is_empty() {
        errs.push(CutError {
            kind: CutErrorKind::ParseError(ParseError::RequiredBlockMissing(CutBlockKind::Stderr)),
            span: NO_SPAN,
        });
    } else if stderr_blocks.len() > 1 {
        for (_, span) in stderr_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Stderr)),
                span: span.clone(),
            });
        }
    }

    let stderr_block_span = &stderr_blocks.first().unwrap().0.1;

    let exit_blocks = extract_codeblocks(source, EXIT_BLOCK);

    if exit_blocks.is_empty() {
        errs.push(CutError {
            kind: CutErrorKind::ParseError(ParseError::RequiredBlockMissing(CutBlockKind::Exit)),
            span: NO_SPAN,
        });
    } else if exit_blocks.len() > 1 {
        for (_, span) in exit_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Exit)),
                span: span.clone(),
            });
        }
    }

    let exit_block_span = &exit_blocks.first().unwrap().0.1;

    // Optional

    let env_blocks = extract_codeblocks(source, ENV_BLOCK);

    if env_blocks.len() > 1 {
        for (_, span) in env_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Env)),
                span: span.clone(),
            });
        }
    }

    let stdin_blocks = extract_codeblocks(source, STDIN_BLOCK);

    if stdin_blocks.len() > 1 {
        for (_, span) in stdin_blocks.iter().skip(1) {
            errs.push(CutError {
                kind: CutErrorKind::ParseError(ParseError::TooManyOfBlock(CutBlockKind::Stdin)),
                span: span.clone(),
            });
        }
    }

    if !errs.is_empty() {
        return Err(CutError {
            kind: CutErrorKind::Aggregate(errs),
            span: NO_SPAN,
        });
    }

    Ok(ParsedCutFile {
        frontmatter,
        shell: CutBlockFromSource::new(CutBlockKind::Shell, shell_block_span),
        stdout: CutBlockFromSource::new(CutBlockKind::Stdout, stdout_block_span),
        stderr: CutBlockFromSource::new(CutBlockKind::Stderr, stderr_block_span),
        exit: CutBlockFromSource::new(CutBlockKind::Exit, exit_block_span),
    })
}
