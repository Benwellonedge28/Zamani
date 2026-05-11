
//! Zenith Universal Meta-Compiler (UMC) Error Reporting System
//!
//! This module centralizes and standardizes the error reporting mechanism
//! for the entire Zenith compiler. It defines a unified error type, provides
//! rich formatting capabilities, and aims to offer contextual information,
//! severity levels, and suggested fixes to enhance the developer experience.

use crate::source_map::{Span, SourceMap}; // Import SourceMap for error location
use std::fmt;

// Re-export specific error types for convenience in other modules
pub use crate::lexer::LexerError;
pub use crate::parser::ParserError;
pub use crate::semantic::SemanticError;
pub use crate::ir_gen::IrGenError;
pub use crate::optimizer::OptimizerError;
pub use crate::backend::BackendError;


/// Enum representing the severity level of a compiler diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Note,
    Warning,
    Error,
    Fatal,
}

/// A unified error type that can encapsulate diagnostics from any compiler stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
    Semantic(SemanticError),
    IrGen(IrGenError),
    Optimizer(OptimizerError),
    Backend(BackendError),
    // Add other general compiler errors here
    Internal(String, Span), // For unexpected internal compiler errors
    Generic(String, Span, Severity), // For general purpose errors
}

impl CompilerError {
    /// Returns the source span associated with the error.
    pub fn span(&self) -> Span {
        match self {
            CompilerError::Lexer(e) => e.span.clone(),
            CompilerError::Parser(e) => e.span.clone(),
            CompilerError::Semantic(e) => e.span.clone(),
            CompilerError::IrGen(e) => e.span.clone(),
            CompilerError::Optimizer(e) => e.span.clone(),
            CompilerError::Backend(e) => e.span.clone(),
            CompilerError::Internal(_, span) => span.clone(),
            CompilerError::Generic(_, span, _) => span.clone(),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> String {
        match self {
            CompilerError::Lexer(e) => e.message.clone(),
            CompilerError::Parser(e) => e.message.clone(),
            CompilerError::Semantic(e) => e.message.clone(),
            CompilerError::IrGen(e) => e.message.clone(),
            CompilerError::Optimizer(e) => e.message.clone(),
            CompilerError::Backend(e) => e.message.clone(),
            CompilerError::Internal(msg, _) => format!("Internal Compiler Error: {}", msg),
            CompilerError::Generic(msg, _, _) => msg.clone(),
        }
    }

    /// Returns the severity of the error.
    pub fn severity(&self) -> Severity {
        match self {
            CompilerError::Lexer(_) | CompilerError::Parser(_) | CompilerError::Semantic(_) |
            CompilerError::IrGen(_) | CompilerError::Optimizer(_) | CompilerError::Backend(_) |
            CompilerError::Internal(_, _) => Severity::Error, // Default to Error for stage-specific
            CompilerError::Generic(_, _, sev) => sev.clone(),
        }
    }

    /// Generates a structured diagnostic report for the error.
    /// Now takes a &SourceMap for contextual information.
    pub fn report(&self, source_map: &SourceMap) -> String { // Modified signature
        let span = self.span();
        let message = self.message();
        let severity = self.severity();

        let mut report = String::new();
        report.push_str(&format!("{:?} [{}] at {}:{}:{}
",
            severity,
            "Z0000", // Placeholder for actual error code
            source_map.get_file(span.file_id).map_or("<unknown>".to_string(), |sf| sf.name.clone()),
            span.start_line,
            span.start_column,
        ));
        report.push_str(&format!("  {}
", message));

        // Attempt to get the source line and highlight the error
        if let Some(source_line) = source_map.get_source_line(span.file_id, span.start_line) {
            let line_num_str = span.start_line.to_string();
            let start_col = span.start_column.saturating_sub(1) as usize; // 0-indexed
            let end_col = span.end.column.saturating_sub(1) as usize; // 0-indexed
            let highlight_len = end_col.saturating_sub(start_col); // Length in chars

            report.push_str(&format!("{:>width$} | {}
", line_num_str, source_line, width = line_num_str.len() + 1));
            report.push_str(&format!("{:>width$} | {}{}^
",
                " ",
                " ".repeat(start_col),
                if highlight_len > 0 { "~".repeat(highlight_len.saturating_sub(1)) } else { "".to_string() }, // Use ~ for range, ^ for single char
                width = line_num_str.len() + 1
            ));
        }
        
        // Add suggestions/notes if available (conceptual)
        match self {
            CompilerError::Semantic(SemanticError { message, span: _ }) if message.contains("Unresolved identifier") => {
                report.push_str("  Hint: Did you mean to declare this variable or import a module?\n");
            }
            CompilerError::Semantic(SemanticError { message, span: _ }) if message.contains("Mismatched types") => {
                report.push_str("  Hint: Check the expected type and the type of the expression.\n");
            }
            _ => {}
        }

        report
    }
}

// ... (From implementations and Diagnostics struct remain the same) ...
