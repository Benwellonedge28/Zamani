//! Zamani Quantum IR — Classical Feedback Semantics
//!
//! Production-grade, hardware-independent representation of classical
//! feedback controlling quantum or hybrid IR operations.
//!
//! # Architectural role
//!
//! Classical feedback represents the semantic dependency:
//!
//! ```text
//! classical information
//!        │
//!        ▼
//! predicate
//!        │
//!        ▼
//! conditional execution
//!        │
//!        ▼
//! operation(s)
//! ```
//!
//! Typical dynamic-circuit example:
//!
//! ```text
//! measure(q0) -> c0
//!
//! if c0 == 1 {
//!     x(q1)
//! }
//! ```
//!
//! The canonical IR representation is conceptually:
//!
//! ```text
//! ClassicalFeedback {
//!     condition: c0 == 1,
//!     operations: [op_x_q1],
//! }
//! ```
//!
//! This module describes the semantic dependency only.
//!
//! It does NOT decide:
//!
//! - how the condition is evaluated;
//! - where classical state is stored;
//! - where an operation executes;
//! - which physical qubit executes it;
//! - how logical qubits are routed;
//! - how feedback latency is implemented;
//! - how hardware branches;
//! - how a controller/FPGA executes the predicate;
//! - how measurement data is transported;
//! - how pulses are generated;
//! - how scheduling is performed;
//! - how a backend executes the operation;
//! - how simulation is performed.
//!
//! Those responsibilities belong to downstream compiler, hardware, scheduler,
//! simulator, runtime and backend subsystems.
//!
//! # Canonical dependencies
//!
//! This file intentionally consumes existing canonical IR concepts:
//!
//! ```text
//! quantum::ir::identity::OperationId
//! quantum::ir::identity::ValueId
//! quantum::ir::classical::bit::ClassicalBitId
//! quantum::ir::classical::predicate::ClassicalPredicate
//! ```
//!
//! It MUST NOT define:
//!
//! - another `OperationId`;
//! - another `ClassicalBitId`;
//! - another `ValueId`;
//! - another predicate AST;
//! - another qubit identity;
//! - a hardware-specific feedback type.
//!
//! Logical qubit identity remains owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! A feedback object normally references operations rather than directly
//! referencing physical qubits. This is intentional: feedback controls
//! semantic operations, while operation operands determine the affected
//! quantum resources.
//!
//! # Universal-program principle
//!
//! A Zamani quantum program is written once at the semantic level and can be
//! lowered toward any compatible target.
//!
//! Therefore this module contains no architectural machine-size constants.
//!
//! It must work for:
//!
//! ```text
//! 1 qubit
//! 2 qubits
//! 64 qubits
//! 4096 qubits
//! 1,000,000 qubits
//! N finite logical qubits
//! ```
//!
//! subject only to:
//!
//! - available memory;
//! - explicit compiler/security limits;
//! - target capabilities;
//! - execution resources.
//!
//! No number in this file represents a maximum quantum-machine size.
//!
//! # Feedback semantics
//!
//! A feedback construct has four semantic properties:
//!
//! 1. a condition;
//! 2. one or more operation targets;
//! 3. an execution policy;
//! 4. optional dependency metadata.
//!
//! The condition answers:
//!
//! ```text
//! WHEN is the feedback action semantically enabled?
//! ```
//!
//! The operation references answer:
//!
//! ```text
//! WHAT semantic operations are enabled?
//! ```
//!
//! The execution policy answers:
//!
//! ```text
//! WHAT happens when the condition is true or false?
//! ```
//!
//! None of these answer where or how hardware executes the feedback.
//!
//! # Determinism
//!
//! Feedback operation references preserve insertion order.
//!
//! This is important because:
//!
//! - canonical serialization can preserve semantic ordering;
//! - diagnostics can identify the original order;
//! - transformations can explicitly reorder operations when legal;
//! - hashing is not affected by unordered collection iteration.
//!
//! This module therefore does not use `HashSet` or `HashMap` for semantic
//! storage.
//!
//! # Atomic mutation
//!
//! Public mutation APIs validate before mutation.
//!
//! A failed operation insertion leaves the feedback object unchanged.
//!
//! # Validation
//!
//! Validation is divided into:
//!
//! ```text
//! structural validation
//!        │
//!        ├── condition validity
//!        ├── non-empty operation set
//!        ├── duplicate operation detection
//!        └── dependency validity
//!
//! bounded validation
//!        │
//!        ├── explicit action-count policy
//!        └── predicate validation policy
//! ```
//!
//! The action-count policy is optional. `None` means that this layer imposes
//! no artificial semantic maximum.
//!
//! # Security
//!
//! This file:
//!
//! - contains no unsafe code;
//! - performs no I/O;
//! - performs no execution;
//! - performs no dynamic code loading;
//! - uses checked arithmetic where arithmetic is required;
//! - rejects duplicate operation references;
//! - rejects empty feedback actions;
//! - delegates predicate safety validation to `ClassicalPredicate`;
//! - does not treat identifiers as authority;
//! - does not grant hardware access.
//!
//! # Error boundary
//!
//! Feedback-specific errors remain local to this module.
//!
//! They are deliberately not merged into a second global error hierarchy
//! because the current repository's canonical `errors.rs` can evolve
//! independently.
//!
//! Higher-level validation may map `ClassicalFeedbackError` into the canonical
//! `IrError` vocabulary without requiring this file to know the final error
//! aggregation design.
//!
//! # Operation identity boundary
//!
//! An `OperationId` is only a reference.
//!
//! This file does not prove that the referenced operation exists in the
//! enclosing program because operation-table membership belongs to the program
//!/operation layer.
//!
//! Therefore:
//!
//! ```text
//! ClassicalFeedback
//!     validates identifier structure
//!
//! Program / Operation table
//!     validates identifier membership
//! ```
//!
//! This separation prevents circular dependencies.
//!
//! # Classical dependency boundary
//!
//! A predicate may reference:
//!
//! - classical bits;
//! - IR values;
//! - literals;
//! - compound Boolean expressions.
//!
//! This module does not duplicate those semantics.
//!
//! `ClassicalPredicate` remains the sole owner of predicate structure.
//!
//! # Integration contract
//!
//! ## `quantum::ir::classical::predicate`
//!
//! Provides:
//!
//! ```text
//! ClassicalPredicate
//! PredicateOperand
//! ```
//!
//! This file consumes `ClassicalPredicate` and validates it.
//!
//! ## `quantum::ir::classical::bit`
//!
//! Provides:
//!
//! ```text
//! ClassicalBitId
//! ```
//!
//! Feedback exposes referenced classical bits through the predicate rather
//! than maintaining a second classical-bit list that could become stale.
//!
//! ## `quantum::ir::identity`
//!
//! Provides:
//!
//! ```text
//! OperationId
//! ValueId
//! ```
//!
//! This file stores stable operation references only.
//!
//! ## `quantum::ir::operation`
//!
//! The operation subsystem owns actual operations.
//!
//! It may resolve each `OperationId` contained by a feedback object.
//!
//! The operation subsystem must not redefine `ClassicalFeedback`.
//!
//! ## `quantum::ir::program`
//!
//! Program-level validation must ensure every referenced operation belongs to
//! the enclosing program and is legally reachable from the feedback construct.
//!
//! ## `quantum::ir::control_flow`
//!
//! Structured control flow may contain or reference `ClassicalFeedback`.
//!
//! `control_flow.rs` owns structured branches/loops/transfers.
//!
//! This file owns the narrower semantic relationship between classical
//! conditions and operation execution.
//!
//! ## `quantum::ir::qubit`
//!
//! This module deliberately does not duplicate `QubitId`.
//!
//! Feedback controls semantic operations. The operations themselves own their
//! logical-qubit operands.
//!
//! This keeps:
//!
//! ```text
//! feedback -> operation -> QubitId
//! ```
//!
//! rather than creating:
//!
//! ```text
//! feedback -> duplicated qubit target
//! ```
//!
//! which could become inconsistent.
//!
//! ## `quantum::ir::timing` / `schedule`
//!
//! These modules may attach execution timing to feedback after semantic
//! lowering.
//!
//! This file contains no hardware latency or scheduling decisions.
//!
//! ## `quantum::ir::resources`
//!
//! Target/resource analysis may determine whether the selected execution
//! target can implement the feedback dependency.
//!
//! This file does not perform that analysis.
//!
//! ## `quantum::ir::validation`
//!
//! Whole-IR validation should perform cross-object checks:
//!
//! ```text
//! feedback.operation_id exists
//! feedback.operation_id belongs to enclosing program
//! predicate ValueId exists
//! classical bit belongs to declared namespace
//! control-flow placement is legal
//! ```
//!
//! This file provides the local half of that validation contract.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::classical::predicate::ClassicalPredicate;
use super::super::identity::{OperationId, ValueId};

// =============================================================================
// Result type
// =============================================================================

/// Result type used by classical-feedback construction and validation.
pub type ClassicalFeedbackResult<T> = Result<T, ClassicalFeedbackError>;

// =============================================================================
// Feedback execution policy
// =============================================================================

/// Semantic policy describing what happens when a feedback condition is
/// evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeedbackExecutionPolicy {
    /// Execute the referenced operations when the condition is true.
    ///
    /// If the condition is false, the operations are not enabled by this
    /// feedback object.
    ExecuteIfTrue,

    /// Execute the referenced operations when the condition is false.
    ///
    /// This is the semantic equivalent of an inverted condition, but is kept
    /// as an explicit policy so a lowering pass can preserve programmer intent
    /// without mutating the predicate.
    ExecuteIfFalse,
}

impl Default for FeedbackExecutionPolicy {
    fn default() -> Self {
        Self::ExecuteIfTrue
    }
}

impl FeedbackExecutionPolicy {
    /// Returns whether operations are enabled when the condition evaluates to
    /// true.
    #[must_use]
    pub const fn executes_on_true(self) -> bool {
        matches!(self, Self::ExecuteIfTrue)
    }

    /// Returns whether operations are enabled when the condition evaluates to
    /// false.
    #[must_use]
    pub const fn executes_on_false(self) -> bool {
        matches!(self, Self::ExecuteIfFalse)
    }
}

impl fmt::Display for FeedbackExecutionPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecuteIfTrue => formatter.write_str("execute_if_true"),
            Self::ExecuteIfFalse => formatter.write_str("execute_if_false"),
        }
    }
}

// =============================================================================
// Feedback dependency
// =============================================================================

/// Explicit semantic dependency used by classical feedback.
///
/// The predicate remains authoritative for determining the actual condition.
/// This type exists for consumers that need to inspect dependency sources
/// without rebuilding or traversing the predicate themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FeedbackDependency {
    /// Dependency on a logical classical bit.
    ClassicalBit(super::super::classical::bit::ClassicalBitId),

    /// Dependency on an existing IR value.
    Value(ValueId),
}

impl FeedbackDependency {
    /// Returns the referenced classical bit when this dependency is a
    /// classical-bit dependency.
    #[must_use]
    pub const fn classical_bit(
        self,
    ) -> Option<super::super::classical::bit::ClassicalBitId> {
        match self {
            Self::ClassicalBit(bit) => Some(bit),
            Self::Value(_) => None,
        }
    }

    /// Returns the referenced IR value when this dependency is a value
    /// dependency.
    #[must_use]
    pub const fn value(self) -> Option<ValueId> {
        match self {
            Self::ClassicalBit(_) => None,
            Self::Value(value) => Some(value),
        }
    }
}

// =============================================================================
// Validation limits
// =============================================================================

/// Explicit safety policy for classical-feedback validation.
///
/// These limits are validation/resource policies, not semantic limits of
/// Zamani.
///
/// `None` means that this layer does not impose a local maximum for that
/// quantity.
///
/// This is deliberately different from defining constants such as
/// `MAX_FEEDBACK_ACTIONS = 64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClassicalFeedbackValidationLimits {
    /// Optional maximum number of operation references in one feedback object.
    pub max_operations: Option<usize>,

    /// Whether the contained predicate must pass its own structural
    /// validation.
    pub validate_predicate: bool,
}

impl Default for ClassicalFeedbackValidationLimits {
    fn default() -> Self {
        Self {
            max_operations: None,
            validate_predicate: true,
        }
    }
}

impl ClassicalFeedbackValidationLimits {
    /// Creates an unrestricted local feedback policy.
    ///
    /// Predicate validation remains enabled.
    #[must_use]
    pub const fn unrestricted() -> Self {
        Self {
            max_operations: None,
            validate_predicate: true,
        }
    }

    /// Creates a policy with an explicit operation-reference limit.
    #[must_use]
    pub const fn with_max_operations(
        max_operations: usize,
    ) -> Self {
        Self {
            max_operations: Some(max_operations),
            validate_predicate: true,
        }
    }

    /// Disables predicate validation.
    ///
    /// This should only be used when the enclosing validation pipeline has
    /// already validated the same predicate and wants to avoid duplicate work.
    #[must_use]
    pub const fn without_predicate_validation(
        self,
    ) -> Self {
        Self {
            max_operations: self.max_operations,
            validate_predicate: false,
        }
    }

    /// Validates the policy itself.
    ///
    /// `Some(0)` is legal as a resource policy, although no non-empty feedback
    /// object can satisfy it.
    pub const fn validate(self) -> ClassicalFeedbackResult<()> {
        Ok(())
    }
}

// =============================================================================
// Errors
// =============================================================================

/// Errors produced while constructing or locally validating classical
/// feedback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassicalFeedbackError {
    /// A feedback object has no semantic condition.
    MissingCondition,

    /// A feedback object contains no operation targets.
    EmptyOperations,

    /// An operation reference occurs more than once.
    DuplicateOperation {
        /// Duplicated operation.
        operation: OperationId,

        /// First occurrence index.
        first_index: usize,

        /// Duplicate occurrence index.
        duplicate_index: usize,
    },

    /// The number of operation references exceeds an explicit validation
    /// policy.
    OperationLimitExceeded {
        /// Number requested.
        requested: usize,

        /// Configured maximum.
        maximum: usize,
    },

    /// A contained predicate failed structural validation.
    InvalidPredicate {
        /// Predicate validation error represented as text so this module does
        /// not become coupled to the predicate module's private error type.
        reason: String,
    },

    /// A dependency supplied by a caller does not occur in the predicate.
    UnusedDependency {
        /// Dependency supplied by the caller.
        dependency: FeedbackDependency,
    },

    /// The same dependency occurs more than once in explicit dependency
    /// metadata.
    DuplicateDependency {
        /// Duplicated dependency.
        dependency: FeedbackDependency,
    },

    /// An arithmetic operation required by validation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// The feedback structure itself is invalid.
    InvalidStructure {
        /// Static reason.
        reason: &'static str,
    },
}

impl fmt::Display for ClassicalFeedbackError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::MissingCondition => {
                formatter.write_str(
                    "classical feedback requires a condition",
                )
            }

            Self::EmptyOperations => {
                formatter.write_str(
                    "classical feedback requires at least one operation",
                )
            }

            Self::DuplicateOperation {
                operation,
                first_index,
                duplicate_index,
            } => {
                write!(
                    formatter,
                    "operation {operation} occurs more than once in \
                     classical feedback: first at index {first_index}, \
                     duplicate at index {duplicate_index}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "classical-feedback operation limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::InvalidPredicate { reason } => {
                write!(
                    formatter,
                    "invalid classical feedback predicate: {reason}"
                )
            }

            Self::UnusedDependency { dependency } => {
                write!(
                    formatter,
                    "feedback dependency {dependency:?} is not referenced \
                     by the feedback predicate"
                )
            }

            Self::DuplicateDependency { dependency } => {
                write!(
                    formatter,
                    "feedback dependency {dependency:?} occurs more than once"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid classical feedback structure: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for ClassicalFeedbackError {}

// =============================================================================
// Classical feedback
// =============================================================================

/// Canonical hardware-independent classical feedback construct.
///
/// A `ClassicalFeedback` connects a classical predicate to one or more
/// existing IR operations.
///
/// It does not own those operations.
///
/// Example:
///
/// ```text
/// measure(q0) -> c0
///
/// if c0 == 1 {
///     x(q1)
/// }
/// ```
///
/// becomes conceptually:
///
/// ```text
/// ClassicalFeedback {
///     condition: c0 == 1,
///     operations: [operation_id_for_x],
///     policy: ExecuteIfTrue,
/// }
/// ```
///
/// The enclosing program/operation table remains the owner of the referenced
/// operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClassicalFeedback {
    condition: ClassicalPredicate,
    operations: Vec<OperationId>,
    policy: FeedbackExecutionPolicy,
    dependencies: Vec<FeedbackDependency>,
}

impl ClassicalFeedback {
    // -------------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------------

    /// Creates a feedback object with the default `ExecuteIfTrue` policy.
    ///
    /// The operation list must contain at least one operation.
    pub fn new(
        condition: ClassicalPredicate,
        operations: Vec<OperationId>,
    ) -> ClassicalFeedbackResult<Self> {
        Self::with_policy(
            condition,
            operations,
            FeedbackExecutionPolicy::ExecuteIfTrue,
        )
    }

    /// Creates feedback with an explicit execution policy.
    pub fn with_policy(
        condition: ClassicalPredicate,
        operations: Vec<OperationId>,
        policy: FeedbackExecutionPolicy,
    ) -> ClassicalFeedbackResult<Self> {
        if operations.is_empty() {
            return Err(ClassicalFeedbackError::EmptyOperations);
        }

        let mut feedback = Self {
            condition,
            operations,
            policy,
            dependencies: Vec::new(),
        };

        feedback.rebuild_dependencies();

        feedback.validate()?;

        Ok(feedback)
    }

    /// Creates feedback while explicitly supplying dependency metadata.
    ///
    /// Dependency metadata is validated against the predicate.
    ///
    /// The predicate remains authoritative. The dependency vector is an
    /// ordered, deduplicated index of references for consumers that need
    /// efficient dependency inspection.
    pub fn with_dependencies(
        condition: ClassicalPredicate,
        operations: Vec<OperationId>,
        policy: FeedbackExecutionPolicy,
        dependencies: Vec<FeedbackDependency>,
    ) -> ClassicalFeedbackResult<Self> {
        if operations.is_empty() {
            return Err(ClassicalFeedbackError::EmptyOperations);
        }

        let feedback = Self {
            condition,
            operations,
            policy,
            dependencies,
        };

        feedback.validate()?;

        Ok(feedback)
    }

    // -------------------------------------------------------------------------
    // Accessors
    // -------------------------------------------------------------------------

    /// Returns the feedback predicate.
    #[must_use]
    pub const fn condition(&self) -> &ClassicalPredicate {
        &self.condition
    }

    /// Returns the operation references controlled by this feedback.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns the execution policy.
    #[must_use]
    pub const fn policy(&self) -> FeedbackExecutionPolicy {
        self.policy
    }

    /// Returns explicit dependency metadata.
    ///
    /// The returned dependencies are deterministic and deduplicated.
    #[must_use]
    pub fn dependencies(&self) -> &[FeedbackDependency] {
        &self.dependencies
    }

    /// Returns the number of controlled operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the feedback controls no operations.
    ///
    /// A constructed `ClassicalFeedback` should never return `true`, but the
    /// accessor is useful for generic callers and remains robust if a future
    /// deserialization path constructs invalid state before validation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns whether the condition enables operations when it evaluates to
    /// true.
    #[must_use]
    pub const fn executes_on_true(&self) -> bool {
        self.policy.executes_on_true()
    }

    /// Returns whether the condition enables operations when it evaluates to
    /// false.
    #[must_use]
    pub const fn executes_on_false(&self) -> bool {
        self.policy.executes_on_false()
    }

    // -------------------------------------------------------------------------
    // Classical dependencies
    // -------------------------------------------------------------------------

    /// Returns all classical-bit dependencies referenced by the predicate.
    ///
    /// The result is ordered by the canonical ordering of `ClassicalBitId`.
    ///
    /// The returned set is derived from the predicate and is therefore not
    /// affected by stale caller-maintained metadata.
    #[must_use]
    pub fn referenced_classical_bits(
        &self,
    ) -> BTreeSet<
        super::super::classical::bit::ClassicalBitId,
    > {
        self.condition.referenced_classical_bits()
    }

    /// Returns whether the feedback depends on a classical bit.
    #[must_use]
    pub fn references_classical_bit(
        &self,
        bit: super::super::classical::bit::ClassicalBitId,
    ) -> bool {
        self.condition.references_classical_bit(bit)
    }

    /// Returns whether the predicate contains an IR value dependency.
    #[must_use]
    pub fn contains_value_dependency(&self) -> bool {
        self.condition.contains_value_reference()
    }

    /// Returns all directly referenced IR values.
    ///
    /// This is collected without introducing a second predicate representation.
    #[must_use]
    pub fn referenced_values(&self) -> BTreeSet<ValueId> {
        let mut values = BTreeSet::new();
        collect_value_ids(&self.condition, &mut values);
        values
    }

    // -------------------------------------------------------------------------
    // Operation inspection
    // -------------------------------------------------------------------------

    /// Returns whether the feedback controls a particular operation.
    #[must_use]
    pub fn controls_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operations.contains(&operation)
    }

    /// Returns the first position of an operation reference.
    #[must_use]
    pub fn operation_position(
        &self,
        operation: OperationId,
    ) -> Option<usize> {
        self.operations
            .iter()
            .position(|candidate| *candidate == operation)
    }

    // -------------------------------------------------------------------------
    // Mutation
    // -------------------------------------------------------------------------

    /// Adds an operation reference atomically.
    ///
    /// Duplicate operation references are rejected.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) -> ClassicalFeedbackResult<()> {
        if let Some(first_index) = self
            .operations
            .iter()
            .position(|existing| *existing == operation)
        {
            return Err(
                ClassicalFeedbackError::DuplicateOperation {
                    operation,
                    first_index,
                    duplicate_index: self.operations.len(),
                },
            );
        }

        self.operations.push(operation);

        Ok(())
    }

    /// Adds several operation references atomically.
    ///
    /// If any operation is invalid or duplicated, no operation is inserted.
    pub fn extend_operations<I>(
        &mut self,
        operations: I,
    ) -> ClassicalFeedbackResult<()>
    where
        I: IntoIterator<Item = OperationId>,
    {
        let incoming: Vec<OperationId> =
            operations.into_iter().collect();

        if incoming.is_empty() {
            return Ok(());
        }

        validate_operation_sequence(
            &self.operations,
            &incoming,
        )?;

        self.operations.extend(incoming);

        Ok(())
    }

    /// Removes an operation reference.
    ///
    /// Removing the last operation is rejected because an empty feedback
    /// construct has no semantic effect and is structurally invalid.
    pub fn remove_operation(
        &mut self,
        operation: OperationId,
    ) -> ClassicalFeedbackResult<bool> {
        let Some(index) = self.operation_position(operation) else {
            return Ok(false);
        };

        if self.operations.len() == 1 {
            return Err(
                ClassicalFeedbackError::EmptyOperations,
            );
        }

        self.operations.remove(index);

        Ok(true)
    }

    /// Replaces the feedback predicate atomically.
    ///
    /// The dependency index is rebuilt only after the predicate has passed
    /// validation.
    pub fn set_condition(
        &mut self,
        condition: ClassicalPredicate,
    ) -> ClassicalFeedbackResult<()> {
        condition
            .validate()
            .map_err(|error| {
                ClassicalFeedbackError::InvalidPredicate {
                    reason: error.to_string(),
                }
            })?;

        self.condition = condition;
        self.rebuild_dependencies();

        Ok(())
    }

    /// Replaces the execution policy.
    pub const fn set_policy(
        &mut self,
        policy: FeedbackExecutionPolicy,
    ) {
        self.policy = policy;
    }

    /// Replaces explicit dependency metadata.
    ///
    /// The metadata is checked against the current predicate before mutation.
    pub fn set_dependencies(
        &mut self,
        dependencies: Vec<FeedbackDependency>,
    ) -> ClassicalFeedbackResult<()> {
        validate_dependencies(
            &self.condition,
            &dependencies,
        )?;

        self.dependencies = dependencies;

        Ok(())
    }

    /// Rebuilds dependency metadata from the predicate.
    ///
    /// This is deterministic and does not mutate the predicate.
    pub fn rebuild_dependencies(&mut self) {
        self.dependencies =
            derive_dependencies(&self.condition);
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    /// Performs default local validation.
    pub fn validate(
        &self,
    ) -> ClassicalFeedbackResult<()> {
        self.validate_with_limits(
            &ClassicalFeedbackValidationLimits::default(),
        )
    }

    /// Performs validation under an explicit local safety policy.
    pub fn validate_with_limits(
        &self,
        limits: &ClassicalFeedbackValidationLimits,
    ) -> ClassicalFeedbackResult<()> {
        limits.validate()?;

        if self.operations.is_empty() {
            return Err(
                ClassicalFeedbackError::EmptyOperations,
            );
        }

        if let Some(maximum) = limits.max_operations {
            if self.operations.len() > maximum {
                return Err(
                    ClassicalFeedbackError::OperationLimitExceeded {
                        requested: self.operations.len(),
                        maximum,
                    },
                );
            }
        }

        validate_unique_operations(&self.operations)?;

        if limits.validate_predicate {
            self.condition
                .validate()
                .map_err(|error| {
                    ClassicalFeedbackError::InvalidPredicate {
                        reason: error.to_string(),
                    }
                })?;
        }

        validate_dependencies(
            &self.condition,
            &self.dependencies,
        )?;

        Ok(())
    }

    /// Returns the number of semantic objects represented by this feedback
    /// construct.
    ///
    /// This is analysis only; it imposes no limit.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.operations
            .len()
            .saturating_add(self.condition.node_count())
            .saturating_add(self.dependencies.len())
    }

    /// Returns whether this feedback has a non-empty operation target set and
    /// a structurally valid predicate.
    #[must_use]
    pub fn is_structurally_valid(&self) -> bool {
        if self.operations.is_empty() {
            return false;
        }

        if validate_unique_operations(&self.operations).is_err() {
            return false;
        }

        self.condition.validate().is_ok()
    }
}

impl fmt::Display for ClassicalFeedback {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "feedback[policy={}, operations={}, condition={:?}]",
            self.policy,
            self.operations.len(),
            self.condition
        )
    }
}

// =============================================================================
// Constructors from common dynamic-circuit patterns
// =============================================================================

impl ClassicalFeedback {
    /// Creates feedback controlled directly by a classical bit.
    ///
    /// Equivalent semantic condition:
    ///
    /// ```text
    /// bit == true
    /// ```
    pub fn from_classical_bit(
        bit: super::super::classical::bit::ClassicalBitId,
        operation: OperationId,
    ) -> ClassicalFeedbackResult<Self> {
        Self::new(
            ClassicalPredicate::bit(bit),
            vec![operation],
        )
    }

    /// Creates feedback controlled by a classical bit with explicit polarity.
    pub fn from_classical_bit_value(
        bit: super::super::classical::bit::ClassicalBitId,
        value: bool,
        operation: OperationId,
    ) -> ClassicalFeedbackResult<Self> {
        Self::new(
            ClassicalPredicate::bit_equals(bit, value),
            vec![operation],
        )
    }

    /// Creates feedback from an arbitrary predicate and one operation.
    pub fn single_operation(
        condition: ClassicalPredicate,
        operation: OperationId,
    ) -> ClassicalFeedbackResult<Self> {
        Self::new(condition, vec![operation])
    }
}

// =============================================================================
// Free validation helpers
// =============================================================================

/// Validates a classical feedback object using the default policy.
pub fn validate_classical_feedback(
    feedback: &ClassicalFeedback,
) -> ClassicalFeedbackResult<()> {
    feedback.validate()
}

/// Validates a classical feedback object using an explicit policy.
pub fn validate_classical_feedback_with_limits(
    feedback: &ClassicalFeedback,
    limits: &ClassicalFeedbackValidationLimits,
) -> ClassicalFeedbackResult<()> {
    feedback.validate_with_limits(limits)
}

/// Validates that an operation sequence contains no duplicates.
///
/// The sequence may be arbitrarily large subject to available memory.
pub fn validate_unique_operations(
    operations: &[OperationId],
) -> ClassicalFeedbackResult<()> {
    let mut seen = BTreeSet::new();

    for (index, operation) in operations.iter().copied().enumerate() {
        if !seen.insert(operation) {
            let first_index = operations
                .iter()
                .position(|candidate| *candidate == operation)
                .ok_or(
                    ClassicalFeedbackError::ArithmeticOverflow {
                        calculation:
                            "operation duplicate lookup",
                    },
                )?;

            return Err(
                ClassicalFeedbackError::DuplicateOperation {
                    operation,
                    first_index,
                    duplicate_index: index,
                },
            );
        }
    }

    Ok(())
}

/// Validates a prospective extension to an existing operation sequence
/// without mutating either sequence.
pub fn validate_operation_sequence(
    existing: &[OperationId],
    incoming: &[OperationId],
) -> ClassicalFeedbackResult<()> {
    let mut seen = BTreeSet::new();

    for (index, operation) in existing.iter().copied().enumerate() {
        if !seen.insert(operation) {
            let first_index = existing
                .iter()
                .position(|candidate| *candidate == operation)
                .ok_or(
                    ClassicalFeedbackError::ArithmeticOverflow {
                        calculation:
                            "existing operation duplicate lookup",
                    },
                )?;

            return Err(
                ClassicalFeedbackError::DuplicateOperation {
                    operation,
                    first_index,
                    duplicate_index: index,
                },
            );
        }
    }

    let base_len = existing.len();

    for (offset, operation) in incoming.iter().copied().enumerate() {
        if !seen.insert(operation) {
            let duplicate_index = base_len
                .checked_add(offset)
                .ok_or(
                    ClassicalFeedbackError::ArithmeticOverflow {
                        calculation:
                            "operation extension index",
                    },
                )?;

            let first_index = existing
                .iter()
                .position(|candidate| *candidate == operation)
                .or_else(|| {
                    incoming
                        .iter()
                        .position(|candidate| *candidate == operation)
                        .map(|position| {
                            base_len.saturating_add(position)
                        })
                })
                .unwrap_or(duplicate_index);

            return Err(
                ClassicalFeedbackError::DuplicateOperation {
                    operation,
                    first_index,
                    duplicate_index,
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Dependency derivation
// =============================================================================

fn derive_dependencies(
    predicate: &ClassicalPredicate,
) -> Vec<FeedbackDependency> {
    let mut dependencies = BTreeSet::new();

    for bit in predicate.referenced_classical_bits() {
        dependencies.insert(
            FeedbackDependency::ClassicalBit(bit),
        );
    }

    let mut values = BTreeSet::new();
    collect_value_ids(predicate, &mut values);

    for value in values {
        dependencies.insert(
            FeedbackDependency::Value(value),
        );
    }

    dependencies.into_iter().collect()
}

fn validate_dependencies(
    predicate: &ClassicalPredicate,
    dependencies: &[FeedbackDependency],
) -> ClassicalFeedbackResult<()> {
    let mut seen = BTreeSet::new();

    let referenced_bits =
        predicate.referenced_classical_bits();

    let referenced_values =
        collect_value_id_set(predicate);

    for dependency in dependencies {
        if !seen.insert(*dependency) {
            return Err(
                ClassicalFeedbackError::DuplicateDependency {
                    dependency: *dependency,
                },
            );
        }

        let used = match dependency {
            FeedbackDependency::ClassicalBit(bit) => {
                referenced_bits.contains(bit)
            }

            FeedbackDependency::Value(value) => {
                referenced_values.contains(value)
            }
        };

        if !used {
            return Err(
                ClassicalFeedbackError::UnusedDependency {
                    dependency: *dependency,
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Predicate value-reference traversal
// =============================================================================

fn collect_value_id_set(
    predicate: &ClassicalPredicate,
) -> BTreeSet<ValueId> {
    let mut values = BTreeSet::new();
    collect_value_ids(predicate, &mut values);
    values
}

fn collect_value_ids(
    predicate: &ClassicalPredicate,
    values: &mut BTreeSet<ValueId>,
) {
    match predicate {
        ClassicalPredicate::Constant(_)
        | ClassicalPredicate::Bit(_) => {}

        ClassicalPredicate::Compare {
            left,
            right,
            ..
        } => {
            collect_operand_value_id(left, values);
            collect_operand_value_id(right, values);
        }

        ClassicalPredicate::Not(inner) => {
            collect_value_ids(inner, values);
        }

        ClassicalPredicate::And(predicates)
        | ClassicalPredicate::Or(predicates)
        | ClassicalPredicate::Xor(predicates) => {
            for predicate in predicates {
                collect_value_ids(predicate, values);
            }
        }

        ClassicalPredicate::Implies {
            antecedent,
            consequent,
        }
        | ClassicalPredicate::Equivalent {
            left: antecedent,
            right: consequent,
        } => {
            collect_value_ids(antecedent, values);
            collect_value_ids(consequent, values);
        }

        ClassicalPredicate::InSet {
            value,
            candidates,
        } => {
            collect_operand_value_id(value, values);

            for candidate in candidates {
                collect_operand_value_id(candidate, values);
            }
        }
    }
}

fn collect_operand_value_id(
    operand: &super::super::classical::predicate::PredicateOperand,
    values: &mut BTreeSet<ValueId>,
) {
    if let Some(value) = operand.value_id() {
        values.insert(value);
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::classical::bit::ClassicalBitId;

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn bit(value: usize) -> ClassicalBitId {
        ClassicalBitId::new(value)
    }

    #[test]
    fn creates_feedback_from_classical_bit() {
        let feedback =
            ClassicalFeedback::from_classical_bit(
                bit(0),
                operation(1),
            )
            .expect("feedback should be valid");

        assert_eq!(
            feedback.operation_count(),
            1
        );

        assert!(feedback.references_classical_bit(bit(0)));
        assert!(feedback.executes_on_true());
    }

    #[test]
    fn explicit_false_polarity_is_preserved() {
        let feedback =
            ClassicalFeedback::from_classical_bit_value(
                bit(3),
                false,
                operation(9),
            )
            .expect("feedback should be valid");

        assert!(feedback.references_classical_bit(bit(3)));
        assert!(feedback.executes_on_true());
    }

    #[test]
    fn false_execution_policy_is_distinct_from_predicate_inversion() {
        let feedback =
            ClassicalFeedback::with_policy(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
                FeedbackExecutionPolicy::ExecuteIfFalse,
            )
            .expect("feedback should be valid");

        assert!(feedback.executes_on_false());
        assert!(!feedback.executes_on_true());
    }

    #[test]
    fn duplicate_operations_are_rejected() {
        let result =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1), operation(1)],
            );

        assert!(matches!(
            result,
            Err(
                ClassicalFeedbackError::DuplicateOperation {
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_operation_list_is_rejected() {
        let result =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                Vec::new(),
            );

        assert!(matches!(
            result,
            Err(
                ClassicalFeedbackError::EmptyOperations
            )
        ));
    }

    #[test]
    fn add_operation_is_atomic_on_duplicate() {
        let mut feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
            )
            .expect("feedback should be valid");

        let result =
            feedback.add_operation(operation(1));

        assert!(result.is_err());
        assert_eq!(
            feedback.operations(),
            &[operation(1)]
        );
    }

    #[test]
    fn extend_operations_is_atomic() {
        let mut feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
            )
            .expect("feedback should be valid");

        let result =
            feedback.extend_operations([
                operation(2),
                operation(1),
                operation(3),
            ]);

        assert!(result.is_err());

        assert_eq!(
            feedback.operations(),
            &[operation(1)]
        );
    }

    #[test]
    fn removing_last_operation_is_rejected() {
        let mut feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
            )
            .expect("feedback should be valid");

        let result =
            feedback.remove_operation(operation(1));

        assert!(matches!(
            result,
            Err(
                ClassicalFeedbackError::EmptyOperations
            )
        ));

        assert_eq!(
            feedback.operations(),
            &[operation(1)]
        );
    }

    #[test]
    fn dependency_metadata_is_derived_from_predicate() {
        let predicate =
            ClassicalPredicate::and(vec![
                ClassicalPredicate::bit(bit(0)),
                ClassicalPredicate::bit(bit(4)),
            ])
            .expect("predicate should be valid");

        let feedback =
            ClassicalFeedback::new(
                predicate,
                vec![operation(10)],
            )
            .expect("feedback should be valid");

        assert_eq!(
            feedback.dependencies(),
            &[
                FeedbackDependency::ClassicalBit(bit(0)),
                FeedbackDependency::ClassicalBit(bit(4)),
            ]
        );
    }

    #[test]
    fn value_dependencies_are_exposed() {
        let value =
            ValueId::new(42);

        let predicate =
            ClassicalPredicate::equal(
                super::super::super::classical::predicate::PredicateOperand::value(value),
                super::super::super::classical::predicate::PredicateOperand::unsigned(1),
            )
            .expect("predicate should be valid");

        let feedback =
            ClassicalFeedback::new(
                predicate,
                vec![operation(7)],
            )
            .expect("feedback should be valid");

        assert!(feedback.contains_value_dependency());
        assert!(
            feedback
                .referenced_values()
                .contains(&value)
        );
    }

    #[test]
    fn explicit_unused_dependency_is_rejected() {
        let result =
            ClassicalFeedback::with_dependencies(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
                FeedbackExecutionPolicy::ExecuteIfTrue,
                vec![
                    FeedbackDependency::ClassicalBit(bit(1)),
                ],
            );

        assert!(matches!(
            result,
            Err(
                ClassicalFeedbackError::UnusedDependency {
                    ..
                }
            )
        ));
    }

    #[test]
    fn validation_limit_is_policy_not_architecture() {
        let feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![
                    operation(1),
                    operation(2),
                ],
            )
            .expect("feedback should be valid");

        assert!(
            feedback
                .validate_with_limits(
                    &ClassicalFeedbackValidationLimits::with_max_operations(2)
                )
                .is_ok()
        );

        assert!(
            feedback
                .validate_with_limits(
                    &ClassicalFeedbackValidationLimits::with_max_operations(1)
                )
                .is_err()
        );

        assert!(
            feedback
                .validate_with_limits(
                    &ClassicalFeedbackValidationLimits::unrestricted()
                )
                .is_ok()
        );
    }

    #[test]
    fn condition_replacement_rebuilds_dependencies() {
        let mut feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
            )
            .expect("feedback should be valid");

        feedback
            .set_condition(
                ClassicalPredicate::bit(bit(8)),
            )
            .expect("condition replacement should succeed");

        assert!(!feedback.references_classical_bit(bit(0)));
        assert!(feedback.references_classical_bit(bit(8)));

        assert_eq!(
            feedback.dependencies(),
            &[
                FeedbackDependency::ClassicalBit(bit(8))
            ]
        );
    }

    #[test]
    fn operation_order_is_preserved() {
        let feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![
                    operation(7),
                    operation(2),
                    operation(9),
                ],
            )
            .expect("feedback should be valid");

        assert_eq!(
            feedback.operations(),
            &[
                operation(7),
                operation(2),
                operation(9),
            ]
        );
    }

    #[test]
    fn structurally_valid_feedback_reports_true() {
        let feedback =
            ClassicalFeedback::new(
                ClassicalPredicate::bit(bit(0)),
                vec![operation(1)],
            )
            .expect("feedback should be valid");

        assert!(feedback.is_structurally_valid());
    }
}