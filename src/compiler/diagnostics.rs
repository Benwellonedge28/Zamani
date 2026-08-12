#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Compiler — Advanced Diagnostic Engine

use crate::source_map::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: String,
    pub message: String,
    pub span: Option<Span>,
    pub suggestion: Option<String>,
}

pub struct DiagnosticEngine {
    pub diagnostics: Vec<Diagnostic>,
    pub has_errors: bool,
}

impl DiagnosticEngine {
    pub fn new() -> Self {
        DiagnosticEngine {
            diagnostics: Vec::new(),
            has_errors: false,
        }
    }

    pub fn emit(&mut self, level: DiagnosticLevel, code: impl Into<String>, message: impl Into<String>, span: Option<Span>, suggestion: Option<String>) {
        if level == DiagnosticLevel::Error {
            self.has_errors = true;
        }
        self.diagnostics.push(Diagnostic {
            level,
            code: code.into(),
            message: message.into(),
            span,
            suggestion,
        });
    }

    pub fn render_report(&self) {
        println!("\n[Diagnostics] Rendering compiler diagnostic report...");
        for d in &self.diagnostics {
            let prefix = match d.level {
                DiagnosticLevel::Error => "\x1b[31merror[E" ,
                DiagnosticLevel::Warning => "\x1b[33mwarning[W",
                DiagnosticLevel::Note => "\x1b[36mnote[N",
                DiagnosticLevel::Help => "\x1b[32mhelp[H",
            };
            println!("{}{}]\x1b[00m: {}", prefix, d.code, d.message);
            if let Some(sug) = &d.suggestion {
                println!("  = \x1b[32msuggestion\x1b[00m: {}", sug);
            }
        }
        println!("[Diagnostics] Total diagnostics emitted: {} (Has Errors: {})", self.diagnostics.len(), self.has_errors);
    }
}
