//! # Zamani Frontend AST — Statement
//!
//! Canonical source-level representation of statements in the Zamani frontend
//! AST.
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
//! ┌─────────────────────────────────────┐
//! │ Native Zamani AST                   │
//! │                                     │
//! │ statements/statement.rs ← this file │
//! └──────────────────┬──────────────────┘
//!                    │
//!                    ▼
//!          structural validation
//!                    │
//!                    ▼
//!             semantic analysis
//!                    │
//!                    ▼
//!              semantic model
//!                    │
//!                    ▼
//!                   ZUIR
//!                    │
//!       ┌────────────┼────────────┐
//!       ▼            ▼            ▼
//!   classical      quantum       HDL
//!      IR            IR           IR
//! ```
//!
//! ## Responsibility
//!
//! This file owns the canonical **source-level statement abstraction**.
//!
//! A statement describes source-language control, sequencing, binding and
//! termination intent. It does not describe how that intent is implemented by
//! a CPU, GPU, FPGA, QPU, simulator, distributed runtime, quantum technology,
//! hardware topology, backend, scheduler, router, or execution service.
//!
//! The statement representation is therefore intentionally target-neutral and
//! compatible with Zamani's:
//!
//! `Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever`
//!
//! (POCO-REAF) objective.
//!
//! ## Critical architectural boundary
//!
//! This file must NOT contain:
//!
//! - physical qubit IDs;
//! - physical CPU/GPU/FPGA identifiers;
//! - hardware topology;
//! - routing decisions;
//! - scheduling decisions;
//! - calibration data;
//! - QEC implementation;
//! - decoder implementation;
//! - noise models;
//! - resilience implementation;
//! - backend instructions;
//! - vendor APIs;
//! - QIR values;
//! - LLVM values;
//! - MLIR operations;
//! - runtime state;
//! - resource allocation decisions;
//! - target-specific optimization policy.
//!
//! Those belong to later compiler layers.
//!
//! ## Statement graph model
//!
//! The canonical Zamani AST uses `NodeId` references for relationships between
//! AST nodes rather than recursively embedding concrete AST values.
//!
//! ```text
//! Statement
//! ├── Node
//! └── StatementKind
//!      ├── NodeId
//!      ├── NodeId
//!      └── NodeId ...
//! ```
//!
//! The owning AST graph is responsible for storing the actual nodes.
//!
//! This design avoids recursive Rust ownership structures and allows traversal
//! infrastructure to use an explicit worklist/stack rather than depending on
//! the native call stack. That is important for very large or deeply nested
//! programs.
//!
//! ## Scalability
//!
//! There is deliberately no:
//!
//! - maximum statement count;
//! - maximum block size;
//! - maximum loop count;
//! - maximum nesting depth;
//! - maximum quantum-resource count;
//! - maximum register count;
//! - maximum machine size;
//! - maximum operand count;
//! - maximum program size;
//! - fixed hardware topology.
//!
//! Collections grow according to available resources and caller-supplied
//! compiler resource policies.
//!
//! "Infinity" in POCO-REAF means that this AST layer does not impose an
//! artificial finite machine/program-size semantic limit. Actual compilation
//! remains bounded by available memory, address space, storage, execution time,
//! compiler policy, and the representational limits of the host environment.
//!
//! ## Domain neutrality
//!
//! Quantum computation is not represented through a closed statement enum such
//! as:
//!
//! ```text
//! QuantumStatement::Hadamard
//! QuantumStatement::CNOT
//! QuantumStatement::Measure
//! ```
//!
//! Such representations would make the native AST depend on a particular
//! quantum vocabulary and would require modification whenever a new quantum
//! operation or computational technology appeared.
//!
//! Quantum-specific source constructs should instead use generic core
//! statements and/or registered extension statements.
//!
//! The same rule applies to:
//!
//! - classical accelerators;
//! - AI;
//! - HDL;
//! - neuromorphic computing;
//! - photonic computing;
//! - analog computing;
//! - distributed computing;
//! - future computational domains.
//!
//! ## Integration with existing AST infrastructure
//!
//! This file deliberately uses the repository's existing foundational types:
//!
//! - [`Node`]
//! - [`NodeId`]
//! - [`NodeKind`]
//! - [`CoreNodeKind`]
//!
//! It does not redefine them.
//!
//! `Node` already owns the common AST identity, source span, node kind and
//! metadata contract. `NodeId` already provides opaque deterministic AST
//! identity. `NodeKind` already provides an extensible core/extension
//! classification system.
//!
//! These existing contracts are therefore authoritative.
//!
//! ## Parser boundary
//!
//! The parser constructs `Statement` values after syntactic recognition.
//!
//! The parser is responsible for:
//!
//! - allocating node IDs;
//! - determining source spans;
//! - constructing statement structure;
//! - preserving source-level distinctions.
//!
//! The parser must NOT perform:
//!
//! - type checking;
//! - name resolution;
//! - resource allocation;
//! - quantum routing;
//! - hardware mapping;
//! - scheduling;
//! - QEC;
//! - backend selection.
//!
//! ## Semantic boundary
//!
//! Semantic analysis consumes validated statements and resolves:
//!
//! - names;
//! - types;
//! - patterns;
//! - control-flow semantics;
//! - effects;
//! - capabilities;
//! - resources;
//! - domain semantics;
//! - generic substitutions.
//!
//! Semantic information must not be inserted into this source AST.
//!
//! ## ZUIR boundary
//!
//! Statements are lowered through the semantic model into ZUIR.
//!
//! ```text
//! Statement AST
//!       │
//!       ▼
//! Semantic Model
//!       │
//!       ▼
//! ZUIR
//!       │
//!       ├── classical lowering
//!       ├── quantum lowering
//!       ├── HDL lowering
//!       └── future-domain lowering
//! ```
//!
//! This file must not import ZUIR.
//!
//! ## Quantum integration
//!
//! A quantum program can use ordinary source-level statements for:
//!
//! - sequencing;
//! - binding;
//! - conditional execution;
//! - loops;
//! - return/control transfer;
//! - blocks;
//! - generic calls;
//! - generic resource manipulation.
//!
//! Quantum-specific operations are represented by expressions or registered
//! language extensions and referenced from statements where appropriate.
//!
//! Consequently:
//!
//! ```text
//! statement AST
//!       │
//!       ├── classical expression
//!       ├── generic operation expression
//!       ├── quantum extension expression
//!       ├── resource expression
//!       └── future-domain expression
//! ```
//!
//! No statement variant needs to know whether a child eventually becomes a
//! CPU instruction, quantum gate, pulse sequence, distributed operation, or
//! something not yet invented.
//!
//! ## Hardware independence
//!
//! A statement must never encode assumptions such as:
//!
//! ```text
//! MAX_QUBITS = 32
//! DEVICE = ...
//! TOPOLOGY = ...
//! CPU_COUNT = ...
//! GPU_COUNT = ...
//! ```
//!
//! If a programmer explicitly requests a target constraint, that constraint
//! belongs to a source-level constraint/capability representation and is
//! interpreted downstream. It must not change the fundamental statement model.
//!
//! ## Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - edition 2021;
//! - stable Rust;
//! - no nightly features;
//! - no `unsafe`.
//!
//! ## Security
//!
//! This file is compiler-input infrastructure and therefore must treat AST
//! construction and serialized data as potentially untrusted.
//!
//! It:
//!
//! - performs no I/O;
//! - executes no source-level calls;
//! - uses no raw pointers;
//! - uses no global mutable state;
//! - uses no unchecked indexing;
//! - contains no `unsafe`;
//! - does not recursively traverse child nodes;
//! - does not impose hidden computational limits.
//!
//! Resource limits, if required to protect a compiler service, are supplied
//! explicitly through validation policies rather than encoded as language
//! semantics.
//!
//! ## Determinism
//!
//! This module uses ordered source collections (`Vec`) and opaque `NodeId`
//! values. It does not use hash-map iteration as a source-order representation.
//!
//! The statement itself never allocates IDs implicitly. The caller owns ID
//! allocation, preserving deterministic and reproducible AST construction.
//!
//! ## Serialization
//!
//! The public structures derive Serde serialization.
//!
//! Serialization schema versioning belongs to the AST serialization subsystem.
//! This file does not invent a second serialization-version system.
//!
//! ## No-re-edit integration guarantee
//!
//! The public contract of this file is intentionally based only on stable
//! foundational AST concepts:
//!
//! - `Node`;
//! - `NodeId`;
//! - `NodeKind`;
//! - `CoreNodeKind`;
//! - opaque child-node references.
//!
//! Parser, semantic analysis, ZUIR, quantum compilation, scheduling, routing,
//! optimization and backend implementations consume this contract rather than
//! modifying it.
//!
//! A new statement primitive should be added here only when it is genuinely a
//! native Zamani source-language construct. Domain-specific behavior should
//! normally use the existing extension mechanism.
//!
//! # File contract
//!
//! **Owns**
//!
//! - canonical statement identity;
//! - canonical statement-kind representation;
//! - statement child-reference representation;
//! - local structural validation;
//! - deterministic child enumeration;
//! - statement schema contract.
//!
//! **Does not own**
//!
//! - AST graph storage;
//! - name resolution;
//! - type checking;
//! - semantic resources;
//! - ZUIR;
//! - quantum IR;
//! - routing;
//! - scheduling;
//! - hardware;
//! - runtime execution.
//!
//! **Inputs**
//!
//! - a caller-owned `NodeId`;
//! - a source `Span` through `Node`;
//! - a valid `NodeKind`;
//! - child `NodeId` references;
//! - source-level statement information.
//!
//! **Outputs**
//!
//! - a canonical `Statement`;
//! - deterministic local child enumeration;
//! - structural validation results.
//!
//! **Allowed dependencies**
//!
//! - Rust standard library;
//! - Serde;
//! - `super::super::node`;
//! - `super::super::node_id`;
//! - `super::super::node_kind`.
//!
//! **Forbidden dependencies**
//!
//! - semantic analysis;
//! - compiler driver;
//! - ZUIR;
//! - quantum IR;
//! - quantum backend;
//! - hardware;
//! - optimizer;
//! - scheduler;
//! - router;
//! - runtime.
//!
//! **Thread safety**
//!
//! No global mutable state is used. Immutable statements can be shared across
//! compiler phases when their contained types are `Send + Sync`.
//!
//! **Ownership**
//!
//! The statement owns only its local source-level representation. Referenced
//! nodes remain owned by the enclosing AST graph.
//!
//! **Identity**
//!
//! `NodeId` identifies the AST node represented by the statement. Child
//! `NodeId`s identify referenced AST nodes and do not imply semantic or runtime
//! identities.
//!
//! **Scalability**
//!
//! No machine-size or computational-resource limit is embedded in the statement
//! representation.
//!
//! **Completion criterion**
//!
//! This file is complete when its public contract remains sufficient for the
//! parser, AST validation, traversal, semantic analysis and lowering layers to
//! consume statements without adding implementation-specific fields here.
//!
//! =============================================================================
//! Implementation
//! =============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::fmt;

use serde::{Deserialize, Serialize};

use super::super::node::AstNode;
use super::super::node::Node;
use super::super::node_id::NodeId;
use super::super::node_kind::{CoreNodeKind, NodeKind};

// =============================================================================
// Constants
// =============================================================================

/// Schema version for the statement representation.
///
/// This is deliberately independent from the overall AST schema version and
/// the Zamani language version.
pub const STATEMENT_SCHEMA_VERSION: u16 = 1;

// =============================================================================
// Result and errors
// =============================================================================

/// Result type used by statement construction and local structural validation.
pub type StatementResult<T> = Result<T, StatementError>;

/// Errors that can be detected locally without semantic analysis.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum StatementError {
    /// The supplied node kind is not a valid statement kind.
    InvalidNodeKind {
        /// Actual node kind supplied by the caller.
        actual: NodeKind,
    },

    /// A required child reference was not supplied.
    MissingChild {
        /// Logical name of the missing field.
        field: &'static str,
    },

    /// A required child collection was empty.
    EmptyChildren {
        /// Logical name of the collection.
        field: &'static str,
    },

    /// A statement-specific field contains an invalid node reference.
    InvalidChildReference {
        /// Logical name of the field.
        field: &'static str,
    },

    /// A caller-supplied validation policy was exceeded.
    LimitExceeded {
        /// Name of the policy limit.
        limit: &'static str,

        /// Observed value.
        actual: usize,

        /// Maximum permitted by the caller.
        maximum: usize,
    },

    /// An extension statement has an invalid namespace.
    EmptyExtensionNamespace,

    /// An extension statement has an invalid name.
    EmptyExtensionName,
}

impl fmt::Display for StatementError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNodeKind { actual } => {
                write!(
                    formatter,
                    "statement has invalid AST node kind: {actual}"
                )
            }

            Self::MissingChild { field } => {
                write!(
                    formatter,
                    "statement is missing required child: {field}"
                )
            }

            Self::EmptyChildren { field } => {
                write!(
                    formatter,
                    "statement requires at least one child in: {field}"
                )
            }

            Self::InvalidChildReference { field } => {
                write!(
                    formatter,
                    "statement contains an invalid child reference in: {field}"
                )
            }

            Self::LimitExceeded {
                limit,
                actual,
                maximum,
            } => {
                write!(
                    formatter,
                    "statement validation limit `{limit}` exceeded: \
                     {actual} > {maximum}"
                )
            }

            Self::EmptyExtensionNamespace => {
                formatter.write_str(
                    "statement extension namespace must not be empty",
                )
            }

            Self::EmptyExtensionName => {
                formatter.write_str(
                    "statement extension name must not be empty",
                )
            }
        }
    }
}

impl std::error::Error for StatementError {}

// =============================================================================
// Validation policy
// =============================================================================

/// Caller-supplied structural validation policy.
///
/// These limits are compiler/resource-service policy, not language semantics.
///
/// `None` means that this policy does not impose a limit for that dimension.
///
/// No fixed machine, qubit, register, or program-size limit is represented
/// here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatementValidationPolicy {
    /// Maximum number of direct child references inspected by validation.
    pub max_children: Option<usize>,

    /// Maximum extension namespace length in bytes.
    pub max_extension_namespace_bytes: Option<usize>,

    /// Maximum extension name length in bytes.
    pub max_extension_name_bytes: Option<usize>,
}

impl Default for StatementValidationPolicy {
    fn default() -> Self {
        Self {
            max_children: None,
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
        }
    }
}

impl StatementValidationPolicy {
    /// Creates an unrestricted structural policy.
    ///
    /// This is suitable when resource limits are controlled by an outer
    /// compiler/session policy.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_children: None,
            max_extension_namespace_bytes: None,
            max_extension_name_bytes: None,
        }
    }

    fn check_children(
        self,
        count: usize,
    ) -> StatementResult<()> {
        if let Some(maximum) = self.max_children {
            if count > maximum {
                return Err(StatementError::LimitExceeded {
                    limit: "max_children",
                    actual: count,
                    maximum,
                });
            }
        }

        Ok(())
    }

    fn check_extension_namespace(
        self,
        namespace: &str,
    ) -> StatementResult<()> {
        if namespace.is_empty() {
            return Err(StatementError::EmptyExtensionNamespace);
        }

        if let Some(maximum) = self.max_extension_namespace_bytes {
            let actual = namespace.len();

            if actual > maximum {
                return Err(StatementError::LimitExceeded {
                    limit: "max_extension_namespace_bytes",
                    actual,
                    maximum,
                });
            }
        }

        Ok(())
    }

    fn check_extension_name(
        self,
        name: &str,
    ) -> StatementResult<()> {
        if name.is_empty() {
            return Err(StatementError::EmptyExtensionName);
        }

        if let Some(maximum) = self.max_extension_name_bytes {
            let actual = name.len();

            if actual > maximum {
                return Err(StatementError::LimitExceeded {
                    limit: "max_extension_name_bytes",
                    actual,
                    maximum,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Statement kind
// =============================================================================

/// Source-level classification of a statement.
///
/// The representation deliberately uses generic source-language concepts.
/// Quantum, accelerator, hardware and future-domain constructs must not become
/// a closed list here.
///
/// Domain-specific statements can use [`StatementKind::Extension`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StatementKind {
    /// A standalone expression evaluated for its effects.
    Expression {
        /// Expression node.
        expression: NodeId,
    },

    /// A mutable/local binding statement.
    ///
    /// `pattern` identifies the source binding pattern.
    ///
    /// `type_annotation` is optional because the language may permit type
    /// inference.
    ///
    /// `initializer` is optional when the language permits declaration without
    /// immediate initialization.
    Let {
        /// Binding pattern.
        pattern: NodeId,

        /// Optional explicit type annotation.
        type_annotation: Option<NodeId>,

        /// Optional initializer expression.
        initializer: Option<NodeId>,
    },

    /// A constant binding statement.
    Const {
        /// Binding pattern.
        pattern: NodeId,

        /// Optional explicit type annotation.
        type_annotation: Option<NodeId>,

        /// Initializer expression.
        initializer: NodeId,
    },

    /// Returns control from the current callable.
    Return {
        /// Optional returned expression.
        value: Option<NodeId>,
    },

    /// Terminates the nearest applicable loop or labeled construct.
    Break {
        /// Optional source-level label.
        label: Option<NodeId>,

        /// Optional value where the language permits valued breaks.
        value: Option<NodeId>,
    },

    /// Continues the nearest applicable loop or labeled construct.
    Continue {
        /// Optional source-level label.
        label: Option<NodeId>,
    },

    /// Conditional execution.
    ///
    /// `condition` is evaluated by the semantic layer. `then_branch` and
    /// `else_branch` identify statement/block nodes in the AST graph.
    If {
        /// Condition expression.
        condition: NodeId,

        /// Then branch.
        then_branch: NodeId,

        /// Optional else branch.
        else_branch: Option<NodeId>,
    },

    /// While loop.
    While {
        /// Loop condition.
        condition: NodeId,

        /// Loop body.
        body: NodeId,
    },

    /// For/range/iterator loop.
    For {
        /// Loop binding pattern.
        pattern: NodeId,

        /// Iterable/range expression.
        iterable: NodeId,

        /// Loop body.
        body: NodeId,
    },

    /// A sequence of statements represented by references into the AST graph.
    Block {
        /// Statements in source order.
        statements: Vec<NodeId>,
    },

    /// Match/control-flow statement.
    ///
    /// Match arms are represented by AST nodes so this file does not need to
    /// depend on a future pattern/arm implementation.
    Match {
        /// Scrutinee expression.
        scrutinee: NodeId,

        /// Match-arm nodes in source order.
        arms: Vec<NodeId>,
    },

    /// A declaration used in statement position where the language permits it.
    ///
    /// The declaration itself remains owned by the declaration AST subsystem.
    Declaration {
        /// Declaration node.
        declaration: NodeId,
    },

    /// Explicitly preserved parser recovery statement.
    ///
    /// This is source syntax that could not be represented as a valid native
    /// statement during recovery. It must never silently acquire semantic
    /// meaning.
    Error {
        /// Optional preserved source/recovery node.
        recovered: Option<NodeId>,
    },

    /// Extensible source-language statement.
    ///
    /// The AST core stores only the extension identity and child references.
    /// The extension implementation owns interpretation.
    Extension {
        /// Extension namespace.
        namespace: String,

        /// Extension-local statement name.
        name: String,

        /// Extension-owned child references in source order.
        children: Vec<NodeId>,
    },
}

impl StatementKind {
    /// Returns the canonical native [`CoreNodeKind`] associated with this
    /// statement.
    ///
    /// This method does not inspect or resolve child nodes.
    #[must_use]
    pub const fn core_node_kind(&self) -> CoreNodeKind {
        match self {
            Self::Expression { .. } => CoreNodeKind::ExpressionStatement,
            Self::Let { .. } => CoreNodeKind::LetStatement,
            Self::Const { .. } => CoreNodeKind::ConstStatement,
            Self::Return { .. } => CoreNodeKind::ReturnStatement,
            Self::Break { .. } => CoreNodeKind::BreakStatement,
            Self::Continue { .. } => CoreNodeKind::ContinueStatement,
            Self::If { .. } => CoreNodeKind::Statement,
            Self::While { .. } => CoreNodeKind::WhileStatement,
            Self::For { .. } => CoreNodeKind::ForStatement,
            Self::Block { .. } => CoreNodeKind::Statement,
            Self::Match { .. } => CoreNodeKind::MatchStatement,
            Self::Declaration { .. } => CoreNodeKind::Declaration,
            Self::Error { .. } => CoreNodeKind::Error,
            Self::Extension { .. } => CoreNodeKind::Opaque,
        }
    }

    /// Returns whether this is an extension statement.
    #[must_use]
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension { .. })
    }

    /// Returns the number of direct child references.
    ///
    /// This operation is O(1) for all variants except variants that contain
    /// dynamically sized child collections, where it remains O(1) because
    /// `Vec::len` is constant time.
    #[must_use]
    pub fn child_count(&self) -> usize {
        match self {
            Self::Expression { .. } => 1,

            Self::Let {
                type_annotation,
                initializer,
                ..
            } => {
                1 + usize::from(type_annotation.is_some())
                    + usize::from(initializer.is_some())
            }

            Self::Const { .. } => 3,

            Self::Return { value } => usize::from(value.is_some()),

            Self::Break { label, value } => {
                usize::from(label.is_some()) + usize::from(value.is_some())
            }

            Self::Continue { label } => usize::from(label.is_some()),

            Self::If {
                else_branch,
                ..
            } => 2 + usize::from(else_branch.is_some()),

            Self::While { .. } => 2,

            Self::For { .. } => 3,

            Self::Block { statements } => statements.len(),

            Self::Match { arms, .. } => 1 + arms.len(),

            Self::Declaration { .. } => 1,

            Self::Error { recovered } => usize::from(recovered.is_some()),

            Self::Extension { children, .. } => children.len(),
        }
    }

    /// Returns direct child references in deterministic source/structural order.
    ///
    /// The returned vector is intentionally a caller-owned snapshot.
    ///
    /// AST graph traversal infrastructure that needs zero-allocation traversal
    /// should use a specialized visitor/traversal layer rather than repeatedly
    /// materializing this vector.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        let mut children = Vec::with_capacity(self.child_count());

        match self {
            Self::Expression { expression } => {
                children.push(*expression);
            }

            Self::Let {
                pattern,
                type_annotation,
                initializer,
            } => {
                children.push(*pattern);

                if let Some(type_annotation) = type_annotation {
                    children.push(*type_annotation);
                }

                if let Some(initializer) = initializer {
                    children.push(*initializer);
                }
            }

            Self::Const {
                pattern,
                type_annotation,
                initializer,
            } => {
                children.push(*pattern);

                if let Some(type_annotation) = type_annotation {
                    children.push(*type_annotation);
                }

                children.push(*initializer);
            }

            Self::Return { value } => {
                if let Some(value) = value {
                    children.push(*value);
                }
            }

            Self::Break { label, value } => {
                if let Some(label) = label {
                    children.push(*label);
                }

                if let Some(value) = value {
                    children.push(*value);
                }
            }

            Self::Continue { label } => {
                if let Some(label) = label {
                    children.push(*label);
                }
            }

            Self::If {
                condition,
                then_branch,
                else_branch,
            } => {
                children.push(*condition);
                children.push(*then_branch);

                if let Some(else_branch) = else_branch {
                    children.push(*else_branch);
                }
            }

            Self::While { condition, body } => {
                children.push(*condition);
                children.push(*body);
            }

            Self::For {
                pattern,
                iterable,
                body,
            } => {
                children.push(*pattern);
                children.push(*iterable);
                children.push(*body);
            }

            Self::Block { statements } => {
                children.extend(statements.iter().copied());
            }

            Self::Match { scrutinee, arms } => {
                children.push(*scrutinee);
                children.extend(arms.iter().copied());
            }

            Self::Declaration { declaration } => {
                children.push(*declaration);
            }

            Self::Error { recovered } => {
                if let Some(recovered) = recovered {
                    children.push(*recovered);
                }
            }

            Self::Extension { children: extension_children, .. } => {
                children.extend(extension_children.iter().copied());
            }
        }

        children
    }

    /// Performs local structural validation.
    ///
    /// This function does not inspect the referenced AST graph. The graph
    /// validator is responsible for verifying that every referenced `NodeId`
    /// exists and has the expected kind.
    pub fn validate_structure(
        &self,
        policy: StatementValidationPolicy,
    ) -> StatementResult<()> {
        let child_count = self.child_count();

        policy.check_children(child_count)?;

        match self {
            Self::Expression { expression } => {
                validate_required_child(*expression, "expression")?;
            }

            Self::Let {
                pattern,
                type_annotation,
                initializer,
            } => {
                validate_required_child(*pattern, "pattern")?;

                validate_optional_child(*type_annotation, "type_annotation")?;

                validate_optional_child(*initializer, "initializer")?;
            }

            Self::Const {
                pattern,
                type_annotation,
                initializer,
            } => {
                validate_required_child(*pattern, "pattern")?;

                validate_optional_child(*type_annotation, "type_annotation")?;

                validate_required_child(*initializer, "initializer")?;
            }

            Self::Return { value } => {
                validate_optional_child(*value, "value")?;
            }

            Self::Break { label, value } => {
                validate_optional_child(*label, "label")?;
                validate_optional_child(*value, "value")?;
            }

            Self::Continue { label } => {
                validate_optional_child(*label, "label")?;
            }

            Self::If {
                condition,
                then_branch,
                else_branch,
            } => {
                validate_required_child(*condition, "condition")?;
                validate_required_child(*then_branch, "then_branch")?;
                validate_optional_child(*else_branch, "else_branch")?;
            }

            Self::While { condition, body } => {
                validate_required_child(*condition, "condition")?;
                validate_required_child(*body, "body")?;
            }

            Self::For {
                pattern,
                iterable,
                body,
            } => {
                validate_required_child(*pattern, "pattern")?;
                validate_required_child(*iterable, "iterable")?;
                validate_required_child(*body, "body")?;
            }

            Self::Block { statements } => {
                for statement in statements {
                    validate_required_child(*statement, "statements")?;
                }
            }

            Self::Match { scrutinee, arms } => {
                validate_required_child(*scrutinee, "scrutinee")?;

                for arm in arms {
                    validate_required_child(*arm, "arms")?;
                }
            }

            Self::Declaration { declaration } => {
                validate_required_child(*declaration, "declaration")?;
            }

            Self::Error { recovered } => {
                validate_optional_child(*recovered, "recovered")?;
            }

            Self::Extension {
                namespace,
                name,
                children,
            } => {
                policy.check_extension_namespace(namespace)?;
                policy.check_extension_name(name)?;

                for child in children {
                    validate_required_child(*child, "extension.children")?;
                }
            }
        }

        Ok(())
    }
}

fn validate_required_child(
    child: NodeId,
    field: &'static str,
) -> StatementResult<()> {
    if child.get() == 0 {
        return Err(StatementError::InvalidChildReference { field });
    }

    Ok(())
}

fn validate_optional_child(
    child: Option<NodeId>,
    field: &'static str,
) -> StatementResult<()> {
    if let Some(child) = child {
        validate_required_child(child, field)?;
    }

    Ok(())
}

// =============================================================================
// Statement
// =============================================================================

/// Canonical source-level Zamani statement.
///
/// The common [`Node`] owns identity, source location, node kind and metadata.
/// [`StatementKind`] owns statement-specific source structure.
///
/// Child nodes remain owned by the surrounding AST graph and are referred to
/// by [`NodeId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Statement {
    /// Common source-level node information.
    node: Node,

    /// Statement-specific source structure.
    kind: StatementKind,
}

impl Statement {
    /// Creates a statement from an already allocated common AST node and a
    /// statement kind.
    ///
    /// The constructor performs only local structural consistency checks.
    /// It does not resolve or inspect child nodes.
    ///
    /// The caller is responsible for providing a `NodeKind` corresponding to
    /// the statement kind.
    pub fn new(
        node: Node,
        kind: StatementKind,
    ) -> StatementResult<Self> {
        let statement = Self { node, kind };

        statement.validate_structure(
            StatementValidationPolicy::unrestricted(),
        )?;

        statement.validate_node_kind()?;

        Ok(statement)
    }

    /// Creates a statement while deriving the canonical core node kind from
    /// the supplied statement kind.
    ///
    /// This constructor is useful to parsers/builders because it prevents the
    /// common error of constructing a statement whose `NodeKind` disagrees with
    /// its local `StatementKind`.
    pub fn from_node(
        node: Node,
        kind: StatementKind,
    ) -> StatementResult<Self> {
        let expected = kind.core_node_kind();

        if node.kind().as_core() != Some(expected)
            && !matches!(kind, StatementKind::Extension { .. })
        {
            return Err(StatementError::InvalidNodeKind {
                actual: node.kind_owned(),
            });
        }

        Self::new(node, kind)
    }

    /// Creates a native statement using the supplied node ID and statement
    /// kind.
    ///
    /// This convenience constructor uses the node's existing source span and
    /// metadata supplied through `Node`.
    ///
    /// `NodeKind` remains explicit so parser/builder code cannot silently lose
    /// source-level classification.
    pub fn with_node(
        node: Node,
        kind: StatementKind,
    ) -> StatementResult<Self> {
        Self::from_node(node, kind)
    }

    /// Returns the statement-specific representation.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> &StatementKind {
        &self.kind
    }

    /// Returns mutable access to the statement-specific representation.
    ///
    /// The caller must re-run structural validation after changing the kind.
    #[inline]
    pub fn kind_mut(&mut self) -> &mut StatementKind {
        &mut self.kind
    }

    /// Replaces the statement-specific representation.
    ///
    /// The new representation is structurally validated before replacement.
    pub fn replace_kind(
        &mut self,
        kind: StatementKind,
    ) -> StatementResult<StatementKind> {
        kind.validate_structure(
            StatementValidationPolicy::unrestricted(),
        )?;

        let expected = kind.core_node_kind();

        if !kind.is_extension()
            && self.node.kind().as_core() != Some(expected)
        {
            return Err(StatementError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        Ok(core::mem::replace(&mut self.kind, kind))
    }

    /// Returns the canonical AST node.
    #[inline]
    #[must_use]
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Returns mutable access to the canonical AST node.
    #[inline]
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }

    /// Returns the stable AST node identity.
    #[inline]
    #[must_use]
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Returns the node's source span.
    #[inline]
    #[must_use]
    pub fn span(&self) -> &super::super::source::Span {
        self.node.span()
    }

    /// Returns source-level node metadata.
    #[inline]
    #[must_use]
    pub fn metadata(
        &self,
    ) -> &super::super::metadata::NodeMetadata {
        self.node.metadata()
    }

    /// Returns the node classification.
    #[inline]
    #[must_use]
    pub fn node_kind(&self) -> &NodeKind {
        self.node.kind()
    }

    /// Returns the number of direct AST child references.
    #[inline]
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.kind.child_count()
    }

    /// Returns direct child references in deterministic structural order.
    ///
    /// This method performs one allocation for the returned snapshot.
    ///
    /// General compiler traversal should normally use the dedicated traversal
    /// subsystem so that very large ASTs can be traversed without allocating a
    /// new vector at every statement.
    #[must_use]
    pub fn child_node_ids(&self) -> Vec<NodeId> {
        self.kind.child_node_ids()
    }

    /// Performs local structural validation using an explicit caller policy.
    ///
    /// This method does not resolve child IDs against the AST graph.
    pub fn validate_structure(
        &self,
        policy: StatementValidationPolicy,
    ) -> StatementResult<()> {
        self.kind.validate_structure(policy)?;
        self.validate_node_kind()
    }

    /// Validates consistency between the common node classification and the
    /// concrete statement kind.
    pub fn validate_node_kind(&self) -> StatementResult<()> {
        let expected = self.kind.core_node_kind();

        if self.kind.is_extension() {
            if !self.node.kind().is_extension() {
                return Err(StatementError::InvalidNodeKind {
                    actual: self.node.kind_owned(),
                });
            }

            return Ok(());
        }

        if self.node.kind().as_core() != Some(expected) {
            return Err(StatementError::InvalidNodeKind {
                actual: self.node.kind_owned(),
            });
        }

        Ok(())
    }

    /// Returns whether this statement is an extension statement.
    #[inline]
    #[must_use]
    pub fn is_extension(&self) -> bool {
        self.kind.is_extension()
    }

    /// Returns the statement schema version.
    #[inline]
    #[must_use]
    pub const fn schema_version() -> u16 {
        STATEMENT_SCHEMA_VERSION
    }
}

impl AstNode for Statement {
    #[inline]
    fn node(&self) -> &Node {
        &self.node
    }

    #[inline]
    fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

// =============================================================================
// Statement constructors
// =============================================================================

impl Statement {
    /// Constructs an expression statement.
    pub fn expression(
        node: Node,
        expression: NodeId,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Expression { expression },
        )
    }

    /// Constructs a `let` statement.
    pub fn let_binding(
        node: Node,
        pattern: NodeId,
        type_annotation: Option<NodeId>,
        initializer: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Let {
                pattern,
                type_annotation,
                initializer,
            },
        )
    }

    /// Constructs a constant binding statement.
    pub fn const_binding(
        node: Node,
        pattern: NodeId,
        type_annotation: Option<NodeId>,
        initializer: NodeId,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Const {
                pattern,
                type_annotation,
                initializer,
            },
        )
    }

    /// Constructs a return statement.
    pub fn return_statement(
        node: Node,
        value: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(node, StatementKind::Return { value })
    }

    /// Constructs a break statement.
    pub fn break_statement(
        node: Node,
        label: Option<NodeId>,
        value: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Break { label, value },
        )
    }

    /// Constructs a continue statement.
    pub fn continue_statement(
        node: Node,
        label: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Continue { label },
        )
    }

    /// Constructs an if statement.
    pub fn if_statement(
        node: Node,
        condition: NodeId,
        then_branch: NodeId,
        else_branch: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::If {
                condition,
                then_branch,
                else_branch,
            },
        )
    }

    /// Constructs a while statement.
    pub fn while_statement(
        node: Node,
        condition: NodeId,
        body: NodeId,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::While { condition, body },
        )
    }

    /// Constructs a for statement.
    pub fn for_statement(
        node: Node,
        pattern: NodeId,
        iterable: NodeId,
        body: NodeId,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::For {
                pattern,
                iterable,
                body,
            },
        )
    }

    /// Constructs a statement block.
    pub fn block(
        node: Node,
        statements: Vec<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Block { statements },
        )
    }

    /// Constructs a match statement.
    pub fn match_statement(
        node: Node,
        scrutinee: NodeId,
        arms: Vec<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Match {
                scrutinee,
                arms,
            },
        )
    }

    /// Constructs a declaration statement.
    pub fn declaration(
        node: Node,
        declaration: NodeId,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Declaration { declaration },
        )
    }

    /// Constructs an error-recovery statement.
    pub fn error(
        node: Node,
        recovered: Option<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Error { recovered },
        )
    }

    /// Constructs an extension statement.
    ///
    /// Extension identity remains namespaced and opaque to the native AST.
    ///
    /// The caller must provide an extension `NodeKind` in the common `Node`.
    pub fn extension(
        node: Node,
        namespace: impl Into<String>,
        name: impl Into<String>,
        children: Vec<NodeId>,
    ) -> StatementResult<Self> {
        Self::from_node(
            node,
            StatementKind::Extension {
                namespace: namespace.into(),
                name: name.into(),
                children,
            },
        )
    }
}

// =============================================================================
// Debugging / display
// =============================================================================

impl fmt::Display for Statement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}#{}",
            self.node.kind(),
            self.node.id().get()
        )
    }
}

// =============================================================================
// Compile-time integration assertions
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ast::node::metadata::NodeMetadata;
    use crate::frontend::ast::node::node_id::NodeId;
    use crate::frontend::ast::node::node_kind::CoreNodeKind;
    use crate::frontend::ast::node::source::Span;

    fn node(
        id: u64,
        kind: CoreNodeKind,
    ) -> Node {
        let id = NodeId::new(id).expect("test node IDs must be non-zero");

        Node::without_metadata(
            id,
            NodeKind::core(kind),
            Span::default(),
        )
    }

    #[test]
    fn expression_statement_has_expected_kind() {
        let statement = Statement::expression(
            node(1, CoreNodeKind::ExpressionStatement),
            NodeId::new(2).expect("non-zero"),
        )
        .expect("valid expression statement");

        assert_eq!(
            statement.node_kind().as_core(),
            Some(CoreNodeKind::ExpressionStatement)
        );

        assert_eq!(statement.child_count(), 1);
    }

    #[test]
    fn let_statement_preserves_children() {
        let pattern = NodeId::new(2).expect("non-zero");
        let type_annotation = NodeId::new(3).expect("non-zero");
        let initializer = NodeId::new(4).expect("non-zero");

        let statement = Statement::let_binding(
            node(1, CoreNodeKind::LetStatement),
            pattern,
            Some(type_annotation),
            Some(initializer),
        )
        .expect("valid let statement");

        assert_eq!(
            statement.child_node_ids(),
            vec![pattern, type_annotation, initializer]
        );
    }

    #[test]
    fn block_is_source_ordered() {
        let first = NodeId::new(2).expect("non-zero");
        let second = NodeId::new(3).expect("non-zero");
        let third = NodeId::new(4).expect("non-zero");

        let statement = Statement::block(
            node(1, CoreNodeKind::Statement),
            vec![first, second, third],
        )
        .expect("valid block");

        assert_eq!(
            statement.child_node_ids(),
            vec![first, second, third]
        );
    }

    #[test]
    fn while_statement_is_target_neutral() {
        let condition = NodeId::new(2).expect("non-zero");
        let body = NodeId::new(3).expect("non-zero");

        let statement = Statement::while_statement(
            node(1, CoreNodeKind::WhileStatement),
            condition,
            body,
        )
        .expect("valid while statement");

        assert_eq!(
            statement.child_node_ids(),
            vec![condition, body]
        );
    }

    #[test]
    fn extension_statement_does_not_require_core_statement_kind() {
        let extension_kind = NodeKind::extension(
            "zamani.example",
            "custom-statement",
        )
        .expect("valid extension kind");

        let node = Node::without_metadata(
            NodeId::new(1).expect("non-zero"),
            extension_kind,
            Span::default(),
        );

        let statement = Statement::extension(
            node,
            "zamani.example",
            "custom-statement",
            vec![],
        )
        .expect("valid extension statement");

        assert!(statement.is_extension());
    }

    #[test]
    fn unrestricted_validation_has_no_artificial_child_limit() {
        let mut children = Vec::new();

        for value in 1_u64..=1024 {
            children.push(
                NodeId::new(value)
                    .expect("test IDs must be non-zero"),
            );
        }

        let kind = StatementKind::Block {
            statements: children,
        };

        kind.validate_structure(
            StatementValidationPolicy::unrestricted(),
        )
        .expect("unrestricted policy must not impose a fixed AST limit");
    }

    #[test]
    fn validation_policy_can_be_applied_externally() {
        let kind = StatementKind::Block {
            statements: vec![
                NodeId::new(1).expect("non-zero"),
                NodeId::new(2).expect("non-zero"),
            ],
        };

        let policy = StatementValidationPolicy {
            max_children: Some(1),
            ..StatementValidationPolicy::unrestricted()
        };

        let error = kind
            .validate_structure(policy)
            .expect_err("policy should reject two children");

        assert!(matches!(
            error,
            StatementError::LimitExceeded {
                limit: "max_children",
                actual: 2,
                maximum: 1,
            }
        ));
    }

    #[test]
    fn schema_version_is_stable() {
        assert_eq!(Statement::schema_version(), 1);
    }

    #[test]
    fn no_backend_information_is_required() {
        let statement = StatementKind::Expression {
            expression: NodeId::new(1).expect("non-zero"),
        };

        assert!(!statement.is_extension());
        assert_eq!(
            statement.core_node_kind(),
            CoreNodeKind::ExpressionStatement
        );
    }

    #[test]
    fn metadata_remains_owned_by_common_node() {
        let metadata = NodeMetadata::default();

        let node = Node::new(
            NodeId::new(1).expect("non-zero"),
            NodeKind::core(CoreNodeKind::ReturnStatement),
            Span::default(),
            metadata,
        );

        let statement = Statement::return_statement(
            node,
            None,
        )
        .expect("valid return statement");

        assert_eq!(
            statement.node_kind().as_core(),
            Some(CoreNodeKind::ReturnStatement)
        );
    }
}