#![forbid(unsafe_code)]

//! Zamani frontend lexical subsystem.
//!
//! File: `src/frontend/lexer/mod.rs`
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Assemble the canonical frontend lexer from independently maintained
//! modules and expose a stable, target-independent public interface.
//!
//! This module is the integration boundary for lexical recognition. It
//! does not implement scanning itself and does not determine the semantic
//! meaning of classical, quantum, HDL, distributed, AI, tensor, networking,
//! or other language constructs.
//!
//! # Architecture
//!
//! ```text
//! Source text
//!     |
//!     v
//! api.rs -------------------- Public streaming interface
//!     |
//!     v
//! scanner.rs ---------------- Token production and coordination
//!     |
//!     +-- cursor.rs ---------- UTF-8-safe source traversal
//!     +-- identifier.rs ----- Identifier recognition
//!     +-- number.rs ---------- Numeric literal recognition
//!     +-- string.rs ---------- String literal recognition
//!     +-- character.rs ------- Character literal recognition
//!     +-- comment.rs --------- Comment recognition
//!     +-- quantum.rs --------- Quantum-state literal recognition
//!     +-- annotation.rs ------ Annotation-marker recognition
//!     +-- whitespace.rs ------ Whitespace recognition
//!     |
//!     +-- keyword_registry.rs Keyword classification
//!     +-- operator_registry.rs Operator matching
//!     +-- dialect.rs --------- Explicit lexical extensions
//!     +-- token_kind.rs ------ Canonical token identities
//!     +-- token_metadata.rs -- Token metadata and lifecycle
//!     +-- token.rs ----------- Token representation
//!     +-- span.rs ------------ Source spans
//!     +-- diagnostics.rs ----- Structured diagnostics
//!     +-- recovery.rs -------- Lexical recovery primitives
//!     +-- config.rs ---------- Configuration and policy
//!     +-- limits.rs ---------- Resource-budget enforcement
//!     |
//!     v
//! Parser and frontend consumers
//!     |
//!     v
//! AST -> semantic validation -> canonical IR -> lowering -> realization
//! ```
//!
//! # Ownership
//!
//! This module owns only:
//!
//! - the lexer module tree;
//! - the stable re-exports defined below;
//! - the test-module registration;
//! - the documented boundary between lexical and downstream layers.
//!
//! It does not own:
//!
//! - lexical recognition algorithms;
//! - keyword or operator definitions;
//! - the normative lexical specification;
//! - parser grammar or AST construction;
//! - semantic analysis or type checking;
//! - quantum-operation catalogs or quantum execution;
//! - HDL elaboration or synthesis;
//! - target discovery, hardware allocation, routing, or scheduling;
//! - runtime execution or deployment.
//!
//! # Normative authority
//!
//! Lexical behavior must conform to the repository's authoritative
//! language specification and canonical grammar contracts, including:
//!
//! - `grammar/specification/lexical.md`;
//! - `grammar/spec/lexical.md`;
//! - `grammar/Zamani.g4`;
//! - `grammar/antlr/ZamaniLexer.g4`;
//! - the applicable contracts in `grammar/lexer/`.
//!
//! This Rust module does not override those sources. Where the specifications
//! conflict, resolve the conflict through the repository's language-change
//! and conformance process rather than silently changing behavior here.
//!
//! # Portability and scalability
//!
//! The lexer expresses lexical meaning independently of the machine that
//! will eventually execute the program. It must not encode hardware
//! capacities, vendor catalogs, qubit counts, processor counts, device
//! counts, tensor dimensions, or deployment topology as language limits.
//!
//! Configurable implementation budgets belong in `config.rs` and
//! `limits.rs`. An explicit budget is a processing policy, not a declaration
//! that the language or target architecture has a fixed capacity.
//!
//! Streaming consumers should use `Lexer` or `TokenSource` where suitable.
//! Collecting a complete stream necessarily consumes memory proportional
//! to the collected tokens. No finite implementation can promise unlimited
//! physical memory, execution time, address space, or input size.
//!
//! # Integration contract
//!
//! 1. `token_kind.rs` owns canonical token identities.
//! 2. `token.rs` owns token values and channels.
//! 3. `span.rs` owns source-span representation.
//! 4. `token_metadata.rs` owns token metadata and lifecycle information.
//! 5. `config.rs` owns lexer configuration and lexical policies.
//! 6. `limits.rs` owns reusable resource-budget enforcement primitives.
//! 7. `cursor.rs` owns source traversal and coordinate tracking.
//! 8. Recognition modules identify lexical forms without constructing
//!    AST nodes or assigning semantic meaning.
//! 9. Registries classify identifiers and match configured operators.
//! 10. `dialect.rs` resolves explicitly activated lexical extensions.
//! 11. `scanner.rs` coordinates recognition and implements the token-source
//!     contract defined by `api.rs`.
//! 12. `diagnostics.rs` owns structured diagnostic records.
//! 13. `recovery.rs` provides recovery primitives; the scanner decides
//!     when and how to apply them according to configuration.
//! 14. Parser and semantic consumers must not duplicate lexical rules.
//! 15. Legacy lexer/parser consumers must be migrated through an explicit
//!     compatibility adapter rather than an implicit, lossy conversion.
//!
//! # Module visibility
//!
//! Implementation modules are public so compiler components, tests, and
//! tooling can depend on the narrowest appropriate interface. The stable
//! convenience re-exports below are the preferred starting point for
//! ordinary lexer consumers.
//!
//! A public module is not a promise that every internal type or method is
//! permanently stable. Compatibility-sensitive API changes still require
//! documentation, tests, and a deliberate migration plan.

/// Public token-source interface and streaming lexer facade.
pub mod api;

/// Configuration, lexical policies, and resource-budget definitions.
pub mod config;

/// UTF-8-safe cursor and source-coordinate tracking.
pub mod cursor;

/// Lexical recognition of annotation markers.
pub mod annotation;

/// Lexical recognition of character literals.
pub mod character;

/// Lexical recognition of comments and documentation comments.
pub mod comment;

/// Structured lexical diagnostics and diagnostic reporting.
pub mod diagnostics;

/// Versioned, data-driven lexical dialect registration and resolution.
pub mod dialect;

/// Unicode-aware identifier recognition.
pub mod identifier;

/// Data-driven keyword registry.
pub mod keyword_registry;

/// Reusable resource-budget checks and accounting.
pub mod limits;

/// Numeric literal recognition.
pub mod number;

/// Data-driven operator registry and maximal-munch matching.
pub mod operator_registry;

/// Quantum-state literal recognition.
pub mod quantum;

/// Deterministic lexical recovery and synchronization primitives.
pub mod recovery;

/// Streaming scanner coordinating lexical recognition.
pub mod scanner;

/// String literal recognition.
pub mod string;

/// Canonical source spans and source-range operations.
pub mod span;

/// Canonical token representation and token channels.
pub mod token;

/// Canonical token identities and broad lexical categories.
pub mod token_kind;

/// Token metadata, stability, and provenance.
pub mod token_metadata;

/// Whitespace recognition.
pub mod whitespace;

// ---------------------------------------------------------------------------
// Stable convenience re-exports
// ---------------------------------------------------------------------------

/// Streaming facade for consumers that do not need scanner internals.
pub use api::{collect_tokens, Lexer, TokenSource};

/// Canonical configuration and resource-budget policy types.
pub use config::{LexerConfig, LexerConfigBuilder, LexerLimits, ResourceLimit};

/// Canonical source cursor and source positions.
pub use cursor::{Cursor, CursorPosition};

/// Canonical lexical diagnostics.
pub use diagnostics::{
    DiagnosticSeverity,
    LexicalDiagnostic,
};

/// Canonical token representation and channel.
pub use token::{Token, TokenChannel};

/// Canonical token identities and their broad categories.
pub use token_kind::{TokenCategory, TokenKind};

/// Canonical source span.
pub use span::Span;

/// Production scanner and its structured errors.
pub use scanner::{Scanner, ScannerError, ScannerErrorKind};

// ---------------------------------------------------------------------------
// Test registration
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;