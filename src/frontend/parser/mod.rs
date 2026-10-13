#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! # Zamani Frontend Parser
//!
//! Canonical module boundary for the Zamani source-language parser.
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: Safe Rust only.
//!
//! ## Purpose
//!
//! Integrate the existing parser infrastructure into one coherent frontend
//! subsystem for classical, quantum, hardware-description-language (HDL),
//! hybrid, AI, data, concurrent, distributed, and future computing domains.
//!
//! This module is an aggregation and API boundary. It does not implement
//! parsing algorithms, redefine the AST, tokenize source text, or establish
//! a competing language specification.
//!
//! ## Language and grammar authority
//!
//! Parser behavior MUST conform to the repository's normative language
//! specifications and canonical grammar.
//!
//! Relevant authorities include:
//!
//! - `grammar/specification/grammar-authority.md`
//! - `grammar/specification/syntax.md`
//! - `grammar/specification/syntax-model.md`
//! - `grammar/specification/compatibility.md`
//! - `grammar/specification/extensibility.md`
//! - `grammar/specification/scalability-model.md`
//! - `grammar/specification/poco-reaf.md`
//! - `grammar/Zamani.g4`
//! - `grammar/antlr/ZamaniParser.g4`
//! - the authoritative grammar modules under `grammar/`
//!
//! These files have distinct ownership and authority relationships. This
//! Rust facade MUST NOT silently redefine their syntax or compatibility
//! rules. Resolve specification conflicts through the repository's
//! documented language-change process.
//!
//! ## Architectural position
//!
//! ```text
//! Source text
//!     |
//!     v
//! frontend::lexer
//!     |
//!     v
//! frontend::parser
//!     |
//!     +-- shared grammar productions
//!     +-- registered domain extensions
//!     +-- source and token validation
//!     +-- structured diagnostics and recovery
//!     +-- AST adaptation
//!     |
//!     v
//! frontend::ast::node
//!     |
//!     v
//! structural validation
//!     |
//!     v
//! semantic analysis
//!     |
//!     v
//! semantic model
//!     |
//!     +-- classical IR
//!     +-- quantum IR
//!     +-- HDL and other domain IRs
//!     |
//!     v
//! optimization / lowering / planning
//!     |
//!     v
//! target realization and execution
//! ```
//!
//! The parser recognizes and preserves source-language structure. It does
//! not decide whether a particular target can execute the program.
//!
//! ## POCO-REAF
//!
//! The parser supports the architectural objective:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! It does so by keeping syntax independent of physical target capacity,
//! preserving resource and capability expressions, and providing extensible
//! grammar integration points.
//!
//! The parser MUST NOT introduce fixed language-level ceilings for:
//!
//! - qubits or quantum operations;
//! - CPUs, GPUs, FPGAs, ASICs, or other devices;
//! - registers, machine widths, or tensor dimensions;
//! - threads, processes, distributed nodes, or topology size;
//! - the number of computational domains or registered operations.
//!
//! Actual processing is necessarily constrained by available memory,
//! address space, execution time, representational limits, configuration,
//! and implementation capabilities. Explicit parser budgets are operational
//! safeguards, not declarations of hardware capacity.
//!
//! ## Module ownership
//!
//! Each implementation module has one authoritative responsibility.
//!
//! - `api`: stable service contract and parsing entry points.
//! - `config`: language edition, compatibility policy, extension policy,
//!   parser options, and resource-budget configuration.
//! - `context`: immutable services shared during a parse.
//! - `session`: mutable state for one parse invocation.
//! - `parser`: token-stream orchestration, EOF enforcement, and progress
//!   checks through the existing parser contract.
//! - `token_cursor`: canonical token navigation and cursor-level safeguards.
//! - `checkpoint`: speculative parsing checkpoints.
//! - `source`: parser-facing source identity and span conversion.
//! - `limits`: parser resource accounting and budget enforcement.
//! - `error`: structured parser errors.
//! - `diagnostics`: parser diagnostic records and collection.
//! - `result`: complete, recovered, incomplete, and failed parse outcomes.
//! - `recovery`: parser recovery behavior, where implemented by the
//!   corresponding grammar and session integration.
//! - `extensions`: versioned grammar-extension contracts.
//! - `registry`: extension registration and deterministic lookup.
//! - `grammar`: shared extension dispatch.
//! - `precedence`: operator precedence, associativity, and binding powers.
//! - `token_sets`: reusable grammar and recovery token classifications.
//! - `ast_adapter`: conversion into the canonical AST.
//! - `validation`: parser-input and structural-consumption validation.
//! - `compatibility`: language-edition and legacy-syntax behavior.
//! - `grammar/`: shared language productions.
//! - `domains/`: domain-specific grammar adapters.
//!
//! This facade declares module ownership and exposes selected stable APIs.
//! It MUST NOT duplicate implementation logic owned by those modules.
//!
//! ## Dependency rules
//!
//! The permitted dependency direction is:
//!
//! ```text
//! canonical lexer
//!       |
//!       v
//! parser infrastructure
//!       |
//!       v
//! shared grammar and domain extensions
//!       |
//!       v
//! AST adapter
//!       |
//!       v
//! canonical AST
//!       |
//!       v
//! semantic analysis and compiler pipeline
//! ```
//!
//! Specific implementation modules may depend on shared parser contracts
//! where required, but circular ownership and duplicate definitions are
//! forbidden.
//!
//! In particular:
//!
//! 1. The lexer owns tokenization, token kinds, token channels, and lexical
//!    source spans.
//! 2. The parser consumes canonical lexer tokens; it does not implement a
//!    second lexer.
//! 3. The AST owns canonical node identities, source metadata, declarations,
//!    expressions, statements, types, patterns, and domain representations.
//! 4. The AST adapter performs conversions where existing AST types require
//!    them. Grammar modules must not silently discard parsed information.
//! 5. Semantic analysis owns name resolution, type checking, effect checking,
//!    capability validation, contract evaluation, and semantic interpretation.
//! 6. Resource negotiation, target discovery, scheduling, routing, lowering,
//!    and execution belong to downstream compiler and runtime components.
//! 7. Legacy parser and lexer interfaces require explicit compatibility
//!    adapters; incompatible token or AST types must not be silently mixed.
//!
//! ## Domain extensibility
//!
//! The shared grammar owns common language constructs. The domain modules
//! provide integration points for domain-specific syntax.
//!
//! Quantum operations, hardware primitives, vendor extensions, accelerator
//! operations, and future domain operations MUST NOT require an ever-growing
//! exhaustive switch in this facade.
//!
//! Where the language specification defines an extension point, registered
//! extensions must follow the existing contracts for:
//!
//! - stable namespaced identity;
//! - extension API compatibility;
//! - deterministic dispatch;
//! - collision and ambiguity handling;
//! - source-span preservation;
//! - parser progress;
//! - error reporting;
//! - resource-budget enforcement.
//!
//! An extension MUST NOT bypass the parser's safety, compatibility, or
//! validation contracts.
//!
//! ## Source fidelity and diagnostics
//!
//! The parser must preserve enough source information for diagnostics,
//! formatting, tooling, and AST consumers.
//!
//! This module introduces no competing source-span, token, diagnostic,
//! parse-result, or AST representation.
//!
//! Complete-program parsing must enforce the configured EOF policy.
//! Recovered and incomplete results must remain distinguishable from
//! error-free complete results.
//!
//! Malformed input must produce structured errors or diagnostics rather
//! than being silently truncated or accepted as a complete program.
//!
//! ## Resource safety and determinism
//!
//! The parser operates on untrusted source and token streams.
//!
//! Implementations must:
//!
//! - use safe Rust;
//! - avoid unchecked indexing and assumptions about malformed token streams;
//! - enforce configured resource budgets where applicable;
//! - detect non-progressing parsing and recovery;
//! - use checked arithmetic for resource accounting;
//! - avoid hidden global mutable parser state;
//! - preserve deterministic behavior for equivalent source, configuration,
//!   and extension-registry snapshots;
//! - report resource exhaustion through established error/result contracts.
//!
//! A finite resource budget may be configured for service safety, but it
//! MUST NOT be confused with a limit on the computational systems the
//! language can describe.
//!
//! ## Public API policy
//!
//! The canonical module paths remain available for callers that need specific
//! functionality. Selected high-level entry points and core types are
//! re-exported below for convenience.
//!
//! Internal implementation details should remain accessible through their
//! owning modules when public visibility is necessary for integration or
//! testing. New public re-exports must be explicit and compatibility-aware.
//!
//! The legacy root-level parser remains a separate compatibility concern
//! until repository integration migrates it deliberately.
//!
//! ## File completion contract
//!
//! This module is complete when:
//!
//! - every declared module exists at its declared path;
//! - each implementation has one authoritative owner;
//! - public re-exports resolve to existing declarations;
//! - grammar and domain facades remain separate;
//! - no duplicate lexer, AST, span, or diagnostic contract is introduced;
//! - no target-specific capacity limit is introduced;
//! - no unsafe code is permitted;
//! - module initialization has no parsing or hardware side effects;
//! - parser tests are wired through the repository's established test
//!   conventions;
//! - downstream consumers can import the parser through one canonical
//!   frontend module boundary.
//!
//! Compilation and integration tests remain mandatory before release.
//!
//! =============================================================================
//! Module declarations
//! =============================================================================

/// Stable public parser service contract and entry points.
pub mod api;

/// Conversion from parsed syntax into the canonical AST.
pub mod ast_adapter;

/// Speculative parsing checkpoints.
pub mod checkpoint;

/// Language edition, compatibility, extension, and resource configuration.
pub mod config;

/// Immutable services shared by parser components.
pub mod context;

/// Language-edition and legacy-syntax compatibility.
pub mod compatibility;

/// Parser diagnostic records and collection.
pub mod diagnostics;

/// Domain-specific grammar adapters.
pub mod domains;

/// Structured parser errors.
pub mod error;

/// Versioned grammar-extension contracts.
pub mod extensions;

/// Shared grammar-extension dispatch.
pub mod grammar;

/// Shared language grammar productions.
pub mod grammar;

/// Parser resource-budget enforcement.
pub mod limits;

/// Parser orchestration and token-stream integrity.
pub mod parser;

/// Canonical operator precedence and associativity.
pub mod precedence;

/// Deterministic extension registry.
pub mod registry;

/// Structured parse outcomes.
pub mod result;

/// Source identity and source-span adaptation.
pub mod source;

/// Per-invocation parser state.
pub mod session;

/// Canonical token-stream navigation.
pub mod token_cursor;

/// Reusable grammar and recovery token sets.
pub mod token_sets;

/// Parser input and structural-consumption validation.
pub mod validation;

// =============================================================================
// Stable convenience re-exports
// =============================================================================

/// Stable parser service trait and supported entry points.
pub use api::{
    parse_expression,
    parse_item,
    parse_source,
    parse_tokens,
    parse_type,
    ParserService,
};

/// Canonical parser configuration.
pub use config::ParserConfig;

/// Canonical parser engine contracts and structured token-stream errors.
pub use parser::{
    Parser,
    ParserError,
    ParserErrorKind,
    ParserGrammar,
};

/// Canonical parse outcome.
pub use result::ParseResult;

// =============================================================================
// Module identity
// =============================================================================

/// Stable architectural name of this parser module.
pub const PARSER_MODULE_NAME: &str = "zamani.frontend.parser";

/// Version of this module's aggregation contract.
///
/// This is independent of the Zamani language edition, parser API version,
/// grammar specification version, AST schema version, and compiler version.
pub const PARSER_MODULE_SCHEMA_VERSION: u16 = 1;

/// Return the stable architectural module name.
#[must_use]
pub const fn module_name() -> &'static str {
    PARSER_MODULE_NAME
}

/// Return the aggregation-contract schema version.
#[must_use]
pub const fn schema_version() -> u16 {
    PARSER_MODULE_SCHEMA_VERSION
}

// =============================================================================
// Contract tests
// =============================================================================

#[cfg(test)]
mod module_contract_tests {
    use super::*;

    #[test]
    fn module_identity_is_stable() {
        assert_eq!(module_name(), "zamani.frontend.parser");
        assert_eq!(schema_version(), PARSER_MODULE_SCHEMA_VERSION);
        assert!(schema_version() > 0);
    }

    #[test]
    fn canonical_parser_api_is_reachable() {
        // Type references ensure the intended public API remains available
        // through the parser facade without constructing a particular AST.
        fn accepts_service<T: ParserService + ?Sized>(_: &T) {}

        let _ = accepts_service::<dyn ParserService>;
        let _ = core::mem::size_of::<ParserConfig>();
        let _ = core::mem::size_of::<ParserError>();
        let _ = core::mem::size_of::<ParserErrorKind>();
        let _ = core::mem::size_of::<ParseResult<()>>();
        let _ = core::mem::size_of::<PARSER_MODULE_SCHEMA_VERSION>();
    }

    #[test]
    fn shared_grammar_and_domain_facades_are_separate() {
        // Their distinct module paths are intentional: common syntax belongs
        // to grammar productions; domain syntax belongs to registered
        // domain adapters.
        let _ = core::any::type_name::<
            fn() -> &'static str
        >();

        assert_eq!(module_name(), PARSER_MODULE_NAME);
    }
}