//! Zamani Quantum IR — Conditional Branch
//!
//! Production-grade, hardware-independent representation of a two-way
//! conditional control-flow branch.
//!
//! # Architectural role
//!
//! `control::branch` represents the semantic meaning of:
//!
//! ```text
//! if <condition> {
//!     <true region>
//! } else {
//!     <false region>
//! }
//! ```
//!
//! A branch answers:
//!
//! > Which control-flow destination is selected when a classical predicate is
//! > true, and which destination is selected when it is false?
//!
//! It does NOT decide:
//!
//! - how the predicate is evaluated;
//! - which CPU, FPGA, QPU, or controller evaluates the predicate;
//! - which physical qubits are used;
//! - how quantum operations inside either destination are implemented;
//! - how the branch is scheduled;
//! - how the branch is routed;
//! - how pulses are generated;
//! - how hardware performs classical feedback;
//! - how a simulator executes the branch;
//! - how a backend lowers the branch;
//! - how an optimization pass transforms the branch.
//!
//! Those responsibilities belong to the appropriate downstream IR,
//! optimization, hardware, scheduling, simulator, and backend layers.
//!
//! # Canonical dependency boundary
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! canonical Quantum IR
//!      │
//!      ├── classical::predicate
//!      │          │
//!      │          ▼
//!      │      control::branch  ← this file
//!      │          │
//!      │          ▼
//!      │      program::block
//!      │          │
//!      │          ▼
//!      │      program::successor
//!      │
//!      ▼
//! validation
//!      │
//!      ├── optimization
//!      ├── routing
//!      ├── scheduling
//!      ├── hardware
//!      └── backend
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and can be lowered
//! to any compatible target for which sufficient resources and capabilities
//! exist.
//!
//! Therefore this file contains no assumptions about:
//!
//! - number of logical qubits;
//! - number of physical qubits;
//! - number of classical bits;
//! - machine width;
//! - operation count;
//! - branch count;
//! - block count;
//! - register width;
//! - hardware topology;
//! - quantum technology;
//! - vendor;
//! - controller architecture.
//!
//! There is no architectural maximum represented by this type.
//!
//! A branch containing one predicate and two destinations has exactly the same
//! semantic representation whether the surrounding program contains one qubit
//! or an extremely large finite number of qubits.
//!
//! Concrete resource limits are policy concerns and belong to the IR limits,
//! compiler, service, or deployment layers.
//!
//! # Important distinction: semantic branch versus successor
//!
//! `program::successor` owns the generic representation of one outgoing CFG
//! edge.
//!
//! This file owns the higher-level *two-way conditional branch* abstraction.
//!
//! Conceptually:
//!
//! ```text
//! Branch
//!   │
//!   ├── condition
//!   │
//!   ├── true target
//!   │       ├── BlockId
//!   │       └── transferred ValueId values
//!   │
//!   └── false target
//!           ├── BlockId
//!           └── transferred ValueId values
//! ```
//!
//! The enclosing `program::block` / `program::successor` infrastructure can
//! lower this semantic branch into its constituent CFG edges.
//!
//! This prevents the branch abstraction from becoming coupled to a particular
//! CFG storage strategy.
//!
//! # Classical predicate boundary
//!
//! Conditions are represented by the canonical:
//!
//! ```text
//! quantum::ir::classical::predicate::ClassicalPredicate
//! ```
//!
//! This file does not define another predicate type.
//!
//! A condition may originate from:
//!
//! ```text
//! measure(q)
//!     │
//!     ▼
//! ClassicalBitId
//!     │
//!     ▼
//! ClassicalPredicate
//!     │
//!     ▼
//! Branch
//! ```
//!
//! The branch does not know how the measurement was produced.
//!
//! # Qubit boundary
//!
//! A branch itself does not own a quantum-qubit reference.
//!
//! This is intentional.
//!
//! Quantum operations in the branch target blocks may reference:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! through the canonical operation/operand infrastructure.
//!
//! Therefore this file MUST NOT invent another qubit identity and MUST NOT
//! import `super::qubits`.
//!
//! When a branch-related API eventually needs an explicit logical-qubit
//! reference, it MUST use:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! # Value transfer boundary
//!
//! Branch destinations may accept block arguments.
//!
//! The values transferred to those destinations are represented by:
//!
//! ```text
//! quantum::ir::identity::ValueId
//! ```
//!
//! Concrete value definitions remain owned by the program/operation/value
//! infrastructure.
//!
//! This file therefore does not own:
//!
//! - SSA definitions;
//! - operation results;
//! - classical storage;
//! - quantum state;
//! - block argument declarations.
//!
//! It only records which semantic values are transferred along each branch
//! destination edge.
//!
//! # Global versus local validation
//!
//! This file performs local structural validation.
//!
//! It can verify:
//!
//! - a branch has a condition;
//! - true and false destinations are explicitly represented;
//! - transferred values are unique within each destination;
//! - no semantic vector mutation occurs after a failed fallible reservation;
//! - branch metadata remains internally consistent.
//!
//! It cannot determine without surrounding program context:
//!
//! - whether a target `BlockId` actually exists;
//! - whether a `ValueId` actually exists;
//! - whether a transferred value has the correct type;
//! - whether a destination block accepts the supplied number of arguments;
//! - whether the condition is reachable;
//! - whether the condition is ultimately derived from a measurement;
//! - whether the branch is reachable from the program entry point;
//! - whether the target hardware supports dynamic branching.
//!
//! Those checks belong to whole-IR validation and target capability validation.
//!
//! # Determinism
//!
//! Semantic ordering of transferred values is represented by `Vec<ValueId>`.
//!
//! The order is significant:
//!
//! ```text
//! target(v0, v1, v2)
//! ```
//!
//! is not automatically equivalent to:
//!
//! ```text
//! target(v2, v1, v0)
//! ```
//!
//! No `HashMap` or unordered collection is used in semantic storage.
//!
//! # Allocation safety
//!
//! Constructors that need to allocate can expose fallible variants using
//! `try_reserve`.
//!
//! This is important when constructing IR from:
//!
//! - untrusted input;
//! - generated programs;
//! - remote compilation requests;
//! - very large programs;
//! - fuzzing;
//! - language-server workloads;
//! - distributed compilation.
//!
//! No allocation failure is silently converted into a semantic error.
//!
//! # No hidden resource ceilings
//!
//! This file deliberately contains no constants such as:
//!
//! ```text
//! MAX_BRANCHES
//! MAX_BLOCKS
//! MAX_VALUES
//! MAX_QUBITS
//! MAX_CLASSICAL_BITS
//! MAX_DEPTH
//! ```
//!
//! Such limits belong to explicit resource/security policies such as
//! `QuantumIrLimits`.
//!
//! A compiler may impose a stricter policy for one invocation, but that policy
//! must not change the semantic meaning of `Branch`.
//!
//! # No unsafe code
//!
//! This module explicitly forbids unsafe code.
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
//! # Integration contract
//!
//! ## `identity.rs`
//!
//! Supplies the canonical:
//!
//! - `BlockId`;
//! - `ValueId`.
//!
//! This file does not define duplicate identities.
//!
//! ## `classical/predicate.rs`
//!
//! Supplies `ClassicalPredicate`.
//!
//! The branch stores the predicate but never evaluates it.
//!
//! ## `program/block.rs`
//!
//! Owns the containing block and operation ordering.
//!
//! A block may eventually lower this branch into outgoing successor edges.
//!
//! ## `program/successor.rs`
//!
//! Owns generic outgoing CFG edge semantics.
//!
//! A future integration layer can convert:
//!
//! ```text
//! Branch::true_target()
//! Branch::false_target()
//! ```
//!
//! into `Successor` records.
//!
//! This file intentionally does not depend on `Successor` so that a change in
//! the generic CFG representation cannot force a semantic branch redesign.
//!
//! ## `program/operation.rs`
//!
//! May represent branch/control-flow operations using this type as semantic
//! payload.
//!
//! This file does not own `OperationId`.
//!
//! ## `region.rs` / `program/region.rs`
//!
//! Resolve destination blocks and verify region membership.
//!
//! ## `validation.rs`
//!
//! Performs whole-program validation, including:
//!
//! - destination existence;
//! - value existence;
//! - value/type compatibility;
//! - CFG reachability;
//! - terminator legality;
//! - region ownership;
//! - control-flow consistency.
//!
//! ## `analysis.rs`
//!
//! May inspect the branch to calculate:
//!
//! - control-flow dependencies;
//! - branch counts;
//! - CFG structure;
//! - reachability;
//! - critical paths;
//! - dynamic-circuit properties.
//!
//! It must not mutate this semantic object.
//!
//! ## `serialization.rs`
//!
//! Must serialize every semantic field:
//!
//! - condition;
//! - true target;
//! - false target;
//! - true transferred values;
//! - false transferred values.
//!
//! Field order must be defined by the canonical serializer rather than by
//! incidental Rust memory layout.
//!
//! ## `hash.rs`
//!
//! May hash the canonical semantic representation.
//!
//! It must not hash pointers, addresses, allocation order, or capacity.
//!
//! ## `optimization/`
//!
//! May replace a branch with an equivalent structure, but must preserve
//! semantic behavior unless the optimization explicitly changes the surrounding
//! IR under a valid transformation contract.
//!
//! ## `routing/`
//!
//! Does not own this branch.
//!
//! Routing may inspect the quantum operations contained in destination blocks.
//!
//! ## `scheduling/`
//!
//! May derive timing constraints from branch structure but does not redefine
//! branch semantics.
//!
//! ## `hardware/`
//!
//! Determines whether the target supports dynamic conditional execution and,
//! if not, whether legal lowering/emulation is possible.
//!
//! # API stability rule
//!
//! The following concepts are stable semantic concepts:
//!
//! ```text
//! Branch
//! BranchTarget
//! BranchError
//! BranchResult
//! ```
//!
//! Internal storage may evolve as long as these semantic guarantees remain
//! intact.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::classical::predicate::ClassicalPredicate;
use super::super::identity::{BlockId, ValueId};

// =============================================================================
// Result
// =============================================================================

/// Result type for branch construction and mutation.
pub type BranchResult<T> = Result<T, BranchError>;

// =============================================================================
// Errors
// =============================================================================

/// Local structural errors produced by conditional-branch construction.
///
/// Global program/CFG errors remain the responsibility of the surrounding
/// validation layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchError {
    /// A branch condition was not supplied.
    MissingCondition,

    /// A branch destination was not supplied.
    MissingTarget {
        /// Semantic name of the missing destination.
        target: &'static str,
    },

    /// A transferred value was duplicated within one branch destination.
    DuplicateTransferredValue {
        /// Duplicated value.
        value: ValueId,

        /// Destination in which the duplication occurred.
        target: BranchTargetKind,
    },

    /// A transfer value cannot be structurally represented.
    InvalidTransferredValue {
        /// Invalid value identity.
        value: ValueId,

        /// Destination to which the value belongs.
        target: BranchTargetKind,
    },

    /// A fallible collection reservation failed.
    ///
    /// The standard library's allocation error is deliberately not embedded in
    /// the semantic IR error because doing so would couple the stable semantic
    /// API to allocator-specific implementation details.
    AllocationFailure {
        /// Semantic collection that could not be grown.
        collection: &'static str,
    },

    /// The branch structure is internally inconsistent.
    InvalidStructure {
        /// Static reason for the invalid structure.
        reason: &'static str,
    },
}

impl fmt::Display for BranchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCondition => {
                formatter.write_str("conditional branch requires a condition")
            }

            Self::MissingTarget { target } => {
                write!(
                    formatter,
                    "conditional branch requires a {target} target"
                )
            }

            Self::DuplicateTransferredValue {
                value,
                target,
            } => {
                write!(
                    formatter,
                    "value {value} is transferred more than once on \
                     {target} branch destination"
                )
            }

            Self::InvalidTransferredValue {
                value,
                target,
            } => {
                write!(
                    formatter,
                    "invalid transferred value {value} on \
                     {target} branch destination"
                )
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to allocate storage for branch {collection}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(
                    formatter,
                    "invalid conditional branch structure: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for BranchError {}

// =============================================================================
// Branch target kind
// =============================================================================

/// Identifies which destination side of a conditional branch is being
/// described.
///
/// This is semantic metadata used for deterministic diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BranchTargetKind {
    /// Destination selected when the condition evaluates to true.
    True,

    /// Destination selected when the condition evaluates to false.
    False,
}

impl fmt::Display for BranchTargetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => formatter.write_str("true"),
            Self::False => formatter.write_str("false"),
        }
    }
}

// =============================================================================
// Branch target
// =============================================================================

/// Destination of one side of a conditional branch.
///
/// A target contains a block identity and the ordered values transferred to
/// that block's arguments.
///
/// The target does not own the destination block itself.
///
/// Conceptually:
///
/// ```text
/// BranchTarget
///     │
///     ├── BlockId
///     └── [ValueId, ValueId, ...]
/// ```
///
/// The surrounding program/region owns the actual block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BranchTarget {
    block: BlockId,
    arguments: Vec<ValueId>,
}

impl BranchTarget {
    /// Creates a target with no transferred values.
    ///
    /// This constructor cannot fail because the empty vector requires no
    /// allocation.
    #[must_use]
    pub const fn new(block: BlockId) -> Self {
        Self {
            block,
            arguments: Vec::new(),
        }
    }

    /// Creates a target with an existing ordered argument list.
    ///
    /// The supplied vector is consumed without an additional allocation.
    ///
    /// Duplicate values are rejected because a destination argument list must
    /// not contain the same semantic value more than once.
    pub fn try_with_arguments(
        block: BlockId,
        arguments: Vec<ValueId>,
    ) -> BranchResult<Self> {
        validate_arguments(&arguments, BranchTargetKind::True)?;

        Ok(Self { block, arguments })
    }

    /// Returns the destination block identity.
    #[must_use]
    pub const fn block(&self) -> BlockId {
        self.block
    }

    /// Returns the ordered destination argument values.
    ///
    /// The returned slice is immutable, preserving ownership of the target.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns the number of values transferred to the destination.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether no values are transferred.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Reserves additional argument capacity without changing semantic state.
    ///
    /// The method performs a fallible allocation attempt. If allocation fails,
    /// the target remains unchanged.
    pub fn try_reserve_arguments(
        &mut self,
        additional: usize,
    ) -> BranchResult<()> {
        self.arguments
            .try_reserve(additional)
            .map_err(|_| BranchError::AllocationFailure {
                collection: "destination arguments",
            })
    }

    /// Appends one transferred value.
    ///
    /// The operation is atomic from the semantic perspective: duplicate values
    /// are rejected before mutation.
    pub fn try_push_argument(
        &mut self,
        value: ValueId,
    ) -> BranchResult<()> {
        if self.arguments.contains(&value) {
            return Err(BranchError::DuplicateTransferredValue {
                value,
                target: BranchTargetKind::True,
            });
        }

        self.arguments
            .try_reserve(1)
            .map_err(|_| BranchError::AllocationFailure {
                collection: "destination arguments",
            })?;

        self.arguments.push(value);

        Ok(())
    }

    /// Returns an iterator over destination argument identities.
    pub fn arguments_iter(
        &self,
    ) -> impl ExactSizeIterator<Item = &ValueId> {
        self.arguments.iter()
    }

    /// Validates local target invariants.
    ///
    /// The target block's existence cannot be checked here because that
    /// requires the surrounding region/program.
    pub fn validate(&self) -> BranchResult<()> {
        validate_arguments(
            &self.arguments,
            BranchTargetKind::True,
        )
    }

    /// Consumes this target and returns its components.
    ///
    /// This is useful for lowering into `program::successor` without exposing
    /// internal storage.
    #[must_use]
    pub fn into_parts(self) -> (BlockId, Vec<ValueId>) {
        (self.block, self.arguments)
    }
}

// =============================================================================
// Conditional branch
// =============================================================================

/// Canonical two-way conditional control-flow branch.
///
/// A `Branch` represents semantic control flow only:
///
/// ```text
///                 condition
///                    │
///              ┌─────┴─────┐
///             true        false
///              │            │
///              ▼            ▼
///         true block    false block
/// ```
///
/// The branch does not evaluate the predicate and does not execute either
/// destination itself.
///
/// # Invariants
///
/// A valid `Branch` always contains:
///
/// 1. one classical predicate;
/// 2. one true destination;
/// 3. one false destination;
/// 4. unique transferred values within each destination;
/// 5. deterministic argument ordering.
///
/// The true and false destinations are permitted to refer to the same block.
/// Such convergence is valid control-flow and must not be rejected merely
/// because the identities are equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Branch {
    condition: ClassicalPredicate,
    true_target: BranchTarget,
    false_target: BranchTarget,
}

impl Branch {
    /// Creates a conditional branch.
    ///
    /// Both destinations are explicit. This is deliberate: the canonical
    /// semantic representation does not use an implicit fallthrough block.
    ///
    /// If source-language syntax has no explicit `else`, the frontend/lowering
    /// layer should construct the appropriate CFG destination before creating
    /// this canonical branch.
    pub fn try_new(
        condition: ClassicalPredicate,
        true_target: BranchTarget,
        false_target: BranchTarget,
    ) -> BranchResult<Self> {
        let branch = Self {
            condition,
            true_target,
            false_target,
        };

        branch.validate()?;

        Ok(branch)
    }

    /// Returns the branch predicate.
    #[must_use]
    pub fn condition(&self) -> &ClassicalPredicate {
        &self.condition
    }

    /// Returns the true destination.
    #[must_use]
    pub const fn true_target(&self) -> &BranchTarget {
        &self.true_target
    }

    /// Returns the false destination.
    #[must_use]
    pub const fn false_target(&self) -> &BranchTarget {
        &self.false_target
    }

    /// Returns the true destination block.
    #[must_use]
    pub const fn true_block(&self) -> BlockId {
        self.true_target.block()
    }

    /// Returns the false destination block.
    #[must_use]
    pub const fn false_block(&self) -> BlockId {
        self.false_target.block()
    }

    /// Returns the values transferred to the true destination.
    #[must_use]
    pub fn true_arguments(&self) -> &[ValueId] {
        self.true_target.arguments()
    }

    /// Returns the values transferred to the false destination.
    #[must_use]
    pub fn false_arguments(&self) -> &[ValueId] {
        self.false_target.arguments()
    }

    /// Returns whether both branch paths converge on the same block.
    ///
    /// This is valid and is useful for CFGs where both paths perform different
    /// work before converging.
    #[must_use]
    pub const fn converges_immediately(&self) -> bool {
        self.true_block() == self.false_block()
    }

    /// Returns the total number of transferred values across both destinations.
    ///
    /// Overflow is impossible for two `usize` values when using checked
    /// arithmetic; this method deliberately uses checked addition so that the
    /// invariant remains explicit even on future refactors.
    pub fn total_transferred_values(&self) -> BranchResult<usize> {
        self.true_target
            .argument_count()
            .checked_add(self.false_target.argument_count())
            .ok_or(BranchError::InvalidStructure {
                reason: "transferred-value count overflow",
            })
    }

    /// Returns the number of values transferred to the selected destination.
    ///
    /// `BranchTargetKind` is semantic metadata and does not evaluate the
    /// predicate.
    #[must_use]
    pub fn arguments_for(
        &self,
        target: BranchTargetKind,
    ) -> &[ValueId] {
        match target {
            BranchTargetKind::True => self.true_arguments(),
            BranchTargetKind::False => self.false_arguments(),
        }
    }

    /// Returns the destination selected by the supplied semantic Boolean.
    ///
    /// This method is intentionally generic over an already-determined
    /// Boolean result and does not evaluate `ClassicalPredicate`.
    ///
    /// It is useful to lowering/interpretation layers that have already
    /// evaluated the predicate.
    #[must_use]
    pub fn target_for_bool(
        &self,
        condition_value: bool,
    ) -> &BranchTarget {
        if condition_value {
            self.true_target()
        } else {
            self.false_target()
        }
    }

    /// Replaces the branch condition.
    ///
    /// The destination structure remains unchanged.
    pub fn set_condition(
        &mut self,
        condition: ClassicalPredicate,
    ) {
        self.condition = condition;
    }

    /// Replaces the true destination.
    ///
    /// The new target is validated before state is changed.
    pub fn set_true_target(
        &mut self,
        target: BranchTarget,
    ) -> BranchResult<()> {
        target.validate()?;
        self.true_target = target;
        Ok(())
    }

    /// Replaces the false destination.
    ///
    /// The new target is validated before state is changed.
    pub fn set_false_target(
        &mut self,
        target: BranchTarget,
    ) -> BranchResult<()> {
        target.validate()?;
        self.false_target = target;
        Ok(())
    }

    /// Validates all locally enforceable branch invariants.
    ///
    /// Global program validation must additionally resolve destination blocks
    /// and transferred values against the containing IR.
    pub fn validate(&self) -> BranchResult<()> {
        self.true_target.validate()?;

        validate_arguments(
            self.false_target.arguments(),
            BranchTargetKind::False,
        )?;

        Ok(())
    }

    /// Consumes the branch into its semantic components.
    ///
    /// This provides a stable integration boundary for lowering layers.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ClassicalPredicate,
        BranchTarget,
        BranchTarget,
    ) {
        (
            self.condition,
            self.true_target,
            self.false_target,
        )
    }
}

// =============================================================================
// Argument validation
// =============================================================================

/// Validates uniqueness and structural integrity of a destination argument
/// list.
///
/// This function deliberately uses a simple deterministic scan instead of a
/// `HashSet`. Destination argument lists are normally small, and avoiding a
/// second collection keeps semantic storage minimal and deterministic.
///
/// The surrounding limits/validation layer can impose an explicit bound if
/// desired for hostile or exceptionally large input.
fn validate_arguments(
    arguments: &[ValueId],
    target: BranchTargetKind,
) -> BranchResult<()> {
    let length = arguments.len();

    let mut index = 0usize;

    while index < length {
        let value = arguments[index];

        // An identity is opaque and currently has no invalid sentinel in the
        // canonical identity model. Therefore the local validity invariant is
        // uniqueness. Program-wide existence is checked elsewhere.
        let mut next = index + 1;

        while next < length {
            if arguments[next] == value {
                return Err(BranchError::DuplicateTransferredValue {
                    value,
                    target,
                });
            }

            next += 1;
        }

        index += 1;
    }

    Ok(())
}

// =============================================================================
// Constructors for common branch forms
// =============================================================================

impl Branch {
    /// Creates a branch whose destinations receive no block arguments.
    ///
    /// This is the common form for:
    ///
    /// ```text
    /// if predicate {
    ///     ...
    /// } else {
    ///     ...
    /// }
    /// ```
    pub fn try_unparameterized(
        condition: ClassicalPredicate,
        true_block: BlockId,
        false_block: BlockId,
    ) -> BranchResult<Self> {
        Self::try_new(
            condition,
            BranchTarget::new(true_block),
            BranchTarget::new(false_block),
        )
    }

    /// Creates a branch with independently transferred true/false values.
    ///
    /// The supplied vectors are consumed by the resulting targets.
    pub fn try_with_arguments(
        condition: ClassicalPredicate,
        true_block: BlockId,
        true_arguments: Vec<ValueId>,
        false_block: BlockId,
        false_arguments: Vec<ValueId>,
    ) -> BranchResult<Self> {
        let true_target = BranchTarget::try_with_arguments(
            true_block,
            true_arguments,
        )?;

        let false_target = {
            validate_arguments(
                &false_arguments,
                BranchTargetKind::False,
            )?;

            BranchTarget {
                block: false_block,
                arguments: false_arguments,
            }
        };

        Self::try_new(
            condition,
            true_target,
            false_target,
        )
    }
}

// =============================================================================
// Debug/inspection helpers
// =============================================================================

impl Branch {
    /// Returns whether the branch has any transferred values.
    #[must_use]
    pub fn has_transferred_values(&self) -> bool {
        !self.true_arguments().is_empty()
            || !self.false_arguments().is_empty()
    }

    /// Returns whether both paths transfer the same number of values.
    ///
    /// Equal counts do not imply type compatibility; that requires whole-IR
    /// validation against the destination block signatures.
    #[must_use]
    pub fn has_equal_argument_arity(&self) -> bool {
        self.true_arguments().len() == self.false_arguments().len()
    }

    /// Returns the maximum destination argument count.
    ///
    /// This is analysis information, not a resource limit.
    #[must_use]
    pub fn max_argument_arity(&self) -> usize {
        self.true_arguments()
            .len()
            .max(self.false_arguments().len())
    }

    /// Returns an iterator over both branch destinations in deterministic order.
    ///
    /// The true destination is always yielded first, followed by the false
    /// destination.
    pub fn targets(
        &self,
    ) -> impl Iterator<Item = (BranchTargetKind, &BranchTarget)> {
        [
            (BranchTargetKind::True, self.true_target()),
            (BranchTargetKind::False, self.false_target()),
        ]
        .into_iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::classical::predicate::ClassicalPredicate;

    fn block(value: u64) -> BlockId {
        BlockId::new(value)
    }

    fn value(value: u64) -> ValueId {
        ValueId::new(value)
    }

    #[test]
    fn creates_unparameterized_branch() {
        let branch = Branch::try_unparameterized(
            ClassicalPredicate::always(),
            block(1),
            block(2),
        )
        .expect("branch should be valid");

        assert_eq!(branch.true_block(), block(1));
        assert_eq!(branch.false_block(), block(2));
        assert!(branch.true_arguments().is_empty());
        assert!(branch.false_arguments().is_empty());
        assert!(!branch.has_transferred_values());
        assert!(!branch.converges_immediately());
    }

    #[test]
    fn allows_same_destination_block() {
        let branch = Branch::try_unparameterized(
            ClassicalPredicate::never(),
            block(7),
            block(7),
        )
        .expect("convergent branch should be valid");

        assert!(branch.converges_immediately());
    }

    #[test]
    fn preserves_argument_order() {
        let branch = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            vec![value(10), value(11), value(12)],
            block(2),
            vec![value(20), value(21)],
        )
        .expect("branch should be valid");

        assert_eq!(
            branch.true_arguments(),
            &[value(10), value(11), value(12)]
        );

        assert_eq!(
            branch.false_arguments(),
            &[value(20), value(21)]
        );
    }

    #[test]
    fn rejects_duplicate_true_argument() {
        let result = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            vec![value(10), value(10)],
            block(2),
            Vec::new(),
        );

        assert_eq!(
            result,
            Err(BranchError::DuplicateTransferredValue {
                value: value(10),
                target: BranchTargetKind::True,
            })
        );
    }

    #[test]
    fn rejects_duplicate_false_argument() {
        let result = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            Vec::new(),
            block(2),
            vec![value(20), value(20)],
        );

        assert_eq!(
            result,
            Err(BranchError::DuplicateTransferredValue {
                value: value(20),
                target: BranchTargetKind::False,
            })
        );
    }

    #[test]
    fn true_and_false_arguments_are_independent() {
        let branch = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            vec![value(10)],
            block(2),
            vec![value(10)],
        )
        .expect("the same value may be passed along separate mutually exclusive edges");

        assert_eq!(branch.true_arguments(), &[value(10)]);
        assert_eq!(branch.false_arguments(), &[value(10)]);
    }

    #[test]
    fn target_selection_does_not_evaluate_predicate() {
        let branch = Branch::try_unparameterized(
            ClassicalPredicate::bit(
                super::super::super::classical::bit::ClassicalBitId::new(0),
            ),
            block(1),
            block(2),
        )
        .expect("branch should be valid");

        assert_eq!(
            branch.target_for_bool(true).block(),
            block(1)
        );

        assert_eq!(
            branch.target_for_bool(false).block(),
            block(2)
        );
    }

    #[test]
    fn total_transferred_values_is_checked() {
        let branch = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            vec![value(1), value(2)],
            block(2),
            vec![value(3)],
        )
        .expect("branch should be valid");

        assert_eq!(
            branch.total_transferred_values().unwrap(),
            3
        );
    }

    #[test]
    fn target_can_be_mutated_without_invalid_intermediate_state() {
        let mut target = BranchTarget::new(block(1));

        target
            .try_push_argument(value(10))
            .expect("first value should be accepted");

        let result = target.try_push_argument(value(10));

        assert_eq!(
            result,
            Err(BranchError::DuplicateTransferredValue {
                value: value(10),
                target: BranchTargetKind::True,
            })
        );

        assert_eq!(target.arguments(), &[value(10)]);
    }

    #[test]
    fn branch_replacement_is_validated_before_commit() {
        let mut branch = Branch::try_unparameterized(
            ClassicalPredicate::always(),
            block(1),
            block(2),
        )
        .expect("branch should be valid");

        let invalid_target = BranchTarget {
            block: block(3),
            arguments: vec![value(30), value(30)],
        };

        assert!(branch.set_true_target(invalid_target).is_err());

        assert_eq!(branch.true_block(), block(1));
        assert!(branch.true_arguments().is_empty());
    }

    #[test]
    fn targets_are_deterministic() {
        let branch = Branch::try_unparameterized(
            ClassicalPredicate::always(),
            block(1),
            block(2),
        )
        .expect("branch should be valid");

        let collected: Vec<_> = branch
            .targets()
            .map(|(kind, target)| (kind, target.block()))
            .collect();

        assert_eq!(
            collected,
            vec![
                (BranchTargetKind::True, block(1)),
                (BranchTargetKind::False, block(2)),
            ]
        );
    }

    #[test]
    fn clone_and_equality_are_semantic() {
        let branch = Branch::try_with_arguments(
            ClassicalPredicate::always(),
            block(1),
            vec![value(10)],
            block(2),
            vec![value(20)],
        )
        .expect("branch should be valid");

        let cloned = branch.clone();

        assert_eq!(branch, cloned);
    }
}