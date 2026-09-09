//! # Zamani Native AST — Expression
//!
//! Canonical source-level representation of expressions in the Zamani
//! frontend AST.
//!
//! ## Architectural position
//!
//! ```text
//! Zamani source
//!     │
//!     ▼
//! lexer
//!     │
//!     ▼
//! parser
//!     │
//!     ▼
//! Expression  ← this module
//!     │
//!     ▼
//! structural AST validation
//!     │
//!     ▼
//! semantic analysis
//!     │
//!     ▼
//! semantic model
//!     │
//!     ▼
//! ZUIR
//!     │
//!     ├── classical IR
//!     ├── quantum IR
//!     ├── HDL IR
//!     └── future domain IRs
//! ```
//!
//! ## Responsibility
//!
//! This module owns the **source-level expression node**.
//!
//! It describes what an expression is structurally, without describing how
//! that expression will eventually be implemented on:
//!
//! - a CPU;
//! - a GPU;
//! - an FPGA;
//! - an ASIC;
//! - a QPU;
//! - a simulator;
//! - a distributed system;
//! - a particular quantum technology;
//! - a particular vendor;
//! - a particular instruction set;
//! - a particular topology;
//! - a particular runtime.
//!
//! The AST is therefore compatible with the Zamani
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF) architectural goal.
//!
//! ## Critical design rule
//!
//! This type is a **source AST node**, not an IR.
//!
//! It must not contain:
//!
//! - semantic types;
//! - resolved symbols;
//! - SSA values;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - physical qubits;
//! - logical-qubit allocation;
//! - hardware topology;
//! - routing decisions;
//! - scheduling decisions;
//! - calibration data;
//! - error-correction implementation;
//! - resilience implementation;
//! - backend identifiers;
//! - runtime state.
//!
//! Those belong to later compiler layers.
//!
//! ## Child representation
//!
//! Child AST nodes are represented by [`NodeId`] rather than recursively
//! embedding concrete AST node values.
//!
//! This is an intentional property of the canonical Zamani AST graph:
//!
//! ```text
//! Expression
//! ├── Node
//! └── ExpressionKind
//!      ├── child NodeId
//!      ├── child NodeId
//!      └── ...
//! ```
//!
//! The actual AST graph owns the nodes. This module owns only the expression's
//! local structure.
//!
//! This avoids recursive Rust ownership structures and permits AST graph
//! traversal to be implemented independently.
//!
//! ## Extensibility
//!
//! The core expression vocabulary deliberately contains only source-language
//! concepts that are sufficiently fundamental to Zamani.
//!
//! Domain-specific constructs are represented by [`ExpressionKind::Extension`].
//!
//! For example, a quantum frontend extension may represent a quantum-specific
//! expression through a namespaced extension:
//!
//! ```text
//! namespace = "zamani.quantum"
//! name      = "operation"
//! ```
//!
//! This module does not need to be modified when a new quantum technology,
//! accelerator, computational domain, or language extension is introduced.
//!
//! The extension itself is responsible for its own schema and semantic
//! interpretation.
//!
//! ## No hardware coupling
//!
//! A quantum operation is deliberately **not** represented as:
//!
//! ```text
//! enum QuantumGate {
//!     X,
//!     H,
//!     CNOT,
//!     ...
//! }
//! ```
//!
//! Such an enum would make the AST's operation vocabulary a closed list and
//! would incorrectly couple source syntax to a particular gate taxonomy.
//!
//! Instead, generic calls and namespaced extensions represent source intent.
//!
//! Actual operation legality, decomposition, routing, scheduling and physical
//! realization are downstream concerns.
//!
//! ## Legacy AST migration
//!
//! The legacy `src/ast/mod.rs` contained an expression representation based on:
//!
//! ```text
//! Prefix(Span, TokenType, Box<Expression>)
//! Infix(Span, Box<Expression>, TokenType, Box<Expression>)
//! ```
//!
//! That representation is intentionally not reproduced here.
//!
//! Operators are source-level semantic identities represented by [`OperatorRef`]
//! rather than lexer implementation types. This removes a dependency from the
//! AST to the lexer.
//!
//! The legacy AST also embedded recursive expressions directly and contained
//! technology-specific variants such as `QuantumOp`, `Entangle`, and `NanoOp`.
//! Those are migrated to generic/extensible source representations.
//!
//! ## Source spans
//!
//! Every expression owns a common [`Node`] containing its source span.
//!
//! The expression itself therefore does not duplicate source-coordinate logic.
//!
//! ## Validation boundary
//!
//! [`Expression::validate_structure`] performs only local structural checks.
//!
//! It does not perform:
//!
//! - type checking;
//! - name resolution;
//! - overload resolution;
//! - generic substitution;
//! - borrow checking;
//! - ownership checking;
//! - capability resolution;
//! - resource allocation;
//! - quantum legality checking;
//! - hardware compatibility;
//! - backend selection.
//!
//! Those belong to semantic or later compiler stages.
//!
//! ## Traversal
//!
//! Local child enumeration is provided by [`Expression::child_node_ids`].
//!
//! The method is deliberately non-recursive.
//!
//! An entire AST can therefore be traversed by an external graph walker without
//! requiring this expression node to recurse through the Rust call stack.
//!
//! ## Scalability
//!
//! No expression-count, operand-count, argument-count, register-size, qubit-count
//! or machine-size constant exists in this module.
//!
//! Collection sizes are represented by normal dynamically sized Rust
//! collections. Practical limits belong to compiler resource-policy layers.
//!
//! "Infinity" in the POCO-REAF architecture means that this AST layer introduces
//! no artificial finite machine or program-size limit. Actual compilation
//! remains bounded by available address space, memory, storage, compiler
//! resource policy and the representational limits of the executing platform.
//!
//! ## Determinism
//!
//! Source-order collections use [`Vec`].
//!
//! There is no hash-map-backed semantic storage in this module.
//!
//! Extension identifiers are represented explicitly and deterministically.
//!
//! ## Serialization
//!
//! All public expression structures derive Serde serialization.
//!
//! Schema versioning belongs to the AST serialization subsystem and is not
//! duplicated in each individual expression node.
//!
//! ## Security
//!
//! This module:
//!
//! - forbids unsafe Rust;
//! - does not dereference raw pointers;
//! - does not perform I/O;
//! - does not execute calls;
//! - does not access global mutable state;
//! - does not perform unchecked indexing;
//! - does not recursively traverse children;
//! - does not impose machine-specific resource limits.
//!
//! ## Rust compatibility
//!
//! Designed for:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Integration contract
//!
//! ### `node.rs`
//!
//! Provides [`Node`], which owns:
//!
//! - [`NodeId`];
//! - [`NodeKind`];
//! - [`Span`];
//! - [`NodeMetadata`].
//!
//! ### `node_id.rs`
//!
//! Owns stable AST identity.
//!
//! ### `node_kind.rs`
//!
//! Owns source-level expression classification.
//!
//! This file uses existing `CoreNodeKind` values rather than introducing a
//! second expression-kind taxonomy.
//!
//! ### `source`
//!
//! Owns source coordinates and spans.
//!
//! ### parser
//!
//! Constructs expressions after syntactic parsing and supplies:
//!
//! - node identity;
//! - source span;
//! - expression kind;
//! - child node references.
//!
//! The parser must not resolve semantic types or hardware.
//!
//! ### structural validation
//!
//! Calls [`Expression::validate_structure`] and validates the referenced
//! child nodes through the AST graph.
//!
//! ### semantic analysis
//!
//! Consumes validated expressions and resolves:
//!
//! - identifiers;
//! - operators;
//! - types;
//! - calls;
//! - overloads;
//! - effects;
//! - capabilities;
//! - resources;
//! - domain meaning.
//!
//! ### ZUIR
//!
//! Expression semantics are lowered downstream into ZUIR.
//!
//! This file must not import ZUIR.
//!
//! ### quantum frontend extensions
//!
//! Quantum-specific syntax can be represented using
//! [`ExpressionKind::Extension`] or generic expression constructs.
//!
//! This prevents the native AST from becoming a quantum-backend AST.
//!
//! ### optimization
//!
//! Optimizers may transform expressions only through explicit AST transformation
//! infrastructure or semantic IR passes while preserving source semantics.
//!
//! This file contains no optimization policy.
//!
//! ## No-re-edit integration guarantee
//!
//! The public contract of this file is intentionally self-contained:
//!
//! - expression identity is owned by [`Node`];
//! - children are identified by [`NodeId`];
//! - operators have an extensible source-level representation;
//! - extension expressions have namespaced identities;
//! - validation is local;
//! - traversal is local;
//! - semantic interpretation remains downstream.
//!
//! Ordinary implementation of parser, semantic analysis, ZUIR, quantum
//! extensions, optimization or hardware backends should therefore consume this
//! contract rather than modifying this file.
//!
//! A new source-level expression primitive should be added here only when it is
//! genuinely part of the Zamani language core. A domain-specific feature should
//! normally use the extension mechanism instead.
//!
//! # File contract
//!
//! **Inputs**
//!
//! - foundational AST metadata;
//! - `NodeId` child references;
//! - source-level operator/extension identities.
//!
//! **Outputs**
//!
//! - canonical `Expression` nodes;
//! - deterministic child enumeration;
//! - local structural validation.
//!
//! **Forbidden dependencies**
//!
//! - semantic analysis;
//! - ZUIR;
//! - quantum IR;
//! - quantum hardware;
//! - optimization;
//! - routing;
//! - scheduling;
//! - calibration;
//! - QEC;
//! - resilience;
//! - runtime;
//! - backend providers.
//!
//! **Thread safety**
//!
//! No global mutable state is used. Thread-safety follows from the contained
//! types. Immutable expressions can therefore be shared between compiler phases
//! when their constituent types are `Send + Sync`.
//!
//! **Source preservation**
//!
//! The common [`Node`] preserves the canonical source span and metadata.
//!
//! **Semantic preservation**
//!
//! This module does not discard source-level distinctions required by later
//! semantic analysis.
//!
//! **Target independence**
//!
//! No machine, device, processor, topology, qubit count, register width or
//! vendor appears in the expression representation.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::metadata::NodeMetadata;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};
use super::super::source::Span;

// =============================================================================
// Constants
// =============================================================================

/// Schema version of the local expression contract.
///
/// This is intentionally separate from the overall AST schema version and the
/// Zamani language version.
pub const EXPRESSION_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type used by expression construction and validation.
pub type ExpressionResult<T> = Result<T, ExpressionError>;

/// Structural errors that can be detected without semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExpressionError {
    /// The common node has a non-expression node kind.
    InvalidNodeKind {
        /// Actual node kind.
        actual: NodeKind,
    },

    /// A required expression child is missing.
    MissingChild {
        /// Logical name of the missing child.
        field: &'static str,
    },

    /// An extension namespace is empty.
    EmptyExtensionNamespace,

    /// An extension name is empty.
    EmptyExtensionName,

    /// An operator namespace is empty.
    EmptyOperatorNamespace,

    /// An operator name is empty.
    EmptyOperatorName,

    /// An operator identifier contains an invalid namespace/name pair.
    InvalidOperatorIdentity,

    /// A node ID appears more than once where uniqueness is required.
    DuplicateChild {
        /// Duplicated child ID.
        id: NodeId,
    },

    /// A structurally invalid child reference was supplied.
    InvalidChildReference {
        /// Logical child field.
        field: &'static str,
    },

    /// A configured validation limit was exceeded.
    LimitExceeded {
        /// Name of the limit.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Maximum permitted by the caller's policy.
        maximum: usize,
    },

    /// An extension expression has no operands and no attributes when its
    /// contract requires at least one structural component.
    InvalidExtensionStructure,
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "expression node has invalid AST node kind: {actual}"
                )
            }

            Self::MissingChild { field } => {
                write!(formatter, "expression is missing required child: {field}")
            }

            Self::EmptyExtensionNamespace => {
                formatter.write_str("expression extension namespace is empty")
            }

            Self::EmptyExtensionName => {
                formatter.write_str("expression extension name is empty")
            }

            Self::EmptyOperatorNamespace => {
                formatter.write_str("operator namespace is empty")
            }

            Self::EmptyOperatorName => {
                formatter.write_str("operator name is empty")
            }

            Self::InvalidOperatorIdentity => {
                formatter.write_str("operator identity is structurally invalid")
            }

            Self::DuplicateChild { id } => {
                write!(formatter, "expression contains duplicate child node id: {id:?}")
            }

            Self::InvalidChildReference { field } => {
                write!(
                    formatter,
                    "expression contains invalid child reference in field: {field}"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "expression validation limit `{limit}` exceeded: {actual} > {maximum}"
                )
            }

            Self::InvalidExtensionStructure => {
                formatter.write_str("expression extension has invalid structure")
            }
        }
    }
}

impl std::error::Error for ExpressionError {}

// =============================================================================
// Validation policy
// =============================================================================

/// Caller-supplied structural validation policy.
///
/// These values are **compiler safety policy**, not language or machine limits.
///
/// `None` means that this particular policy imposes no limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExpressionValidationPolicy {
    /// Maximum number of direct child references inspected by local validation.
    pub max_children: Option<usize>,

    /// Maximum byte length of extension namespaces.
    pub max_extension_namespace_bytes: Option<usize>,

    /// Maximum byte length of extension names.
    pub max_extension_name_bytes: Option<usize>,

    /// Maximum byte length of operator namespaces.
    pub max_operator_namespace_bytes: Option<usize>,

    /// Maximum byte length of operator names.
    pub max_operator_name_bytes: Option<usize>,
}

impl Default for ExpressionValidationPolicy {
    fn default() -> Self {
        Self {
            max_children: None,
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_operator_namespace_bytes: None,
            max_operator_name_bytes: None,
        }
    }
}

impl ExpressionValidationPolicy {
    /// Creates an unrestricted expression policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_children: None,
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_operator_namespace_bytes: None,
            max_operator_name_bytes: None,
        }
    }

    /// Creates a bounded policy.
    #[must_use]
    pub const fn bounded(max_children: usize) -> Self {
        Self {
            max_children: Some(max_children),
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
            max_operator_namespace_bytes: None,
            max_operator_name_bytes: None,
        }
    }

    /// Adds an extension namespace limit.
    #[must_use]
    pub const fn with_max_extension_namespace_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_extension_namespace_bytes = Some(maximum);
        self
    }

    /// Adds an extension name limit.
    #[must_use]
    pub const fn with_max_extension_name_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_extension_name_bytes = Some(maximum);
        self
    }

    /// Adds an operator namespace limit.
    #[must_use]
    pub const fn with_max_operator_namespace_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_operator_namespace_bytes = Some(maximum);
        self
    }

    /// Adds an operator name limit.
    #[must_use]
    pub const fn with_max_operator_name_bytes(
        mut self,
        maximum: usize,
    ) -> Self {
        self.max_operator_name_bytes = Some(maximum);
        self
    }

    /// Validates that policy values themselves are usable.
    pub fn validate(self) -> ExpressionResult<()> {
        if matches!(self.max_children, Some(0)) {
            return Err(ExpressionError::LimitExceeded {
                limit: "max_children",
                actual: 0,
                maximum: 0,
            });
        }

        if matches!(self.max_extension_namespace_bytes, Some(0)) {
            return Err(ExpressionError::LimitExceeded {
                limit: "max_extension_namespace_bytes",
                actual: 0,
                maximum: 0,
            });
        }

        if matches!(self.max_extension_name_bytes, Some(0)) {
            return Err(ExpressionError::LimitExceeded {
                limit: "max_extension_name_bytes",
                actual: 0,
                maximum: 0,
            });
        }

        if matches!(self.max_operator_namespace_bytes, Some(0)) {
            return Err(ExpressionError::LimitExceeded {
                limit: "max_operator_namespace_bytes",
                actual: 0,
                maximum: 0,
            });
        }

        if matches!(self.max_operator_name_bytes, Some(0)) {
            return Err(ExpressionError::LimitExceeded {
                limit: "max_operator_name_bytes",
                actual: 0,
                maximum: 0,
            });
        }

        Ok(())
    }
}

// =============================================================================
// Operator identity
// =============================================================================

/// Extensible source-level operator identity.
///
/// Operators are deliberately not represented by lexer token types.
///
/// The lexer recognizes source syntax; this type represents the operator
/// identity after parsing.
///
/// The namespace/name model prevents the AST from becoming a closed enum of
/// every operator that Zamani may ever support.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct OperatorRef {
    /// Operator namespace.
    ///
    /// The empty namespace is reserved for the native/default operator space.
    namespace: String,

    /// Operator name.
    name: String,
}

impl OperatorRef {
    /// Creates an operator reference.
    ///
    /// Empty names are rejected because an operator must have a stable
    /// source-level identity.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> ExpressionResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if name.is_empty() {
            return Err(ExpressionError::EmptyOperatorName);
        }

        if namespace.contains('\0') || name.contains('\0') {
            return Err(ExpressionError::InvalidOperatorIdentity);
        }

        Ok(Self { namespace, name })
    }

    /// Creates an operator in the native/default namespace.
    pub fn native(name: impl Into<String>) -> ExpressionResult<Self> {
        Self::new(String::new(), name)
    }

    /// Returns the operator namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the operator name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether this operator belongs to the default/native namespace.
    #[must_use]
    pub fn is_native(&self) -> bool {
        self.namespace.is_empty()
    }

    /// Returns a deterministic qualified operator name.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            let mut result =
                String::with_capacity(self.namespace.len() + 1 + self.name.len());

            result.push_str(&self.namespace);
            result.push(':');
            result.push_str(&self.name);

            result
        }
    }

    /// Performs local structural validation.
    pub fn validate(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<()> {
        if self.name.is_empty() {
            return Err(ExpressionError::EmptyOperatorName);
        }

        if self.namespace.contains('\0') || self.name.contains('\0') {
            return Err(ExpressionError::InvalidOperatorIdentity);
        }

        if let Some(maximum) = policy.max_operator_namespace_bytes {
            if self.namespace.len() > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_operator_namespace_bytes",
                    actual: self.namespace.len(),
                    maximum,
                });
            }
        }

        if let Some(maximum) = policy.max_operator_name_bytes {
            if self.name.len() > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_operator_name_bytes",
                    actual: self.name.len(),
                    maximum,
                });
            }
        }

        Ok(())
    }
}

impl fmt::Display for OperatorRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.qualified_name())
    }
}

// =============================================================================
// Expression extension identity
// =============================================================================

/// Namespaced source-level expression extension.
///
/// The AST core treats the identity as opaque.
///
/// Semantic interpretation belongs to the extension registry/semantic layer.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
pub struct ExpressionExtension {
    /// Extension namespace.
    namespace: String,

    /// Extension construct name.
    name: String,

    /// Optional source-level extension payload/arguments.
    ///
    /// Child nodes remain references in the AST graph.
    operands: Vec<NodeId>,

    /// Source-level attributes attached to the extension construct.
    attributes: Vec<NodeId>,
}

impl ExpressionExtension {
    /// Creates an expression extension.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        operands: Vec<NodeId>,
        attributes: Vec<NodeId>,
    ) -> ExpressionResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.is_empty() {
            return Err(ExpressionError::EmptyExtensionNamespace);
        }

        if name.is_empty() {
            return Err(ExpressionError::EmptyExtensionName);
        }

        if namespace.contains('\0') || name.contains('\0') {
            return Err(ExpressionError::InvalidExtensionStructure);
        }

        Ok(Self {
            namespace,
            name,
            operands,
            attributes,
        })
    }

    /// Returns the extension namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the complete qualified extension identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        let mut result =
            String::with_capacity(self.namespace.len() + 1 + self.name.len());

        result.push_str(&self.namespace);
        result.push(':');
        result.push_str(&self.name);

        result
    }

    /// Returns extension operands in source order.
    #[must_use]
    pub fn operands(&self) -> &[NodeId] {
        &self.operands
    }

    /// Returns extension attributes in source order.
    #[must_use]
    pub fn attributes(&self) -> &[NodeId] {
        &self.attributes
    }

    /// Returns the number of structural child references owned by this
    /// extension.
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.operands.len() + self.attributes.len()
    }

    /// Performs local structural validation.
    pub fn validate(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<()> {
        if self.namespace.is_empty() {
            return Err(ExpressionError::EmptyExtensionNamespace);
        }

        if self.name.is_empty() {
            return Err(ExpressionError::EmptyExtensionName);
        }

        if self.namespace.contains('\0') || self.name.contains('\0') {
            return Err(ExpressionError::InvalidExtensionStructure);
        }

        if let Some(maximum) = policy.max_extension_namespace_bytes {
            if self.namespace.len() > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_extension_namespace_bytes",
                    actual: self.namespace.len(),
                    maximum,
                });
            }
        }

        if let Some(maximum) = policy.max_extension_name_bytes {
            if self.name.len() > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_extension_name_bytes",
                    actual: self.name.len(),
                    maximum,
                });
            }
        }

        if let Some(maximum) = policy.max_children {
            let count = self.child_count();

            if count > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_children",
                    actual: count,
                    maximum,
                });
            }
        }

        Ok(())
    }

    /// Returns extension child node IDs in deterministic source order.
    ///
    /// Operands are followed by attributes.
    #[must_use]
    pub fn child_node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.operands
            .iter()
            .copied()
            .chain(self.attributes.iter().copied())
    }
}

// =============================================================================
// Expression kind
// =============================================================================

/// Source-level expression structure.
///
/// All relationships to other AST nodes are represented by [`NodeId`].
///
/// This keeps the expression graph independent from the concrete Rust layout
/// of other AST node types.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
pub enum ExpressionKind {
    /// Reference to a source-level identifier node.
    Identifier {
        /// Identifier AST node.
        identifier: NodeId,
    },

    /// Reference to a literal AST node.
    Literal {
        /// Literal AST node.
        literal: NodeId,
    },

    /// Prefix/unary operation.
    Unary {
        /// Operator identity.
        operator: OperatorRef,

        /// Operand expression.
        operand: NodeId,
    },

    /// Binary/infix operation.
    Binary {
        /// Left operand.
        left: NodeId,

        /// Operator identity.
        operator: OperatorRef,

        /// Right operand.
        right: NodeId,
    },

    /// Conditional expression.
    ///
    /// `condition`, `then_branch`, and optional `else_branch` are AST node
    /// references.
    Conditional {
        /// Condition expression.
        condition: NodeId,

        /// Then branch.
        then_branch: NodeId,

        /// Optional else branch.
        else_branch: Option<NodeId>,
    },

    /// Block expression.
    ///
    /// Child nodes are maintained in source order.
    Block {
        /// Statements/items belonging to the block.
        statements: Vec<NodeId>,
    },

    /// Match expression.
    ///
    /// The actual pattern/arm nodes belong to the corresponding AST modules.
    Match {
        /// Scrutinee expression.
        scrutinee: NodeId,

        /// Match-arm nodes in source order.
        arms: Vec<NodeId>,
    },

    /// Loop expression.
    Loop {
        /// Loop body expression.
        body: NodeId,
    },

    /// Function/lambda/closure expression.
    Lambda {
        /// Parameter AST nodes in source order.
        parameters: Vec<NodeId>,

        /// Lambda body.
        body: NodeId,
    },

    /// General callable expression.
    ///
    /// The callee may be a function, closure, operation, generic callable,
    /// resource operation or another source-level callable construct.
    Call {
        /// Callable expression.
        callee: NodeId,

        /// Arguments in source order.
        arguments: Vec<NodeId>,
    },

    /// Member/property access.
    MemberAccess {
        /// Base expression.
        object: NodeId,

        /// Member identifier AST node.
        member: NodeId,
    },

    /// Indexing expression.
    Index {
        /// Indexed expression.
        object: NodeId,

        /// Index expression.
        index: NodeId,
    },

    /// Range expression.
    Range {
        /// Start expression.
        start: Option<NodeId>,

        /// End expression.
        end: Option<NodeId>,

        /// Whether the end is inclusive.
        inclusive: bool,
    },

    /// Array/list expression.
    Array {
        /// Elements in source order.
        elements: Vec<NodeId>,
    },

    /// Tuple expression.
    Tuple {
        /// Elements in source order.
        elements: Vec<NodeId>,
    },

    /// Struct/object literal expression.
    Struct {
        /// Type/constructor identifier.
        type_name: NodeId,

        /// Field values in source order.
        fields: Vec<NodeId>,
    },

    /// Type cast.
    Cast {
        /// Expression being cast.
        expression: NodeId,

        /// Type AST node.
        target_type: NodeId,
    },

    /// Type ascription.
    TypeAscription {
        /// Expression being ascribed.
        expression: NodeId,

        /// Type AST node.
        type_node: NodeId,
    },

    /// Assignment expression.
    Assignment {
        /// Assignment target.
        target: NodeId,

        /// Assigned value.
        value: NodeId,
    },

    /// Compound assignment.
    CompoundAssignment {
        /// Assignment target.
        target: NodeId,

        /// Operator used for the compound assignment.
        operator: OperatorRef,

        /// Assigned value.
        value: NodeId,
    },

    /// Error-propagation/try expression.
    Try {
        /// Expression whose result is propagated.
        expression: NodeId,
    },

    /// Await expression.
    Await {
        /// Awaited expression.
        expression: NodeId,
    },

    /// Async expression.
    Async {
        /// Async body.
        body: NodeId,
    },

    /// Spawn/concurrency expression.
    Spawn {
        /// Spawned computation.
        expression: NodeId,
    },

    /// Construction expression.
    Construction {
        /// Type/constructor identifier.
        type_name: NodeId,

        /// Constructor arguments.
        arguments: Vec<NodeId>,
    },

    /// A source-level extension expression.
    ///
    /// This is the primary extensibility boundary for quantum, accelerator,
    /// HDL and future domain-specific syntax.
    Extension(ExpressionExtension),
}

// =============================================================================
// Expression
// =============================================================================

/// Canonical native Zamani expression AST node.
///
/// The common [`Node`] is authoritative for:
///
/// - identity;
/// - source span;
/// - node classification;
/// - metadata.
///
/// [`ExpressionKind`] is authoritative for expression-specific structure.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct Expression {
    /// Common AST identity and source metadata.
    node: Node,

    /// Source-level expression structure.
    kind: ExpressionKind,
}

impl Expression {
    /// Creates an expression from an already-constructed common [`Node`].
    ///
    /// The node must have an appropriate expression `CoreNodeKind`.
    ///
    /// The constructor intentionally does not allocate node identity. Identity
    /// allocation belongs to the parser/AST construction layer.
    #[must_use]
    pub fn from_node(
        node: Node,
        kind: ExpressionKind,
    ) -> Self {
        Self { node, kind }
    }

    /// Creates an expression using a canonical native [`CoreNodeKind`].
    ///
    /// The caller supplies the identity and source span because these are
    /// foundational AST concerns and must not be silently manufactured.
    #[must_use]
    pub fn new(
        id: NodeId,
        kind: ExpressionKind,
        span: Span,
        metadata: NodeMetadata,
    ) -> Self {
        let node_kind = Self::node_kind_for(&kind);

        let node = Node::new(
            id,
            NodeKind::core(node_kind),
            span,
            metadata,
        );

        Self { node, kind }
    }

    /// Creates an expression with default metadata.
    #[must_use]
    pub fn without_metadata(
        id: NodeId,
        kind: ExpressionKind,
        span: Span,
    ) -> Self {
        Self::new(
            id,
            kind,
            span,
            NodeMetadata::default(),
        )
    }

    /// Returns the common AST node.
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the common node metadata.
    ///
    /// Identity, kind and source span remain controlled by [`Node`].
    #[must_use]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identity.
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the source span.
    #[must_use]
    pub fn span(&self) -> &Span {
        self.node.span()
    }

    /// Returns the source span by value.
    #[must_use]
    pub fn span_owned(&self) -> Span {
        self.node.span_owned()
    }

    /// Returns source-level metadata.
    #[must_use]
    pub fn metadata(&self) -> &NodeMetadata {
        self.node.metadata()
    }

    /// Returns the expression structure.
    #[must_use]
    pub fn kind(&self) -> &ExpressionKind {
        &self.kind
    }

    /// Returns mutable access to expression structure.
    ///
    /// Callers performing structural mutation remain responsible for preserving
    /// the local expression/node-kind invariant. Prefer dedicated AST
    /// transformation infrastructure for large-scale transformations.
    #[must_use]
    pub fn kind_mut(&mut self) -> &mut ExpressionKind {
        &mut self.kind
    }

    /// Consumes the expression and returns its common node.
    #[must_use]
    pub fn into_node(self) -> Node {
        self.node
    }

    /// Consumes the expression and returns its expression structure.
    #[must_use]
    pub fn into_kind(self) -> ExpressionKind {
        self.kind
    }

    /// Returns the canonical AST node kind corresponding to an expression
    /// structure.
    #[must_use]
    pub const fn node_kind_for(
        kind: &ExpressionKind,
    ) -> CoreNodeKind {
        match kind {
            ExpressionKind::Identifier { .. } => CoreNodeKind::Identifier,

            ExpressionKind::Literal { .. } => CoreNodeKind::Literal,

            ExpressionKind::Unary { .. } => CoreNodeKind::UnaryExpression,

            ExpressionKind::Binary { .. } => CoreNodeKind::BinaryExpression,

            ExpressionKind::Conditional { .. } => {
                CoreNodeKind::ConditionalExpression
            }

            ExpressionKind::Block { .. } => {
                CoreNodeKind::BlockExpression
            }

            ExpressionKind::Match { .. } => {
                CoreNodeKind::MatchExpression
            }

            ExpressionKind::Loop { .. } => {
                CoreNodeKind::LoopExpression
            }

            ExpressionKind::Lambda { .. } => {
                CoreNodeKind::LambdaExpression
            }

            ExpressionKind::Call { .. } => {
                CoreNodeKind::CallExpression
            }

            ExpressionKind::MemberAccess { .. } => {
                CoreNodeKind::MemberAccess
            }

            ExpressionKind::Index { .. } => {
                CoreNodeKind::IndexExpression
            }

            ExpressionKind::Range { .. } => {
                CoreNodeKind::RangeExpression
            }

            ExpressionKind::Array { .. } => {
                CoreNodeKind::ArrayExpression
            }

            ExpressionKind::Tuple { .. } => {
                CoreNodeKind::TupleExpression
            }

            ExpressionKind::Struct { .. } => {
                CoreNodeKind::StructExpression
            }

            ExpressionKind::Cast { .. } => {
                CoreNodeKind::CastExpression
            }

            ExpressionKind::TypeAscription { .. } => {
                CoreNodeKind::TypeAscription
            }

            ExpressionKind::Assignment { .. }
            | ExpressionKind::CompoundAssignment { .. } => {
                CoreNodeKind::Assignment
            }

            ExpressionKind::Try { .. } => {
                CoreNodeKind::CallExpression
            }

            ExpressionKind::Await { .. } => {
                CoreNodeKind::AwaitExpression
            }

            ExpressionKind::Async { .. } => {
                CoreNodeKind::AsyncExpression
            }

            ExpressionKind::Spawn { .. } => {
                CoreNodeKind::SpawnExpression
            }

            ExpressionKind::Construction { .. } => {
                CoreNodeKind::ConstructionExpression
            }

            ExpressionKind::Extension(_) => {
                CoreNodeKind::Opaque
            }
        }
    }

    /// Returns all direct child AST node IDs in deterministic source order.
    ///
    /// This method never recursively traverses child expressions.
    ///
    /// The returned ordering is the canonical local traversal ordering used by
    /// expression visitors:
    ///
    /// ```text
    /// source order
    /// ```
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        match &self.kind {
            ExpressionKind::Identifier { identifier } => {
                vec![*identifier]
            }

            ExpressionKind::Literal { literal } => {
                vec![*literal]
            }

            ExpressionKind::Unary { operand, .. } => {
                vec![*operand]
            }

            ExpressionKind::Binary { left, right, .. } => {
                vec![*left, *right]
            }

            ExpressionKind::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                let mut children = Vec::with_capacity(
                    2 + usize::from(else_branch.is_some()),
                );

                children.push(*condition);
                children.push(*then_branch);

                if let Some(branch) = else_branch {
                    children.push(*branch);
                }

                children
            }

            ExpressionKind::Block { statements } => {
                statements.clone()
            }

            ExpressionKind::Match { scrutinee, arms } => {
                let mut children = Vec::with_capacity(
                    1usize.saturating_add(arms.len()),
                );

                children.push(*scrutinee);
                children.extend(arms.iter().copied());

                children
            }

            ExpressionKind::Loop { body } => {
                vec![*body]
            }

            ExpressionKind::Lambda {
                parameters,
                body,
            } => {
                let mut children = Vec::with_capacity(
                    parameters.len().saturating_add(1),
                );

                children.extend(parameters.iter().copied());
                children.push(*body);

                children
            }

            ExpressionKind::Call {
                callee,
                arguments,
            } => {
                let mut children = Vec::with_capacity(
                    arguments.len().saturating_add(1),
                );

                children.push(*callee);
                children.extend(arguments.iter().copied());

                children
            }

            ExpressionKind::MemberAccess {
                object,
                member,
            } => {
                vec![*object, *member]
            }

            ExpressionKind::Index { object, index } => {
                vec![*object, *index]
            }

            ExpressionKind::Range {
                start,
                end,
                ..
            } => {
                let mut children =
                    Vec::with_capacity(usize::from(start.is_some()) + usize::from(end.is_some()));

                if let Some(start) = start {
                    children.push(*start);
                }

                if let Some(end) = end {
                    children.push(*end);
                }

                children
            }

            ExpressionKind::Array { elements }
            | ExpressionKind::Tuple { elements } => {
                elements.clone()
            }

            ExpressionKind::Struct {
                type_name,
                fields,
            } => {
                let mut children =
                    Vec::with_capacity(fields.len().saturating_add(1));

                children.push(*type_name);
                children.extend(fields.iter().copied());

                children
            }

            ExpressionKind::Cast {
                expression,
                target_type,
            } => {
                vec![*expression, *target_type]
            }

            ExpressionKind::TypeAscription {
                expression,
                type_node,
            } => {
                vec![*expression, *type_node]
            }

            ExpressionKind::Assignment { target, value } => {
                vec![*target, *value]
            }

            ExpressionKind::CompoundAssignment {
                target,
                value,
                ..
            } => {
                vec![*target, *value]
            }

            ExpressionKind::Try { expression }
            | ExpressionKind::Await { expression }
            | ExpressionKind::Spawn { expression } => {
                vec![*expression]
            }

            ExpressionKind::Async { body } => {
                vec![*body]
            }

            ExpressionKind::Construction {
                type_name,
                arguments,
            } => {
                let mut children = Vec::with_capacity(
                    arguments.len().saturating_add(1),
                );

                children.push(*type_name);
                children.extend(arguments.iter().copied());

                children
            }

            ExpressionKind::Extension(extension) => {
                extension.child_node_ids().collect()
            }
        }
    }

    /// Returns the number of direct child references.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match &self.kind {
            ExpressionKind::Identifier { .. }
            | ExpressionKind::Literal { .. }
            | ExpressionKind::Unary { .. }
            | ExpressionKind::Loop { .. } => 1,

            ExpressionKind::Binary { .. }
            | ExpressionKind::MemberAccess { .. }
            | ExpressionKind::Index { .. }
            | ExpressionKind::Cast { .. }
            | ExpressionKind::TypeAscription { .. }
            | ExpressionKind::Assignment { .. }
            | ExpressionKind::CompoundAssignment { .. } => 2,

            ExpressionKind::Conditional { else_branch, .. } => {
                2 + usize::from(else_branch.is_some())
            }

            ExpressionKind::Block { statements }
            | ExpressionKind::Array { elements: statements }
            | ExpressionKind::Tuple { elements: statements }
            | ExpressionKind::Match { arms: statements, .. } => {
                match &self.kind {
                    ExpressionKind::Match { .. } => {
                        1usize.saturating_add(statements.len())
                    }

                    _ => statements.len(),
                }
            }

            ExpressionKind::Lambda {
                parameters,
                ..
            } => parameters.len().saturating_add(1),

            ExpressionKind::Call {
                arguments,
                ..
            }
            | ExpressionKind::Construction {
                arguments,
                ..
            } => arguments.len().saturating_add(1),

            ExpressionKind::Range { start, end, .. } => {
                usize::from(start.is_some()) + usize::from(end.is_some())
            }

            ExpressionKind::Struct { fields, .. } => {
                fields.len().saturating_add(1)
            }

            ExpressionKind::Try { .. }
            | ExpressionKind::Await { .. }
            | ExpressionKind::Async { .. }
            | ExpressionKind::Spawn { .. } => 1,

            ExpressionKind::Extension(extension) => {
                extension.child_count()
            }
        }
    }

    /// Performs local structural validation.
    ///
    /// This does not inspect the referenced nodes. The AST graph validator is
    /// responsible for validating those nodes.
    pub fn validate_structure(
        &self,
        policy: ExpressionValidationPolicy,
    ) -> ExpressionResult<()> {
        policy.validate()?;

        let expected_kind = Self::node_kind_for(&self.kind);

        match self.node.kind().as_core() {
            Some(actual) if actual == expected_kind => {}
            _ => {
                return Err(ExpressionError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                });
            }
        }

        if let Some(maximum) = policy.max_children {
            let count = self.child_count();

            if count > maximum {
                return Err(ExpressionError::LimitExceeded {
                    limit: "max_children",
                    actual: count,
                    maximum,
                });
            }
        }

        match &self.kind {
            ExpressionKind::Identifier { identifier } => {
                Self::validate_child(*identifier, "identifier")?;
            }

            ExpressionKind::Literal { literal } => {
                Self::validate_child(*literal, "literal")?;
            }

            ExpressionKind::Unary {
                operator,
                operand,
            } => {
                operator.validate(policy)?;
                Self::validate_child(*operand, "operand")?;
            }

            ExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                operator.validate(policy)?;
                Self::validate_child(*left, "left")?;
                Self::validate_child(*right, "right")?;
            }

            ExpressionKind::Conditional {
                condition,
                then_branch,
                else_branch,
            } => {
                Self::validate_child(*condition, "condition")?;
                Self::validate_child(*then_branch, "then_branch")?;

                if let Some(branch) = else_branch {
                    Self::validate_child(*branch, "else_branch")?;
                }
            }

            ExpressionKind::Block { statements } => {
                Self::validate_children(statements, "statements")?;
            }

            ExpressionKind::Match {
                scrutinee,
                arms,
            } => {
                Self::validate_child(*scrutinee, "scrutinee")?;
                Self::validate_children(arms, "arms")?;
            }

            ExpressionKind::Loop { body } => {
                Self::validate_child(*body, "body")?;
            }

            ExpressionKind::Lambda {
                parameters,
                body,
            } => {
                Self::validate_children(parameters, "parameters")?;
                Self::validate_child(*body, "body")?;
            }

            ExpressionKind::Call {
                callee,
                arguments,
            } => {
                Self::validate_child(*callee, "callee")?;
                Self::validate_children(arguments, "arguments")?;
            }

            ExpressionKind::MemberAccess { object, member } => {
                Self::validate_child(*object, "object")?;
                Self::validate_child(*member, "member")?;
            }

            ExpressionKind::Index { object, index } => {
                Self::validate_child(*object, "object")?;
                Self::validate_child(*index, "index")?;
            }

            ExpressionKind::Range { start, end, .. } => {
                if let Some(start) = start {
                    Self::validate_child(*start, "start")?;
                }

                if let Some(end) = end {
                    Self::validate_child(*end, "end")?;
                }
            }

            ExpressionKind::Array { elements }
            | ExpressionKind::Tuple { elements } => {
                Self::validate_children(elements, "elements")?;
            }

            ExpressionKind::Struct {
                type_name,
                fields,
            } => {
                Self::validate_child(*type_name, "type_name")?;
                Self::validate_children(fields, "fields")?;
            }

            ExpressionKind::Cast {
                expression,
                target_type,
            } => {
                Self::validate_child(*expression, "expression")?;
                Self::validate_child(*target_type, "target_type")?;
            }

            ExpressionKind::TypeAscription {
                expression,
                type_node,
            } => {
                Self::validate_child(*expression, "expression")?;
                Self::validate_child(*type_node, "type_node")?;
            }

            ExpressionKind::Assignment { target, value } => {
                Self::validate_child(*target, "target")?;
                Self::validate_child(*value, "value")?;
            }

            ExpressionKind::CompoundAssignment {
                target,
                operator,
                value,
            } => {
                operator.validate(policy)?;
                Self::validate_child(*target, "target")?;
                Self::validate_child(*value, "value")?;
            }

            ExpressionKind::Try { expression }
            | ExpressionKind::Await { expression }
            | ExpressionKind::Spawn { expression } => {
                Self::validate_child(*expression, "expression")?;
            }

            ExpressionKind::Async { body } => {
                Self::validate_child(*body, "body")?;
            }

            ExpressionKind::Construction {
                type_name,
                arguments,
            } => {
                Self::validate_child(*type_name, "type_name")?;
                Self::validate_children(arguments, "arguments")?;
            }

            ExpressionKind::Extension(extension) => {
                extension.validate(policy)?;
            }
        }

        Ok(())
    }

    /// Validates a child ID without dereferencing the AST graph.
    ///
    /// The default `NodeId` represents an absent/uninitialized identity in the
    /// repository's foundational node model, so it is rejected here.
    fn validate_child(
        id: NodeId,
        field: &'static str,
    ) -> ExpressionResult<()> {
        if id == NodeId::default() {
            return Err(ExpressionError::InvalidChildReference { field });
        }

        Ok(())
    }

    /// Validates a sequence of child IDs without recursively traversing them.
    fn validate_children(
        children: &[NodeId],
        field: &'static str,
    ) -> ExpressionResult<()> {
        let mut previous: Option<NodeId> = None;

        for child in children {
            Self::validate_child(*child, field)?;

            // Adjacent duplicate references are not universally invalid in an
            // AST (for example, `x + x` is perfectly valid), so this only
            // performs the structural non-default check here.
            //
            // `previous` is deliberately retained only to make the source-order
            // traversal contract explicit without imposing uniqueness semantics.
            previous = Some(*child);
        }

        let _ = previous;

        Ok(())
    }

    /// Returns the stable schema version of this expression contract.
    #[must_use]
    pub const fn schema_version() -> u16 {
        EXPRESSION_SCHEMA_VERSION
    }

    /// Returns whether this expression is an extension expression.
    #[must_use]
    pub fn is_extension(&self) -> bool {
        matches!(self.kind, ExpressionKind::Extension(_))
    }

    /// Returns the extension identity when this is an extension expression.
    #[must_use]
    pub fn extension(&self) -> Option<&ExpressionExtension> {
        match &self.kind {
            ExpressionKind::Extension(extension) => Some(extension),
            _ => None,
        }
    }

    /// Returns the operator when this expression directly owns one.
    #[must_use]
    pub fn operator(&self) -> Option<&OperatorRef> {
        match &self.kind {
            ExpressionKind::Unary { operator, .. }
            | ExpressionKind::Binary { operator, .. }
            | ExpressionKind::CompoundAssignment { operator, .. } => {
                Some(operator)
            }

            _ => None,
        }
    }

    /// Returns whether the expression has no direct child nodes.
    #[must_use]
    pub fn is_leaf(&self) -> bool {
        self.child_count() == 0
    }

    /// Returns a deterministic diagnostic summary without exposing arbitrary
    /// extension payload contents.
    #[must_use]
    pub fn diagnostic_summary(&self) -> ExpressionDiagnosticSummary {
        ExpressionDiagnosticSummary {
            id: self.id(),
            node_kind: self.node.kind_owned(),
            span: self.span_owned(),
            expression_kind: self.kind.classification_name(),
        }
    }
}

// =============================================================================
// ExpressionKind helpers
// =============================================================================

impl ExpressionKind {
    /// Returns a stable source-level classification name.
    ///
    /// This is intended for diagnostics and tooling, not semantic dispatch.
    #[must_use]
    pub const fn classification_name(&self) -> &'static str {
        match self {
            Self::Identifier { .. } => "identifier",
            Self::Literal { .. } => "literal",
            Self::Unary { .. } => "unary-expression",
            Self::Binary { .. } => "binary-expression",
            Self::Conditional { .. } => "conditional-expression",
            Self::Block { .. } => "block-expression",
            Self::Match { .. } => "match-expression",
            Self::Loop { .. } => "loop-expression",
            Self::Lambda { .. } => "lambda-expression",
            Self::Call { .. } => "call-expression",
            Self::MemberAccess { .. } => "member-access",
            Self::Index { .. } => "index-expression",
            Self::Range { .. } => "range-expression",
            Self::Array { .. } => "array-expression",
            Self::Tuple { .. } => "tuple-expression",
            Self::Struct { .. } => "struct-expression",
            Self::Cast { .. } => "cast-expression",
            Self::TypeAscription { .. } => "type-ascription",
            Self::Assignment { .. } => "assignment",
            Self::CompoundAssignment { .. } => "compound-assignment",
            Self::Try { .. } => "try-expression",
            Self::Await { .. } => "await-expression",
            Self::Async { .. } => "async-expression",
            Self::Spawn { .. } => "spawn-expression",
            Self::Construction { .. } => "construction-expression",
            Self::Extension(_) => "extension-expression",
        }
    }

    /// Returns the number of direct children without allocating a vector.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match self {
            Self::Identifier { .. }
            | Self::Literal { .. }
            | Self::Unary { .. }
            | Self::Loop { .. }
            | Self::Try { .. }
            | Self::Await { .. }
            | Self::Async { .. }
            | Self::Spawn { .. } => 1,

            Self::Binary { .. }
            | Self::MemberAccess { .. }
            | Self::Index { .. }
            | Self::Cast { .. }
            | Self::TypeAscription { .. }
            | Self::Assignment { .. }
            | Self::CompoundAssignment { .. } => 2,

            Self::Conditional { else_branch, .. } => {
                2 + usize::from(else_branch.is_some())
            }

            Self::Block { statements } => statements.len(),

            Self::Match { arms, .. } => {
                1usize.saturating_add(arms.len())
            }

            Self::Lambda {
                parameters,
                ..
            } => parameters.len().saturating_add(1),

            Self::Call {
                arguments,
                ..
            }
            | Self::Construction {
                arguments,
                ..
            } => arguments.len().saturating_add(1),

            Self::Range { start, end, .. } => {
                usize::from(start.is_some()) + usize::from(end.is_some())
            }

            Self::Array { elements }
            | Self::Tuple { elements } => elements.len(),

            Self::Struct { fields, .. } => {
                fields.len().saturating_add(1)
            }

            Self::Extension(extension) => extension.child_count(),
        }
    }
}

// =============================================================================
// Diagnostic representation
// =============================================================================

/// Compact expression information suitable for diagnostics.
///
/// Extension payloads are intentionally not copied into this type.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpressionDiagnosticSummary {
    /// Stable AST identity.
    pub id: NodeId,

    /// Common AST node classification.
    pub node_kind: NodeKind,

    /// Source span.
    pub span: Span,

    /// Stable expression classification.
    pub expression_kind: &'static str,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn source_span() -> Span {
        Span::default()
    }

    fn valid_id(value: u64) -> NodeId {
        NodeId::from_raw(value)
    }

    #[test]
    fn native_operator_is_constructible() {
        let operator = OperatorRef::native("add")
            .expect("native operator should be valid");

        assert!(operator.is_native());
        assert_eq!(operator.name(), "add");
        assert_eq!(operator.qualified_name(), "add");
    }

    #[test]
    fn namespaced_operator_is_deterministic() {
        let operator = OperatorRef::new(
            "zamani.operator",
            "add",
        )
        .expect("operator should be valid");

        assert_eq!(
            operator.qualified_name(),
            "zamani.operator:add"
        );
    }

    #[test]
    fn empty_operator_name_is_rejected() {
        let result = OperatorRef::native("");

        assert_eq!(
            result,
            Err(ExpressionError::EmptyOperatorName)
        );
    }

    #[test]
    fn extension_identity_is_namespaced() {
        let extension = ExpressionExtension::new(
            "zamani.quantum",
            "operation",
            vec![valid_id(2), valid_id(3)],
            Vec::new(),
        )
        .expect("extension should be valid");

        assert_eq!(
            extension.qualified_name(),
            "zamani.quantum:operation"
        );

        assert_eq!(extension.child_count(), 2);
    }

    #[test]
    fn identifier_expression_uses_identifier_node_kind() {
        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Identifier {
                identifier: valid_id(2),
            },
            source_span(),
        );

        assert_eq!(
            expression.node().kind().as_core(),
            Some(CoreNodeKind::Identifier)
        );

        assert_eq!(
            expression.child_node_ids(),
            vec![valid_id(2)]
        );
    }

    #[test]
    fn binary_expression_preserves_source_child_order() {
        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Binary {
                left: valid_id(2),
                operator: OperatorRef::native("add")
                    .expect("operator should be valid"),
                right: valid_id(3),
            },
            source_span(),
        );

        assert_eq!(
            expression.child_node_ids(),
            vec![valid_id(2), valid_id(3)]
        );

        assert_eq!(expression.child_count(), 2);
    }

    #[test]
    fn conditional_expression_preserves_optional_else() {
        let without_else = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Conditional {
                condition: valid_id(2),
                then_branch: valid_id(3),
                else_branch: None,
            },
            source_span(),
        );

        assert_eq!(
            without_else.child_node_ids(),
            vec![valid_id(2), valid_id(3)]
        );

        let with_else = Expression::without_metadata(
            valid_id(4),
            ExpressionKind::Conditional {
                condition: valid_id(5),
                then_branch: valid_id(6),
                else_branch: Some(valid_id(7)),
            },
            source_span(),
        );

        assert_eq!(
            with_else.child_node_ids(),
            vec![
                valid_id(5),
                valid_id(6),
                valid_id(7)
            ]
        );
    }

    #[test]
    fn extension_expression_is_not_hardware_specific() {
        let extension = ExpressionExtension::new(
            "zamani.quantum",
            "operation",
            vec![valid_id(2)],
            Vec::new(),
        )
        .expect("extension should be valid");

        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Extension(extension),
            source_span(),
        );

        assert!(expression.is_extension());
        assert_eq!(
            expression.extension()
                .expect("extension should exist")
                .namespace(),
            "zamani.quantum"
        );
    }

    #[test]
    fn validation_rejects_default_child_id() {
        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Identifier {
                identifier: NodeId::default(),
            },
            source_span(),
        );

        assert_eq!(
            expression.validate_structure(
                ExpressionValidationPolicy::unrestricted()
            ),
            Err(ExpressionError::InvalidChildReference {
                field: "identifier",
            })
        );
    }

    #[test]
    fn validation_rejects_wrong_node_kind() {
        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Identifier {
                identifier: valid_id(2),
            },
            source_span(),
        );

        let mut node = expression.node.clone();

        node.replace_kind(
            NodeKind::core(CoreNodeKind::Literal)
        );

        let malformed = Expression::from_node(
            node,
            expression.kind.clone(),
        );

        assert!(matches!(
            malformed.validate_structure(
                ExpressionValidationPolicy::unrestricted()
            ),
            Err(ExpressionError::InvalidNodeKind { .. })
        ));
    }

    #[test]
    fn unrestricted_policy_does_not_create_ast_limits() {
        let policy =
            ExpressionValidationPolicy::unrestricted();

        assert_eq!(policy.max_children, None);
        assert_eq!(
            policy.max_extension_namespace_bytes,
            None
        );
        assert_eq!(
            policy.max_extension_name_bytes,
            None
        );
    }

    #[test]
    fn expression_schema_version_is_stable() {
        assert_eq!(
            Expression::schema_version(),
            EXPRESSION_SCHEMA_VERSION
        );
    }

    #[test]
    fn diagnostic_summary_does_not_expand_children() {
        let expression = Expression::without_metadata(
            valid_id(1),
            ExpressionKind::Unary {
                operator: OperatorRef::native("neg")
                    .expect("operator should be valid"),
                operand: valid_id(2),
            },
            source_span(),
        );

        let summary = expression.diagnostic_summary();

        assert_eq!(summary.id, valid_id(1));
        assert_eq!(
            summary.expression_kind,
            "unary-expression"
        );
    }
}