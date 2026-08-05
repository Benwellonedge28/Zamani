//! Zamani UMC Error Reporting System
//!
//! Centralised diagnostics for all compiler phases.
//! Extended error types (parser, semantic, ir_gen, etc.) will be added
//! as those modules are progressively enabled.

use crate::lexer::LexerError;
use crate::source_map::{SourceMap, Span};
use std::fmt;

/// Severity level of a compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Note,
    Warning,
    Error,
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Note => write!(f, "note"),
            Severity::Warning => write!(f, "warning"),
            Severity::Error => write!(f, "error"),
            Severity::Fatal => write!(f, "fatal error"),
        }
    }
}

/// A unified compiler diagnostic.
#[derive(Debug, Clone)]
pub struct CompilerError {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub span: Option<Span>,
    pub hint: Option<String>,
}

impl CompilerError {
    pub fn error(message: impl Into<String>, span: Option<Span>) -> Self {
        CompilerError {
            severity: Severity::Error,
            code: None,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn warning(message: impl Into<String>, span: Option<Span>) -> Self {
        CompilerError {
            severity: Severity::Warning,
            code: None,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.severity, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\n  hint: {}", hint)?;
        }
        Ok(())
    }
}

/// Convert a lexer error into a unified CompilerError.
impl From<LexerError> for CompilerError {
    fn from(e: LexerError) -> Self {
        CompilerError::error(e.message, Some(e.span))
    }
}

/// Report all errors to stderr in a human-readable format.
pub fn report_errors(errors: &[CompilerError], _source_map: &SourceMap) {
    for err in errors {
        eprintln!("{}", err);
    }
}
