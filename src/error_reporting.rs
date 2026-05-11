
//! Zenith Universal Meta-Compiler (UMC) Error Reporting System
//!
//! This module centralizes and standardizes the error reporting mechanism
//! for the entire Zenith compiler. It defines a unified error type, provides
//! rich formatting capabilities, and aims to offer contextual information,
//! severity levels, and suggested fixes to enhance the developer experience.

use crate::source_map::Span; // Import Span for error location
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
    pub fn report(&self, source_code: &str) -> String {
        let span = self.span();
        let message = self.message();
        let severity = self.severity();

        // Conceptual: Load line from source_code using span.start.line
        let line_num = span.start.line as usize;
        let line_content = source_code.lines().nth(line_num - 1).unwrap_or("");
        
        // Adjust column to be 0-indexed for tilde pointer
        let start_col = span.start.column - 1;
        let end_col = span.end.column - 1;
        let num_chars_to_highlight = if end_col > start_col { end_col - start_col } else { 1 };

        let mut report = String::new();
        report.push_str(&format!("{:?} [{}] at Line {}:{}
",
            severity,
            // Add a unique error code later (e.g., Z0001 for LexerError, Z0101 for ParserError)
            "Z0000",
            line_num,
            span.start.column,
        ));
        report.push_str(&format!("  {}
", message));
        report.push_str(&format!("{:>4} | {}
", line_num, line_content));
        // Add a pointer to the exact location in the line
        report.push_str(&format!("{:>4} | {}{}^
", " ", " ".repeat(start_col as usize), "~".repeat(num_chars_to_highlight as usize - 1)));
        
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

// --- Implement From traits to easily convert specific errors into CompilerError ---
impl From<LexerError> for CompilerError {
    fn from(err: LexerError) -> Self { CompilerError::Lexer(err) }
}
impl From<ParserError> for CompilerError {
    fn from(err: ParserError) -> Self { CompilerError::Parser(err) }
}
impl From<SemanticError> for CompilerError {
    fn from(err: SemanticError) -> Self { CompilerError::Semantic(err) }
}
impl From<IrGenError> for CompilerError {
    fn from(err: IrGenError) -> Self { CompilerError::IrGen(err) }
}
impl From<OptimizerError> for CompilerError {
    fn from(err: OptimizerError) -> Self { CompilerError::Optimizer(err) }
}
impl From<BackendError> for CompilerError {
    fn from(err: BackendError) -> Self { CompilerError::Backend(err) }
}

// A collection of errors, useful for returning multiple diagnostics.
pub struct Diagnostics(pub Vec<CompilerError>);

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for err in &self.0 {
            // Placeholder: A real implementation would pass the source code to err.report()
            writeln!(f, "{:?}: {}", err.severity(), err.message())?; // This won't show source code. Report() needs source_code.
        }
        Ok(())
    }
}
