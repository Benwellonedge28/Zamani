//! Zamani Quantum IR — Program Successors
//!
//! Production-grade representation of structured control-flow edges.
//!
//! # Architectural role
//!
//! `quantum::ir::program::successor` owns the semantic representation of an
//! outgoing control-flow edge from one IR block to another.
//!
//! A successor answers:
//!
//! > Which block executes next, and which values/resources are transferred
//! > into that destination block?
//!
//! It does NOT decide:
//!
//! - which physical machine executes the edge;
//! - how blocks are scheduled;
//! - how blocks are routed;
//! - how quantum operations are optimized;
//! - how a backend implements a branch;
//! - how a classical processor implements a predicate;
//! - how a quantum device performs measurement;
//! - which hardware topology is used.
//!
//! Those concerns belong to downstream IR consumers.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! canonical quantum::ir
//!      │
//!      ├── program
//!      │    ├── block
//!      │    └── successor   ← this file
//!      │
//!      ├── operation
//!      ├── control flow
//!      ├── quantum semantics
//!      └── classical semantics
//!      │
//!      ▼
//! validation
//!      │
//!      ▼
//! optimization / routing / scheduling
//!      │
//!      ▼
//! hardware
//! ```
//!
//! # Universal-program principle
//!
//! A successor is a semantic control-flow relationship.
//!
//! It must therefore work for:
//!
//! - one block;
//! - many blocks;
//! - deeply nested regions;
//! - very large finite control-flow graphs;
//! - dynamic quantum circuits;
//! - classical feedback;
//! - loops;
//! - conditional branches;
//! - switch-like control flow;
//! - functions/subroutines;
//! - error/recovery paths;
//! - future control-flow dialects.
//!
//! There is deliberately no fixed:
//!
//! - block count;
//! - successor count;
//! - qubit count;
//! - classical value count;
//! - register size;
//! - machine size;
//! - topology;
//! - operation arity.
//!
//! Actual allocation/security limits belong to the IR limits layer.
//!
//! # Identity ownership
//!
//! Block identities are owned by:
//!
//! `quantum::ir::identity::BlockId`
//!
//! Operation identities are owned by:
//!
//! `quantum::ir::identity::OperationId`
//!
//! Value identities are owned by:
//!
//! `quantum::ir::identity::ValueId`
//!
//! Logical and physical qubit identities are owned by:
//!
//! `quantum::ir::qubit::QubitId`
//! `quantum::ir::qubit::PhysicalQubitId`
//!
//! This module MUST NOT define duplicate identity types.
//!
//! # Important dependency rule
//!
//! This module does not depend on:
//!
//! - frontend code;
//! - optimization;
//! - routing;
//! - scheduling;
//! - hardware;
//! - simulator;
//! - QEC;
//! - backend execution.
//!
//! Those systems may depend on this module.
//!
//! # Block integration
//!
//! `program::block` owns the ordered list of successor edges associated with a
//! block.
//!
//! This file owns the semantic representation of one edge.
//!
//! Therefore:
//!
//! ```text
//! Block
//!   │
//!   ├── OperationId
//!   ├── OperationId
//!   ├── OperationId
//!   │
//!   └── Successor
//!          │
//!          ├── target BlockId
//!          ├── transferred values
//!          ├── edge kind
//!          └── optional semantic condition
//! ```
//!
//! The block owns ordering.
//! The successor owns edge semantics.
//!
//! # Destination arguments
//!
//! A successor may transfer zero or more values to its destination block.
//!
//! This is deliberately represented using `ValueId` rather than concrete
//! `Operation` objects.
//!
//! Therefore the successor remains independent from the operation storage
//! architecture.
//!
//! Example:
//!
//! ```text
//! block_a
//!     │
//!     │ condition = c0
//!     │ arguments = [v1, v2]
//!     ▼
//! block_b(v1, v2)
//! ```
//!
//! # Quantum resources
//!
//! Quantum values may be represented by `ValueId` and/or explicit qubit
//! references in the edge metadata.
//!
//! This module uses the canonical:
//!
//! `quantum::ir::qubit`
//!
//! namespace.
//!
//! It MUST NOT use a legacy `qubits` module.
//!
//! # Conditional control flow
//!
//! A conditional successor represents semantic branching.
//!
//! Example:
//!
//! ```text
//! measure q -> c
//!
//! if c == 1 {
//!     block_true
//! } else {
//!     block_false
//! }
//! ```
//!
//! The condition is represented semantically. This module does not evaluate
//! it.
//!
//! # Switch control flow
//!
//! Switch-like control flow can have multiple outgoing edges.
//!
//! Each edge may carry a case discriminator.
//!
//! The discriminator is represented as semantic data rather than being tied to
//! a particular classical integer width.
//!
//! # Loops
//!
//! Loops are represented by ordinary successor edges.
//!
//! Example:
//!
//! ```text
//! header ───────► body
//!   ▲              │
//!   │              │
//!   └──────────────┘
//! ```
//!
//! No special machine-level loop implementation is required here.
//!
//! # Exception/error/recovery paths
//!
//! Error and recovery control flow are represented through edge kinds.
//!
//! This allows future runtimes and backends to interpret the edge without
//! requiring this module to know how recovery is implemented.
//!
//! # Determinism
//!
//! A successor stores transferred values in `Vec` because ordering is
//! semantically meaningful for block arguments.
//!
//! A `BTreeSet` is used internally only for validation of uniqueness.
//!
//! No `HashMap` iteration order contributes to semantic representation.
//!
//! # Allocation safety
//!
//! Fallible collection growth uses `try_reserve`.
//!
//! This allows callers processing untrusted or extremely large IR to
//! distinguish ordinary semantic errors from allocation failure.
//!
//! No `unsafe` code is used.
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
//! - no `unsafe`.
//!
//! # Integration contract
//!
//! `identity.rs`
//!     Supplies canonical `BlockId` and `ValueId`.
//!
//! `qubit.rs`
//!     Supplies canonical `QubitId` and `PhysicalQubitId`.
//!
//! `block.rs`
//!     Owns blocks and their ordered successor collection.
//!
//! `region.rs`
//!     Owns region-level structure and block relationships.
//!
//! `operation.rs`
//!     Owns concrete semantic operations. Successors reference values, not
//!     concrete operations.
//!
//! `control_flow.rs`
//!     May construct or interpret successor conditions and edge kinds.
//!
//! `validation.rs`
//!     Performs whole-program checks such as whether the destination block
//!     actually exists and whether transferred values are type-compatible with
//!     destination block arguments.
//!
//! `analysis.rs`
//!     May inspect successors without mutating them.
//!
//! `serialization.rs`
//!     Owns canonical persistence. This file only guarantees deterministic
//!     field access.
//!
//! `hash.rs`
//!     Owns canonical content hashing. This file does not hash itself.
//!
//! `optimization/`
//!     May redirect or reconstruct edges while preserving semantic invariants.
//!
//! `routing/`
//!     May inspect qubit metadata but does not own successor semantics.
//!
//! `scheduling/`
//!     May derive timing from control-flow edges but does not redefine them.
//!
//! `hardware/`
//!     May lower successor semantics to target-specific execution mechanisms.
//!
//! # Local versus global validation
//!
//! This module performs local structural validation.
//!
//! It can verify:
//!
//! - target identity is structurally valid;
//! - transferred values are unique;
//! - condition references are structurally valid;
//! - case labels are unique within an edge set where applicable;
//! - edge metadata is internally consistent;
//! - logical/physical qubit references are not duplicated.
//!
//! It intentionally cannot verify:
//!
//! - whether the target block exists in the containing region;
//! - whether a value actually exists in the program;
//! - whether the value's type matches a destination argument;
//! - whether the condition value is actually boolean;
//! - whether a qubit belongs to the surrounding program;
//! - whether a control-flow graph is globally reachable.
//!
//! Those require surrounding program context and belong to `validation.rs`.
//!
//! # Production-readiness contract
//!
//! A successor is production-ready when:
//!
//! 1. its target block is explicit;
//! 2. edge semantics are explicit;
//! 3. transferred values preserve deterministic order;
//! 4. duplicate transferred values are rejected;
//! 5. optional condition semantics are explicit;
//! 6. switch/case semantics are explicit;
//! 7. quantum resource references use canonical qubit identities;
//! 8. no hardware assumptions are embedded;
//! 9. no fixed-size assumptions are embedded;
//! 10. no `usize` value is used as semantic identity;
//! 11. collection growth can be handled fallibly;
//! 12. cloning is deterministic;
//! 13. serialization can inspect every semantic field;
//! 14. hashing can inspect every semantic field;
//! 15. global validation remains possible without hidden state.
//!
//! # Stability principle
//!
//! New control-flow mechanisms should normally be introduced by adding an
//! extensible `SuccessorKind` or metadata/extension rather than changing the
//! meaning of existing fields.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::identity::{
    BlockId,
    ValueId,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
};

// =============================================================================
// Result
// =============================================================================

/// Result type used by successor construction and mutation.
pub type SuccessorResult<T> = Result<T, SuccessorError>;

// =============================================================================
// Errors
// =============================================================================

/// Local structural errors produced by successor construction and mutation.
///
/// Global graph errors belong to the surrounding program/validation layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuccessorError {
    /// The destination block identity is structurally invalid.
    InvalidTarget {
        /// Invalid destination block.
        target: BlockId,
    },

    /// A transferred value identity is structurally invalid.
    InvalidValue {
        /// Invalid value.
        value: ValueId,
    },

    /// The same value was supplied more than once.
    DuplicateValue {
        /// Duplicated value.
        value: ValueId,
    },

    /// A condition value is structurally invalid.
    InvalidCondition {
        /// Invalid condition value.
        value: ValueId,
    },

    /// A case discriminator is invalid.
    InvalidCaseValue,

    /// The same case discriminator was inserted more than once.
    DuplicateCase,

    /// A logical qubit reference is structurally invalid.
    InvalidLogicalQubit {
        /// Invalid logical qubit.
        qubit: QubitId,
    },

    /// The same logical qubit was inserted more than once.
    DuplicateLogicalQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A physical qubit reference is structurally invalid.
    InvalidPhysicalQubit {
        /// Invalid physical qubit.
        qubit: PhysicalQubitId,
    },

    /// The same physical qubit was inserted more than once.
    DuplicatePhysicalQubit {
        /// Duplicated physical qubit.
        qubit: PhysicalQubitId,
    },

    /// A switch/default edge was given incompatible metadata.
    InvalidSwitchMetadata,

    /// A conditional edge was given incompatible metadata.
    InvalidConditionalMetadata,

    /// An unconditional edge was given conditional metadata.
    InvalidUnconditionalMetadata,

    /// A collection could not reserve memory.
    AllocationFailure {
        /// Name of the collection.
        collection: &'static str,
    },

    /// A structurally invalid successor was encountered.
    InvalidSuccessor {
        /// Static diagnostic.
        message: &'static str,
    },
}

impl fmt::Display for SuccessorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTarget { target } => {
                write!(formatter, "invalid successor target {target}")
            }

            Self::InvalidValue { value } => {
                write!(formatter, "invalid successor value {value}")
            }

            Self::DuplicateValue { value } => {
                write!(formatter, "successor value {value} appears more than once")
            }

            Self::InvalidCondition { value } => {
                write!(formatter, "invalid successor condition value {value}")
            }

            Self::InvalidCaseValue => {
                formatter.write_str("invalid switch case value")
            }

            Self::DuplicateCase => {
                formatter.write_str("duplicate switch case")
            }

            Self::InvalidLogicalQubit { qubit } => {
                write!(formatter, "invalid logical qubit {qubit}")
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} appears more than once"
                )
            }

            Self::InvalidPhysicalQubit { qubit } => {
                write!(formatter, "invalid physical qubit {qubit}")
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "physical qubit {qubit} appears more than once"
                )
            }

            Self::InvalidSwitchMetadata => {
                formatter.write_str("invalid switch successor metadata")
            }

            Self::InvalidConditionalMetadata => {
                formatter.write_str("invalid conditional successor metadata")
            }

            Self::InvalidUnconditionalMetadata => {
                formatter.write_str("unconditional successor contains conditional metadata")
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for successor {collection}"
                )
            }

            Self::InvalidSuccessor { message } => {
                write!(formatter, "invalid successor: {message}")
            }
        }
    }
}

impl std::error::Error for SuccessorError {}

// =============================================================================
// Successor kind
// =============================================================================

/// Semantic category of a control-flow edge.
///
/// These categories describe program semantics, not hardware execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuccessorKind {
    /// Unconditional transfer.
    Unconditional,

    /// True/selected branch.
    ConditionalTrue,

    /// False/unselected branch.
    ConditionalFalse,

    /// Loop back-edge.
    LoopBack,

    /// Loop exit edge.
    LoopExit,

    /// Loop continuation edge.
    LoopContinue,

    /// Switch case edge.
    SwitchCase,

    /// Default switch edge.
    SwitchDefault,

    /// Function/subroutine return edge.
    Return,

    /// Exceptional/error/recovery edge.
    Error,

    /// Extension-defined control-flow edge.
    Extension,
}

impl Default for SuccessorKind {
    fn default() -> Self {
        Self::Unconditional
    }
}

impl SuccessorKind {
    /// Returns whether this edge is semantically conditional.
    #[must_use]
    pub const fn is_conditional(self) -> bool {
        matches!(
            self,
            Self::ConditionalTrue
                | Self::ConditionalFalse
                | Self::SwitchCase
                | Self::SwitchDefault
        )
    }

    /// Returns whether this edge is a loop-related edge.
    #[must_use]
    pub const fn is_loop_edge(self) -> bool {
        matches!(
            self,
            Self::LoopBack
                | Self::LoopExit
                | Self::LoopContinue
        )
    }

    /// Returns whether this edge terminates a control-flow path.
    #[must_use]
    pub const fn is_return(self) -> bool {
        matches!(self, Self::Return)
    }

    /// Returns whether this edge represents an error/recovery path.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns a stable semantic name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unconditional => "unconditional",
            Self::ConditionalTrue => "conditional_true",
            Self::ConditionalFalse => "conditional_false",
            Self::LoopBack => "loop_back",
            Self::LoopExit => "loop_exit",
            Self::LoopContinue => "loop_continue",
            Self::SwitchCase => "switch_case",
            Self::SwitchDefault => "switch_default",
            Self::Return => "return",
            Self::Error => "error",
            Self::Extension => "extension",
        }
    }
}

impl fmt::Display for SuccessorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Switch case value
// =============================================================================

/// A target-independent switch discriminator.
///
/// This is deliberately represented as an arbitrary-width bit vector rather
/// than a fixed Rust integer type.
///
/// The number of bits is part of the semantic value.
///
/// The representation is canonical big-endian bytes.
///
/// Example:
///
/// ```text
/// 0x00
/// 0x01
/// 0xFF
/// 0x0100
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SwitchCaseValue {
    bit_width: u64,
    bytes: Vec<u8>,
}

impl SwitchCaseValue {
    /// Creates a switch case value from a fixed-width big-endian byte vector.
    ///
    /// The byte vector is preserved exactly so that leading zeroes remain
    /// semantically meaningful when the declared width requires them.
    pub fn new(bit_width: u64, bytes: Vec<u8>) -> SuccessorResult<Self> {
        if bit_width == 0 {
            return Err(SuccessorError::InvalidCaseValue);
        }

        let required_bytes = bit_width
            .checked_add(7)
            .ok_or(SuccessorError::InvalidCaseValue)?
            / 8;

        if required_bytes != bytes.len() as u64 {
            return Err(SuccessorError::InvalidCaseValue);
        }

        if bit_width % 8 != 0 {
            let unused_bits = 8 - (bit_width % 8);
            let mask = 0xFFu8 >> unused_bits;

            if let Some(first) = bytes.first() {
                if first & !mask != 0 {
                    return Err(SuccessorError::InvalidCaseValue);
                }
            }
        }

        Ok(Self {
            bit_width,
            bytes,
        })
    }

    /// Creates a switch case value from a `u64`.
    ///
    /// This convenience constructor does not restrict the IR to 64-bit
    /// switch values. Arbitrary-width values remain available through
    /// [`SwitchCaseValue::new`].
    pub fn from_u64(value: u64) -> Self {
        let mut bytes = value.to_be_bytes().to_vec();

        while bytes.len() > 1 && bytes[0] == 0 {
            bytes.remove(0);
        }

        let bit_width = (bytes.len() as u64) * 8;

        Self {
            bit_width,
            bytes,
        }
    }

    /// Returns the semantic bit width.
    #[must_use]
    pub const fn bit_width(&self) -> u64 {
        self.bit_width
    }

    /// Returns the canonical big-endian byte representation.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the value and returns its bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns whether the value is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.bytes.iter().all(|byte| *byte == 0)
    }
}

// =============================================================================
// Edge condition
// =============================================================================

/// Semantic condition attached to a successor edge.
///
/// Conditions are references to canonical IR values. Evaluation belongs to
/// the control-flow/runtime layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuccessorCondition {
    /// A boolean-like value selects this edge.
    Value(ValueId),

    /// A switch selector is compared against a case value.
    SwitchCase {
        /// Selector value.
        selector: ValueId,

        /// Case discriminator.
        case: SwitchCaseValue,
    },

    /// Extension-defined condition.
    Extension {
        /// Stable extension namespace/name.
        name: String,

        /// Extension payload.
        payload: Vec<u8>,
    },
}

impl SuccessorCondition {
    /// Creates a boolean/value condition.
    pub const fn value(value: ValueId) -> Self {
        Self::Value(value)
    }

    /// Creates a switch-case condition.
    pub fn switch_case(
        selector: ValueId,
        case: SwitchCaseValue,
    ) -> Self {
        Self::SwitchCase { selector, case }
    }

    /// Returns the referenced value when one exists.
    #[must_use]
    pub const fn value_id(&self) -> Option<ValueId> {
        match self {
            Self::Value(value) => Some(*value),
            Self::SwitchCase { selector, .. } => Some(*selector),
            Self::Extension { .. } => None,
        }
    }

    /// Returns whether the condition is a switch case.
    #[must_use]
    pub const fn is_switch_case(&self) -> bool {
        matches!(self, Self::SwitchCase { .. })
    }
}

// =============================================================================
// Successor metadata
// =============================================================================

/// Optional semantic metadata associated with a successor.
///
/// This metadata remains target-independent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuccessorMetadata {
    /// No additional metadata.
    None,

    /// Conditional edge.
    Condition(SuccessorCondition),

    /// Switch case edge.
    SwitchCase {
        /// Selector.
        selector: ValueId,

        /// Case discriminator.
        case: SwitchCaseValue,
    },

    /// Extension-defined metadata.
    Extension {
        /// Stable extension name.
        name: String,

        /// Opaque extension payload.
        payload: Vec<u8>,
    },
}

impl Default for SuccessorMetadata {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// Successor
// =============================================================================

/// Canonical structured control-flow successor.
///
/// A successor represents one directed edge:
///
/// ```text
/// source block ─────────────► target block
///                    │
///                    ├── values
///                    ├── condition
///                    └── metadata
/// ```
///
/// The source block is intentionally not stored here.
///
/// The owner (`Block`) already provides the source context. Storing the source
/// again would duplicate graph ownership and create unnecessary consistency
/// requirements.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Successor {
    target: BlockId,
    kind: SuccessorKind,
    arguments: Vec<ValueId>,
    metadata: SuccessorMetadata,
    logical_qubits: Vec<QubitId>,
    physical_qubits: Vec<PhysicalQubitId>,
}

impl Successor {
    /// Creates an unconditional successor with no transferred values.
    pub fn new(target: BlockId) -> SuccessorResult<Self> {
        Self::with_kind(target, SuccessorKind::Unconditional)
    }

    /// Creates a successor with an explicit semantic kind.
    pub fn with_kind(
        target: BlockId,
        kind: SuccessorKind,
    ) -> SuccessorResult<Self> {
        Self::validate_target(target)?;

        Ok(Self {
            target,
            kind,
            arguments: Vec::new(),
            metadata: SuccessorMetadata::None,
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
        })
    }

    /// Creates a conditional-true successor.
    pub fn conditional_true(
        target: BlockId,
        condition: ValueId,
    ) -> SuccessorResult<Self> {
        Self::conditional(
            target,
            SuccessorKind::ConditionalTrue,
            condition,
        )
    }

    /// Creates a conditional-false successor.
    pub fn conditional_false(
        target: BlockId,
        condition: ValueId,
    ) -> SuccessorResult<Self> {
        Self::conditional(
            target,
            SuccessorKind::ConditionalFalse,
            condition,
        )
    }

    /// Creates a conditional successor.
    pub fn conditional(
        target: BlockId,
        kind: SuccessorKind,
        condition: ValueId,
    ) -> SuccessorResult<Self> {
        if !matches!(
            kind,
            SuccessorKind::ConditionalTrue
                | SuccessorKind::ConditionalFalse
        ) {
            return Err(SuccessorError::InvalidConditionalMetadata);
        }

        Self::validate_target(target)?;
        Self::validate_value(condition)?;

        let mut successor =
            Self::with_kind(target, kind)?;

        successor.metadata =
            SuccessorMetadata::Condition(
                SuccessorCondition::Value(condition)
            );

        Ok(successor)
    }

    /// Creates a switch-case successor.
    pub fn switch_case(
        target: BlockId,
        selector: ValueId,
        case: SwitchCaseValue,
    ) -> SuccessorResult<Self> {
        Self::validate_target(target)?;
        Self::validate_value(selector)?;

        let mut successor =
            Self::with_kind(
                target,
                SuccessorKind::SwitchCase,
            )?;

        successor.metadata =
            SuccessorMetadata::SwitchCase {
                selector,
                case,
            };

        Ok(successor)
    }

    /// Creates the default edge of a switch.
    pub fn switch_default(
        target: BlockId,
        selector: ValueId,
    ) -> SuccessorResult<Self> {
        Self::validate_target(target)?;
        Self::validate_value(selector)?;

        let mut successor =
            Self::with_kind(
                target,
                SuccessorKind::SwitchDefault,
            )?;

        successor.metadata =
            SuccessorMetadata::Condition(
                SuccessorCondition::Value(selector)
            );

        Ok(successor)
    }

    /// Returns the destination block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Changes the destination block.
    ///
    /// The operation is validated before mutation.
    pub fn set_target(
        &mut self,
        target: BlockId,
    ) -> SuccessorResult<()> {
        Self::validate_target(target)?;
        self.target = target;
        Ok(())
    }

    /// Returns the semantic edge kind.
    #[must_use]
    pub const fn kind(&self) -> SuccessorKind {
        self.kind
    }

    /// Changes the semantic edge kind.
    ///
    /// Existing metadata is validated against the new kind.
    pub fn set_kind(
        &mut self,
        kind: SuccessorKind,
    ) -> SuccessorResult<()> {
        Self::validate_kind_metadata(
            kind,
            &self.metadata,
        )?;

        self.kind = kind;
        Ok(())
    }

    /// Returns the values transferred to the destination block.
    ///
    /// Ordering is semantic and must be preserved.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns the number of transferred values.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Returns whether no values are transferred.
    #[must_use]
    pub fn has_no_arguments(&self) -> bool {
        self.arguments.is_empty()
    }

    /// Adds one transferred value.
    pub fn add_argument(
        &mut self,
        value: ValueId,
    ) -> SuccessorResult<()> {
        Self::validate_value(value)?;

        if self.arguments.contains(&value) {
            return Err(SuccessorError::DuplicateValue {
                value,
            });
        }

        self.arguments
            .try_reserve(1)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "arguments",
            })?;

        self.arguments.push(value);
        Ok(())
    }

    /// Adds multiple transferred values atomically.
    ///
    /// No mutation occurs if the supplied sequence contains an invalid or
    /// duplicate value.
    pub fn add_arguments<I>(
        &mut self,
        values: I,
    ) -> SuccessorResult<()>
    where
        I: IntoIterator<Item = ValueId>,
    {
        let incoming: Vec<ValueId> =
            values.into_iter().collect();

        let mut seen =
            BTreeSet::<ValueId>::new();

        for value in &incoming {
            Self::validate_value(*value)?;

            if !seen.insert(*value)
                || self.arguments.contains(value)
            {
                return Err(
                    SuccessorError::DuplicateValue {
                        value: *value,
                    },
                );
            }
        }

        self.arguments
            .try_reserve(incoming.len())
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "arguments",
            })?;

        self.arguments.extend(incoming);
        Ok(())
    }

    /// Removes one transferred value.
    pub fn remove_argument(
        &mut self,
        value: ValueId,
    ) -> bool {
        if let Some(position) =
            self.arguments.iter().position(
                |candidate| *candidate == value
            )
        {
            self.arguments.remove(position);
            true
        } else {
            false
        }
    }

    /// Clears all transferred values.
    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    /// Returns optional edge metadata.
    #[must_use]
    pub const fn metadata(&self) -> &SuccessorMetadata {
        &self.metadata
    }

    /// Replaces edge metadata after validating it against the edge kind.
    pub fn set_metadata(
        &mut self,
        metadata: SuccessorMetadata,
    ) -> SuccessorResult<()> {
        Self::validate_kind_metadata(
            self.kind,
            &metadata,
        )?;

        self.metadata = metadata;
        Ok(())
    }

    /// Removes all edge metadata.
    ///
    /// This is only valid for an unconditional edge.
    pub fn clear_metadata(
        &mut self,
    ) -> SuccessorResult<()> {
        self.set_metadata(
            SuccessorMetadata::None
        )
    }

    /// Returns the condition value if the edge has one.
    #[must_use]
    pub fn condition(&self) -> Option<ValueId> {
        match &self.metadata {
            SuccessorMetadata::Condition(condition) => {
                condition.value_id()
            }

            SuccessorMetadata::SwitchCase {
                selector,
                ..
            } => Some(*selector),

            SuccessorMetadata::Extension { .. }
            | SuccessorMetadata::None => None,
        }
    }

    /// Returns the switch case when this is a switch-case edge.
    #[must_use]
    pub fn switch_case_value(
        &self,
    ) -> Option<&SwitchCaseValue> {
        match &self.metadata {
            SuccessorMetadata::SwitchCase {
                case,
                ..
            } => Some(case),

            _ => None,
        }
    }

    /// Adds a logical qubit reference to this edge.
    ///
    /// The canonical logical qubit type comes from
    /// `quantum::ir::qubit`.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> SuccessorResult<()> {
        if qubit.value() == 0 {
            return Err(
                SuccessorError::InvalidLogicalQubit {
                    qubit,
                },
            );
        }

        if self.logical_qubits.contains(&qubit) {
            return Err(
                SuccessorError::DuplicateLogicalQubit {
                    qubit,
                },
            );
        }

        self.logical_qubits
            .try_reserve(1)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "logical_qubits",
            })?;

        self.logical_qubits.push(qubit);
        Ok(())
    }

    /// Adds a physical qubit reference to this edge.
    ///
    /// Physical identities remain distinct from logical identities.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> SuccessorResult<()> {
        if qubit.value() == 0 {
            return Err(
                SuccessorError::InvalidPhysicalQubit {
                    qubit,
                },
            );
        }

        if self.physical_qubits.contains(&qubit) {
            return Err(
                SuccessorError::DuplicatePhysicalQubit {
                    qubit,
                },
            );
        }

        self.physical_qubits
            .try_reserve(1)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "physical_qubits",
            })?;

        self.physical_qubits.push(qubit);
        Ok(())
    }

    /// Returns logical qubit references carried by this edge.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubit references carried by this edge.
    #[must_use]
    pub fn physical_qubits(
        &self,
    ) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns whether this edge is unconditional.
    #[must_use]
    pub const fn is_unconditional(&self) -> bool {
        matches!(
            self.kind,
            SuccessorKind::Unconditional
        )
    }

    /// Returns whether this edge is conditional.
    #[must_use]
    pub const fn is_conditional(&self) -> bool {
        self.kind.is_conditional()
    }

    /// Returns whether this edge is a loop edge.
    #[must_use]
    pub const fn is_loop_edge(&self) -> bool {
        self.kind.is_loop_edge()
    }

    /// Returns whether this edge is a return edge.
    #[must_use]
    pub const fn is_return(&self) -> bool {
        self.kind.is_return()
    }

    /// Returns whether this edge is an error/recovery edge.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.kind.is_error()
    }

    /// Validates all local invariants.
    ///
    /// This does not require access to the containing program.
    pub fn validate(
        &self,
    ) -> SuccessorResult<()> {
        Self::validate_target(self.target)?;

        let mut values =
            BTreeSet::<ValueId>::new();

        for value in &self.arguments {
            Self::validate_value(*value)?;

            if !values.insert(*value) {
                return Err(
                    SuccessorError::DuplicateValue {
                        value: *value,
                    },
                );
            }
        }

        let mut logical =
            BTreeSet::<QubitId>::new();

        for qubit in &self.logical_qubits {
            if qubit.value() == 0 {
                return Err(
                    SuccessorError::InvalidLogicalQubit {
                        qubit: *qubit,
                    },
                );
            }

            if !logical.insert(*qubit) {
                return Err(
                    SuccessorError::DuplicateLogicalQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        let mut physical =
            BTreeSet::<PhysicalQubitId>::new();

        for qubit in &self.physical_qubits {
            if qubit.value() == 0 {
                return Err(
                    SuccessorError::InvalidPhysicalQubit {
                        qubit: *qubit,
                    },
                );
            }

            if !physical.insert(*qubit) {
                return Err(
                    SuccessorError::DuplicatePhysicalQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        Self::validate_kind_metadata(
            self.kind,
            &self.metadata,
        )?;

        Ok(())
    }

    /// Returns a deterministic structural fingerprint tuple.
    ///
    /// This is not a cryptographic hash. The canonical hashing subsystem owns
    /// cryptographic content identity.
    #[must_use]
    pub fn structural_components(
        &self,
    ) -> (
        BlockId,
        SuccessorKind,
        &[ValueId],
        &SuccessorMetadata,
        &[QubitId],
        &[PhysicalQubitId],
    ) {
        (
            self.target,
            self.kind,
            &self.arguments,
            &self.metadata,
            &self.logical_qubits,
            &self.physical_qubits,
        )
    }

    // -------------------------------------------------------------------------
    // Internal validation
    // -------------------------------------------------------------------------

    fn validate_target(
        target: BlockId,
    ) -> SuccessorResult<()> {
        if target.value() == 0 {
            return Err(
                SuccessorError::InvalidTarget {
                    target,
                },
            );
        }

        Ok(())
    }

    fn validate_value(
        value: ValueId,
    ) -> SuccessorResult<()> {
        if value.value() == 0 {
            return Err(
                SuccessorError::InvalidValue {
                    value,
                },
            );
        }

        Ok(())
    }

    fn validate_kind_metadata(
        kind: SuccessorKind,
        metadata: &SuccessorMetadata,
    ) -> SuccessorResult<()> {
        match kind {
            SuccessorKind::Unconditional => {
                if !matches!(
                    metadata,
                    SuccessorMetadata::None
                ) {
                    return Err(
                        SuccessorError::InvalidUnconditionalMetadata,
                    );
                }
            }

            SuccessorKind::ConditionalTrue
            | SuccessorKind::ConditionalFalse => {
                match metadata {
                    SuccessorMetadata::Condition(
                        SuccessorCondition::Value(value),
                    ) => {
                        Self::validate_value(*value)?;
                    }

                    SuccessorMetadata::Extension { .. } => {}

                    _ => {
                        return Err(
                            SuccessorError::InvalidConditionalMetadata,
                        );
                    }
                }
            }

            SuccessorKind::SwitchCase => {
                match metadata {
                    SuccessorMetadata::SwitchCase {
                        selector,
                        case,
                    } => {
                        Self::validate_value(*selector)?;

                        if case.bit_width() == 0 {
                            return Err(
                                SuccessorError::InvalidCaseValue,
                            );
                        }
                    }

                    SuccessorMetadata::Extension { .. } => {}

                    _ => {
                        return Err(
                            SuccessorError::InvalidSwitchMetadata,
                        );
                    }
                }
            }

            SuccessorKind::SwitchDefault => {
                match metadata {
                    SuccessorMetadata::Condition(
                        SuccessorCondition::Value(value),
                    ) => {
                        Self::validate_value(*value)?;
                    }

                    SuccessorMetadata::Extension { .. } => {}

                    _ => {
                        return Err(
                            SuccessorError::InvalidSwitchMetadata,
                        );
                    }
                }
            }

            SuccessorKind::LoopBack
            | SuccessorKind::LoopExit
            | SuccessorKind::LoopContinue
            | SuccessorKind::Return
            | SuccessorKind::Error
            | SuccessorKind::Extension => {
                match metadata {
                    SuccessorMetadata::None
                    | SuccessorMetadata::Extension { .. } => {}

                    _ => {
                        return Err(
                            SuccessorError::InvalidSuccessor {
                                message:
                                    "metadata is incompatible with this successor kind",
                            },
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for Successor {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} -> {}",
            self.kind,
            self.target
        )
    }
}

// =============================================================================
// Successor collection
// =============================================================================

/// Deterministic collection of outgoing successors.
///
/// The collection preserves insertion order because control-flow edge order
/// can be meaningful to serialization, diagnostics, and deterministic
/// compiler behavior.
///
/// Duplicate destination blocks are rejected by default because a block should
/// normally contain at most one semantic edge to the same destination. If a
/// future dialect needs multiple semantically distinct edges to the same
/// destination, they must be distinguished by their `SuccessorKind` and/or
/// metadata at a higher-level control-flow contract.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SuccessorList {
    successors: Vec<Successor>,
}

impl Default for SuccessorList {
    fn default() -> Self {
        Self::new()
    }
}

impl SuccessorList {
    /// Creates an empty successor collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            successors: Vec::new(),
        }
    }

    /// Creates a collection with fallible capacity reservation.
    pub fn with_capacity(
        capacity: usize,
    ) -> SuccessorResult<Self> {
        let mut successors =
            Vec::new();

        successors
            .try_reserve(capacity)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "successors",
            })?;

        Ok(Self { successors })
    }

    /// Returns the number of successors.
    #[must_use]
    pub fn len(&self) -> usize {
        self.successors.len()
    }

    /// Returns whether there are no successors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.successors.is_empty()
    }

    /// Returns all successors in semantic order.
    #[must_use]
    pub fn as_slice(&self) -> &[Successor] {
        &self.successors
    }

    /// Returns the successor at an index.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&Successor> {
        self.successors.get(index)
    }

    /// Returns a mutable successor at an index.
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut Successor> {
        self.successors.get_mut(index)
    }

    /// Adds a successor after validating it.
    ///
    /// Duplicate destination blocks are rejected.
    pub fn push(
        &mut self,
        successor: Successor,
    ) -> SuccessorResult<()> {
        successor.validate()?;

        if self
            .successors
            .iter()
            .any(|existing| {
                existing.target() == successor.target()
            })
        {
            return Err(
                SuccessorError::InvalidSuccessor {
                    message:
                        "duplicate successor target in successor list",
                },
            );
        }

        self.successors
            .try_reserve(1)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "successors",
            })?;

        self.successors.push(successor);
        Ok(())
    }

    /// Inserts a successor at a deterministic position.
    pub fn insert(
        &mut self,
        index: usize,
        successor: Successor,
    ) -> SuccessorResult<()> {
        successor.validate()?;

        if index > self.successors.len() {
            return Err(
                SuccessorError::InvalidSuccessor {
                    message:
                        "successor insertion index is outside the collection",
                },
            );
        }

        if self
            .successors
            .iter()
            .any(|existing| {
                existing.target() == successor.target()
            })
        {
            return Err(
                SuccessorError::InvalidSuccessor {
                    message:
                        "duplicate successor target in successor list",
                },
            );
        }

        self.successors
            .try_reserve(1)
            .map_err(|_| SuccessorError::AllocationFailure {
                collection: "successors",
            })?;

        self.successors.insert(index, successor);
        Ok(())
    }

    /// Removes the successor at an index.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Option<Successor> {
        if index < self.successors.len() {
            Some(self.successors.remove(index))
        } else {
            None
        }
    }

    /// Removes the successor targeting a block.
    pub fn remove_target(
        &mut self,
        target: BlockId,
    ) -> Option<Successor> {
        let position = self
            .successors
            .iter()
            .position(|successor| {
                successor.target() == target
            })?;

        Some(self.successors.remove(position))
    }

    /// Returns whether a destination block is already present.
    #[must_use]
    pub fn contains_target(
        &self,
        target: BlockId,
    ) -> bool {
        self.successors
            .iter()
            .any(|successor| {
                successor.target() == target
            })
    }

    /// Returns an iterator over successors.
    pub fn iter(
        &self,
    ) -> std::slice::Iter<'_, Successor> {
        self.successors.iter()
    }

    /// Returns a mutable iterator over successors.
    pub fn iter_mut(
        &mut self,
    ) -> std::slice::IterMut<'_, Successor> {
        self.successors.iter_mut()
    }

    /// Clears all successors.
    pub fn clear(&mut self) {
        self.successors.clear();
    }

    /// Validates every successor and the local destination uniqueness rule.
    pub fn validate(
        &self,
    ) -> SuccessorResult<()> {
        let mut targets =
            BTreeSet::<BlockId>::new();

        for successor in &self.successors {
            successor.validate()?;

            if !targets.insert(successor.target()) {
                return Err(
                    SuccessorError::InvalidSuccessor {
                        message:
                            "duplicate successor destination",
                    },
                );
            }
        }

        Ok(())
    }
}

impl<'a> IntoIterator for &'a SuccessorList {
    type Item = &'a Successor;
    type IntoIter =
        std::slice::Iter<'a, Successor>;

    fn into_iter(self) -> Self::IntoIter {
        self.successors.iter()
    }
}

impl<'a> IntoIterator for &'a mut SuccessorList {
    type Item = &'a mut Successor;
    type IntoIter =
        std::slice::IterMut<'a, Successor>;

    fn into_iter(self) -> Self::IntoIter {
        self.successors.iter_mut()
    }
}

impl IntoIterator for SuccessorList {
    type Item = Successor;
    type IntoIter =
        std::vec::IntoIter<Successor>;

    fn into_iter(self) -> Self::IntoIter {
        self.successors.into_iter()
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn block(value: u64) -> BlockId {
        BlockId::new(value)
    }

    fn value(value: u64) -> ValueId {
        ValueId::new(value)
    }

    fn logical_qubit(value: u64) -> QubitId {
        QubitId::new(value)
    }

    fn physical_qubit(value: u64) -> PhysicalQubitId {
        PhysicalQubitId::new(value)
    }

    #[test]
    fn creates_unconditional_successor() {
        let successor =
            Successor::new(block(1))
                .expect("valid successor");

        assert_eq!(
            successor.target(),
            block(1)
        );

        assert_eq!(
            successor.kind(),
            SuccessorKind::Unconditional
        );

        assert!(successor.arguments().is_empty());
        assert!(successor.metadata() == &SuccessorMetadata::None);
    }

    #[test]
    fn rejects_zero_target() {
        let result =
            Successor::new(block(0));

        assert!(matches!(
            result,
            Err(SuccessorError::InvalidTarget { .. })
        ));
    }

    #[test]
    fn arguments_are_ordered_and_unique() {
        let mut successor =
            Successor::new(block(1))
                .expect("valid successor");

        successor
            .add_argument(value(10))
            .expect("first argument");

        successor
            .add_argument(value(20))
            .expect("second argument");

        assert_eq!(
            successor.arguments(),
            &[value(10), value(20)]
        );

        let duplicate =
            successor.add_argument(value(10));

        assert!(matches!(
            duplicate,
            Err(SuccessorError::DuplicateValue { .. })
        ));
    }

    #[test]
    fn conditional_true_requires_condition() {
        let successor =
            Successor::conditional_true(
                block(2),
                value(7),
            )
            .expect("valid conditional successor");

        assert_eq!(
            successor.kind(),
            SuccessorKind::ConditionalTrue
        );

        assert_eq!(
            successor.condition(),
            Some(value(7))
        );

        assert!(successor.is_conditional());
    }

    #[test]
    fn conditional_false_requires_condition() {
        let successor =
            Successor::conditional_false(
                block(3),
                value(8),
            )
            .expect("valid conditional successor");

        assert_eq!(
            successor.kind(),
            SuccessorKind::ConditionalFalse
        );

        assert_eq!(
            successor.condition(),
            Some(value(8))
        );
    }

    #[test]
    fn switch_case_is_target_independent() {
        let case =
            SwitchCaseValue::from_u64(42);

        let successor =
            Successor::switch_case(
                block(4),
                value(9),
                case.clone(),
            )
            .expect("valid switch case");

        assert_eq!(
            successor.kind(),
            SuccessorKind::SwitchCase
        );

        assert_eq!(
            successor.switch_case_value(),
            Some(&case)
        );
    }

    #[test]
    fn arbitrary_width_switch_values_are_supported() {
        let bytes = vec![
            0x01, 0x02, 0x03, 0x04,
            0x05, 0x06, 0x07, 0x08,
            0x09, 0x0A, 0x0B, 0x0C,
        ];

        let case =
            SwitchCaseValue::new(
                96,
                bytes.clone(),
            )
            .expect("96-bit case");

        assert_eq!(
            case.bit_width(),
            96
        );

        assert_eq!(
            case.bytes(),
            bytes.as_slice()
        );
    }

    #[test]
    fn rejects_wrong_switch_width() {
        let result =
            SwitchCaseValue::new(
                16,
                vec![0x01],
            );

        assert!(matches!(
            result,
            Err(SuccessorError::InvalidCaseValue)
        ));
    }

    #[test]
    fn logical_qubits_use_canonical_ir_qubit_type() {
        let mut successor =
            Successor::new(block(5))
                .expect("valid successor");

        successor
            .add_logical_qubit(
                logical_qubit(1)
            )
            .expect("valid logical qubit");

        assert_eq!(
            successor.logical_qubits(),
            &[logical_qubit(1)]
        );
    }

    #[test]
    fn physical_and_logical_qubits_are_distinct() {
        let mut successor =
            Successor::new(block(6))
                .expect("valid successor");

        successor
            .add_logical_qubit(
                logical_qubit(1)
            )
            .expect("logical qubit");

        successor
            .add_physical_qubit(
                physical_qubit(1)
            )
            .expect("physical qubit");

        assert_eq!(
            successor.logical_qubits(),
            &[logical_qubit(1)]
        );

        assert_eq!(
            successor.physical_qubits(),
            &[physical_qubit(1)]
        );
    }

    #[test]
    fn successor_list_preserves_order() {
        let mut list =
            SuccessorList::new();

        list.push(
            Successor::new(block(1))
                .expect("successor"),
        )
        .expect("insert");

        list.push(
            Successor::new(block(2))
                .expect("successor"),
        )
        .expect("insert");

        assert_eq!(
            list.as_slice()[0].target(),
            block(1)
        );

        assert_eq!(
            list.as_slice()[1].target(),
            block(2)
        );
    }

    #[test]
    fn successor_list_rejects_duplicate_destination() {
        let mut list =
            SuccessorList::new();

        list.push(
            Successor::new(block(1))
                .expect("successor"),
        )
        .expect("first insert");

        let result =
            list.push(
                Successor::conditional_true(
                    block(1),
                    value(10),
                )
                .expect("conditional successor"),
            );

        assert!(matches!(
            result,
            Err(SuccessorError::InvalidSuccessor { .. })
        ));
    }

    #[test]
    fn validation_is_deterministic() {
        let mut successor =
            Successor::new(block(7))
                .expect("successor");

        successor
            .add_argument(value(1))
            .expect("argument");

        successor
            .add_logical_qubit(
                logical_qubit(1)
            )
            .expect("logical qubit");

        assert!(
            successor.validate().is_ok()
        );

        assert_eq!(
            successor.clone(),
            successor
        );
    }

    #[test]
    fn loop_edges_are_supported() {
        let successor =
            Successor::with_kind(
                block(8),
                SuccessorKind::LoopBack,
            )
            .expect("loop edge");

        assert!(successor.is_loop_edge());
        assert!(!successor.is_conditional());
    }

    #[test]
    fn return_edges_are_supported() {
        let successor =
            Successor::with_kind(
                block(9),
                SuccessorKind::Return,
            )
            .expect("return edge");

        assert!(successor.is_return());
    }

    #[test]
    fn error_edges_are_supported() {
        let successor =
            Successor::with_kind(
                block(10),
                SuccessorKind::Error,
            )
            .expect("error edge");

        assert!(successor.is_error());
    }
}