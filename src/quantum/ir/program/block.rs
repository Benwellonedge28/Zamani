//! Zamani Quantum IR — Canonical Program Block
//!
//! Production-grade, hardware-independent representation of a structured
//! program block.
//!
//! # Architectural role
//!
//! `program::block` owns the structural representation of one IR block.
//!
//! A block is an ordered sequence of operation identities together with:
//!
//! - a stable `BlockId`;
//! - semantic block kind;
//! - block arguments;
//! - operation ordering;
//! - control-flow successor references;
//! - optional parent region identity;
//! - qubit-scope information;
//! - completion/termination state;
//! - explicit structural metadata required by downstream IR consumers.
//!
//! The block does NOT own concrete `Operation` objects.
//!
//! Concrete operations belong to the canonical operation/program storage
//! layer and are referenced here by `quantum::ir::identity::OperationId`.
//!
//! # Core architectural invariant
//!
//! ```text
//! Block
//!   │
//!   ├── BlockId
//!   ├── arguments
//!   ├── OperationId
//!   ├── OperationId
//!   ├── OperationId
//!   └── terminator
//! ```
//!
//! This separation prevents the block from becoming coupled to the concrete
//! operation representation.
//!
//! # Universal-program principle
//!
//! A Zamani program is written once at the semantic level and may be lowered
//! to different quantum machines and architectures.
//!
//! Therefore this module contains:
//!
//! - no maximum qubit count;
//! - no maximum block count;
//! - no maximum operation count;
//! - no fixed hardware topology;
//! - no vendor-specific operation;
//! - no physical-device allocation;
//! - no scheduling algorithm;
//! - no routing algorithm;
//! - no backend implementation.
//!
//! A block containing one operation and a block containing an extremely large
//! finite number of operations use exactly the same representation.
//!
//! Concrete allocation limits are policy decisions owned by the appropriate
//! IR/compiler limits layer.
//!
//! # Logical and physical qubits
//!
//! The canonical qubit identities are owned by:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! quantum::ir::qubit::QubitRef
//! ```
//!
//! This file therefore imports them from `super::qubit`.
//!
//! It MUST NOT use:
//!
//! ```text
//! super::qubits
//! ```
//!
//! The repository's canonical module is `quantum::ir::qubit`.
//!
//! # Operation references
//!
//! A block stores `OperationId` values rather than concrete `Operation`
//! instances.
//!
//! This gives the IR the dependency direction:
//!
//! ```text
//! identity.rs
//!      │
//!      ▼
//! block.rs
//!      │
//!      ▼
//! program / operation registry
//! ```
//!
//! rather than creating recursive ownership between blocks and operations.
//!
//! # Control flow
//!
//! A block may have zero or more successor references while it is being built,
//! but a finalized executable/control-flow graph must have an explicit
//! terminator contract.
//!
//! The block therefore distinguishes:
//!
//! ```text
//! Open
//! Return
//! Branch
//! ConditionalBranch
//! Switch
//! Unreachable
//! Extension
//! ```
//!
//! No scheduling or execution decision is made here.
//!
//! # Determinism
//!
//! Semantic ordering is always represented by `Vec`.
//!
//! We deliberately do not use `HashMap` for ordered operation storage.
//!
//! The same sequence of mutations produces the same block ordering.
//!
//! # Mutation safety
//!
//! Mutating operations are validated before state is changed whenever
//! practical.
//!
//! Collection growth uses `try_reserve` where a fallible API is appropriate,
//! allowing callers handling untrusted or very large IR to distinguish
//! allocation failure from ordinary semantic failure.
//!
//! No `unsafe` is used.
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
//!     Supplies `BlockId`, `OperationId`, `RegionId`, and `ValueId`.
//!
//! `qubit.rs`
//!     Supplies canonical logical/physical qubit identities.
//!
//! `operation.rs`
//!     Owns concrete semantic operations. Blocks reference them by
//!     `OperationId`.
//!
//! `region.rs`
//!     Owns region-level structure. It must use this `Block` type rather than
//!     define a second block implementation.
//!
//! `program.rs`
//!     Owns the program-level registry/container and resolves operation and
//!     region identities.
//!
//! `validation.rs`
//!     Performs whole-program validation beyond the local invariants enforced
//!     here.
//!
//! `control_flow.rs`
//!     May consume block terminators/successors when constructing or analyzing
//!     dynamic control flow.
//!
//! `analysis.rs`
//!     May inspect operation order, arguments and successors without mutating
//!     the block.
//!
//! `optimization/`
//!     May transform operation references while preserving block invariants.
//!
//! `routing/`
//!     May consume qubit references but must not redefine block identity.
//!
//! `scheduling/`
//!     May derive execution order/timing but must not redefine semantic block
//!     ordering.
//!
//! `serialization.rs`
//!     Owns canonical persistence; this file only guarantees deterministic
//!     structural access.
//!
//! `hash.rs`
//!     Owns canonical content hashing; this file does not calculate hashes.
//!
//! # Important ownership rule
//!
//! This file owns BLOCK structure.
//!
//! It does not own:
//!
//! - program-wide operation registries;
//! - region registries;
//! - hardware;
//! - topology;
//! - routing;
//! - scheduling;
//! - calibration;
//! - pulse synthesis;
//! - simulation;
//! - QEC decoding;
//! - frontend parsing;
//! - optimization policy.
//!
//! # Production-readiness invariant
//!
//! A block is valid when:
//!
//! 1. its identity is explicit;
//! 2. every operation identity is explicitly recorded;
//! 3. operation ordering is deterministic;
//! 4. operation identities are unique within the block;
//! 5. block arguments have unique `ValueId`s;
//! 6. successor references are unique;
//! 7. successor arguments are structurally valid;
//! 8. a closed block has a non-open terminator;
//! 9. no fixed machine-size assumption is embedded;
//! 10. logical and physical qubit namespaces remain distinct;
//! 11. no hardware-specific behavior is encoded;
//! 12. all mutations preserve local invariants.
//!
//! Whole-program existence checks for operation IDs, region IDs and value IDs
//! are intentionally deferred to the program/validation layer.
//!
//! This prevents `block.rs` from becoming coupled to a particular storage
//! implementation.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::collections::BTreeSet;
use std::fmt;

use super::super::identity::{
    BlockId,
    OperationId,
    RegionId,
    ValueId,
};
use super::super::qubit::{
    PhysicalQubitId,
    QubitId,
    QubitRef,
};

// =============================================================================
// Result
// =============================================================================

/// Result type used by canonical block construction and mutation.
pub type BlockResult<T> = Result<T, BlockError>;

// =============================================================================
// Block error
// =============================================================================

/// Errors produced by local block construction and mutation.
///
/// These are intentionally local structural errors.
///
/// Whole-program namespace errors belong to `validation.rs` and the program
/// owner because only those layers can determine whether an identity actually
/// exists in the surrounding IR graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    /// The supplied block identifier is invalid.
    InvalidBlockId,

    /// The supplied operation identifier is invalid.
    InvalidOperationId {
        /// Invalid identity.
        operation: OperationId,
    },

    /// The same operation identity was inserted more than once.
    DuplicateOperation {
        /// Duplicated operation.
        operation: OperationId,
    },

    /// The supplied value identifier is invalid.
    InvalidValueId {
        /// Invalid value identity.
        value: ValueId,
    },

    /// Two block arguments use the same value identity.
    DuplicateArgument {
        /// Duplicated argument identity.
        value: ValueId,
    },

    /// A block argument has no meaningful binding.
    InvalidArgumentBinding,

    /// The same successor was inserted more than once.
    DuplicateSuccessor {
        /// Duplicated successor.
        block: BlockId,
    },

    /// A successor target is invalid.
    InvalidSuccessorTarget {
        /// Invalid target.
        block: BlockId,
    },

    /// A closed block cannot be modified without explicitly reopening it.
    BlockAlreadyTerminated,

    /// A terminator requiring a successor was supplied without one.
    MissingSuccessor,

    /// A conditional branch requires a condition value.
    MissingCondition,

    /// A switch requires a selector value.
    MissingSelector,

    /// A return terminator cannot contain duplicate result values.
    DuplicateReturnValue {
        /// Duplicated result.
        value: ValueId,
    },

    /// A successor argument list contains duplicates.
    DuplicateSuccessorArgument {
        /// Duplicated argument.
        value: ValueId,
    },

    /// A logical qubit appears more than once in the block scope.
    DuplicateLogicalQubit {
        /// Duplicated qubit.
        qubit: QubitId,
    },

    /// A physical qubit appears more than once in the block scope.
    DuplicatePhysicalQubit {
        /// Duplicated qubit.
        qubit: PhysicalQubitId,
    },

    /// A collection could not reserve requested capacity.
    AllocationFailure {
        /// Semantic collection being grown.
        collection: &'static str,
    },

    /// A checked arithmetic calculation overflowed.
    ArithmeticOverflow {
        /// Description of the calculation.
        calculation: &'static str,
    },

    /// An operation was requested at an invalid position.
    InvalidOperationPosition {
        /// Requested position.
        position: usize,
    },

    /// The block is structurally invalid.
    InvalidBlock {
        /// Static reason.
        message: &'static str,
    },
}

impl fmt::Display for BlockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBlockId => {
                formatter.write_str("invalid block identifier")
            }

            Self::InvalidOperationId { operation } => {
                write!(formatter, "invalid operation identifier {operation}")
            }

            Self::DuplicateOperation { operation } => {
                write!(formatter, "operation {operation} already exists in block")
            }

            Self::InvalidValueId { value } => {
                write!(formatter, "invalid value identifier {value}")
            }

            Self::DuplicateArgument { value } => {
                write!(formatter, "block argument {value} already exists")
            }

            Self::InvalidArgumentBinding => {
                formatter.write_str("block argument has an invalid binding")
            }

            Self::DuplicateSuccessor { block } => {
                write!(formatter, "successor block {block} already exists")
            }

            Self::InvalidSuccessorTarget { block } => {
                write!(formatter, "invalid successor block {block}")
            }

            Self::BlockAlreadyTerminated => {
                formatter.write_str("block is already terminated")
            }

            Self::MissingSuccessor => {
                formatter.write_str("terminator requires at least one successor")
            }

            Self::MissingCondition => {
                formatter.write_str("conditional branch requires a condition value")
            }

            Self::MissingSelector => {
                formatter.write_str("switch terminator requires a selector value")
            }

            Self::DuplicateReturnValue { value } => {
                write!(formatter, "return value {value} appears more than once")
            }

            Self::DuplicateSuccessorArgument { value } => {
                write!(
                    formatter,
                    "successor argument {value} appears more than once"
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is already declared in this block"
                )
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "physical qubit {qubit} is already declared in this block"
                )
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for block {collection}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidOperationPosition { position } => {
                write!(
                    formatter,
                    "invalid operation insertion position {position}"
                )
            }

            Self::InvalidBlock { message } => {
                write!(formatter, "invalid block: {message}")
            }
        }
    }
}

impl std::error::Error for BlockError {}

// =============================================================================
// Block kind
// =============================================================================

/// Semantic role of a block.
///
/// This classification does not prescribe hardware execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlockKind {
    /// Ordinary sequential block.
    Basic,

    /// Entry block.
    Entry,

    /// Exit block.
    Exit,

    /// Branching block.
    Branch,

    /// Loop header.
    LoopHeader,

    /// Loop body.
    LoopBody,

    /// Loop continuation.
    LoopContinue,

    /// Loop exit.
    LoopExit,

    /// Function entry.
    FunctionEntry,

    /// Function exit.
    FunctionExit,

    /// Error/recovery block.
    Error,

    /// Extension-defined block.
    Extension,
}

impl Default for BlockKind {
    fn default() -> Self {
        Self::Basic
    }
}

impl fmt::Display for BlockKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Basic => "basic",
            Self::Entry => "entry",
            Self::Exit => "exit",
            Self::Branch => "branch",
            Self::LoopHeader => "loop_header",
            Self::LoopBody => "loop_body",
            Self::LoopContinue => "loop_continue",
            Self::LoopExit => "loop_exit",
            Self::FunctionEntry => "function_entry",
            Self::FunctionExit => "function_exit",
            Self::Error => "error",
            Self::Extension => "extension",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Block argument binding
// =============================================================================

/// Semantic namespace represented by a block argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlockArgumentBinding {
    /// Logical quantum resource.
    LogicalQubit(QubitId),

    /// Physical quantum resource.
///
/// Physical references are allowed only for IR stages that explicitly
/// represent physical placement. Canonical source-level semantic IR should
/// normally use logical qubits.
    PhysicalQubit(PhysicalQubitId),

    /// Generic SSA-like IR value.
    Value,

    /// Classical resource.
    Classical,

    /// Parameter value.
    Parameter,

    /// Abstract resource.
    Resource,

    /// Extension-defined binding.
    Extension,
}

impl BlockArgumentBinding {
    /// Returns whether the binding is logical.
    #[must_use]
    pub const fn is_logical_qubit(self) -> bool {
        matches!(self, Self::LogicalQubit(_))
    }

    /// Returns whether the binding is physical.
    #[must_use]
    pub const fn is_physical_qubit(self) -> bool {
        matches!(self, Self::PhysicalQubit(_))
    }

    /// Returns the logical qubit when applicable.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        match self {
            Self::LogicalQubit(qubit) => Some(qubit),
            _ => None,
        }
    }

    /// Returns the physical qubit when applicable.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        match self {
            Self::PhysicalQubit(qubit) => Some(qubit),
            _ => None,
        }
    }
}

// =============================================================================
// Block argument
// =============================================================================

/// A value entering a block.
///
/// The block argument is identified by a canonical `ValueId`.
///
/// The concrete type of the value belongs to the canonical IR type/value
/// system. This structure only records the structural binding needed by the
/// block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockArgument {
    value: ValueId,
    binding: BlockArgumentBinding,
}

impl BlockArgument {
    /// Creates a generic block argument.
    pub const fn value(value: ValueId) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::Value,
        })
    }

    /// Creates a logical-qubit block argument.
    pub const fn logical_qubit(
        value: ValueId,
        qubit: QubitId,
    ) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::LogicalQubit(qubit),
        })
    }

    /// Creates a physical-qubit block argument.
    pub const fn physical_qubit(
        value: ValueId,
        qubit: PhysicalQubitId,
    ) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::PhysicalQubit(qubit),
        })
    }

    /// Creates a classical block argument.
    pub const fn classical(value: ValueId) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::Classical,
        })
    }

    /// Creates a parameter block argument.
    pub const fn parameter(value: ValueId) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::Parameter,
        })
    }

    /// Creates a resource block argument.
    pub const fn resource(value: ValueId) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self {
            value,
            binding: BlockArgumentBinding::Resource,
        })
    }

    /// Returns the argument's value identity.
    #[must_use]
    pub const fn value(self) -> ValueId {
        self.value
    }

    /// Returns the argument binding.
    #[must_use]
    pub const fn binding(self) -> BlockArgumentBinding {
        self.binding
    }

    /// Returns the logical qubit, when this argument represents one.
    #[must_use]
    pub const fn logical_qubit(self) -> Option<QubitId> {
        self.binding.logical_qubit()
    }

    /// Returns the physical qubit, when this argument represents one.
    #[must_use]
    pub const fn physical_qubit(self) -> Option<PhysicalQubitId> {
        self.binding.physical_qubit()
    }
}

// =============================================================================
// Successor argument
// =============================================================================

/// A value passed from one block to a successor block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SuccessorArgument {
    value: ValueId,
}

impl SuccessorArgument {
    /// Creates a successor argument.
    pub const fn new(value: ValueId) -> Result<Self, BlockError> {
        if value.value() == 0 {
            return Err(BlockError::InvalidValueId { value });
        }

        Ok(Self { value })
    }

    /// Returns the value being passed.
    #[must_use]
    pub const fn value(self) -> ValueId {
        self.value
    }
}

// =============================================================================
// Successor
// =============================================================================

/// A control-flow successor of a block.
///
/// The target is an identity only. The containing region/program owns the
/// actual target block.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockSuccessor {
    target: BlockId,
    arguments: Vec<SuccessorArgument>,
}

impl BlockSuccessor {
    /// Creates a successor without arguments.
    pub const fn new(target: BlockId) -> Result<Self, BlockError> {
        if target.value() == 0 {
            return Err(BlockError::InvalidSuccessorTarget { block: target });
        }

        Ok(Self {
            target,
            arguments: Vec::new(),
        })
    }

    /// Creates a successor with explicit arguments.
    pub fn with_arguments(
        target: BlockId,
        arguments: Vec<SuccessorArgument>,
    ) -> Result<Self, BlockError> {
        if target.value() == 0 {
            return Err(BlockError::InvalidSuccessorTarget { block: target });
        }

        validate_unique_successor_arguments(&arguments)?;

        Ok(Self {
            target,
            arguments,
        })
    }

    /// Returns the target block identity.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Returns successor arguments.
    #[must_use]
    pub fn arguments(&self) -> &[SuccessorArgument] {
        &self.arguments
    }

    /// Returns the number of arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Adds an argument to this successor.
    pub fn push_argument(
        &mut self,
        argument: SuccessorArgument,
    ) -> BlockResult<()> {
        if self
            .arguments
            .iter()
            .any(|existing| existing.value() == argument.value())
        {
            return Err(BlockError::DuplicateSuccessorArgument {
                value: argument.value(),
            });
        }

        self.arguments
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "successor arguments",
            })?;

        self.arguments.push(argument);

        Ok(())
    }

    /// Returns whether this successor has no arguments.
    #[must_use]
    pub fn is_argumentless(&self) -> bool {
        self.arguments.is_empty()
    }
}

// =============================================================================
// Terminator
// =============================================================================

/// Structural terminator of a block.
///
/// A terminator describes control-flow structure only.
///
/// It does not execute anything and does not perform scheduling.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockTerminator {
    /// The block remains open and may receive additional operations.
    Open,

    /// Return from the surrounding region/function.
    Return {
        /// Values returned from the block.
        values: Vec<ValueId>,
    },

    /// Unconditional branch.
    Branch {
        /// Destination.
        target: BlockSuccessor,
    },

    /// Conditional branch.
    ConditionalBranch {
        /// Condition value.
        condition: ValueId,

        /// Destination when condition is true.
        then_target: BlockSuccessor,

        /// Destination when condition is false.
        else_target: BlockSuccessor,
    },

    /// Multi-way branch selected by a value.
    Switch {
        /// Selector value.
        selector: ValueId,

        /// Ordered case destinations.
        cases: Vec<SwitchCase>,

        /// Default destination.
        default_target: BlockSuccessor,
    },

    /// The block cannot continue normally.
    Unreachable,

    /// Extension-defined terminator.
    Extension {
        /// Stable extension identity.
        extension: super::super::identity::ExtensionId,
    },
}

impl Default for BlockTerminator {
    fn default() -> Self {
        Self::Open
    }
}

impl BlockTerminator {
    /// Returns whether the block is still open.
    #[must_use]
    pub const fn is_open(&self) -> bool {
        matches!(self, Self::Open)
    }

    /// Returns whether this terminator ends normal control flow.
    #[must_use]
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Return { .. }
                | Self::Unreachable
                | Self::Extension { .. }
        )
    }

    /// Returns whether this terminator has successors.
    #[must_use]
    pub const fn has_successors(&self) -> bool {
        match self {
            Self::Open | Self::Return { .. } | Self::Unreachable => false,

            Self::Branch { .. } => true,

            Self::ConditionalBranch { .. } => true,

            Self::Switch { .. } => true,

            Self::Extension { .. } => false,
        }
    }

    /// Returns the number of outgoing control-flow edges.
    #[must_use]
    pub fn successor_count(&self) -> usize {
        match self {
            Self::Open
            | Self::Return { .. }
            | Self::Unreachable
            | Self::Extension { .. } => 0,

            Self::Branch { .. } => 1,

            Self::ConditionalBranch { .. } => 2,

            Self::Switch {
                cases,
                ..
            } => cases.len().saturating_add(1),
        }
    }

    /// Returns all successor targets in deterministic semantic order.
    #[must_use]
    pub fn successors(&self) -> Vec<&BlockSuccessor> {
        match self {
            Self::Open
            | Self::Return { .. }
            | Self::Unreachable
            | Self::Extension { .. } => Vec::new(),

            Self::Branch { target } => vec![target],

            Self::ConditionalBranch {
                then_target,
                else_target,
                ..
            } => vec![then_target, else_target],

            Self::Switch {
                cases,
                default_target,
                ..
            } => {
                let mut result = Vec::with_capacity(cases.len() + 1);

                for case in cases {
                    result.push(&case.target);
                }

                result.push(default_target);

                result
            }
        }
    }
}

// =============================================================================
// Switch case
// =============================================================================

/// One switch-case destination.
///
/// The case key is represented as an opaque `i128` because the canonical
/// classical value system may later support richer symbolic case keys. The
/// block structure must not depend on one concrete classical integer type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchCase {
    key: i128,
    target: BlockSuccessor,
}

impl SwitchCase {
    /// Creates a switch case.
    pub fn new(
        key: i128,
        target: BlockSuccessor,
    ) -> Self {
        Self { key, target }
    }

    /// Returns the case key.
    #[must_use]
    pub const fn key(&self) -> i128 {
        self.key
    }

    /// Returns the case destination.
    #[must_use]
    pub const fn target(&self) -> &BlockSuccessor {
        &self.target
    }

    /// Returns the mutable case destination.
    pub fn target_mut(&mut self) -> &mut BlockSuccessor {
        &mut self.target
    }
}

// =============================================================================
// Block
// =============================================================================

/// Canonical structured program block.
///
/// The block is deliberately an identity/reference container rather than an
/// owner of concrete operations.
///
/// # Storage model
///
/// ```text
/// Block
/// ├── identity
/// ├── parent region
/// ├── arguments
/// ├── ordered operation IDs
/// ├── qubit scope
/// ├── successors/terminator
/// └── semantic kind
/// ```
///
/// This model is suitable for:
///
/// - gate-based circuits;
/// - dynamic circuits;
/// - classical feedback;
/// - pulse programs;
/// - analog programs;
/// - logical/fault-tolerant programs;
/// - distributed programs;
/// - hybrid programs;
/// - future dialects.
///
/// No machine-specific assumption is embedded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    id: BlockId,

    kind: BlockKind,

    parent_region: Option<RegionId>,

    arguments: Vec<BlockArgument>,

    operations: Vec<OperationId>,

    logical_qubits: Vec<QubitId>,

    physical_qubits: Vec<PhysicalQubitId>,

    terminator: BlockTerminator,
}

impl Block {
    /// Creates an empty basic block.
    pub const fn new(id: BlockId) -> Result<Self, BlockError> {
        if id.value() == 0 {
            return Err(BlockError::InvalidBlockId);
        }

        Ok(Self {
            id,
            kind: BlockKind::Basic,
            parent_region: None,
            arguments: Vec::new(),
            operations: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            terminator: BlockTerminator::Open,
        })
    }

    /// Creates an empty block with a semantic kind.
    pub const fn with_kind(
        id: BlockId,
        kind: BlockKind,
    ) -> Result<Self, BlockError> {
        if id.value() == 0 {
            return Err(BlockError::InvalidBlockId);
        }

        Ok(Self {
            id,
            kind,
            parent_region: None,
            arguments: Vec::new(),
            operations: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            terminator: BlockTerminator::Open,
        })
    }

    /// Returns the stable block identity.
    #[must_use]
    pub const fn id(&self) -> BlockId {
        self.id
    }

    /// Returns the semantic block kind.
    #[must_use]
    pub const fn kind(&self) -> BlockKind {
        self.kind
    }

    /// Changes the semantic block kind.
    pub fn set_kind(&mut self, kind: BlockKind) {
        self.kind = kind;
    }

    /// Returns the parent region, if one has been assigned.
    #[must_use]
    pub const fn parent_region(&self) -> Option<RegionId> {
        self.parent_region
    }

    /// Assigns the parent region.
    ///
    /// The region owner is responsible for ensuring that a block is not
    /// simultaneously owned by multiple incompatible regions.
    pub fn set_parent_region(
        &mut self,
        region: RegionId,
    ) -> BlockResult<()> {
        if region.value() == 0 {
            return Err(BlockError::InvalidBlock {
                message: "parent region identity cannot be zero",
            });
        }

        self.parent_region = Some(region);

        Ok(())
    }

    /// Removes the parent-region reference.
    pub fn clear_parent_region(&mut self) {
        self.parent_region = None;
    }

    // =========================================================================
    // Arguments
    // =========================================================================

    /// Returns all block arguments in deterministic declaration order.
    #[must_use]
    pub fn arguments(&self) -> &[BlockArgument] {
        &self.arguments
    }

    /// Returns the number of block arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Adds a block argument.
    pub fn add_argument(
        &mut self,
        argument: BlockArgument,
    ) -> BlockResult<()> {
        if self
            .arguments
            .iter()
            .any(|existing| existing.value() == argument.value())
        {
            return Err(BlockError::DuplicateArgument {
                value: argument.value(),
            });
        }

        self.arguments
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "block arguments",
            })?;

        self.arguments.push(argument);

        Ok(())
    }

    /// Removes a block argument by value identity.
    ///
    /// Returns the removed argument when present.
    pub fn remove_argument(
        &mut self,
        value: ValueId,
    ) -> Option<BlockArgument> {
        let index = self
            .arguments
            .iter()
            .position(|argument| argument.value() == value)?;

        Some(self.arguments.remove(index))
    }

    /// Returns whether a value is a block argument.
    #[must_use]
    pub fn has_argument(
        &self,
        value: ValueId,
    ) -> bool {
        self.arguments
            .iter()
            .any(|argument| argument.value() == value)
    }

    // =========================================================================
    // Logical qubit scope
    // =========================================================================

    /// Returns logical qubits explicitly associated with this block.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns the number of logical qubits explicitly associated with this
    /// block.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.logical_qubits.len()
    }

    /// Adds a logical qubit to the block scope.
    ///
    /// This does not allocate a physical qubit and does not perform routing.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> BlockResult<()> {
        if self.logical_qubits.contains(&qubit) {
            return Err(BlockError::DuplicateLogicalQubit { qubit });
        }

        self.logical_qubits
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "logical qubit scope",
            })?;

        self.logical_qubits.push(qubit);

        Ok(())
    }

    /// Adds several logical qubits atomically.
    ///
    /// The input is checked for duplicates both internally and against the
    /// existing block scope before mutation occurs.
    pub fn add_logical_qubits<I>(
        &mut self,
        qubits: I,
    ) -> BlockResult<()>
    where
        I: IntoIterator<Item = QubitId>,
    {
        let incoming: Vec<QubitId> = qubits.into_iter().collect();

        let mut seen = BTreeSet::new();

        for qubit in &incoming {
            if !seen.insert(*qubit)
                || self.logical_qubits.contains(qubit)
            {
                return Err(BlockError::DuplicateLogicalQubit {
                    qubit: *qubit,
                });
            }
        }

        self.logical_qubits
            .try_reserve(incoming.len())
            .map_err(|_| BlockError::AllocationFailure {
                collection: "logical qubit scope",
            })?;

        self.logical_qubits.extend(incoming);

        Ok(())
    }

    /// Removes a logical qubit from this block scope.
    pub fn remove_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> bool {
        if let Some(index) = self
            .logical_qubits
            .iter()
            .position(|candidate| *candidate == qubit)
        {
            self.logical_qubits.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns whether the block contains the supplied logical qubit.
    #[must_use]
    pub fn contains_logical_qubit(
        &self,
        qubit: QubitId,
    ) -> bool {
        self.logical_qubits.contains(&qubit)
    }

    // =========================================================================
    // Physical qubit scope
    // =========================================================================

    /// Returns explicitly recorded physical qubits.
    ///
    /// Physical qubits are normally introduced by a lowered/target-specific
    /// representation. Their presence here does not imply that the hardware
    /// actually provides them.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns the number of explicitly recorded physical qubits.
    #[must_use]
    pub fn physical_qubit_count(&self) -> usize {
        self.physical_qubits.len()
    }

    /// Adds a physical qubit reference.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> BlockResult<()> {
        if self.physical_qubits.contains(&qubit) {
            return Err(BlockError::DuplicatePhysicalQubit { qubit });
        }

        self.physical_qubits
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "physical qubit scope",
            })?;

        self.physical_qubits.push(qubit);

        Ok(())
    }

    /// Adds multiple physical qubit references atomically.
    pub fn add_physical_qubits<I>(
        &mut self,
        qubits: I,
    ) -> BlockResult<()>
    where
        I: IntoIterator<Item = PhysicalQubitId>,
    {
        let incoming: Vec<PhysicalQubitId> = qubits.into_iter().collect();

        let mut seen = BTreeSet::new();

        for qubit in &incoming {
            if !seen.insert(*qubit)
                || self.physical_qubits.contains(qubit)
            {
                return Err(BlockError::DuplicatePhysicalQubit {
                    qubit: *qubit,
                });
            }
        }

        self.physical_qubits
            .try_reserve(incoming.len())
            .map_err(|_| BlockError::AllocationFailure {
                collection: "physical qubit scope",
            })?;

        self.physical_qubits.extend(incoming);

        Ok(())
    }

    /// Removes a physical qubit from this block scope.
    pub fn remove_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> bool {
        if let Some(index) = self
            .physical_qubits
            .iter()
            .position(|candidate| *candidate == qubit)
        {
            self.physical_qubits.remove(index);
            true
        } else {
            false
        }
    }

    /// Returns whether the supplied physical qubit is recorded.
    #[must_use]
    pub fn contains_physical_qubit(
        &self,
        qubit: PhysicalQubitId,
    ) -> bool {
        self.physical_qubits.contains(&qubit)
    }

    /// Returns a qubit reference for every explicitly recorded qubit.
    ///
    /// Logical references are returned before physical references, preserving
    /// each namespace's insertion order.
    #[must_use]
    pub fn qubit_refs(&self) -> Vec<QubitRef> {
        let mut result =
            Vec::with_capacity(
                self.logical_qubits.len()
                    .saturating_add(self.physical_qubits.len()),
            );

        result.extend(
            self.logical_qubits
                .iter()
                .copied()
                .map(QubitRef::Logical),
        );

        result.extend(
            self.physical_qubits
                .iter()
                .copied()
                .map(QubitRef::Physical),
        );

        result
    }

    // =========================================================================
    // Operations
    // =========================================================================

    /// Returns operation identities in exact semantic program order.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns the number of operations in the block.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Returns whether the block contains no operations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns whether an operation is present.
    #[must_use]
    pub fn contains_operation(
        &self,
        operation: OperationId,
    ) -> bool {
        self.operations.contains(&operation)
    }

    /// Returns the operation at a position.
    #[must_use]
    pub fn operation_at(
        &self,
        position: usize,
    ) -> Option<OperationId> {
        self.operations.get(position).copied()
    }

    /// Returns the position of an operation.
    #[must_use]
    pub fn operation_position(
        &self,
        operation: OperationId,
    ) -> Option<usize> {
        self.operations
            .iter()
            .position(|candidate| *candidate == operation)
    }

    /// Appends an operation to the block.
    pub fn append_operation(
        &mut self,
        operation: OperationId,
    ) -> BlockResult<()> {
        self.ensure_open()?;
        validate_operation_id(operation)?;

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation { operation });
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "block operations",
            })?;

        self.operations.push(operation);

        Ok(())
    }

    /// Inserts an operation at an exact position.
    ///
    /// Existing operation identities remain unchanged.
    pub fn insert_operation(
        &mut self,
        position: usize,
        operation: OperationId,
    ) -> BlockResult<()> {
        self.ensure_open()?;
        validate_operation_id(operation)?;

        if position > self.operations.len() {
            return Err(BlockError::InvalidOperationPosition {
                position,
            });
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation { operation });
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| BlockError::AllocationFailure {
                collection: "block operations",
            })?;

        self.operations.insert(position, operation);

        Ok(())
    }

    /// Removes an operation by identity.
    ///
    /// Removing an operation does not automatically alter its definition in
    /// the program-level operation registry.
    pub fn remove_operation(
        &mut self,
        operation: OperationId,
    ) -> Option<OperationId> {
        let position = self.operation_position(operation)?;

        Some(self.operations.remove(position))
    }

    /// Removes an operation at an exact position.
    pub fn remove_operation_at(
        &mut self,
        position: usize,
    ) -> Option<OperationId> {
        if position >= self.operations.len() {
            return None;
        }

        Some(self.operations.remove(position))
    }

    /// Replaces one operation reference without changing the position.
    pub fn replace_operation(
        &mut self,
        position: usize,
        operation: OperationId,
    ) -> BlockResult<OperationId> {
        self.ensure_open()?;
        validate_operation_id(operation)?;

        if position >= self.operations.len() {
            return Err(BlockError::InvalidOperationPosition {
                position,
            });
        }

        if self.operations.contains(&operation) {
            return Err(BlockError::DuplicateOperation { operation });
        }

        let previous = self.operations[position];
        self.operations[position] = operation;

        Ok(previous)
    }

    /// Removes every operation reference.
    ///
    /// This is a structural mutation only. It does not delete operations from
    /// the program-level registry.
    pub fn clear_operations(&mut self) -> BlockResult<()> {
        self.ensure_open()?;

        self.operations.clear();

        Ok(())
    }

    // =========================================================================
    // Terminator
    // =========================================================================

    /// Returns the current terminator.
    #[must_use]
    pub fn terminator(&self) -> &BlockTerminator {
        &self.terminator
    }

    /// Returns whether the block is open for additional operations.
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.terminator.is_open()
    }

    /// Returns whether the block has been terminated.
    #[must_use]
    pub fn is_terminated(&self) -> bool {
        !self.is_open()
    }

    /// Sets a return terminator.
    pub fn terminate_return(
        &mut self,
        values: Vec<ValueId>,
    ) -> BlockResult<()> {
        self.ensure_open()?;
        validate_unique_values(&values)?;

        self.terminator = BlockTerminator::Return {
            values,
        };

        Ok(())
    }

    /// Sets an unconditional branch terminator.
    pub fn terminate_branch(
        &mut self,
        target: BlockSuccessor,
    ) -> BlockResult<()> {
        self.ensure_open()?;

        self.terminator = BlockTerminator::Branch {
            target,
        };

        Ok(())
    }

    /// Sets a conditional branch terminator.
    pub fn terminate_conditional_branch(
        &mut self,
        condition: ValueId,
        then_target: BlockSuccessor,
        else_target: BlockSuccessor,
    ) -> BlockResult<()> {
        self.ensure_open()?;
        validate_value_id(condition)?;

        if then_target.target() == else_target.target() {
            return Err(BlockError::InvalidBlock {
                message:
                    "conditional branch requires distinct then/else targets",
            });
        }

        self.terminator =
            BlockTerminator::ConditionalBranch {
                condition,
                then_target,
                else_target,
            };

        Ok(())
    }

    /// Sets a switch terminator.
    pub fn terminate_switch(
        &mut self,
        selector: ValueId,
        cases: Vec<SwitchCase>,
        default_target: BlockSuccessor,
    ) -> BlockResult<()> {
        self.ensure_open()?;
        validate_value_id(selector)?;

        validate_unique_switch_keys(&cases)?;

        self.terminator = BlockTerminator::Switch {
            selector,
            cases,
            default_target,
        };

        Ok(())
    }

    /// Marks the block unreachable.
    pub fn terminate_unreachable(&mut self) -> BlockResult<()> {
        self.ensure_open()?;

        self.terminator = BlockTerminator::Unreachable;

        Ok(())
    }

    /// Sets an extension-defined terminator.
    pub fn terminate_extension(
        &mut self,
        extension: super::super::identity::ExtensionId,
    ) -> BlockResult<()> {
        self.ensure_open()?;

        if extension.value() == 0 {
            return Err(BlockError::InvalidBlock {
                message: "extension terminator requires a valid extension ID",
            });
        }

        self.terminator = BlockTerminator::Extension {
            extension,
        };

        Ok(())
    }

    /// Reopens a terminated block.
    ///
    /// This operation is intentionally explicit. A caller must consciously
    /// change a block's control-flow state before appending further operations.
    ///
    /// This is useful to compiler passes that construct or rewrite blocks.
    pub fn reopen(&mut self) {
        self.terminator = BlockTerminator::Open;
    }

    /// Clears the terminator and returns the block to its open state.
    pub fn clear_terminator(&mut self) {
        self.reopen();
    }

    // =========================================================================
    // Structural queries
    // =========================================================================

    /// Returns the number of outgoing control-flow successors.
    #[must_use]
    pub fn successor_count(&self) -> usize {
        self.terminator.successor_count()
    }

    /// Returns all successor references in deterministic semantic order.
    #[must_use]
    pub fn successors(&self) -> Vec<&BlockSuccessor> {
        self.terminator.successors()
    }

    /// Returns whether the block has at least one outgoing edge.
    #[must_use]
    pub fn has_successors(&self) -> bool {
        self.terminator.has_successors()
    }

    /// Returns whether this block has a normal return terminator.
    #[must_use]
    pub fn is_return_block(&self) -> bool {
        matches!(
            self.terminator,
            BlockTerminator::Return { .. }
        )
    }

    /// Returns whether this block is unreachable.
    #[must_use]
    pub fn is_unreachable(&self) -> bool {
        matches!(
            self.terminator,
            BlockTerminator::Unreachable
        )
    }

    /// Returns all operation and successor identities used by the block.
    ///
    /// This is useful to validation and dependency analysis without exposing
    /// internal storage.
    #[must_use]
    pub fn referenced_operation_ids(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns the number of values returned by this block.
    #[must_use]
    pub fn return_value_count(&self) -> usize {
        match &self.terminator {
            BlockTerminator::Return { values } => values.len(),
            _ => 0,
        }
    }

    /// Returns whether the block has a parent region.
    #[must_use]
    pub const fn has_parent_region(&self) -> bool {
        self.parent_region.is_some()
    }

    // =========================================================================
    // Validation
    // =========================================================================

    /// Validates all local block invariants.
    ///
    /// This does not verify that referenced operation IDs or region IDs exist
    /// in the surrounding program. That requires the program-level registry.
    pub fn validate(&self) -> BlockResult<()> {
        if self.id.value() == 0 {
            return Err(BlockError::InvalidBlockId);
        }

        if let Some(region) = self.parent_region {
            if region.value() == 0 {
                return Err(BlockError::InvalidBlock {
                    message: "parent region identity cannot be zero",
                });
            }
        }

        validate_unique_operations(&self.operations)?;
        validate_unique_arguments(&self.arguments)?;
        validate_unique_qubits(&self.logical_qubits)?;
        validate_unique_physical_qubits(&self.physical_qubits)?;

        for operation in &self.operations {
            validate_operation_id(*operation)?;
        }

        for argument in &self.arguments {
            validate_value_id(argument.value())?;
        }

        validate_terminator(&self.terminator)?;

        Ok(())
    }

    /// Returns whether this block satisfies all local invariants.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }

    // =========================================================================
    // Internal helpers
    // =========================================================================

    fn ensure_open(&self) -> BlockResult<()> {
        if self.terminator.is_open() {
            Ok(())
        } else {
            Err(BlockError::BlockAlreadyTerminated)
        }
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

fn validate_operation_id(
    operation: OperationId,
) -> BlockResult<()> {
    if operation.value() == 0 {
        return Err(BlockError::InvalidOperationId {
            operation,
        });
    }

    Ok(())
}

fn validate_value_id(
    value: ValueId,
) -> BlockResult<()> {
    if value.value() == 0 {
        return Err(BlockError::InvalidValueId {
            value,
        });
    }

    Ok(())
}

fn validate_unique_operations(
    operations: &[OperationId],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for operation in operations {
        validate_operation_id(*operation)?;

        if !seen.insert(*operation) {
            return Err(BlockError::DuplicateOperation {
                operation: *operation,
            });
        }
    }

    Ok(())
}

fn validate_unique_arguments(
    arguments: &[BlockArgument],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for argument in arguments {
        validate_value_id(argument.value())?;

        if !seen.insert(argument.value()) {
            return Err(BlockError::DuplicateArgument {
                value: argument.value(),
            });
        }
    }

    Ok(())
}

fn validate_unique_values(
    values: &[ValueId],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for value in values {
        validate_value_id(*value)?;

        if !seen.insert(*value) {
            return Err(BlockError::DuplicateReturnValue {
                value: *value,
            });
        }
    }

    Ok(())
}

fn validate_unique_successor_arguments(
    arguments: &[SuccessorArgument],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for argument in arguments {
        validate_value_id(argument.value())?;

        if !seen.insert(argument.value()) {
            return Err(BlockError::DuplicateSuccessorArgument {
                value: argument.value(),
            });
        }
    }

    Ok(())
}

fn validate_unique_qubits(
    qubits: &[QubitId],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(BlockError::DuplicateLogicalQubit {
                qubit: *qubit,
            });
        }
    }

    Ok(())
}

fn validate_unique_physical_qubits(
    qubits: &[PhysicalQubitId],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for qubit in qubits {
        if !seen.insert(*qubit) {
            return Err(BlockError::DuplicatePhysicalQubit {
                qubit: *qubit,
            });
        }
    }

    Ok(())
}

fn validate_unique_switch_keys(
    cases: &[SwitchCase],
) -> BlockResult<()> {
    let mut seen = BTreeSet::new();

    for case in cases {
        if !seen.insert(case.key()) {
            return Err(BlockError::InvalidBlock {
                message: "switch cases must have unique keys",
            });
        }
    }

    Ok(())
}

fn validate_terminator(
    terminator: &BlockTerminator,
) -> BlockResult<()> {
    match terminator {
        BlockTerminator::Open
        | BlockTerminator::Unreachable
        | BlockTerminator::Extension { .. } => Ok(()),

        BlockTerminator::Return { values } => {
            validate_unique_values(values)
        }

        BlockTerminator::Branch { target } => {
            validate_successor(target)
        }

        BlockTerminator::ConditionalBranch {
            condition,
            then_target,
            else_target,
        } => {
            validate_value_id(*condition)?;
            validate_successor(then_target)?;
            validate_successor(else_target)?;

            if then_target.target() == else_target.target() {
                return Err(BlockError::InvalidBlock {
                    message:
                        "conditional branch then/else targets must differ",
                });
            }

            Ok(())
        }

        BlockTerminator::Switch {
            selector,
            cases,
            default_target,
        } => {
            validate_value_id(*selector)?;
            validate_unique_switch_keys(cases)?;

            for case in cases {
                validate_successor(&case.target)?;
            }

            validate_successor(default_target)?;

            Ok(())
        }
    }
}

fn validate_successor(
    successor: &BlockSuccessor,
) -> BlockResult<()> {
    if successor.target().value() == 0 {
        return Err(BlockError::InvalidSuccessorTarget {
            block: successor.target(),
        });
    }

    validate_unique_successor_arguments(successor.arguments())?;

    Ok(())
}

// =============================================================================
// Trait implementations
// =============================================================================

impl Default for Block {
    fn default() -> Self {
        // Zero is intentionally not a valid semantic ID.
        //
        // Default exists for container/building compatibility only. A default
        // block must be rejected by `validate()` before being inserted into a
        // canonical program.
        Self {
            id: BlockId::new(0),
            kind: BlockKind::Basic,
            parent_region: None,
            arguments: Vec::new(),
            operations: Vec::new(),
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            terminator: BlockTerminator::Open,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{} {} ({} args, {} operations, {} successors)",
            self.kind,
            self.id,
            self.arguments.len(),
            self.operations.len(),
            self.successor_count(),
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn block_id(value: u64) -> BlockId {
        BlockId::new(value)
    }

    fn operation_id(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn value_id(value: u64) -> ValueId {
        ValueId::new(value)
    }

    #[test]
    fn creates_valid_empty_block() {
        let block = Block::new(block_id(1))
            .expect("block construction must succeed");

        assert_eq!(block.id(), block_id(1));
        assert_eq!(block.kind(), BlockKind::Basic);
        assert!(block.is_empty());
        assert!(block.is_open());
        assert!(block.is_valid());
    }

    #[test]
    fn rejects_zero_block_identity() {
        let result = Block::new(block_id(0));

        assert_eq!(
            result,
            Err(BlockError::InvalidBlockId)
        );
    }

    #[test]
    fn preserves_operation_order() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .append_operation(operation_id(10))
            .expect("first operation must append");

        block
            .append_operation(operation_id(20))
            .expect("second operation must append");

        block
            .append_operation(operation_id(30))
            .expect("third operation must append");

        assert_eq!(
            block.operations(),
            &[
                operation_id(10),
                operation_id(20),
                operation_id(30)
            ]
        );
    }

    #[test]
    fn rejects_duplicate_operations() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .append_operation(operation_id(10))
            .expect("operation must append");

        assert_eq!(
            block.append_operation(operation_id(10)),
            Err(BlockError::DuplicateOperation {
                operation: operation_id(10)
            })
        );
    }

    #[test]
    fn inserts_operation_without_changing_existing_identity() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .append_operation(operation_id(10))
            .expect("operation must append");

        block
            .append_operation(operation_id(30))
            .expect("operation must append");

        block
            .insert_operation(1, operation_id(20))
            .expect("operation must insert");

        assert_eq!(
            block.operations(),
            &[
                operation_id(10),
                operation_id(20),
                operation_id(30)
            ]
        );

        assert_eq!(
            block.operation_position(operation_id(10)),
            Some(0)
        );

        assert_eq!(
            block.operation_position(operation_id(30)),
            Some(2)
        );
    }

    #[test]
    fn supports_logical_qubit_scope() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let q0 = QubitId::new(0);
        let q1 = QubitId::new(1);

        block
            .add_logical_qubits([q0, q1])
            .expect("logical qubits must be added");

        assert_eq!(
            block.logical_qubits(),
            &[q0, q1]
        );

        assert!(block.contains_logical_qubit(q0));
        assert!(block.contains_logical_qubit(q1));
    }

    #[test]
    fn rejects_duplicate_logical_qubit() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let q0 = QubitId::new(0);

        block
            .add_logical_qubit(q0)
            .expect("qubit must be added");

        assert_eq!(
            block.add_logical_qubit(q0),
            Err(BlockError::DuplicateLogicalQubit {
                qubit: q0
            })
        );
    }

    #[test]
    fn supports_physical_qubit_scope() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let p0 = PhysicalQubitId::new(0);

        block
            .add_physical_qubit(p0)
            .expect("physical qubit must be added");

        assert!(block.contains_physical_qubit(p0));
    }

    #[test]
    fn supports_block_arguments() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let argument =
            BlockArgument::logical_qubit(
                value_id(1),
                QubitId::new(0),
            )
            .expect("argument must be valid");

        block
            .add_argument(argument)
            .expect("argument must be added");

        assert_eq!(
            block.argument_count(),
            1
        );

        assert_eq!(
            block.arguments()[0].logical_qubit(),
            Some(QubitId::new(0))
        );
    }

    #[test]
    fn rejects_duplicate_block_arguments() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let argument =
            BlockArgument::value(value_id(1))
                .expect("argument must be valid");

        block
            .add_argument(argument)
            .expect("argument must be added");

        assert_eq!(
            block.add_argument(argument),
            Err(BlockError::DuplicateArgument {
                value: value_id(1)
            })
        );
    }

    #[test]
    fn supports_unconditional_branch() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let successor =
            BlockSuccessor::new(block_id(2))
                .expect("successor must be valid");

        block
            .terminate_branch(successor)
            .expect("branch must be accepted");

        assert!(block.is_terminated());
        assert_eq!(
            block.successor_count(),
            1
        );
    }

    #[test]
    fn supports_conditional_branch() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        let then_target =
            BlockSuccessor::new(block_id(2))
                .expect("successor must be valid");

        let else_target =
            BlockSuccessor::new(block_id(3))
                .expect("successor must be valid");

        block
            .terminate_conditional_branch(
                value_id(1),
                then_target,
                else_target,
            )
            .expect("conditional branch must be accepted");

        assert_eq!(
            block.successor_count(),
            2
        );
    }

    #[test]
    fn rejects_mutation_after_termination() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .terminate_unreachable()
            .expect("terminator must be accepted");

        assert_eq!(
            block.append_operation(operation_id(1)),
            Err(BlockError::BlockAlreadyTerminated)
        );
    }

    #[test]
    fn can_explicitly_reopen_block() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .terminate_unreachable()
            .expect("terminator must be accepted");

        block.reopen();

        block
            .append_operation(operation_id(1))
            .expect("operation must append after explicit reopen");

        assert!(block.is_open());
        assert_eq!(
            block.operation_count(),
            1
        );
    }

    #[test]
    fn supports_return_values() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .terminate_return(vec![
                value_id(1),
                value_id(2),
            ])
            .expect("return must be accepted");

        assert!(block.is_return_block());
        assert_eq!(
            block.return_value_count(),
            2
        );
    }

    #[test]
    fn rejects_duplicate_return_values() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        assert_eq!(
            block.terminate_return(vec![
                value_id(1),
                value_id(1),
            ]),
            Err(BlockError::DuplicateReturnValue {
                value: value_id(1)
            })
        );
    }

    #[test]
    fn successor_arguments_are_unique() {
        let result =
            BlockSuccessor::with_arguments(
                block_id(2),
                vec![
                    SuccessorArgument::new(value_id(1))
                        .expect("argument must be valid"),
                    SuccessorArgument::new(value_id(1))
                        .expect("argument must be valid"),
                ],
            );

        assert_eq!(
            result,
            Err(BlockError::DuplicateSuccessorArgument {
                value: value_id(1)
            })
        );
    }

    #[test]
    fn validation_detects_default_block() {
        let block = Block::default();

        assert_eq!(
            block.validate(),
            Err(BlockError::InvalidBlockId)
        );
    }

    #[test]
    fn validation_succeeds_for_valid_complex_block() {
        let mut block =
            Block::with_kind(
                block_id(1),
                BlockKind::Branch,
            )
            .expect("block construction must succeed");

        block
            .set_parent_region(RegionId::new(1))
            .expect("parent region must be accepted");

        block
            .add_logical_qubit(QubitId::new(0))
            .expect("qubit must be accepted");

        block
            .add_argument(
                BlockArgument::logical_qubit(
                    value_id(1),
                    QubitId::new(0),
                )
                .expect("argument must be valid"),
            )
            .expect("argument must be accepted");

        block
            .append_operation(operation_id(1))
            .expect("operation must be accepted");

        block
            .terminate_branch(
                BlockSuccessor::new(block_id(2))
                    .expect("successor must be valid"),
            )
            .expect("terminator must be accepted");

        assert!(block.is_valid());
    }

    #[test]
    fn operation_identity_is_independent_of_position() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .append_operation(operation_id(100))
            .expect("operation must append");

        block
            .append_operation(operation_id(200))
            .expect("operation must append");

        block
            .insert_operation(0, operation_id(50))
            .expect("operation must insert");

        assert_eq!(
            block.operations(),
            &[
                operation_id(50),
                operation_id(100),
                operation_id(200)
            ]
        );

        assert!(block.contains_operation(operation_id(100)));
        assert!(block.contains_operation(operation_id(200)));
    }

    #[test]
    fn display_is_deterministic() {
        let mut block =
            Block::new(block_id(1))
                .expect("block construction must succeed");

        block
            .append_operation(operation_id(10))
            .expect("operation must append");

        assert_eq!(
            block.to_string(),
            "basic block1 (0 args, 1 operations, 0 successors)"
        );
    }
}