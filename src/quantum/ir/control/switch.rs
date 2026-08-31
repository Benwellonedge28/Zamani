//! Zamani Quantum IR — Switch Control Flow
//!
//! Production-grade, hardware-independent representation of multi-way
//! classical control flow.
//!
//! # Purpose
//!
//! `control::switch` owns the semantic representation of a switch/multi-way
//! branch:
//!
//! ```text
//! switch selector {
//!     case 0 => block_a
//!     case 1 => block_b
//!     default => block_default
//! }
//! ```
//!
//! The switch is a semantic IR construct. It does not specify how a backend
//! implements the branch.
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! canonical Zamani Quantum IR
//!      │
//!      ├── control::switch  ← this module
//!      │       │
//!      │       ├── selector
//!      │       ├── cases
//!      │       ├── default
//!      │       └── target blocks
//!      │
//!      ▼
//! validation
//!      ▼
//! optimization / lowering / routing / scheduling
//!      ▼
//! target-specific execution
//! ```
//!
//! # Ownership
//!
//! This module owns:
//!
//! - switch semantic structure;
//! - selector representation;
//! - case discriminators;
//! - case-to-block relationships;
//! - default-case representation;
//! - local switch invariants;
//! - deterministic case ordering;
//! - optional semantic resource metadata;
//! - local validation;
//! - scalable mutation APIs.
//!
//! This module does NOT own:
//!
//! - source-language parsing;
//! - frontend ASTs;
//! - concrete `Operation` objects;
//! - `Block` storage;
//! - `Successor` storage;
//! - physical routing;
//! - scheduling;
//! - pulse generation;
//! - calibration;
//! - backend execution;
//! - measurement execution;
//! - simulator state;
//! - QEC decoding;
//! - hardware topology.
//!
//! # Identity ownership
//!
//! Canonical IR identities are imported from:
//!
//! ```text
//! quantum::ir::identity
//! ```
//!
//! This module never defines duplicate identity types.
//!
//! Logical and physical quantum identities, where semantic resource metadata
//! requires them, are imported from:
//!
//! ```text
//! quantum::ir::qubit
//! ```
//!
//! This module never defines a legacy `qubits` identity.
//!
//! # Switch selector
//!
//! A selector is represented by `ValueId` rather than a Rust integer.
//!
//! This is intentional.
//!
//! The switch therefore does not impose:
//!
//! - u8 selectors;
//! - u16 selectors;
//! - u32 selectors;
//! - u64 selectors;
//! - fixed classical register widths;
//! - fixed machine word sizes.
//!
//! The surrounding type system determines the selector's actual semantic type.
//!
//! # Case discriminators
//!
//! Case labels are represented by `SwitchValue`.
//!
//! The representation supports:
//!
//! - signed integers;
//! - unsigned integers;
//! - Boolean values;
//! - bit strings;
//! - symbolic values;
//! - opaque extension-defined values.
//!
//! No particular integer width is required.
//!
//! Case identity is semantic. Two cases with equal discriminators are rejected
//! within one switch.
//!
//! # Default case
//!
//! A switch may have zero or one default target.
//!
//! A switch without a default is valid when the surrounding language/IR
//! semantics permit an unmatched selector to leave the switch without taking a
//! switch edge.
//!
//! Whether that behavior is legal in a complete program is determined by
//! higher-level validation.
//!
//! # Determinism
//!
//! Case ordering is semantically significant for serialization/debugging even
//! though case dispatch itself is based on discriminator equality.
//!
//! Therefore this module preserves insertion order using `Vec`.
//!
//! Duplicate detection is performed separately without relying on hash-map
//! iteration order.
//!
//! # Scaling
//!
//! There is no fixed:
//!
//! - number of cases;
//! - selector width;
//! - number of blocks;
//! - number of qubits;
//! - number of operations;
//! - number of programs;
//! - hardware size.
//!
//! Collection growth uses fallible `try_reserve`.
//!
//! Numeric case values are not converted through `usize`.
//!
//! `usize` is never used as semantic identity.
//!
//! # Quantum resources
//!
//! Switch semantics are fundamentally classical control flow.
//!
//! A switch may nevertheless be associated with quantum resources for analysis,
//! dependency tracking, or downstream lowering.
//!
//! Such resources use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! quantum::ir::qubit::PhysicalQubitId
//! ```
//!
//! They do not turn physical qubits into switch selectors and do not introduce
//! hardware assumptions into this module.
//!
//! # Integration contracts
//!
//! `identity.rs`
//!     Supplies `ValueId`, `BlockId`, and `OperationId`.
//!
//! `qubit.rs`
//!     Supplies canonical logical and physical qubit identities.
//!
//! `program::block`
//!     Owns actual destination blocks. This module stores only `BlockId`.
//!
//! `program::successor`
//!     Owns concrete CFG edge representation. This module can be lowered into
//!     switch successors without defining another successor type.
//!
//! `program::operation`
//!     Owns concrete operations. This module references values/operations only
//!     through canonical IDs.
//!
//! `control_flow.rs`
//!     May contain or consume this switch representation when constructing
//!     higher-level structured control flow.
//!
//! `validation.rs`
//!     Performs whole-program checks such as selector type correctness,
//!     destination existence, reachability, and compatibility with block
//!     arguments.
//!
//! `analysis.rs`
//!     May inspect switches without mutating them.
//!
//! `serialization.rs`
//!     Serializes every public semantic field in deterministic order.
//!
//! `hash.rs`
//!     Includes every semantic field in canonical hashing.
//!
//! `dialect/*`
//!     May define standardized or extension-specific selector/case semantics.
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
//! - no unsafe code;
//! - no external dependencies.
//!
//! # Security
//!
//! This module does not execute selector expressions.
//!
//! It only represents them.
//!
//! Therefore untrusted switches cannot directly execute arbitrary code through
//! this module.
//!
//! Resource exhaustion is handled by fallible collection growth and explicit
//! caller/compiler limits.
//!
//! # Production invariant
//!
//! A valid switch satisfies:
//!
//! 1. selector identity is explicit;
//! 2. every case has a destination block;
//! 3. every case discriminator is unique;
//! 4. at most one default destination exists;
//! 5. case order is deterministic;
//! 6. selector semantics are not evaluated here;
//! 7. no hardware is embedded;
//! 8. no fixed case count exists;
//! 9. no fixed integer width exists;
//! 10. no fixed qubit count exists;
//! 11. no semantic identity uses `usize`;
//! 12. local mutations preserve invariants;
//! 13. complete-program validation remains possible;
//! 14. every semantic field is inspectable by serialization/hashing;
//! 15. no information is silently discarded.
//!
//! -----------------------------------------------------------------------------
//! Implementation
//! -----------------------------------------------------------------------------

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::super::identity::{BlockId, OperationId, ValueId};
use super::super::qubit::{PhysicalQubitId, QubitId};

// =============================================================================
// Result
// =============================================================================

/// Result type for switch construction and mutation.
pub type SwitchResult<T> = Result<T, SwitchError>;

// =============================================================================
// Switch error
// =============================================================================

/// Errors produced by local switch construction and validation.
///
/// Errors requiring knowledge of the surrounding program belong to the
/// program-wide validation layer and are intentionally not duplicated here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchError {
    /// The selector identity is structurally invalid according to the local
    /// switch contract.
    InvalidSelector {
        /// Selector value.
        selector: ValueId,
    },

    /// A case discriminator is invalid.
    InvalidCaseValue,

    /// A case discriminator already exists.
    DuplicateCase {
        /// The duplicated discriminator.
        value: SwitchValue,
    },

    /// A destination block identity is invalid.
    InvalidTarget {
        /// Destination block.
        target: BlockId,
    },

    /// A default target was already installed.
    DuplicateDefault,

    /// An operation dependency was duplicated.
    DuplicateOperationDependency {
        /// Duplicated operation.
        operation: OperationId,
    },

    /// A logical qubit dependency was duplicated.
    DuplicateLogicalQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A physical qubit dependency was duplicated.
    DuplicatePhysicalQubit {
        /// Duplicated physical qubit.
        qubit: PhysicalQubitId,
    },

    /// A collection could not reserve additional memory.
    AllocationFailure {
        /// Semantic collection that could not grow.
        collection: &'static str,
    },

    /// A switch contains an invalid local structure.
    InvalidStructure {
        /// Static reason.
        reason: &'static str,
    },
}

impl fmt::Display for SwitchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSelector { selector } => {
                write!(formatter, "invalid switch selector {selector}")
            }

            Self::InvalidCaseValue => {
                formatter.write_str("invalid switch case value")
            }

            Self::DuplicateCase { value } => {
                write!(formatter, "duplicate switch case discriminator {value}")
            }

            Self::InvalidTarget { target } => {
                write!(formatter, "invalid switch destination block {target}")
            }

            Self::DuplicateDefault => {
                formatter.write_str("switch already has a default destination")
            }

            Self::DuplicateOperationDependency { operation } => {
                write!(
                    formatter,
                    "switch operation dependency {operation} is duplicated"
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical qubit dependency {qubit} is duplicated"
                )
            }

            Self::DuplicatePhysicalQubit { qubit } => {
                write!(
                    formatter,
                    "physical qubit dependency {qubit} is duplicated"
                )
            }

            Self::AllocationFailure { collection } => {
                write!(
                    formatter,
                    "unable to reserve memory for switch {collection}"
                )
            }

            Self::InvalidStructure { reason } => {
                write!(formatter, "invalid switch structure: {reason}")
            }
        }
    }
}

impl std::error::Error for SwitchError {}

// =============================================================================
// Switch value
// =============================================================================

/// Width-independent semantic value used as a switch discriminator.
///
/// The selector's actual type is owned by the surrounding IR type system.
/// `SwitchValue` only represents a discriminator.
///
/// Integer variants intentionally have arbitrary Rust integer widths only at
/// the representation boundary. They do not imply that the target machine
/// has that width as its native word size.
///
/// For compiler-generated values that exceed native Rust integer widths, use
/// `BitString`, `Symbolic`, or `Opaque` rather than narrowing the value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SwitchValue {
    /// Boolean discriminator.
    Bool(bool),

    /// Signed 64-bit discriminator.
    ///
    /// This is a convenient compact representation, not a language-wide
    /// integer-width restriction.
    Signed(i64),

    /// Unsigned 64-bit discriminator.
    ///
    /// This is a convenient compact representation, not a language-wide
    /// integer-width restriction.
    Unsigned(u64),

    /// Arbitrary-width bit-string discriminator.
    ///
    /// The bit string is stored most-significant-bit first.
    ///
    /// The final byte may contain unused low bits; `bit_width` specifies the
    /// exact semantic width.
    BitString {
        /// Exact semantic width.
        bit_width: u64,

        /// Big-endian bytes.
        bytes: Vec<u8>,
    },

    /// Symbolic discriminator.
    ///
    /// This is useful when case matching is intentionally deferred to a later
    /// compilation stage.
    Symbolic(String),

    /// Extension-defined discriminator.
    ///
    /// The namespace and bytes are preserved without interpreting them.
    Opaque {
        /// Extension namespace.
        namespace: String,

        /// Extension payload.
        bytes: Vec<u8>,
    },
}

impl SwitchValue {
    /// Creates a Boolean discriminator.
    #[must_use]
    pub const fn bool(value: bool) -> Self {
        Self::Bool(value)
    }

    /// Creates a signed discriminator.
    #[must_use]
    pub const fn signed(value: i64) -> Self {
        Self::Signed(value)
    }

    /// Creates an unsigned discriminator.
    #[must_use]
    pub const fn unsigned(value: u64) -> Self {
        Self::Unsigned(value)
    }

    /// Creates a symbolic discriminator.
    ///
    /// Empty symbols are rejected by returning `None`.
    #[must_use]
    pub fn symbolic(value: impl Into<String>) -> Option<Self> {
        let value = value.into();

        if value.is_empty() {
            None
        } else {
            Some(Self::Symbolic(value))
        }
    }

    /// Creates an arbitrary-width bit-string discriminator.
    ///
    /// `bytes` must contain exactly the number of bytes required by
    /// `bit_width`.
    ///
    /// The unused low bits of the final byte must be zero.
    pub fn bit_string(
        bit_width: u64,
        bytes: Vec<u8>,
    ) -> SwitchResult<Self> {
        let required_bytes_u64 = bit_width
            .checked_add(7)
            .ok_or(SwitchError::InvalidCaseValue)?
            / 8;

        let required_bytes = usize::try_from(required_bytes_u64)
            .map_err(|_| SwitchError::InvalidCaseValue)?;

        if bytes.len() != required_bytes {
            return Err(SwitchError::InvalidCaseValue);
        }

        if bit_width == 0 {
            if !bytes.is_empty() {
                return Err(SwitchError::InvalidCaseValue);
            }

            return Ok(Self::BitString {
                bit_width,
                bytes,
            });
        }

        let remainder = bit_width % 8;

        if remainder != 0 {
            let unused_bits = 8 - remainder;
            let mask = u8::MAX << remainder;

            if bytes
                .last()
                .map(|byte| (*byte & mask) != 0)
                .unwrap_or(true)
            {
                return Err(SwitchError::InvalidCaseValue);
            }

            let _ = unused_bits;
        }

        Ok(Self::BitString {
            bit_width,
            bytes,
        })
    }

    /// Creates an extension-defined discriminator.
    pub fn opaque(
        namespace: impl Into<String>,
        bytes: Vec<u8>,
    ) -> SwitchResult<Self> {
        let namespace = namespace.into();

        if namespace.is_empty() {
            return Err(SwitchError::InvalidCaseValue);
        }

        Ok(Self::Opaque { namespace, bytes })
    }

    /// Returns whether this discriminator is Boolean.
    #[must_use]
    pub const fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    /// Returns whether this discriminator is symbolic.
    #[must_use]
    pub const fn is_symbolic(&self) -> bool {
        matches!(self, Self::Symbolic(_))
    }

    /// Returns whether this discriminator is extension-defined.
    #[must_use]
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque { .. })
    }

    /// Returns the exact bit width for a bit-string discriminator.
    #[must_use]
    pub const fn bit_width(&self) -> Option<u64> {
        match self {
            Self::BitString { bit_width, .. } => Some(*bit_width),
            _ => None,
        }
    }

    /// Returns the bit-string bytes when this is a bit-string discriminator.
    #[must_use]
    pub fn bit_string_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::BitString { bytes, .. } => Some(bytes.as_slice()),
            _ => None,
        }
    }
}

impl From<bool> for SwitchValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for SwitchValue {
    fn from(value: i64) -> Self {
        Self::Signed(value)
    }
}

impl From<u64> for SwitchValue {
    fn from(value: u64) -> Self {
        Self::Unsigned(value)
    }
}

impl fmt::Display for SwitchValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(value) => write!(formatter, "{value}"),
            Self::Signed(value) => write!(formatter, "{value}"),
            Self::Unsigned(value) => write!(formatter, "{value}"),
            Self::BitString {
                bit_width,
                bytes,
            } => {
                write!(formatter, "bits<{bit_width}>:0x")?;

                for byte in bytes {
                    write!(formatter, "{byte:02x}")?;
                }

                Ok(())
            }
            Self::Symbolic(value) => write!(formatter, "{value}"),
            Self::Opaque { namespace, bytes } => {
                write!(formatter, "{namespace}:0x")?;

                for byte in bytes {
                    write!(formatter, "{byte:02x}")?;
                }

                Ok(())
            }
        }
    }
}

// =============================================================================
// Switch case
// =============================================================================

/// One switch case.
///
/// A case is a semantic discriminator-to-block relationship.
///
/// It does not own the destination block itself.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchCase {
    /// Case discriminator.
    value: SwitchValue,

    /// Destination block.
    target: BlockId,

    /// Values passed to the destination block.
    ///
    /// Their types and compatibility with the target block's arguments are
    /// validated by the surrounding program validation layer.
    arguments: Vec<ValueId>,
}

impl SwitchCase {
    /// Creates a case with no destination arguments.
    pub fn new(
        value: SwitchValue,
        target: BlockId,
    ) -> SwitchResult<Self> {
        Self::with_arguments(value, target, Vec::new())
    }

    /// Creates a case with destination block arguments.
    pub fn with_arguments(
        value: SwitchValue,
        target: BlockId,
        arguments: Vec<ValueId>,
    ) -> SwitchResult<Self> {
        validate_target(target)?;
        validate_values(&arguments)?;

        let mut seen = std::collections::BTreeSet::new();

        for argument in &arguments {
            if !seen.insert(*argument) {
                return Err(SwitchError::InvalidStructure {
                    reason: "case destination arguments must be unique",
                });
            }
        }

        Ok(Self {
            value,
            target,
            arguments,
        })
    }

    /// Returns the discriminator.
    #[must_use]
    pub const fn value(&self) -> &SwitchValue {
        &self.value
    }

    /// Returns the destination block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Returns destination arguments in deterministic order.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns the number of destination arguments.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Replaces destination arguments after validating local uniqueness.
    pub fn set_arguments(
        &mut self,
        arguments: Vec<ValueId>,
    ) -> SwitchResult<()> {
        validate_values(&arguments)?;

        let mut seen = std::collections::BTreeSet::new();

        for argument in &arguments {
            if !seen.insert(*argument) {
                return Err(SwitchError::InvalidStructure {
                    reason: "case destination arguments must be unique",
                });
            }
        }

        self.arguments = arguments;
        Ok(())
    }

    /// Returns whether this case uses the supplied discriminator.
    #[must_use]
    pub fn matches(&self, value: &SwitchValue) -> bool {
        self.value == *value
    }
}

impl fmt::Display for SwitchCase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "case {} -> {}",
            self.value,
            self.target
        )
    }
}

// =============================================================================
// Switch resource metadata
// =============================================================================

/// Semantic resource metadata associated with a switch.
///
/// This is intentionally optional.
///
/// Switch semantics do not require a switch to declare qubit dependencies.
/// The metadata exists so analysis/lowering can preserve explicit dependency
/// information without coupling the switch to hardware.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SwitchResources {
    /// Logical qubits semantically touched or observed by the switch region.
    logical_qubits: Vec<QubitId>,

    /// Physical qubits known to be relevant after physical lowering.
    ///
    /// Canonical semantic/source-level IR should normally leave this empty.
    physical_qubits: Vec<PhysicalQubitId>,

    /// Operations whose results or side effects participate in switch
    /// selection/dependency analysis.
    operations: Vec<OperationId>,
}

impl SwitchResources {
    /// Creates empty resource metadata.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logical_qubits: Vec::new(),
            physical_qubits: Vec::new(),
            operations: Vec::new(),
        }
    }

    /// Adds a logical qubit dependency.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> SwitchResult<()> {
        if self.logical_qubits.contains(&qubit) {
            return Err(SwitchError::DuplicateLogicalQubit { qubit });
        }

        self.logical_qubits
            .try_reserve(1)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "logical qubit dependencies",
            })?;

        self.logical_qubits.push(qubit);
        Ok(())
    }

    /// Adds a physical qubit dependency.
    ///
    /// Physical references are allowed for post-mapping IR, but this module
    /// never requires them.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> SwitchResult<()> {
        if self.physical_qubits.contains(&qubit) {
            return Err(SwitchError::DuplicatePhysicalQubit { qubit });
        }

        self.physical_qubits
            .try_reserve(1)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "physical qubit dependencies",
            })?;

        self.physical_qubits.push(qubit);
        Ok(())
    }

    /// Adds an operation dependency.
    pub fn add_operation(
        &mut self,
        operation: OperationId,
    ) -> SwitchResult<()> {
        if self.operations.contains(&operation) {
            return Err(SwitchError::DuplicateOperationDependency {
                operation,
            });
        }

        self.operations
            .try_reserve(1)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "operation dependencies",
            })?;

        self.operations.push(operation);
        Ok(())
    }

    /// Returns logical qubit dependencies.
    #[must_use]
    pub fn logical_qubits(&self) -> &[QubitId] {
        &self.logical_qubits
    }

    /// Returns physical qubit dependencies.
    #[must_use]
    pub fn physical_qubits(&self) -> &[PhysicalQubitId] {
        &self.physical_qubits
    }

    /// Returns operation dependencies.
    #[must_use]
    pub fn operations(&self) -> &[OperationId] {
        &self.operations
    }

    /// Returns whether no resource metadata is attached.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_qubits.is_empty()
            && self.physical_qubits.is_empty()
            && self.operations.is_empty()
    }
}

// =============================================================================
// Switch
// =============================================================================

/// Canonical semantic representation of a multi-way switch.
///
/// The structure is intentionally independent of concrete CFG storage.
///
/// ```text
/// Switch
/// ├── selector ValueId
/// ├── case[0] -> BlockId
/// ├── case[1] -> BlockId
/// ├── ...
/// ├── optional default -> BlockId
/// └── resource metadata
/// ```
///
/// The surrounding `program::block`/`program::successor` layers can lower this
/// semantic representation into concrete CFG edges.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Switch {
    /// Classical selector value.
    selector: ValueId,

    /// Ordered case list.
    cases: Vec<SwitchCase>,

    /// Optional default destination.
    default: Option<SwitchDefault>,

    /// Optional semantic resource metadata.
    resources: SwitchResources,
}

impl Switch {
    /// Creates an empty switch.
    ///
    /// An empty switch is useful during construction but is rejected by
    /// `validate()` until at least one case or a default destination exists.
    pub fn new(selector: ValueId) -> SwitchResult<Self> {
        validate_selector(selector)?;

        Ok(Self {
            selector,
            cases: Vec::new(),
            default: None,
            resources: SwitchResources::new(),
        })
    }

    /// Creates a switch with preallocated case capacity.
    ///
    /// This is a performance hint only and is never a semantic limit.
    pub fn with_capacity(
        selector: ValueId,
        capacity: usize,
    ) -> SwitchResult<Self> {
        validate_selector(selector)?;

        let mut cases = Vec::new();

        cases
            .try_reserve(capacity)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "switch cases",
            })?;

        Ok(Self {
            selector,
            cases,
            default: None,
            resources: SwitchResources::new(),
        })
    }

    /// Returns the selector value.
    #[must_use]
    pub const fn selector(&self) -> ValueId {
        self.selector
    }

    /// Replaces the selector.
    pub fn set_selector(
        &mut self,
        selector: ValueId,
    ) -> SwitchResult<()> {
        validate_selector(selector)?;
        self.selector = selector;
        Ok(())
    }

    /// Returns all cases in deterministic insertion order.
    #[must_use]
    pub fn cases(&self) -> &[SwitchCase] {
        &self.cases
    }

    /// Returns the number of explicit cases.
    #[must_use]
    pub fn case_count(&self) -> usize {
        self.cases.len()
    }

    /// Returns whether an explicit case exists for the discriminator.
    #[must_use]
    pub fn contains_case(&self, value: &SwitchValue) -> bool {
        self.cases.iter().any(|case| case.matches(value))
    }

    /// Returns a case by discriminator.
    #[must_use]
    pub fn case(&self, value: &SwitchValue) -> Option<&SwitchCase> {
        self.cases
            .iter()
            .find(|case| case.matches(value))
    }

    /// Returns a mutable case by discriminator.
    pub fn case_mut(
        &mut self,
        value: &SwitchValue,
    ) -> Option<&mut SwitchCase> {
        self.cases
            .iter_mut()
            .find(|case| case.matches(value))
    }

    /// Adds an explicit case.
    pub fn add_case(
        &mut self,
        case: SwitchCase,
    ) -> SwitchResult<()> {
        if self.contains_case(case.value()) {
            return Err(SwitchError::DuplicateCase {
                value: case.value().clone(),
            });
        }

        self.cases
            .try_reserve(1)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "switch cases",
            })?;

        self.cases.push(case);
        Ok(())
    }

    /// Adds an explicit case without destination arguments.
    pub fn add_case_value(
        &mut self,
        value: SwitchValue,
        target: BlockId,
    ) -> SwitchResult<()> {
        self.add_case(SwitchCase::new(value, target)?)
    }

    /// Adds an explicit case with destination arguments.
    pub fn add_case_with_arguments(
        &mut self,
        value: SwitchValue,
        target: BlockId,
        arguments: Vec<ValueId>,
    ) -> SwitchResult<()> {
        self.add_case(SwitchCase::with_arguments(
            value,
            target,
            arguments,
        )?)
    }

    /// Removes an explicit case.
    ///
    /// Returns the removed case when present.
    pub fn remove_case(
        &mut self,
        value: &SwitchValue,
    ) -> Option<SwitchCase> {
        let index = self
            .cases
            .iter()
            .position(|case| case.matches(value))?;

        Some(self.cases.remove(index))
    }

    /// Replaces an existing case.
    ///
    /// The replacement discriminator must not collide with another case.
    pub fn replace_case(
        &mut self,
        old_value: &SwitchValue,
        replacement: SwitchCase,
    ) -> SwitchResult<()> {
        let index = self
            .cases
            .iter()
            .position(|case| case.matches(old_value))
            .ok_or(SwitchError::InvalidStructure {
                reason: "cannot replace a nonexistent switch case",
            })?;

        if replacement.value() != old_value
            && self.contains_case(replacement.value())
        {
            return Err(SwitchError::DuplicateCase {
                value: replacement.value().clone(),
            });
        }

        self.cases[index] = replacement;
        Ok(())
    }

    /// Returns the optional default case.
    #[must_use]
    pub const fn default_case(&self) -> Option<&SwitchDefault> {
        self.default.as_ref()
    }

    /// Returns the default destination block.
    #[must_use]
    pub fn default_target(&self) -> Option<BlockId> {
        self.default
            .as_ref()
            .map(SwitchDefault::target)
    }

    /// Sets the default destination.
    pub fn set_default(
        &mut self,
        target: BlockId,
    ) -> SwitchResult<()> {
        self.set_default_with_arguments(target, Vec::new())
    }

    /// Sets the default destination and destination arguments.
    pub fn set_default_with_arguments(
        &mut self,
        target: BlockId,
        arguments: Vec<ValueId>,
    ) -> SwitchResult<()> {
        let default =
            SwitchDefault::with_arguments(target, arguments)?;

        self.default = Some(default);
        Ok(())
    }

    /// Removes the default destination.
    ///
    /// Returns the removed default when present.
    pub fn remove_default(&mut self) -> Option<SwitchDefault> {
        self.default.take()
    }

    /// Returns the switch's semantic resource metadata.
    #[must_use]
    pub const fn resources(&self) -> &SwitchResources {
        &self.resources
    }

    /// Returns mutable resource metadata.
    pub fn resources_mut(&mut self) -> &mut SwitchResources {
        &mut self.resources
    }

    /// Adds a logical qubit dependency.
    pub fn add_logical_qubit(
        &mut self,
        qubit: QubitId,
    ) -> SwitchResult<()> {
        self.resources.add_logical_qubit(qubit)
    }

    /// Adds a physical qubit dependency.
    pub fn add_physical_qubit(
        &mut self,
        qubit: PhysicalQubitId,
    ) -> SwitchResult<()> {
        self.resources.add_physical_qubit(qubit)
    }

    /// Adds an operation dependency.
    pub fn add_operation_dependency(
        &mut self,
        operation: OperationId,
    ) -> SwitchResult<()> {
        self.resources.add_operation(operation)
    }

    /// Returns the total number of explicit outgoing destinations.
    ///
    /// This count includes explicit cases and the optional default.
    #[must_use]
    pub fn destination_count(&self) -> usize {
        self.cases.len()
            + usize::from(self.default.is_some())
    }

    /// Returns whether a default destination exists.
    #[must_use]
    pub const fn has_default(&self) -> bool {
        self.default.is_some()
    }

    /// Validates local structural invariants.
    ///
    /// This does NOT validate:
    ///
    /// - whether `selector` exists in the surrounding program;
    /// - whether `selector` has an appropriate classical type;
    /// - whether target blocks exist;
    /// - whether destination arguments match target block arguments;
    /// - whether every block is reachable;
    /// - whether the complete CFG is well formed.
    ///
    /// Those checks belong to the program-wide validation layer.
    pub fn validate(&self) -> SwitchResult<()> {
        validate_selector(self.selector)?;

        if self.cases.is_empty() && self.default.is_none() {
            return Err(SwitchError::InvalidStructure {
                reason: "switch must contain at least one case or a default",
            });
        }

        let mut discriminators = std::collections::BTreeSet::new();

        for case in &self.cases {
            validate_target(case.target)?;
            validate_values(&case.arguments)?;

            if !discriminators.insert(case.value.clone()) {
                return Err(SwitchError::DuplicateCase {
                    value: case.value.clone(),
                });
            }
        }

        if let Some(default) = &self.default {
            default.validate()?;
        }

        Ok(())
    }

    /// Returns an iterator over explicit cases.
    pub fn iter_cases(
        &self,
    ) -> std::slice::Iter<'_, SwitchCase> {
        self.cases.iter()
    }
}

impl fmt::Display for Switch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "switch {} with {} case(s)",
            self.selector,
            self.cases.len()
        )?;

        if self.default.is_some() {
            formatter.write_str(" + default")?;
        }

        Ok(())
    }
}

// =============================================================================
// Switch default
// =============================================================================

/// Default switch destination.
///
/// A default is separate from `SwitchCase` because it has no discriminator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwitchDefault {
    /// Destination block.
    target: BlockId,

    /// Values passed to the destination block.
    arguments: Vec<ValueId>,
}

impl SwitchDefault {
    /// Creates a default destination with no arguments.
    pub fn new(target: BlockId) -> SwitchResult<Self> {
        Self::with_arguments(target, Vec::new())
    }

    /// Creates a default destination with block arguments.
    pub fn with_arguments(
        target: BlockId,
        arguments: Vec<ValueId>,
    ) -> SwitchResult<Self> {
        validate_target(target)?;
        validate_values(&arguments)?;

        let mut seen = std::collections::BTreeSet::new();

        for argument in &arguments {
            if !seen.insert(*argument) {
                return Err(SwitchError::InvalidStructure {
                    reason: "default destination arguments must be unique",
                });
            }
        }

        Ok(Self { target, arguments })
    }

    /// Returns the target block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Returns destination arguments.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns destination argument count.
    #[must_use]
    pub fn argument_count(&self) -> usize {
        self.arguments.len()
    }

    /// Validates local invariants.
    pub fn validate(&self) -> SwitchResult<()> {
        validate_target(self.target)?;
        validate_values(&self.arguments)?;

        let mut seen = std::collections::BTreeSet::new();

        for argument in &self.arguments {
            if !seen.insert(*argument) {
                return Err(SwitchError::InvalidStructure {
                    reason: "default destination arguments must be unique",
                });
            }
        }

        Ok(())
    }
}

impl fmt::Display for SwitchDefault {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "default -> {}", self.target)
    }
}

// =============================================================================
// Conversion helpers
// =============================================================================

/// Converts a switch into an ordered view of its destinations.
///
/// The returned representation preserves semantic insertion order.
///
/// The final entry, when present, represents the default destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchDestination {
    /// Optional discriminator.
    ///
    /// `None` means this is the default destination.
    discriminator: Option<SwitchValue>,

    /// Destination block.
    target: BlockId,

    /// Values transferred to the destination.
    arguments: Vec<ValueId>,
}

impl SwitchDestination {
    /// Creates an explicit-case destination.
    fn case(case: &SwitchCase) -> Self {
        Self {
            discriminator: Some(case.value.clone()),
            target: case.target,
            arguments: case.arguments.clone(),
        }
    }

    /// Creates a default destination.
    fn default_case(default: &SwitchDefault) -> Self {
        Self {
            discriminator: None,
            target: default.target,
            arguments: default.arguments.clone(),
        }
    }

    /// Returns the discriminator.
    ///
    /// `None` represents the default edge.
    #[must_use]
    pub const fn discriminator(&self) -> Option<&SwitchValue> {
        self.discriminator.as_ref()
    }

    /// Returns the target block.
    #[must_use]
    pub const fn target(&self) -> BlockId {
        self.target
    }

    /// Returns destination arguments.
    #[must_use]
    pub fn arguments(&self) -> &[ValueId] {
        &self.arguments
    }

    /// Returns whether this destination is the default destination.
    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.discriminator.is_none()
    }
}

impl Switch {
    /// Produces a deterministic ordered destination snapshot.
    ///
    /// This method allocates a new vector so callers may safely transform the
    /// result without borrowing the switch.
    pub fn destinations(
        &self,
    ) -> SwitchResult<Vec<SwitchDestination>> {
        let capacity = self.destination_count();

        let mut destinations = Vec::new();

        destinations
            .try_reserve(capacity)
            .map_err(|_| SwitchError::AllocationFailure {
                collection: "switch destinations",
            })?;

        for case in &self.cases {
            destinations.push(SwitchDestination::case(case));
        }

        if let Some(default) = &self.default {
            destinations.push(SwitchDestination::default_case(default));
        }

        Ok(destinations)
    }
}

// =============================================================================
// Local validation helpers
// =============================================================================

fn validate_selector(selector: ValueId) -> SwitchResult<()> {
    // `ValueId` is an opaque canonical identity. Its existence is checked by
    // the surrounding program validator, not here.
    let _ = selector;
    Ok(())
}

fn validate_target(target: BlockId) -> SwitchResult<()> {
    // `BlockId` is an opaque canonical identity. Its existence is checked by
    // the surrounding region/program validator.
    let _ = target;
    Ok(())
}

fn validate_values(values: &[ValueId]) -> SwitchResult<()> {
    let mut seen = std::collections::BTreeSet::new();

    for value in values {
        if !seen.insert(*value) {
            return Err(SwitchError::InvalidStructure {
                reason: "destination values must be unique",
            });
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_switch() {
        let switch = Switch::new(ValueId::new(1))
            .expect("selector should be accepted");

        assert_eq!(switch.selector(), ValueId::new(1));
        assert_eq!(switch.case_count(), 0);
        assert!(!switch.has_default());
    }

    #[test]
    fn adds_cases() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .add_case_value(
                SwitchValue::Unsigned(0),
                BlockId::new(10),
            )
            .expect("case should be accepted");

        switch
            .add_case_value(
                SwitchValue::Unsigned(1),
                BlockId::new(11),
            )
            .expect("case should be accepted");

        assert_eq!(switch.case_count(), 2);
        assert!(switch.contains_case(&SwitchValue::Unsigned(0)));
        assert!(switch.contains_case(&SwitchValue::Unsigned(1)));
    }

    #[test]
    fn rejects_duplicate_cases() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .add_case_value(
                SwitchValue::Unsigned(0),
                BlockId::new(10),
            )
            .expect("first case should be accepted");

        let result = switch.add_case_value(
            SwitchValue::Unsigned(0),
            BlockId::new(11),
        );

        assert!(matches!(
            result,
            Err(SwitchError::DuplicateCase { .. })
        ));
    }

    #[test]
    fn supports_default() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .set_default(BlockId::new(99))
            .expect("default should be accepted");

        assert!(switch.has_default());
        assert_eq!(
            switch.default_target(),
            Some(BlockId::new(99))
        );
    }

    #[test]
    fn replacing_default_is_allowed() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .set_default(BlockId::new(99))
            .expect("default should be accepted");

        switch
            .set_default(BlockId::new(100))
            .expect("default replacement should be accepted");

        assert_eq!(
            switch.default_target(),
            Some(BlockId::new(100))
        );
    }

    #[test]
    fn validates_non_empty_switch() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        assert!(switch.validate().is_err());

        switch
            .add_case_value(
                SwitchValue::Bool(true),
                BlockId::new(10),
            )
            .expect("case should be accepted");

        assert!(switch.validate().is_ok());
    }

    #[test]
    fn supports_bit_strings() {
        let value =
            SwitchValue::bit_string(12, vec![0x12, 0x30])
                .expect("12-bit value should be valid");

        assert_eq!(value.bit_width(), Some(12));
        assert_eq!(
            value.bit_string_bytes(),
            Some(&[0x12, 0x30][..])
        );
    }

    #[test]
    fn rejects_non_zero_unused_bits() {
        let result =
            SwitchValue::bit_string(12, vec![0x12, 0xF0]);

        assert!(matches!(
            result,
            Err(SwitchError::InvalidCaseValue)
        ));
    }

    #[test]
    fn supports_symbolic_cases() {
        let value =
            SwitchValue::symbolic("theta")
                .expect("symbolic value should be accepted");

        assert!(value.is_symbolic());
    }

    #[test]
    fn supports_opaque_cases() {
        let value =
            SwitchValue::opaque("example.extension", vec![1, 2, 3])
                .expect("opaque value should be accepted");

        assert!(value.is_opaque());
    }

    #[test]
    fn preserves_case_order() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .add_case_value(
                SwitchValue::Unsigned(10),
                BlockId::new(10),
            )
            .expect("case");

        switch
            .add_case_value(
                SwitchValue::Unsigned(2),
                BlockId::new(20),
            )
            .expect("case");

        assert_eq!(
            switch.cases()[0].target(),
            BlockId::new(10)
        );
        assert_eq!(
            switch.cases()[1].target(),
            BlockId::new(20)
        );
    }

    #[test]
    fn removes_case_without_affecting_other_cases() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .add_case_value(
                SwitchValue::Unsigned(0),
                BlockId::new(10),
            )
            .expect("case");

        switch
            .add_case_value(
                SwitchValue::Unsigned(1),
                BlockId::new(11),
            )
            .expect("case");

        let removed =
            switch.remove_case(&SwitchValue::Unsigned(0));

        assert!(removed.is_some());
        assert_eq!(switch.case_count(), 1);
        assert_eq!(
            switch.cases()[0].target(),
            BlockId::new(11)
        );
    }

    #[test]
    fn destination_snapshot_is_deterministic() {
        let mut switch =
            Switch::new(ValueId::new(1)).expect("valid selector");

        switch
            .add_case_value(
                SwitchValue::Unsigned(0),
                BlockId::new(10),
            )
            .expect("case");

        switch
            .add_case_value(
                SwitchValue::Unsigned(1),
                BlockId::new(11),
            )
            .expect("case");

        switch
            .set_default(BlockId::new(99))
            .expect("default");

        let destinations =
            switch.destinations().expect("snapshot");

        assert_eq!(destinations.len(), 3);
        assert!(!destinations[0].is_default());
        assert!(!destinations[1].is_default());
        assert!(destinations[2].is_default());
    }

    #[test]
    fn rejects_duplicate_destination_arguments() {
        let result = SwitchCase::with_arguments(
            SwitchValue::Unsigned(0),
            BlockId::new(10),
            vec![ValueId::new(1), ValueId::new(1)],
        );

        assert!(matches!(
            result,
            Err(SwitchError::InvalidStructure { .. })
        ));
    }

    #[test]
    fn resource_metadata_is_optional() {
        let mut resources = SwitchResources::new();

        assert!(resources.is_empty());

        resources
            .add_logical_qubit(QubitId::new(0))
            .expect("logical qubit");

        resources
            .add_operation(OperationId::new(1))
            .expect("operation");

        assert!(!resources.is_empty());
        assert_eq!(
            resources.logical_qubits(),
            &[QubitId::new(0)]
        );
        assert_eq!(
            resources.operations(),
            &[OperationId::new(1)]
        );
    }

    #[test]
    fn duplicate_resource_dependencies_are_rejected() {
        let mut resources = SwitchResources::new();

        resources
            .add_logical_qubit(QubitId::new(0))
            .expect("first qubit");

        let result =
            resources.add_logical_qubit(QubitId::new(0));

        assert!(matches!(
            result,
            Err(SwitchError::DuplicateLogicalQubit { .. })
        ));
    }

    #[test]
    fn switch_is_cloneable_and_deterministic() {
        let mut switch =
            Switch::new(ValueId::new(7)).expect("selector");

        switch
            .add_case_value(
                SwitchValue::Signed(-1),
                BlockId::new(1),
            )
            .expect("case");

        switch
            .add_case_value(
                SwitchValue::Unsigned(42),
                BlockId::new(2),
            )
            .expect("case");

        switch
            .set_default(BlockId::new(3))
            .expect("default");

        assert_eq!(switch.clone(), switch);
    }
}