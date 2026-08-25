//! Resource and security limits for the quantum frontend.
//!
//! `FrontendLimits` is the single format-independent resource policy for the
//! untrusted-input boundary of `quantum::frontend`.
//!
//! # Boundary
//!
//! ```text
//! untrusted bytes
//!      │
//!      ▼
//! FrontendLimits / FrontendBudget
//!      │
//!      ├── source / include graph
//!      ├── lexer / tokens / literals
//!      ├── parser / AST / nesting
//!      ├── semantic validation / symbols / expressions
//!      ├── lowering / operations / work
//!      ├── diagnostics
//!      └── export / serialized output
//!      │
//!      ▼
//! canonical Quantum IR
//! ```
//!
//! Frontend limits are intentionally different from limits owned by
//! `quantum::ir`:
//!
//! - frontend limits bound untrusted source and compilation work;
//! - IR limits bound valid canonical quantum circuits.
//!
//! This module contains no OpenQASM, QIR, Quil, hardware, filesystem,
//! network, process, or runtime knowledge.
//!
//! # Security invariants
//!
//! - Every externally controlled dimension is finite and explicit.
//! - There is no unlimited mode.
//! - Limits are deterministic and platform-independent.
//! - Configuration is immutable after construction.
//! - Runtime accounting is overflow-safe.
//! - Exhaustion is reported as data.
//! - This module performs no I/O.
//!
//! # Integration contract
//!
//! Later frontend files must use this module rather than inventing local
//! limits.
//!
//! - `source.rs`: source bytes and source-file counts.
//! - `lexer.rs`: tokens and lexical literal/comment/annotation lengths.
//! - `parser.rs`: AST nodes, statements and nesting.
//! - `validation.rs`: symbols, parameters, operands, registers, expressions,
//!   declarations and semantic work.
//! - include/import handling: source files, include depth and include edges.
//! - lowering: operations and total work.
//! - diagnostics: diagnostic count, child entries and snippets.
//! - exporters: serialized output bytes and export work.
//!
//! # Rust compatibility
//!
//! Rust 2021 / Rust 1.97.1.
//! No nightly features are required.

use core::fmt;

// -----------------------------------------------------------------------------
// Default production limits
// -----------------------------------------------------------------------------

/// Maximum bytes accepted for one source file.
pub const DEFAULT_MAX_SOURCE_BYTES: u64 = 16 * 1024 * 1024;

/// Maximum aggregate source bytes across one frontend operation.
pub const DEFAULT_MAX_TOTAL_SOURCE_BYTES: u64 = 64 * 1024 * 1024;

/// Maximum number of source files in one frontend operation.
pub const DEFAULT_MAX_SOURCE_FILES: u64 = 1_024;

/// Maximum lexer tokens produced for one frontend operation.
pub const DEFAULT_MAX_TOKENS: u64 = 1_000_000;

/// Maximum UTF-8 byte length of one identifier.
pub const DEFAULT_MAX_IDENTIFIER_LENGTH: u64 = 4 * 1024;

/// Maximum string literal length in bytes.
pub const DEFAULT_MAX_STRING_LENGTH: u64 = 1 * 1024 * 1024;

/// Maximum numeric literal length before numeric conversion.
pub const DEFAULT_MAX_NUMERIC_LITERAL_LENGTH: u64 = 4 * 1024;

/// Maximum comment length in bytes.
pub const DEFAULT_MAX_COMMENT_LENGTH: u64 = 1 * 1024 * 1024;

/// Maximum annotation length in bytes.
pub const DEFAULT_MAX_ANNOTATION_LENGTH: u64 = 1 * 1024 * 1024;

/// Maximum AST node count.
pub const DEFAULT_MAX_AST_NODES: u64 = 2_000_000;

/// Maximum general parser/semantic nesting depth.
pub const DEFAULT_MAX_NESTING_DEPTH: u64 = 512;

/// Maximum expression nesting depth.
pub const DEFAULT_MAX_EXPRESSION_DEPTH: u64 = 256;

/// Maximum semantic expression-node count.
pub const DEFAULT_MAX_EXPRESSION_NODES: u64 = 1_000_000;

/// Maximum diagnostics retained for one frontend operation.
pub const DEFAULT_MAX_DIAGNOSTICS: u64 = 1_000;

/// Maximum labels/notes/help entries attached to one diagnostic.
pub const DEFAULT_MAX_DIAGNOSTIC_CHILDREN: u64 = 64;

/// Maximum source-excerpt length retained/rendered for one diagnostic.
pub const DEFAULT_MAX_DIAGNOSTIC_SNIPPET_LENGTH: u64 = 16 * 1024;

/// Maximum include/import nesting depth.
pub const DEFAULT_MAX_INCLUDE_DEPTH: u64 = 64;

/// Maximum include/import graph edges.
pub const DEFAULT_MAX_INCLUDE_EDGES: u64 = 10_000;

/// Maximum source-level gate/function definitions.
pub const DEFAULT_MAX_GATE_DEFINITIONS: u64 = 100_000;

/// Maximum operations in one source-level gate/function definition.
pub const DEFAULT_MAX_GATE_OPERATIONS: u64 = 1_000_000;

/// Maximum logical register/array dimension.
pub const DEFAULT_MAX_REGISTER_SIZE: u64 = 1_000_000;

/// Maximum array elements in a source-level array value.
pub const DEFAULT_MAX_ARRAY_ELEMENTS: u64 = 1_000_000;

/// Maximum symbols in one frontend symbol environment.
pub const DEFAULT_MAX_SYMBOLS: u64 = 1_000_000;

/// Maximum parameters in one declaration/callable entity.
pub const DEFAULT_MAX_PARAMETERS: u64 = 1_024;

/// Maximum operands in one source-level operation.
pub const DEFAULT_MAX_OPERANDS: u64 = 1_024;

/// Maximum statements in one source-level block.
pub const DEFAULT_MAX_STATEMENTS_PER_BLOCK: u64 = 1_000_000;

/// Maximum total source-level statements.
pub const DEFAULT_MAX_STATEMENTS: u64 = 2_000_000;

/// Maximum annotations attached to one source item.
pub const DEFAULT_MAX_ANNOTATIONS_PER_ITEM: u64 = 1_024;

/// Maximum quantum/classical operations produced by one frontend operation.
pub const DEFAULT_MAX_OPERATIONS: u64 = 2_000_000;

/// Maximum recursive semantic/import depth.
pub const DEFAULT_MAX_RECURSION_DEPTH: u64 = 512;

/// Maximum serialized output bytes.
pub const DEFAULT_MAX_OUTPUT_BYTES: u64 = 64 * 1024 * 1024;

/// Maximum abstract work units charged to one frontend operation.
pub const DEFAULT_MAX_TOTAL_WORK: u64 = 100_000_000;

// -----------------------------------------------------------------------------
// Limit identity
// -----------------------------------------------------------------------------

/// Stable identity for every independently enforceable frontend limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum FrontendLimitKind {
    SourceBytes,
    TotalSourceBytes,
    SourceFiles,
    Tokens,
    IdentifierLength,
    StringLength,
    NumericLiteralLength,
    CommentLength,
    AnnotationLength,
    AstNodes,
    NestingDepth,
    ExpressionDepth,
    ExpressionNodes,
    Diagnostics,
    DiagnosticChildren,
    DiagnosticSnippetLength,
    IncludeDepth,
    IncludeEdges,
    GateDefinitions,
    GateOperations,
    RegisterSize,
    ArrayElements,
    Symbols,
    Parameters,
    Operands,
    StatementsPerBlock,
    Statements,
    AnnotationsPerItem,
    Operations,
    RecursionDepth,
    OutputBytes,
    TotalWork,
}

impl fmt::Display for FrontendLimitKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::SourceBytes => "source-bytes",
            Self::TotalSourceBytes => "total-source-bytes",
            Self::SourceFiles => "source-files",
            Self::Tokens => "tokens",
            Self::IdentifierLength => "identifier-length",
            Self::StringLength => "string-length",
            Self::NumericLiteralLength => "numeric-literal-length",
            Self::CommentLength => "comment-length",
            Self::AnnotationLength => "annotation-length",
            Self::AstNodes => "ast-nodes",
            Self::NestingDepth => "nesting-depth",
            Self::ExpressionDepth => "expression-depth",
            Self::ExpressionNodes => "expression-nodes",
            Self::Diagnostics => "diagnostics",
            Self::DiagnosticChildren => "diagnostic-children",
            Self::DiagnosticSnippetLength => "diagnostic-snippet-length",
            Self::IncludeDepth => "include-depth",
            Self::IncludeEdges => "include-edges",
            Self::GateDefinitions => "gate-definitions",
            Self::GateOperations => "gate-operations",
            Self::RegisterSize => "register-size",
            Self::ArrayElements => "array-elements",
            Self::Symbols => "symbols",
            Self::Parameters => "parameters",
            Self::Operands => "operands",
            Self::StatementsPerBlock => "statements-per-block",
            Self::Statements => "statements",
            Self::AnnotationsPerItem => "annotations-per-item",
            Self::Operations => "operations",
            Self::RecursionDepth => "recursion-depth",
            Self::OutputBytes => "output-bytes",
            Self::TotalWork => "total-work",
        };

        f.write_str(name)
    }
}

// -----------------------------------------------------------------------------
// Runtime violations
// -----------------------------------------------------------------------------

/// Runtime frontend resource-limit violation.
///
/// This type deliberately does not depend on `core::errors.rs`, preventing a
/// dependency cycle between the foundational limit and error modules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FrontendLimitViolation {
    kind: FrontendLimitKind,
    actual: u64,
    maximum: u64,
}

impl FrontendLimitViolation {
    #[must_use]
    pub const fn new(
        kind: FrontendLimitKind,
        actual: u64,
        maximum: u64,
    ) -> Self {
        Self {
            kind,
            actual,
            maximum,
        }
    }

    #[must_use]
    pub const fn kind(self) -> FrontendLimitKind {
        self.kind
    }

    #[must_use]
    pub const fn actual(self) -> u64 {
        self.actual
    }

    #[must_use]
    pub const fn maximum(self) -> u64 {
        self.maximum
    }
}

impl fmt::Display for FrontendLimitViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "frontend resource limit `{}` exceeded: {} > {}",
            self.kind,
            self.actual,
            self.maximum
        )
    }
}

impl std::error::Error for FrontendLimitViolation {}

// -----------------------------------------------------------------------------
// Configuration errors
// -----------------------------------------------------------------------------

/// Error returned when a `FrontendLimits` configuration is inconsistent.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FrontendLimitConfigError {
    /// A required positive limit was configured as zero.
    ZeroLimit {
        field: &'static str,
    },

    /// Aggregate source capacity is smaller than one source.
    TotalSourceBytesLessThanSingleSource {
        max_source_bytes: u64,
        max_total_source_bytes: u64,
    },

    /// Expression depth cannot exceed the general nesting depth.
    ExpressionDepthExceedsNestingDepth {
        max_expression_depth: u64,
        max_nesting_depth: u64,
    },
}

impl fmt::Display for FrontendLimitConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroLimit { field } => {
                write!(
                    f,
                    "frontend limit `{field}` must be greater than zero"
                )
            }

            Self::TotalSourceBytesLessThanSingleSource {
                max_source_bytes,
                max_total_source_bytes,
            } => {
                write!(
                    f,
                    "frontend limit `max_total_source_bytes` ({max_total_source_bytes}) \
                     must be >= `max_source_bytes` ({max_source_bytes})"
                )
            }

            Self::ExpressionDepthExceedsNestingDepth {
                max_expression_depth,
                max_nesting_depth,
            } => {
                write!(
                    f,
                    "frontend limit `max_expression_depth` ({max_expression_depth}) \
                     must be <= `max_nesting_depth` ({max_nesting_depth})"
                )
            }
        }
    }
}

impl std::error::Error for FrontendLimitConfigError {}

// -----------------------------------------------------------------------------
// Immutable policy
// -----------------------------------------------------------------------------

/// Immutable, format-independent resource policy for frontend processing.
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
    max_array_elements: u64,
    max_symbols: u64,
    max_parameters: u64,
    max_operands: u64,

    max_statements_per_block: u64,
    max_statements: u64,
    max_annotations_per_item: u64,

    max_operations: u64,
    max_recursion_depth: u64,

    max_output_bytes: u64,
    max_total_work: u64,
}

impl Default for FrontendLimits {
    fn default() -> Self {
        Self::production()
    }
}

impl FrontendLimits {
    /// Standard production-safe policy.
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
            max_diagnostic_snippet_length:
                DEFAULT_MAX_DIAGNOSTIC_SNIPPET_LENGTH,

            max_include_depth: DEFAULT_MAX_INCLUDE_DEPTH,
            max_include_edges: DEFAULT_MAX_INCLUDE_EDGES,

            max_gate_definitions: DEFAULT_MAX_GATE_DEFINITIONS,
            max_gate_operations: DEFAULT_MAX_GATE_OPERATIONS,

            max_register_size: DEFAULT_MAX_REGISTER_SIZE,
            max_array_elements: DEFAULT_MAX_ARRAY_ELEMENTS,
            max_symbols: DEFAULT_MAX_SYMBOLS,
            max_parameters: DEFAULT_MAX_PARAMETERS,
            max_operands: DEFAULT_MAX_OPERANDS,

            max_statements_per_block:
                DEFAULT_MAX_STATEMENTS_PER_BLOCK,
            max_statements: DEFAULT_MAX_STATEMENTS,
            max_annotations_per_item:
                DEFAULT_MAX_ANNOTATIONS_PER_ITEM,

            max_operations: DEFAULT_MAX_OPERATIONS,
            max_recursion_depth: DEFAULT_MAX_RECURSION_DEPTH,

            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
            max_total_work: DEFAULT_MAX_TOTAL_WORK,
        }
    }

    /// Strict policy for exposed/adversarial environments.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            max_source_bytes: 1 * 1024 * 1024,
            max_total_source_bytes: 4 * 1024 * 1024,
            max_source_files: 64,

            max_tokens: 100_000,
            max_identifier_length: 1_024,
            max_string_length: 64 * 1024,
            max_numeric_literal_length: 1_024,
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
            max_array_elements: 100_000,
            max_symbols: 100_000,
            max_parameters: 256,
            max_operands: 256,

            max_statements_per_block: 100_000,
            max_statements: 250_000,
            max_annotations_per_item: 128,

            max_operations: 250_000,
            max_recursion_depth: 128,

            max_output_bytes: 4 * 1024 * 1024,
            max_total_work: 10_000_000,
        }
    }

    /// Large but still finite profile for trusted local workloads.
    #[must_use]
    pub const fn large() -> Self {
        Self {
            max_source_bytes: 256 * 1024 * 1024,
            max_total_source_bytes: 1024 * 1024 * 1024,
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
            max_array_elements: 10_000_000,
            max_symbols: 10_000_000,
            max_parameters: 4_096,
            max_operands: 4_096,

            max_statements_per_block: 10_000_000,
            max_statements: 20_000_000,
            max_annotations_per_item: 4_096,

            max_operations: 20_000_000,
            max_recursion_depth: 2_048,

            max_output_bytes: 1024 * 1024 * 1024,
            max_total_work: 2_000_000_000,
        }
    }

    /// Validates the policy itself.
    pub const fn validate(
        self,
    ) -> Result<(), FrontendLimitConfigError> {
        macro_rules! non_zero {
            ($value:expr, $name:literal) => {
                if $value == 0 {
                    return Err(
                        FrontendLimitConfigError::ZeroLimit {
                            field: $name,
                        },
                    );
                }
            };
        }

        non_zero!(self.max_source_bytes, "max_source_bytes");
        non_zero!(
            self.max_total_source_bytes,
            "max_total_source_bytes"
        );
        non_zero!(self.max_source_files, "max_source_files");

        non_zero!(self.max_tokens, "max_tokens");
        non_zero!(
            self.max_identifier_length,
            "max_identifier_length"
        );
        non_zero!(self.max_string_length, "max_string_length");
        non_zero!(
            self.max_numeric_literal_length,
            "max_numeric_literal_length"
        );
        non_zero!(self.max_comment_length, "max_comment_length");
        non_zero!(
            self.max_annotation_length,
            "max_annotation_length"
        );

        non_zero!(self.max_ast_nodes, "max_ast_nodes");
        non_zero!(
            self.max_nesting_depth,
            "max_nesting_depth"
        );
        non_zero!(
            self.max_expression_depth,
            "max_expression_depth"
        );
        non_zero!(
            self.max_expression_nodes,
            "max_expression_nodes"
        );

        non_zero!(self.max_diagnostics, "max_diagnostics");
        non_zero!(
            self.max_diagnostic_children,
            "max_diagnostic_children"
        );
        non_zero!(
            self.max_diagnostic_snippet_length,
            "max_diagnostic_snippet_length"
        );

        non_zero!(
            self.max_include_depth,
            "max_include_depth"
        );
        non_zero!(
            self.max_include_edges,
            "max_include_edges"
        );

        non_zero!(
            self.max_gate_definitions,
            "max_gate_definitions"
        );
        non_zero!(
            self.max_gate_operations,
            "max_gate_operations"
        );

        non_zero!(
            self.max_register_size,
            "max_register_size"
        );
        non_zero!(
            self.max_array_elements,
            "max_array_elements"
        );
        non_zero!(self.max_symbols, "max_symbols");
        non_zero!(self.max_parameters, "max_parameters");
        non_zero!(self.max_operands, "max_operands");

        non_zero!(
            self.max_statements_per_block,
            "max_statements_per_block"
        );
        non_zero!(self.max_statements, "max_statements");
        non_zero!(
            self.max_annotations_per_item,
            "max_annotations_per_item"
        );

        non_zero!(self.max_operations, "max_operations");
        non_zero!(
            self.max_recursion_depth,
            "max_recursion_depth"
        );

        non_zero!(
            self.max_output_bytes,
            "max_output_bytes"
        );
        non_zero!(
            self.max_total_work,
            "max_total_work"
        );

        if self.max_total_source_bytes < self.max_source_bytes {
            return Err(
                FrontendLimitConfigError::
                    TotalSourceBytesLessThanSingleSource {
                        max_source_bytes: self.max_source_bytes,
                        max_total_source_bytes:
                            self.max_total_source_bytes,
                    },
            );
        }

        if self.max_expression_depth > self.max_nesting_depth {
            return Err(
                FrontendLimitConfigError::
                    ExpressionDepthExceedsNestingDepth {
                        max_expression_depth:
                            self.max_expression_depth,
                        max_nesting_depth:
                            self.max_nesting_depth,
                    },
            );
        }

        Ok(())
    }

    /// Creates a production-default builder.
    #[must_use]
    pub const fn builder() -> FrontendLimitsBuilder {
        FrontendLimitsBuilder::new()
    }

    pub const fn max_source_bytes(&self) -> u64 {
        self.max_source_bytes
    }

    pub const fn max_total_source_bytes(&self) -> u64 {
        self.max_total_source_bytes
    }

    pub const fn max_source_files(&self) -> u64 {
        self.max_source_files
    }

    pub const fn max_tokens(&self) -> u64 {
        self.max_tokens
    }

    pub const fn max_identifier_length(&self) -> u64 {
        self.max_identifier_length
    }

    pub const fn max_string_length(&self) -> u64 {
        self.max_string_length
    }

    pub const fn max_numeric_literal_length(&self) -> u64 {
        self.max_numeric_literal_length
    }

    pub const fn max_comment_length(&self) -> u64 {
        self.max_comment_length
    }

    pub const fn max_annotation_length(&self) -> u64 {
        self.max_annotation_length
    }

    pub const fn max_ast_nodes(&self) -> u64 {
        self.max_ast_nodes
    }

    pub const fn max_nesting_depth(&self) -> u64 {
        self.max_nesting_depth
    }

    pub const fn max_expression_depth(&self) -> u64 {
        self.max_expression_depth
    }

    pub const fn max_expression_nodes(&self) -> u64 {
        self.max_expression_nodes
    }

    pub const fn max_diagnostics(&self) -> u64 {
        self.max_diagnostics
    }

    pub const fn max_diagnostic_children(&self) -> u64 {
        self.max_diagnostic_children
    }

    pub const fn max_diagnostic_snippet_length(&self) -> u64 {
        self.max_diagnostic_snippet_length
    }

    pub const fn max_include_depth(&self) -> u64 {
        self.max_include_depth
    }

    pub const fn max_include_edges(&self) -> u64 {
        self.max_include_edges
    }

    pub const fn max_gate_definitions(&self) -> u64 {
        self.max_gate_definitions
    }

    pub const fn max_gate_operations(&self) -> u64 {
        self.max_gate_operations
    }

    pub const fn max_register_size(&self) -> u64 {
        self.max_register_size
    }

    pub const fn max_array_elements(&self) -> u64 {
        self.max_array_elements
    }

    pub const fn max_symbols(&self) -> u64 {
        self.max_symbols
    }

    pub const fn max_parameters(&self) -> u64 {
        self.max_parameters
    }

    pub const fn max_operands(&self) -> u64 {
        self.max_operands
    }

    pub const fn max_statements_per_block(&self) -> u64 {
        self.max_statements_per_block
    }

    pub const fn max_statements(&self) -> u64 {
        self.max_statements
    }

    pub const fn max_annotations_per_item(&self) -> u64 {
        self.max_annotations_per_item
    }

    pub const fn max_operations(&self) -> u64 {
        self.max_operations
    }

    pub const fn max_recursion_depth(&self) -> u64 {
        self.max_recursion_depth
    }

    pub const fn max_output_bytes(&self) -> u64 {
        self.max_output_bytes
    }

    pub const fn max_total_work(&self) -> u64 {
        self.max_total_work
    }

    pub const fn allows_source_bytes(&self, value: u64) -> bool {
        value <= self.max_source_bytes
    }

    pub const fn allows_total_source_bytes(&self, value: u64) -> bool {
        value <= self.max_total_source_bytes
    }

    pub const fn allows_source_files(&self, value: u64) -> bool {
        value <= self.max_source_files
    }

    pub const fn allows_tokens(&self, value: u64) -> bool {
        value <= self.max_tokens
    }

    pub const fn allows_identifier_length(&self, value: u64) -> bool {
        value <= self.max_identifier_length
    }

    pub const fn allows_string_length(&self, value: u64) -> bool {
        value <= self.max_string_length
    }

    pub const fn allows_numeric_literal_length(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_numeric_literal_length
    }

    pub const fn allows_comment_length(&self, value: u64) -> bool {
        value <= self.max_comment_length
    }

    pub const fn allows_annotation_length(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_annotation_length
    }

    pub const fn allows_ast_nodes(&self, value: u64) -> bool {
        value <= self.max_ast_nodes
    }

    pub const fn allows_nesting_depth(&self, value: u64) -> bool {
        value <= self.max_nesting_depth
    }

    pub const fn allows_expression_depth(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_expression_depth
    }

    pub const fn allows_expression_nodes(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_expression_nodes
    }

    pub const fn allows_diagnostics(&self, value: u64) -> bool {
        value <= self.max_diagnostics
    }

    pub const fn allows_diagnostic_children(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_diagnostic_children
    }

    pub const fn allows_diagnostic_snippet_length(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_diagnostic_snippet_length
    }

    pub const fn allows_include_depth(&self, value: u64) -> bool {
        value <= self.max_include_depth
    }

    pub const fn allows_include_edges(&self, value: u64) -> bool {
        value <= self.max_include_edges
    }

    pub const fn allows_gate_definitions(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_gate_definitions
    }

    pub const fn allows_gate_operations(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_gate_operations
    }

    pub const fn allows_register_size(&self, value: u64) -> bool {
        value <= self.max_register_size
    }

    pub const fn allows_array_elements(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_array_elements
    }

    pub const fn allows_symbols(&self, value: u64) -> bool {
        value <= self.max_symbols
    }

    pub const fn allows_parameters(&self, value: u64) -> bool {
        value <= self.max_parameters
    }

    pub const fn allows_operands(&self, value: u64) -> bool {
        value <= self.max_operands
    }

    pub const fn allows_statements_per_block(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_statements_per_block
    }

    pub const fn allows_statements(&self, value: u64) -> bool {
        value <= self.max_statements
    }

    pub const fn allows_annotations_per_item(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_annotations_per_item
    }

    pub const fn allows_operations(&self, value: u64) -> bool {
        value <= self.max_operations
    }

    pub const fn allows_recursion_depth(
        &self,
        value: u64,
    ) -> bool {
        value <= self.max_recursion_depth
    }

    pub const fn allows_output_bytes(&self, value: u64) -> bool {
        value <= self.max_output_bytes
    }

    pub const fn allows_total_work(&self, value: u64) -> bool {
        value <= self.max_total_work
    }

    fn check(
        &self,
        kind: FrontendLimitKind,
        value: u64,
        maximum: u64,
    ) -> Result<(), FrontendLimitViolation> {
        if value <= maximum {
            Ok(())
        } else {
            Err(FrontendLimitViolation::new(
                kind,
                value,
                maximum,
            ))
        }
    }

    pub fn check_source_bytes(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::SourceBytes,
            value,
            self.max_source_bytes,
        )
    }

    pub fn check_total_source_bytes(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::TotalSourceBytes,
            value,
            self.max_total_source_bytes,
        )
    }

    pub fn check_source_files(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::SourceFiles,
            value,
            self.max_source_files,
        )
    }

    pub fn check_tokens(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Tokens,
            value,
            self.max_tokens,
        )
    }

    pub fn check_identifier_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::IdentifierLength,
            value,
            self.max_identifier_length,
        )
    }

    pub fn check_string_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::StringLength,
            value,
            self.max_string_length,
        )
    }

    pub fn check_numeric_literal_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::NumericLiteralLength,
            value,
            self.max_numeric_literal_length,
        )
    }

    pub fn check_comment_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::CommentLength,
            value,
            self.max_comment_length,
        )
    }

    pub fn check_annotation_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::AnnotationLength,
            value,
            self.max_annotation_length,
        )
    }

    pub fn check_ast_nodes(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::AstNodes,
            value,
            self.max_ast_nodes,
        )
    }

    pub fn check_nesting_depth(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::NestingDepth,
            value,
            self.max_nesting_depth,
        )
    }

    pub fn check_expression_depth(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::ExpressionDepth,
            value,
            self.max_expression_depth,
        )
    }

    pub fn check_expression_nodes(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::ExpressionNodes,
            value,
            self.max_expression_nodes,
        )
    }

    pub fn check_diagnostics(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Diagnostics,
            value,
            self.max_diagnostics,
        )
    }

    pub fn check_diagnostic_children(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::DiagnosticChildren,
            value,
            self.max_diagnostic_children,
        )
    }

    pub fn check_diagnostic_snippet_length(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::DiagnosticSnippetLength,
            value,
            self.max_diagnostic_snippet_length,
        )
    }

    pub fn check_include_depth(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::IncludeDepth,
            value,
            self.max_include_depth,
        )
    }

    pub fn check_include_edges(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::IncludeEdges,
            value,
            self.max_include_edges,
        )
    }

    pub fn check_gate_definitions(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::GateDefinitions,
            value,
            self.max_gate_definitions,
        )
    }

    pub fn check_gate_operations(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::GateOperations,
            value,
            self.max_gate_operations,
        )
    }

    pub fn check_register_size(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::RegisterSize,
            value,
            self.max_register_size,
        )
    }

    pub fn check_array_elements(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::ArrayElements,
            value,
            self.max_array_elements,
        )
    }

    pub fn check_symbols(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Symbols,
            value,
            self.max_symbols,
        )
    }

    pub fn check_parameters(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Parameters,
            value,
            self.max_parameters,
        )
    }

    pub fn check_operands(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Operands,
            value,
            self.max_operands,
        )
    }

    pub fn check_statements_per_block(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::StatementsPerBlock,
            value,
            self.max_statements_per_block,
        )
    }

    pub fn check_statements(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Statements,
            value,
            self.max_statements,
        )
    }

    pub fn check_annotations_per_item(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::AnnotationsPerItem,
            value,
            self.max_annotations_per_item,
        )
    }

    pub fn check_operations(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::Operations,
            value,
            self.max_operations,
        )
    }

    pub fn check_recursion_depth(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::RecursionDepth,
            value,
            self.max_recursion_depth,
        )
    }

    pub fn check_output_bytes(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::OutputBytes,
            value,
            self.max_output_bytes,
        )
    }

    pub fn check_total_work(
        &self,
        value: u64,
    ) -> Result<(), FrontendLimitViolation> {
        self.check(
            FrontendLimitKind::TotalWork,
            value,
            self.max_total_work,
        )
    }
}

// -----------------------------------------------------------------------------
// Builder
// -----------------------------------------------------------------------------

/// Builder for [`FrontendLimits`].
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct FrontendLimitsBuilder {
    limits: FrontendLimits,
}

impl Default for FrontendLimitsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FrontendLimitsBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            limits: FrontendLimits::production(),
        }
    }

    #[must_use]
    pub const fn max_source_bytes(mut self, value: u64) -> Self {
        self.limits.max_source_bytes = value;
        self
    }

    #[must_use]
    pub const fn max_total_source_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_total_source_bytes = value;
        self
    }

    #[must_use]
    pub const fn max_source_files(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_source_files = value;
        self
    }

    #[must_use]
    pub const fn max_tokens(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_tokens = value;
        self
    }

    #[must_use]
    pub const fn max_identifier_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_identifier_length = value;
        self
    }

    #[must_use]
    pub const fn max_string_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_string_length = value;
        self
    }

    #[must_use]
    pub const fn max_numeric_literal_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_numeric_literal_length = value;
        self
    }

    #[must_use]
    pub const fn max_comment_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_comment_length = value;
        self
    }

    #[must_use]
    pub const fn max_annotation_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_annotation_length = value;
        self
    }

    #[must_use]
    pub const fn max_ast_nodes(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_ast_nodes = value;
        self
    }

    #[must_use]
    pub const fn max_nesting_depth(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_nesting_depth = value;
        self
    }

    #[must_use]
    pub const fn max_expression_depth(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_expression_depth = value;
        self
    }

    #[must_use]
    pub const fn max_expression_nodes(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_expression_nodes = value;
        self
    }

    #[must_use]
    pub const fn max_diagnostics(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_diagnostics = value;
        self
    }

    #[must_use]
    pub const fn max_diagnostic_children(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_diagnostic_children = value;
        self
    }

    #[must_use]
    pub const fn max_diagnostic_snippet_length(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_diagnostic_snippet_length = value;
        self
    }

    #[must_use]
    pub const fn max_include_depth(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_include_depth = value;
        self
    }

    #[must_use]
    pub const fn max_include_edges(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_include_edges = value;
        self
    }

    #[must_use]
    pub const fn max_gate_definitions(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_gate_definitions = value;
        self
    }

    #[must_use]
    pub const fn max_gate_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_gate_operations = value;
        self
    }

    #[must_use]
    pub const fn max_register_size(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_register_size = value;
        self
    }

    #[must_use]
    pub const fn max_array_elements(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_array_elements = value;
        self
    }

    #[must_use]
    pub const fn max_symbols(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_symbols = value;
        self
    }

    #[must_use]
    pub const fn max_parameters(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_parameters = value;
        self
    }

    #[must_use]
    pub const fn max_operands(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_operands = value;
        self
    }

    #[must_use]
    pub const fn max_statements_per_block(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_statements_per_block = value;
        self
    }

    #[must_use]
    pub const fn max_statements(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_statements = value;
        self
    }

    #[must_use]
    pub const fn max_annotations_per_item(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_annotations_per_item = value;
        self
    }

    #[must_use]
    pub const fn max_operations(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_operations = value;
        self
    }

    #[must_use]
    pub const fn max_recursion_depth(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_recursion_depth = value;
        self
    }

    #[must_use]
    pub const fn max_output_bytes(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_output_bytes = value;
        self
    }

    #[must_use]
    pub const fn max_total_work(
        mut self,
        value: u64,
    ) -> Self {
        self.limits.max_total_work = value;
        self
    }

    /// Finishes construction and validates all cross-field invariants.
    pub const fn build(
        self,
    ) -> Result<FrontendLimits, FrontendLimitConfigError> {
        match self.limits.validate() {
            Ok(()) => Ok(self.limits),
            Err(error) => Err(error),
        }
    }
}

// -----------------------------------------------------------------------------
// Runtime accounting
// -----------------------------------------------------------------------------

/// Per-operation bounded resource accounting.
///
/// The immutable `FrontendLimits` describes policy. `FrontendBudget` tracks
/// what one import/parse/validation/lowering/export operation has consumed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrontendBudget {
    limits: FrontendLimits,

    source_bytes: u64,
    total_source_bytes: u64,
    source_files: u64,

    tokens: u64,
    ast_nodes: u64,
    expression_nodes: u64,

    diagnostics: u64,
    diagnostic_children: u64,

    include_edges: u64,
    gate_definitions: u64,
    gate_operations: u64,

    symbols: u64,
    statements: u64,
    operations: u64,

    total_work: u64,
}

impl FrontendBudget {
    /// Creates an empty budget for a validated policy.
    #[must_use]
    pub const fn new(limits: FrontendLimits) -> Self {
        Self {
            limits,

            source_bytes: 0,
            total_source_bytes: 0,
            source_files: 0,

            tokens: 0,
            ast_nodes: 0,
            expression_nodes: 0,

            diagnostics: 0,
            diagnostic_children: 0,

            include_edges: 0,
            gate_definitions: 0,
            gate_operations: 0,

            symbols: 0,
            statements: 0,
            operations: 0,

            total_work: 0,
        }
    }

    #[must_use]
    pub const fn limits(&self) -> &FrontendLimits {
        &self.limits
    }

    /// Charges one source file atomically.
    ///
    /// Failure does not partially mutate the budget.
    pub fn try_add_source(
        &mut self,
        bytes: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let new_total =
            self.total_source_bytes
                .checked_add(bytes)
                .ok_or_else(|| {
                    FrontendLimitViolation::new(
                        FrontendLimitKind::TotalSourceBytes,
                        u64::MAX,
                        self.limits.max_total_source_bytes,
                    )
                })?;

        let new_files =
            self.source_files
                .checked_add(1)
                .ok_or_else(|| {
                    FrontendLimitViolation::new(
                        FrontendLimitKind::SourceFiles,
                        u64::MAX,
                        self.limits.max_source_files,
                    )
                })?;

        self.limits.check_source_bytes(bytes)?;
        self.limits.check_total_source_bytes(new_total)?;
        self.limits.check_source_files(new_files)?;

        self.source_bytes = bytes;
        self.total_source_bytes = new_total;
        self.source_files = new_files;

        Ok(())
    }

    pub fn try_add_tokens(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.tokens.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::Tokens,
                u64::MAX,
                self.limits.max_tokens,
            )
        })?;

        self.limits.check_tokens(next)?;
        self.tokens = next;

        Ok(())
    }

    pub fn try_add_ast_nodes(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.ast_nodes.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::AstNodes,
                u64::MAX,
                self.limits.max_ast_nodes,
            )
        })?;

        self.limits.check_ast_nodes(next)?;
        self.ast_nodes = next;

        Ok(())
    }

    pub fn try_add_expression_nodes(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self
            .expression_nodes
            .checked_add(count)
            .ok_or_else(|| {
                FrontendLimitViolation::new(
                    FrontendLimitKind::ExpressionNodes,
                    u64::MAX,
                    self.limits.max_expression_nodes,
                )
            })?;

        self.limits.check_expression_nodes(next)?;
        self.expression_nodes = next;

        Ok(())
    }

    pub fn try_add_diagnostics(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.diagnostics.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::Diagnostics,
                u64::MAX,
                self.limits.max_diagnostics,
            )
        })?;

        self.limits.check_diagnostics(next)?;
        self.diagnostics = next;

        Ok(())
    }

    pub fn try_add_diagnostic_children(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self
            .diagnostic_children
            .checked_add(count)
            .ok_or_else(|| {
                FrontendLimitViolation::new(
                    FrontendLimitKind::DiagnosticChildren,
                    u64::MAX,
                    self.limits.max_diagnostic_children,
                )
            })?;

        self.limits.check_diagnostic_children(next)?;
        self.diagnostic_children = next;

        Ok(())
    }

    pub fn try_add_include_edge(
        &mut self,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.include_edges.checked_add(1).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::IncludeEdges,
                u64::MAX,
                self.limits.max_include_edges,
            )
        })?;

        self.limits.check_include_edges(next)?;
        self.include_edges = next;

        Ok(())
    }

    pub fn try_add_gate_definitions(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self
            .gate_definitions
            .checked_add(count)
            .ok_or_else(|| {
                FrontendLimitViolation::new(
                    FrontendLimitKind::GateDefinitions,
                    u64::MAX,
                    self.limits.max_gate_definitions,
                )
            })?;

        self.limits.check_gate_definitions(next)?;
        self.gate_definitions = next;

        Ok(())
    }

    pub fn try_add_gate_operations(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self
            .gate_operations
            .checked_add(count)
            .ok_or_else(|| {
                FrontendLimitViolation::new(
                    FrontendLimitKind::GateOperations,
                    u64::MAX,
                    self.limits.max_gate_operations,
                )
            })?;

        self.limits.check_gate_operations(next)?;
        self.gate_operations = next;

        Ok(())
    }

    pub fn try_add_symbols(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.symbols.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::Symbols,
                u64::MAX,
                self.limits.max_symbols,
            )
        })?;

        self.limits.check_symbols(next)?;
        self.symbols = next;

        Ok(())
    }

    pub fn try_add_statements(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.statements.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::Statements,
                u64::MAX,
                self.limits.max_statements,
            )
        })?;

        self.limits.check_statements(next)?;
        self.statements = next;

        Ok(())
    }

    pub fn try_add_operations(
        &mut self,
        count: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.operations.checked_add(count).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::Operations,
                u64::MAX,
                self.limits.max_operations,
            )
        })?;

        self.limits.check_operations(next)?;
        self.operations = next;

        Ok(())
    }

    pub fn try_add_work(
        &mut self,
        units: u64,
    ) -> Result<(), FrontendLimitViolation> {
        let next = self.total_work.checked_add(units).ok_or_else(|| {
            FrontendLimitViolation::new(
                FrontendLimitKind::TotalWork,
                u64::MAX,
                self.limits.max_total_work,
            )
        })?;

        self.limits.check_total_work(next)?;
        self.total_work = next;

        Ok(())
    }

    #[must_use]
    pub const fn source_bytes(&self) -> u64 {
        self.source_bytes
    }

    #[must_use]
    pub const fn total_source_bytes(&self) -> u64 {
        self.total_source_bytes
    }

    #[must_use]
    pub const fn source_files(&self) -> u64 {
        self.source_files
    }

    #[must_use]
    pub const fn tokens(&self) -> u64 {
        self.tokens
    }

    #[must_use]
    pub const fn ast_nodes(&self) -> u64 {
        self.ast_nodes
    }

    #[must_use]
    pub const fn expression_nodes(&self) -> u64 {
        self.expression_nodes
    }

    #[must_use]
    pub const fn diagnostics(&self) -> u64 {
        self.diagnostics
    }

    #[must_use]
    pub const fn diagnostic_children(&self) -> u64 {
        self.diagnostic_children
    }

    #[must_use]
    pub const fn include_edges(&self) -> u64 {
        self.include_edges
    }

    #[must_use]
    pub const fn gate_definitions(&self) -> u64 {
        self.gate_definitions
    }

    #[must_use]
    pub const fn gate_operations(&self) -> u64 {
        self.gate_operations
    }

    #[must_use]
    pub const fn symbols(&self) -> u64 {
        self.symbols
    }

    #[must_use]
    pub const fn statements(&self) -> u64 {
        self.statements
    }

    #[must_use]
    pub const fn operations(&self) -> u64 {
        self.operations
    }

    #[must_use]
    pub const fn total_work(&self) -> u64 {
        self.total_work
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_valid() {
        assert!(FrontendLimits::production().validate().is_ok());
    }

    #[test]
    fn strict_policy_is_valid() {
        assert!(FrontendLimits::strict().validate().is_ok());
    }

    #[test]
    fn large_policy_is_valid() {
        assert!(FrontendLimits::large().validate().is_ok());
    }

    #[test]
    fn zero_limit_is_rejected() {
        let result = FrontendLimits::builder()
            .max_tokens(0)
            .build();

        assert_eq!(
            result,
            Err(FrontendLimitConfigError::ZeroLimit {
                field: "max_tokens"
            })
        );
    }

    #[test]
    fn aggregate_source_limit_is_validated() {
        let result = FrontendLimits::builder()
            .max_source_bytes(10)
            .max_total_source_bytes(9)
            .build();

        assert_eq!(
            result,
            Err(
                FrontendLimitConfigError::
                    TotalSourceBytesLessThanSingleSource {
                        max_source_bytes: 10,
                        max_total_source_bytes: 9,
                    }
            )
        );
    }

    #[test]
    fn expression_depth_is_bounded_by_nesting_depth() {
        let result = FrontendLimits::builder()
            .max_nesting_depth(4)
            .max_expression_depth(5)
            .build();

        assert_eq!(
            result,
            Err(
                FrontendLimitConfigError::
                    ExpressionDepthExceedsNestingDepth {
                        max_expression_depth: 5,
                        max_nesting_depth: 4,
                    }
            )
        );
    }

    #[test]
    fn builder_changes_requested_values() {
        let limits = FrontendLimits::builder()
            .max_tokens(123)
            .max_operations(456)
            .build()
            .expect("configuration must be valid");

        assert_eq!(limits.max_tokens(), 123);
        assert_eq!(limits.max_operations(), 456);
    }

    #[test]
    fn allow_and_check_are_consistent() {
        let limits = FrontendLimits::builder()
            .max_tokens(2)
            .build()
            .expect("configuration must be valid");

        assert!(limits.allows_tokens(2));
        assert!(!limits.allows_tokens(3));

        assert!(limits.check_tokens(2).is_ok());

        let violation = limits
            .check_tokens(3)
            .expect_err("three tokens must exceed a two-token limit");

        assert_eq!(
            violation.kind(),
            FrontendLimitKind::Tokens
        );
        assert_eq!(violation.actual(), 3);
        assert_eq!(violation.maximum(), 2);
    }

    #[test]
    fn budget_source_failure_is_atomic() {
        let limits = FrontendLimits::builder()
            .max_source_bytes(4)
            .max_total_source_bytes(8)
            .max_source_files(2)
            .build()
            .expect("configuration must be valid");

        let mut budget = FrontendBudget::new(limits);

        budget
            .try_add_source(4)
            .expect("first source must fit");

        assert!(budget.try_add_source(5).is_err());

        assert_eq!(budget.source_bytes(), 4);
        assert_eq!(budget.total_source_bytes(), 4);
        assert_eq!(budget.source_files(), 1);
    }

    #[test]
    fn budget_counters_cannot_wrap() {
        let limits = FrontendLimits::builder()
            .max_tokens(u64::MAX)
            .build()
            .expect("configuration must be valid");

        let mut budget = FrontendBudget::new(limits);

        budget
            .try_add_tokens(u64::MAX)
            .expect("maximum must fit");

        let violation = budget
            .try_add_tokens(1)
            .expect_err("counter overflow must be rejected");

        assert_eq!(
            violation.kind(),
            FrontendLimitKind::Tokens
        );
    }

    #[test]
    fn limit_kind_display_is_stable() {
        assert_eq!(
            FrontendLimitKind::SourceBytes.to_string(),
            "source-bytes"
        );

        assert_eq!(
            FrontendLimitKind::TotalWork.to_string(),
            "total-work"
        );
    }
}