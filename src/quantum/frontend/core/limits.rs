//! Frontend resource and security limits.
//!
//! This module defines resource limits for the quantum frontend boundary.
//!
//! # Architectural boundary
//!
//! `FrontendLimits` protects the frontend from malformed, malicious, or
//! pathological external input. It is intentionally separate from the
//! limits owned by `quantum::ir`.
//!
//! ```text
//! External source
//!      |
//!      v
//! +-----------------------+
//! | FrontendLimits         |
//! | source/token/AST/etc.  |
//! +-----------------------+
//!      |
//!      v
//! lexer -> parser -> validation -> lowering
//!                                      |
//!                                      v
//!                              quantum::ir limits
//! ```
//!
//! The frontend limits answer:
//!
//! > "How much input/complexity are we willing to process?"
//!
//! The Quantum IR limits answer:
//!
//! > "How large/complex may a valid canonical quantum circuit be?"
//!
//! These policies must not be merged.
//!
//! # Format independence
//!
//! This module contains no knowledge of OpenQASM, QIR, Quil, or any other
//! quantum format. A future frontend format must be able to consume
//! `FrontendLimits` without changing this file.
//!
//! # Security properties
//!
//! The limits are:
//!
//! - finite;
//! - deterministic;
//! - explicit;
//! - overflow-safe;
//! - format-independent;
//! - suitable for untrusted input;
//! - immutable once constructed;
//! - free from I/O and global state.
//!
//! No parser should silently replace an exhausted limit with an unlimited
//! value.
//!
//! # Rust compatibility
//!
//! This implementation intentionally uses stable Rust APIs compatible with
//! Rust 1.97.1 and the repository's Rust 2021 edition.

use core::fmt;

/// Default maximum source size in bytes.
///
/// 16 MiB is large enough for ordinary quantum programs while preventing an
/// accidentally or deliberately enormous source document from being treated
/// as normal input.
pub const DEFAULT_MAX_SOURCE_BYTES: u64 = 16 * 1024 * 1024;

/// Default maximum number of lexer tokens.
///
/// This is deliberately independent from source-byte size because a small
/// source can still contain a very large number of tokens.
pub const DEFAULT_MAX_TOKENS: u64 = 1_000_000;

/// Default maximum identifier length in bytes.
///
/// The frontend should measure identifiers according to the representation
/// used by its source model. UTF-8 byte length is the safest common limit for
/// an input boundary.
pub const DEFAULT_MAX_IDENTIFIER_LENGTH: u64 = 4 * 1024;

/// Default maximum number of AST nodes.
///
/// This protects parsers from pathological programs that are compact in
/// source representation but produce enormous syntax trees.
pub const DEFAULT_MAX_AST_NODES: u64 = 2_000_000;

/// Default maximum parser nesting depth.
///
/// This protects recursive-descent and recursive semantic processing from
/// stack exhaustion.
pub const DEFAULT_MAX_NESTING_DEPTH: u64 = 512;

/// Default maximum expression nesting depth.
pub const DEFAULT_MAX_EXPRESSION_DEPTH: u64 = 256;

/// Default maximum number of diagnostics retained for one frontend operation.
///
/// Once this limit is reached, callers should stop accumulating additional
/// diagnostics and report the canonical diagnostic-limit condition.
pub const DEFAULT_MAX_DIAGNOSTICS: u64 = 1_000;

/// Default maximum include/import nesting depth.
///
/// The limit applies to source inclusion mechanisms such as OpenQASM
/// `include`, but the generic policy deliberately does not mention any
/// particular format.
pub const DEFAULT_MAX_INCLUDE_DEPTH: u64 = 64;

/// Default maximum number of user-defined gate/function-like declarations.
///
/// This is a frontend complexity limit, not a Quantum IR resource limit.
pub const DEFAULT_MAX_GATE_DEFINITIONS: u64 = 100_000;

/// Default maximum number of operations contained in one source-level
/// gate/function-like definition.
pub const DEFAULT_MAX_GATE_OPERATIONS: u64 = 1_000_000;

/// Default maximum logical register size.
///
/// This protects source-level declarations before they are lowered to the
/// canonical IR.
pub const DEFAULT_MAX_REGISTER_SIZE: u64 = 1_000_000;

/// Default maximum string literal length in bytes.
pub const DEFAULT_MAX_STRING_LENGTH: u64 = 1 * 1024 * 1024;

/// Default maximum numeric literal length in bytes.
///
/// This limits lexical input before arbitrary-precision or floating-point
/// parsing is attempted.
pub const DEFAULT_MAX_NUMERIC_LITERAL_LENGTH: u64 = 4 * 1024;

/// Default maximum number of source files.
///
/// A future implementation may use this for include/import graphs. Keeping
/// it here prevents an include graph from expanding without bound.
pub const DEFAULT_MAX_SOURCE_FILES: u64 = 1_024;

/// Default maximum number of include/import edges.
///
/// This is separate from include depth because a shallow but highly branching
/// source graph can still consume excessive resources.
pub const DEFAULT_MAX_INCLUDE_EDGES: u64 = 10_000;

/// Default maximum number of classical/semantic expression nodes.
///
/// This is separate from AST node count because semantic expression
/// expansion may create additional work after parsing.
pub const DEFAULT_MAX_EXPRESSION_NODES: u64 = 1_000_000;

/// Default maximum number of symbols in one frontend symbol environment.
pub const DEFAULT_MAX_SYMBOLS: u64 = 1_000_000;

/// Default maximum number of parameters in one source-level declaration or
/// callable entity.
pub const DEFAULT_MAX_PARAMETERS: u64 = 1_024;

/// Default maximum number of operands in one source-level operation.
pub const DEFAULT_MAX_OPERANDS: u64 = 1_024;

/// Default maximum number of statements in one source-level block.
pub const DEFAULT_MAX_STATEMENTS_PER_BLOCK: u64 = 1_000_000;

/// Default maximum total source-level statements.
pub const DEFAULT_MAX_STATEMENTS: u64 = 2_000_000;

/// Default maximum comment length in bytes.
///
/// Comments normally do not affect semantic compilation, but retaining
/// arbitrarily large comments can still consume memory.
pub const DEFAULT_MAX_COMMENT_LENGTH: u64 = 1 * 1024 * 1024;

/// Default maximum annotation length in bytes.
pub const DEFAULT_MAX_ANNOTATION_LENGTH: u64 = 1 * 1024 * 1024;

/// Default maximum number of annotations attached to one source item.
pub const DEFAULT_MAX_ANNOTATIONS_PER_ITEM: u64 = 1_024;

/// Default maximum number of warnings/notes/help entries associated with a
/// single diagnostic.
pub const DEFAULT_MAX_DIAGNOSTIC_CHILDREN: u64 = 64;

/// Default maximum number of characters retained for diagnostic source
/// excerpts.
///
/// This controls diagnostic rendering/storage, not source acceptance.
pub const DEFAULT_MAX_DIAGNOSTIC_SNIPPET_LENGTH: u64 = 16 * 1024;

/// Default maximum total source bytes across all files in one frontend
/// operation.
///
/// This is intentionally larger than `max_source_bytes`, allowing multiple
/// reasonably sized source files while still bounding the complete operation.
pub const DEFAULT_MAX_TOTAL_SOURCE_BYTES: u64 = 64 * 1024 * 1024;

/// Configuration error produced when a `FrontendLimits` configuration is
/// internally inconsistent.
///
/// This is deliberately distinct from the runtime frontend error taxonomy.
///
/// For example:
///
/// - `max_ast_nodes = 0` is a configuration error;
/// - a parser encountering more than `max_ast_nodes` nodes is a runtime
///   `LimitExceeded` frontend error.
///
/// Keeping those concepts separate prevents `limits.rs` from depending on
/// `core::errors`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FrontendLimitConfigError {
    /// A limit that must be positive was configured as zero.
    ZeroLimit {
        /// Stable field name of the invalid limit.
        field: &'static str,
    },

    /// The total source limit is smaller than the individual source limit.
    TotalSourceBytesLessThanSingleSource {
        /// Maximum bytes permitted for one source.
        max_source_bytes: u64,

        /// Maximum bytes permitted across all sources.
        max_total_source_bytes: u64,
    },
}

impl fmt::Display for FrontendLimitConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroLimit { field } => {
                write!(formatter, "frontend limit `{field}` must be greater than zero")
            }
            Self::TotalSourceBytesLessThanSingleSource {
                max_source_bytes,
                max_total_source_bytes,
            } => write!(
                formatter,
                "frontend limit `max_total_source_bytes` ({max_total_source_bytes}) \
                 must be greater than or equal to `max_source_bytes` ({max_source_bytes})"
            ),
        }
    }
}

impl std::error::Error for FrontendLimitConfigError {}

/// Resource and security limits applied to frontend processing.
///
/// `FrontendLimits` is intentionally independent of any particular quantum
/// language or of the canonical Quantum IR.
///
/// The structure is `Copy` because it is a small immutable policy object that
/// can safely be passed by value into lexer/parser/validator configuration.
///
/// All fields are `u64` rather than `usize` so that the policy has a stable,
/// platform-independent representation. Consumers may compare their native
/// counters against these values without storing platform-dependent policy
/// state.
///
/// A limit is always finite. There is deliberately no `unlimited()` mode:
/// unbounded frontend processing is unsafe for an untrusted-input boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FrontendLimits {
    max_source_bytes: u64,
    max_total_source_bytes: u64,
    max_source_files: u64,

    max_tokens: u64,
    max_identifier_length: u64,
    max_string_length: u64,
    max_numeric_literal_length: u64,
    max_comment_length: u64,
    max_annotation_length: u64,

    max_ast_nodes: u64,
    max_nesting_depth: u64,
    max_expression_depth: u64,
    max_expression_nodes: u64,

    max_diagnostics: u64,
    max_diagnostic_children: u64,
    max_diagnostic_snippet_length: u64,

    max_include_depth: u64,
    max_include_edges: u64,

    max_gate_definitions: u64,
    max_gate_operations: u64,

    max_register_size: u64,
    max_symbols: u64,
    max_parameters: u64,
    max_operands: u64,

    max_statements_per_block: u64,
    max_statements: u64,

    max_annotations_per_item: u64,
}

impl Default for FrontendLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl FrontendLimits {
    /// Creates the standard production-safe frontend policy.
    ///
    /// The production policy is deliberately finite and conservative enough
    /// to protect applications from resource-exhaustion input while still
    /// supporting large quantum programs.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            max_source_bytes: DEFAULT_MAX_SOURCE_BYTES,
            max_total_source_bytes: DEFAULT_MAX_TOTAL_SOURCE_BYTES,
            max_source_files: DEFAULT_MAX_SOURCE_FILES,

            max_tokens: DEFAULT_MAX_TOKENS,
            max_identifier_length: DEFAULT_MAX_IDENTIFIER_LENGTH,
            max_string_length: DEFAULT_MAX_STRING_LENGTH,
            max_numeric_literal_length: DEFAULT_MAX_NUMERIC_LITERAL_LENGTH,
            max_comment_length: DEFAULT_MAX_COMMENT_LENGTH,
            max_annotation_length: DEFAULT_MAX_ANNOTATION_LENGTH,

            max_ast_nodes: DEFAULT_MAX_AST_NODES,
            max_nesting_depth: DEFAULT_MAX_NESTING_DEPTH,
            max_expression_depth: DEFAULT_MAX_EXPRESSION_DEPTH,
            max_expression_nodes: DEFAULT_MAX_EXPRESSION_NODES,

            max_diagnostics: DEFAULT_MAX_DIAGNOSTICS,
            max_diagnostic_children: DEFAULT_MAX_DIAGNOSTIC_CHILDREN,
            max_diagnostic_snippet_length: DEFAULT_MAX_DIAGNOSTIC_SNIPPET_LENGTH,

            max_include_depth: DEFAULT_MAX_INCLUDE_DEPTH,
            max_include_edges: DEFAULT_MAX_INCLUDE_EDGES,

            max_gate_definitions: DEFAULT_MAX_GATE_DEFINITIONS,
            max_gate_operations: DEFAULT_MAX_GATE_OPERATIONS,

            max_register_size: DEFAULT_MAX_REGISTER_SIZE,
            max_symbols: DEFAULT_MAX_SYMBOLS,
            max_parameters: DEFAULT_MAX_PARAMETERS,
            max_operands: DEFAULT_MAX_OPERANDS,

            max_statements_per_block: DEFAULT_MAX_STATEMENTS_PER_BLOCK,
            max_statements: DEFAULT_MAX_STATEMENTS,

            max_annotations_per_item: DEFAULT_MAX_ANNOTATIONS_PER_ITEM,
        }
    }

    /// Creates a strict policy suitable for highly constrained or
    /// adversarially exposed environments.
    ///
    /// This is intentionally deterministic and contains no environment
    /// probing or machine-specific behavior.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            max_source_bytes: 1 * 1024 * 1024,
            max_total_source_bytes: 4 * 1024 * 1024,
            max_source_files: 64,

            max_tokens: 100_000,
            max_identifier_length: 1024,
            max_string_length: 64 * 1024,
            max_numeric_literal_length: 1024,
            max_comment_length: 64 * 1024,
            max_annotation_length: 64 * 1024,

            max_ast_nodes: 250_000,
            max_nesting_depth: 128,
            max_expression_depth: 64,
            max_expression_nodes: 100_000,

            max_diagnostics: 256,
            max_diagnostic_children: 32,
            max_diagnostic_snippet_length: 4 * 1024,

            max_include_depth: 16,
            max_include_edges: 512,

            max_gate_definitions: 10_000,
            max_gate_operations: 100_000,

            max_register_size: 100_000,
            max_symbols: 100_000,
            max_parameters: 256,
            max_operands: 256,

            max_statements_per_block: 100_000,
            max_statements: 250_000,

            max_annotations_per_item: 128,
        }
    }

    /// Creates a development-oriented policy for large local programs.
    ///
    /// This remains finite. It is not an unlimited/debug escape hatch.
    #[must_use]
    pub const fn large() -> Self {
        Self {
            max_source_bytes: 256 * 1024 * 1024,
            max_total_source_bytes: 1 * 1024 * 1024 * 1024,
            max_source_files: 8_192,

            max_tokens: 10_000_000,
            max_identifier_length: 16 * 1024,
            max_string_length: 16 * 1024 * 1024,
            max_numeric_literal_length: 64 * 1024,
            max_comment_length: 16 * 1024 * 1024,
            max_annotation_length: 16 * 1024 * 1024,

            max_ast_nodes: 20_000_000,
            max_nesting_depth: 2_048,
            max_expression_depth: 1_024,
            max_expression_nodes: 10_000_000,

            max_diagnostics: 10_000,
            max_diagnostic_children: 256,
            max_diagnostic_snippet_length: 64 * 1024,

            max_include_depth: 256,
            max_include_edges: 100_000,

            max_gate_definitions: 1_000_000,
            max_gate_operations: 10_000_000,

            max_register_size: 10_000_000,
            max_symbols: 10_000_000,
            max_parameters: 4_096,
            max_operands: 4_096,

            max_statements_per_block: 10_000_000,
            max_statements: 20_000_000,

            max_annotations_per_item: 4_096,
        }
    }

    /// Validates the policy itself.
    ///
    /// Runtime input violations are handled by consumers and become
    /// `FrontendErrorKind::LimitExceeded`. This method only validates that
    /// the policy supplied by the application is coherent.
    pub const fn validate(self) -> Result<(), FrontendLimitConfigError> {
        macro_rules! require_non_zero {
            ($value:expr, $name:literal) => {
                if $value == 0 {
                    return Err(FrontendLimitConfigError::ZeroLimit {
                        field: $name,
                    });
                }
            };
        }

        require_non_zero!(self.max_source_bytes, "max_source_bytes");
        require_non_zero!(self.max_total_source_bytes, "max_total_source_bytes");
        require_non_zero!(self.max_source_files, "max_source_files");

        require_non_zero!(self.max_tokens, "max_tokens");
        require_non_zero!(
            self.max_identifier_length,
            "max_identifier_length"
        );
        require_non_zero!(self.max_string_length, "max_string_length");
        require_non_zero!(
            self.max_numeric_literal_length,
            "max_numeric_literal_length"
        );
        require_non_zero!(self.max_comment_length, "max_comment_length");
        require_non_zero!(self.max_annotation_length, "max_annotation_length");

        require_non_zero!(self.max_ast_nodes, "max_ast_nodes");
        require_non_zero!(self.max_nesting_depth, "max_nesting_depth");
        require_non_zero!(
            self.max_expression_depth,
            "max_expression_depth"
        );
        require_non_zero!(
            self.max_expression_nodes,
            "max_expression_nodes"
        );

        require_non_zero!(self.max_diagnostics, "max_diagnostics");
        require_non_zero!(
            self.max_diagnostic_children,
            "max_diagnostic_children"
        );
        require_non_zero!(
            self.max_diagnostic_snippet_length,
            "max_diagnostic_snippet_length"
        );

        require_non_zero!(self.max_include_depth, "max_include_depth");
        require_non_zero!(self.max_include_edges, "max_include_edges");

        require_non_zero!(
            self.max_gate_definitions,
            "max_gate_definitions"
        );
        require_non_zero!(self.max_gate_operations, "max_gate_operations");

        require_non_zero!(self.max_register_size, "max_register_size");
        require_non_zero!(self.max_symbols, "max_symbols");
        require_non_zero!(self.max_parameters, "max_parameters");
        require_non_zero!(self.max_operands, "max_operands");

        require_non_zero!(
            self.max_statements_per_block,
            "max_statements_per_block"
        );
        require_non_zero!(self.max_statements, "max_statements");

        require_non_zero!(
            self.max_annotations_per_item,
            "max_annotations_per_item"
        );

        if self.max_total_source_bytes < self.max_source_bytes {
            return Err(
                FrontendLimitConfigError::TotalSourceBytesLessThanSingleSource {
                    max_source_bytes: self.max_source_bytes,
                    max_total_source_bytes: self.max_total_source_bytes,
                },
            );
        }

        Ok(())
    }

    /// Returns the maximum number of bytes accepted for one source file.
    #[must_use]
    pub const fn max_source_bytes(&self) -> u64 {
        self.max_source_bytes
    }

    /// Returns the maximum total source bytes accepted for one frontend
    /// operation.
    #[must_use]
    pub const fn max_total_source_bytes(&self) -> u64 {
        self.max_total_source_bytes
    }

    /// Returns the maximum number of source files accepted by one frontend
    /// operation.
    #[must_use]
    pub const fn max_source_files(&self) -> u64 {
        self.max_source_files
    }

    /// Returns the maximum number of lexer tokens.
    #[must_use]
    pub const fn max_tokens(&self) -> u64 {
        self.max_tokens
    }

    /// Returns the maximum identifier length.
    #[must_use]
    pub const fn max_identifier_length(&self) -> u64 {
        self.max_identifier_length
    }

    /// Returns the maximum string literal length.
    #[must_use]
    pub const fn max_string_length(&self) -> u64 {
        self.max_string_length
    }

    /// Returns the maximum numeric literal length.
    #[must_use]
    pub const fn max_numeric_literal_length(&self) -> u64 {
        self.max_numeric_literal_length
    }

    /// Returns the maximum comment length.
    #[must_use]
    pub const fn max_comment_length(&self) -> u64 {
        self.max_comment_length
    }

    /// Returns the maximum annotation length.
    #[must_use]
    pub const fn max_annotation_length(&self) -> u64 {
        self.max_annotation_length
    }

    /// Returns the maximum AST node count.
    #[must_use]
    pub const fn max_ast_nodes(&self) -> u64 {
        self.max_ast_nodes
    }

    /// Returns the maximum general nesting depth.
    #[must_use]
    pub const fn max_nesting_depth(&self) -> u64 {
        self.max_nesting_depth
    }

    /// Returns the maximum expression nesting depth.
    #[must_use]
    pub const fn max_expression_depth(&self) -> u64 {
        self.max_expression_depth
    }

    /// Returns the maximum semantic expression-node count.
    #[must_use]
    pub const fn max_expression_nodes(&self) -> u64 {
        self.max_expression_nodes
    }

    /// Returns the maximum number of diagnostics.
    #[must_use]
    pub const fn max_diagnostics(&self) -> u64 {
        self.max_diagnostics
    }

    /// Returns the maximum number of child entries attached to one
    /// diagnostic.
    #[must_use]
    pub const fn max_diagnostic_children(&self) -> u64 {
        self.max_diagnostic_children
    }

    /// Returns the maximum diagnostic source-snippet length.
    #[must_use]
    pub const fn max_diagnostic_snippet_length(&self) -> u64 {
        self.max_diagnostic_snippet_length
    }

    /// Returns the maximum include/import depth.
    #[must_use]
    pub const fn max_include_depth(&self) -> u64 {
        self.max_include_depth
    }

    /// Returns the maximum number of include/import edges.
    #[must_use]
    pub const fn max_include_edges(&self) -> u64 {
        self.max_include_edges
    }

    /// Returns the maximum number of source-level gate definitions.
    #[must_use]
    pub const fn max_gate_definitions(&self) -> u64 {
        self.max_gate_definitions
    }

    /// Returns the maximum number of operations in one source-level
    /// gate/function-like definition.
    #[must_use]
    pub const fn max_gate_operations(&self) -> u64 {
        self.max_gate_operations
    }

    /// Returns the maximum logical register size.
    #[must_use]
    pub const fn max_register_size(&self) -> u64 {
        self.max_register_size
    }

    /// Returns the maximum symbol-table size.
    #[must_use]
    pub const fn max_symbols(&self) -> u64 {
        self.max_symbols
    }

    /// Returns the maximum number of parameters in one declaration.
    #[must_use]
    pub const fn max_parameters(&self) -> u64 {
        self.max_parameters
    }

    /// Returns the maximum number of operands in one operation.
    #[must_use]
    pub const fn max_operands(&self) -> u64 {
        self.max_operands
    }

    /// Returns the maximum statements in one block.
    #[must_use]
    pub const fn max_statements_per_block(&self) -> u64 {
        self.max_statements_per_block
    }

    /// Returns the maximum total source-level statements.
    #[must_use]
    pub const fn max_statements(&self) -> u64 {
        self.max_statements
    }

    /// Returns the maximum annotations attached to one source item.
    #[must_use]
    pub const fn max_annotations_per_item(&self) -> u64 {
        self.max_annotations_per_item
    }

    /// Returns `true` when a source length is permitted.
    #[must_use]
    pub const fn allows_source_bytes(&self, bytes: u64) -> bool {
        bytes <= self.max_source_bytes
    }

    /// Returns `true` when a total source size is permitted.
    #[must_use]
    pub const fn allows_total_source_bytes(&self, bytes: u64) -> bool {
        bytes <= self.max_total_source_bytes
    }

    /// Returns `true` when a source-file count is permitted.
    #[must_use]
    pub const fn allows_source_files(&self, count: u64) -> bool {
        count <= self.max_source_files
    }

    /// Returns `true` when a token count is permitted.
    #[must_use]
    pub const fn allows_tokens(&self, count: u64) -> bool {
        count <= self.max_tokens
    }

    /// Returns `true` when an identifier length is permitted.
    #[must_use]
    pub const fn allows_identifier_length(&self, length: u64) -> bool {
        length <= self.max_identifier_length
    }

    /// Returns `true` when a string length is permitted.
    #[must_use]
    pub const fn allows_string_length(&self, length: u64) -> bool {
        length <= self.max_string_length
    }

    /// Returns `true` when a numeric-literal length is permitted.
    #[must_use]
    pub const fn allows_numeric_literal_length(&self, length: u64) -> bool {
        length <= self.max_numeric_literal_length
    }

    /// Returns `true` when a comment length is permitted.
    #[must_use]
    pub const fn allows_comment_length(&self, length: u64) -> bool {
        length <= self.max_comment_length
    }

    /// Returns `true` when an annotation length is permitted.
    #[must_use]
    pub const fn allows_annotation_length(&self, length: u64) -> bool {
        length <= self.max_annotation_length
    }

    /// Returns `true` when an AST node count is permitted.
    #[must_use]
    pub const fn allows_ast_nodes(&self, count: u64) -> bool {
        count <= self.max_ast_nodes
    }

    /// Returns `true` when a general nesting depth is permitted.
    #[must_use]
    pub const fn allows_nesting_depth(&self, depth: u64) -> bool {
        depth <= self.max_nesting_depth
    }

    /// Returns `true` when an expression depth is permitted.
    #[must_use]
    pub const fn allows_expression_depth(&self, depth: u64) -> bool {
        depth <= self.max_expression_depth
    }

    /// Returns `true` when an expression-node count is permitted.
    #[must_use]
    pub const fn allows_expression_nodes(&self, count: u64) -> bool {
        count <= self.max_expression_nodes
    }

    /// Returns `true` when a diagnostic count is permitted.
    #[must_use]
    pub const fn allows_diagnostics(&self, count: u64) -> bool {
        count <= self.max_diagnostics
    }

    /// Returns `true` when a diagnostic's child-entry count is permitted.
    #[must_use]
    pub const fn allows_diagnostic_children(&self, count: u64) -> bool {
        count <= self.max_diagnostic_children
    }

    /// Returns `true` when a diagnostic source-snippet length is permitted.
    #[must_use]
    pub const fn allows_diagnostic_snippet_length(&self, length: u64) -> bool {
        length <= self.max_diagnostic_snippet_length
    }

    /// Returns `true` when an include/import depth is permitted.
    #[must_use]
    pub const fn allows_include_depth(&self, depth: u64) -> bool {
        depth <= self.max_include_depth
    }

    /// Returns `true` when an include/import edge count is permitted.
    #[must_use]
    pub const fn allows_include_edges(&self, count: u64) -> bool {
        count <= self.max_include_edges
    }

    /// Returns `true` when a gate-definition count is permitted.
    #[must_use]
    pub const fn allows_gate_definitions(&self, count: u64) -> bool {
        count <= self.max_gate_definitions
    }

    /// Returns `true` when a gate-definition operation count is permitted.
    #[must_use]
    pub const fn allows_gate_operations(&self, count: u64) -> bool {
        count <= self.max_gate_operations
    }

    /// Returns `true` when a register size is permitted.
    #[must_use]
    pub const fn allows_register_size(&self, size: u64) -> bool {
        size <= self.max_register_size
    }

    /// Returns `true` when a symbol count is permitted.
    #[must_use]
    pub const fn allows_symbols(&self, count: u64) -> bool {
        count <= self.max_symbols
    }

    /// Returns `true` when a parameter count is permitted.
    #[must_use]
    pub const fn allows_parameters(&self, count: u64) -> bool {
        count <= self.max_parameters
    }

    /// Returns `true` when an operand count is permitted.
    #[must_use]
    pub const fn allows_operands(&self, count: u64) -> bool {
        count <= self.max_operands
    }

    /// Returns `true` when a block statement count is permitted.
    #[must_use]
    pub const fn allows_statements_per_block(&self, count: u64) -> bool {
        count <= self.max_statements_per_block
    }

    /// Returns `true` when the total statement count is permitted.
    #[must_use]
    pub const fn allows_statements(&self, count: u64) -> bool {
        count <= self.max_statements
    }

    /// Returns `true` when an annotation count is permitted.
    #[must_use]
    pub const fn allows_annotations_per_item(&self, count: u64) -> bool {
        count <= self.max_annotations_per_item
    }

    /// Returns a builder initialized with the production policy.
    #[must_use]
    pub const fn builder() -> FrontendLimitsBuilder {
        FrontendLimitsBuilder::new()
    }
}

/// Builder for [`FrontendLimits`].
///
/// The builder keeps construction explicit while ensuring the final
/// `FrontendLimits` object is immutable.
///
/// Each setter returns `Self`, allowing deterministic configuration such as:
///
/// ```
/// use crate::quantum::frontend::core::limits::FrontendLimits;
///
/// let limits = FrontendLimits::builder()
///     .max_source_bytes(32 * 1024 * 1024)
///     .max_tokens(2_000_000)
///     .build()
///     .expect("valid frontend limits");
/// # let _ = limits;
/// ```
///
/// The builder itself does not perform implicit environment-based changes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FrontendLimitsBuilder {
    limits: FrontendLimits,
}

impl FrontendLimitsBuilder {
    /// Creates a builder initialized with production defaults.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: FrontendLimits::production(),
        }
    }

    /// Sets the maximum source size for one source file.
    #[must_use]
    pub const fn max_source_bytes(mut self, value: u64) -> Self {
        self.limits.max_source_bytes = value;
        self
    }

    /// Sets the maximum total source size for one frontend operation.
    #[must_use]
    pub const fn max_total_source_bytes(mut self, value: u64) -> Self {
        self.limits.max_total_source_bytes = value;
        self
    }

    /// Sets the maximum number of source files.
    #[must_use]
    pub const fn max_source_files(mut self, value: u64) -> Self {
        self.limits.max_source_files = value;
        self
    }

    /// Sets the maximum token count.
    #[must_use]
    pub const fn max_tokens(mut self, value: u64) -> Self {
        self.limits.max_tokens = value;
        self
    }

    /// Sets the maximum identifier length.
    #[must_use]
    pub const fn max_identifier_length(mut self, value: u64) -> Self {
        self.limits.max_identifier_length = value;
        self
    }

    /// Sets the maximum string literal length.
    #[must_use]
    pub const fn max_string_length(mut self, value: u64) -> Self {
        self.limits.max_string_length = value;
        self
    }

    /// Sets the maximum numeric literal length.
    #[must_use]
    pub const fn max_numeric_literal_length(mut self, value: u64) -> Self {
        self.limits.max_numeric_literal_length = value;
        self
    }

    /// Sets the maximum comment length.
    #[must_use]
    pub const fn max_comment_length(mut self, value: u64) -> Self {
        self.limits.max_comment_length = value;
        self
    }

    /// Sets the maximum annotation length.
    #[must_use]
    pub const fn max_annotation_length(mut self, value: u64) -> Self {
        self.limits.max_annotation_length = value;
        self
    }

    /// Sets the maximum AST node count.
    #[must_use]
    pub const fn max_ast_nodes(mut self, value: u64) -> Self {
        self.limits.max_ast_nodes = value;
        self
    }

    /// Sets the maximum general nesting depth.
    #[must_use]
    pub const fn max_nesting_depth(mut self, value: u64) -> Self {
        self.limits.max_nesting_depth = value;
        self
    }

    /// Sets the maximum expression nesting depth.
    #[must_use]
    pub const fn max_expression_depth(mut self, value: u64) -> Self {
        self.limits.max_expression_depth = value;
        self
    }

    /// Sets the maximum expression-node count.
    #[must_use]
    pub const fn max_expression_nodes(mut self, value: u64) -> Self {
        self.limits.max_expression_nodes = value;
        self
    }

    /// Sets the maximum number of diagnostics.
    #[must_use]
    pub const fn max_diagnostics(mut self, value: u64) -> Self {
        self.limits.max_diagnostics = value;
        self
    }

    /// Sets the maximum number of child entries per diagnostic.
    #[must_use]
    pub const fn max_diagnostic_children(mut self, value: u64) -> Self {
        self.limits.max_diagnostic_children = value;
        self
    }

    /// Sets the maximum diagnostic source-snippet length.
    #[must_use]
    pub const fn max_diagnostic_snippet_length(mut self, value: u64) -> Self {
        self.limits.max_diagnostic_snippet_length = value;
        self
    }

    /// Sets the maximum include/import depth.
    #[must_use]
    pub const fn max_include_depth(mut self, value: u64) -> Self {
        self.limits.max_include_depth = value;
        self
    }

    /// Sets the maximum include/import edge count.
    #[must_use]
    pub const fn max_include_edges(mut self, value: u64) -> Self {
        self.limits.max_include_edges = value;
        self
    }

    /// Sets the maximum number of source-level gate definitions.
    #[must_use]
    pub const fn max_gate_definitions(mut self, value: u64) -> Self {
        self.limits.max_gate_definitions = value;
        self
    }

    /// Sets the maximum number of operations in one gate definition.
    #[must_use]
    pub const fn max_gate_operations(mut self, value: u64) -> Self {
        self.limits.max_gate_operations = value;
        self
    }

    /// Sets the maximum logical register size.
    #[must_use]
    pub const fn max_register_size(mut self, value: u64) -> Self {
        self.limits.max_register_size = value;
        self
    }

    /// Sets the maximum symbol count.
    #[must_use]
    pub const fn max_symbols(mut self, value: u64) -> Self {
        self.limits.max_symbols = value;
        self
    }

    /// Sets the maximum parameter count.
    #[must_use]
    pub const fn max_parameters(mut self, value: u64) -> Self {
        self.limits.max_parameters = value;
        self
    }

    /// Sets the maximum operand count.
    #[must_use]
    pub const fn max_operands(mut self, value: u64) -> Self {
        self.limits.max_operands = value;
        self
    }

    /// Sets the maximum statements per block.
    #[must_use]
    pub const fn max_statements_per_block(mut self, value: u64) -> Self {
        self.limits.max_statements_per_block = value;
        self
    }

    /// Sets the maximum total statement count.
    #[must_use]
    pub const fn max_statements(mut self, value: u64) -> Self {
        self.limits.max_statements = value;
        self
    }

    /// Sets the maximum annotations per source item.
    #[must_use]
    pub const fn max_annotations_per_item(mut self, value: u64) -> Self {
        self.limits.max_annotations_per_item = value;
        self
    }

    /// Builds and validates the final immutable policy.
    pub const fn build(self) -> Result<FrontendLimits, FrontendLimitConfigError> {
        match self.limits.validate() {
            Ok(()) => Ok(self.limits),
            Err(error) => Err(error),
        }
    }
}

impl Default for FrontendLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_defaults_are_valid() {
        assert_eq!(FrontendLimits::production().validate(), Ok(()));
    }

    #[test]
    fn strict_defaults_are_valid() {
        assert_eq!(FrontendLimits::strict().validate(), Ok(()));
    }

    #[test]
    fn large_defaults_are_valid() {
        assert_eq!(FrontendLimits::large().validate(), Ok(()));
    }

    #[test]
    fn default_is_production() {
        assert_eq!(FrontendLimits::default(), FrontendLimits::production());
    }

    #[test]
    fn builder_defaults_to_production() {
        let limits = FrontendLimits::builder()
            .build()
            .expect("production defaults must be valid");

        assert_eq!(limits, FrontendLimits::production());
    }

    #[test]
    fn builder_changes_only_requested_values() {
        let production = FrontendLimits::production();

        let limits = FrontendLimits::builder()
            .max_tokens(123_456)
            .build()
            .expect("custom limits must be valid");

        assert_eq!(limits.max_tokens(), 123_456);
        assert_eq!(
            limits.max_source_bytes(),
            production.max_source_bytes()
        );
        assert_eq!(
            limits.max_ast_nodes(),
            production.max_ast_nodes()
        );
    }

    #[test]
    fn zero_source_limit_is_rejected() {
        let result = FrontendLimits::builder()
            .max_source_bytes(0)
            .build();

        assert_eq!(
            result,
            Err(FrontendLimitConfigError::ZeroLimit {
                field: "max_source_bytes",
            })
        );
    }

    #[test]
    fn zero_token_limit_is_rejected() {
        let result = FrontendLimits::builder().max_tokens(0).build();

        assert_eq!(
            result,
            Err(FrontendLimitConfigError::ZeroLimit {
                field: "max_tokens",
            })
        );
    }

    #[test]
    fn zero_ast_limit_is_rejected() {
        let result = FrontendLimits::builder()
            .max_ast_nodes(0)
            .build();

        assert_eq!(
            result,
            Err(FrontendLimitConfigError::ZeroLimit {
                field: "max_ast_nodes",
            })
        );
    }

    #[test]
    fn zero_nesting_limit_is_rejected() {
        let result = FrontendLimits::builder()
            .max_nesting_depth(0)
            .build();

        assert_eq!(
            result,
            Err(FrontendLimitConfigError::ZeroLimit {
                field: "max_nesting_depth",
            })
        );
    }

    #[test]
    fn total_source_limit_cannot_be_smaller_than_single_source_limit() {
        let result = FrontendLimits::builder()
            .max_source_bytes(2_000)
            .max_total_source_bytes(1_000)
            .build();

        assert_eq!(
            result,
            Err(
                FrontendLimitConfigError::TotalSourceBytesLessThanSingleSource {
                    max_source_bytes: 2_000,
                    max_total_source_bytes: 1_000,
                }
            )
        );
    }

    #[test]
    fn boundary_values_are_allowed() {
        let limits = FrontendLimits::production();

        assert!(limits.allows_source_bytes(limits.max_source_bytes()));
        assert!(limits.allows_tokens(limits.max_tokens()));
        assert!(limits.allows_ast_nodes(limits.max_ast_nodes()));
        assert!(limits.allows_nesting_depth(limits.max_nesting_depth()));
        assert!(
            limits.allows_expression_depth(
                limits.max_expression_depth()
            )
        );
        assert!(limits.allows_register_size(limits.max_register_size()));
        assert!(limits.allows_symbols(limits.max_symbols()));
        assert!(limits.allows_parameters(limits.max_parameters()));
        assert!(limits.allows_operands(limits.max_operands()));
    }

    #[test]
    fn values_above_limits_are_rejected() {
        let limits = FrontendLimits::production();

        assert!(!limits.allows_source_bytes(
            limits.max_source_bytes().saturating_add(1)
        ));

        assert!(!limits.allows_tokens(
            limits.max_tokens().saturating_add(1)
        ));

        assert!(!limits.allows_ast_nodes(
            limits.max_ast_nodes().saturating_add(1)
        ));

        assert!(!limits.allows_nesting_depth(
            limits.max_nesting_depth().saturating_add(1)
        ));

        assert!(!limits.allows_register_size(
            limits.max_register_size().saturating_add(1)
        ));
    }

    #[test]
    fn maximum_u64_values_are_safe_to_validate() {
        let result = FrontendLimits::builder()
            .max_source_bytes(u64::MAX)
            .max_total_source_bytes(u64::MAX)
            .max_tokens(u64::MAX)
            .max_ast_nodes(u64::MAX)
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn policy_is_copy_and_deterministic() {
        let limits = FrontendLimits::production();
        let copied = limits;

        assert_eq!(limits, copied);
        assert_eq!(
            format!("{limits:?}"),
            format!("{copied:?}")
        );
    }

    #[test]
    fn display_for_configuration_errors_is_stable() {
        let error = FrontendLimitConfigError::ZeroLimit {
            field: "max_tokens",
        };

        assert_eq!(
            error.to_string(),
            "frontend limit `max_tokens` must be greater than zero"
        );
    }

    #[test]
    fn all_production_limits_are_finite() {
        let limits = FrontendLimits::production();

        assert!(limits.max_source_bytes() > 0);
        assert!(limits.max_total_source_bytes() > 0);
        assert!(limits.max_source_files() > 0);
        assert!(limits.max_tokens() > 0);
        assert!(limits.max_identifier_length() > 0);
        assert!(limits.max_string_length() > 0);
        assert!(limits.max_numeric_literal_length() > 0);
        assert!(limits.max_comment_length() > 0);
        assert!(limits.max_annotation_length() > 0);
        assert!(limits.max_ast_nodes() > 0);
        assert!(limits.max_nesting_depth() > 0);
        assert!(limits.max_expression_depth() > 0);
        assert!(limits.max_expression_nodes() > 0);
        assert!(limits.max_diagnostics() > 0);
        assert!(limits.max_diagnostic_children() > 0);
        assert!(limits.max_diagnostic_snippet_length() > 0);
        assert!(limits.max_include_depth() > 0);
        assert!(limits.max_include_edges() > 0);
        assert!(limits.max_gate_definitions() > 0);
        assert!(limits.max_gate_operations() > 0);
        assert!(limits.max_register_size() > 0);
        assert!(limits.max_symbols() > 0);
        assert!(limits.max_parameters() > 0);
        assert!(limits.max_operands() > 0);
        assert!(limits.max_statements_per_block() > 0);
        assert!(limits.max_statements() > 0);
        assert!(limits.max_annotations_per_item() > 0);
    }
}