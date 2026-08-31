//! Zamani Quantum IR — Logical Quantum Computing Model
//!
//! Path:
//!     src/quantum/ir/model/logical.rs
//!
//! # Purpose
//!
//! This module defines the canonical, target-independent semantic model for
//! logical quantum computation.
//!
//! A logical quantum program describes computation over logical quantum
//! resources without committing those resources to:
//!
//! - physical qubits;
//! - a particular quantum processor;
//! - a particular topology;
//! - a particular quantum error-correcting code implementation;
//! - a particular decoder;
//! - a particular syndrome-extraction circuit;
//! - a particular native gate set;
//! - a particular pulse implementation;
//! - a particular simulator;
//! - a particular backend.
//!
//! The central principle is:
//!
//! ```text
//! logical computation = WHAT
//!
//! physical mapping      = WHERE
//!
//! QEC implementation    = HOW ENCODED
//!
//! routing               = HOW CONNECTED
//!
//! scheduling            = WHEN
//!
//! backend               = HOW EXECUTED
//! ```
//!
//! # Architectural position
//!
//! ```text
//! Zamani source
//!      │
//!      ▼
//! frontend
//!      │
//!      ▼
//! canonical Zamani Quantum IR
//!      │
//!      ├── model::logical          ← THIS FILE
//!      │
//!      ├── model::circuit
//!      ├── model::analog
//!      ├── model::hamiltonian
//!      ├── model::annealing
//!      │
//!      ├── optimization
//!      ├── QEC / fault tolerance
//!      ├── routing / mapping
//!      ├── scheduling
//!      ├── hardware compatibility
//!      ├── pulse lowering
//!      └── backend lowering
//!      │
//!      ▼
//! execution
//! ```
//!
//! # Logical versus physical resources
//!
//! This module uses:
//!
//! ```text
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! for logical identities.
//!
//! It deliberately does NOT define another logical-qubit integer type.
//!
//! Physical qubit identity belongs to `quantum::ir::qubit::PhysicalQubitId`
//! and target-specific mapping belongs to the routing/mapping layer.
//!
//! Therefore this module must never contain fields such as:
//!
//! ```text
//! physical_qubit: usize
//! hardware_index: usize
//! device_topology: ...
//! ```
//!
//! # Universal-program principle
//!
//! A Zamani program must be capable of being written once and lowered to
//! different quantum architectures and different machine sizes.
//!
//! Therefore this file contains no architectural constants such as:
//!
//! ```text
//! MAX_LOGICAL_QUBITS
//! MAX_CODE_DISTANCE
//! MAX_LOGICAL_GATES
//! MAX_BLOCK_SIZE
//! MAX_SYNDROME_ROUNDS
//! ```
//!
//! Any resource limits belong to explicit compiler/execution policy.
//!
//! The semantic model itself has no fixed machine-size ceiling.
//!
//! # What this module represents
//!
//! This module represents:
//!
//! - logical qubit identities;
//! - logical registers;
//! - logical encoding descriptions;
//! - quantum-code descriptors;
//! - code parameters;
//! - logical operation semantics;
//! - logical measurement semantics;
//! - logical initialization semantics;
//! - logical reset semantics;
//! - logical resource requirements;
//! - logical fault-tolerance requirements;
//! - logical program operations;
//! - deterministic local validation.
//!
//! # What this module does NOT represent
//!
//! This module does not implement:
//!
//! - physical qubit allocation;
//! - hardware topology;
//! - routing;
//! - scheduling;
//! - physical gate decomposition;
//! - syndrome extraction circuits;
//! - decoder algorithms;
//! - lattice surgery implementation;
//! - magic-state factories;
//! - pulse generation;
//! - calibration;
//! - simulator state;
//! - amplitudes;
//! - density matrices;
//! - execution;
//! - backend APIs;
//! - vendor-specific behavior.
//!
//! Those are downstream concerns.
//!
//! # Fault-tolerant computing
//!
//! A logical qubit may optionally carry an encoding description.
//!
//! The encoding description is intentionally declarative.
//!
//! For example, a logical qubit may state that it uses a code identified by:
//!
//! ```text
//! namespace = "quantum.error_correction"
//! name      = "surface"
//! ```
//!
//! with parameters such as:
//!
//! ```text
//! distance = 7
//! rounds   = 5
//! ```
//!
//! This module does not interpret those parameters as a particular physical
//! layout. A QEC compiler may later lower the declaration into physical
//! resources.
//!
//! # Extensibility
//!
//! Quantum error correction is not limited to a finite set of named codes.
//!
//! Therefore `LogicalEncoding` uses an extensible namespace/name pair rather
//! than a closed enum such as:
//!
//! ```text
//! SurfaceCode
//! SteaneCode
//! ShorCode
//! BaconShorCode
//! ColorCode
//! ...
//! ```
//!
//! Standard codes can be registered by higher-level dialects without changing
//! this core semantic file.
//!
//! # Operation extensibility
//!
//! Logical operations similarly use an extensible operation identity.
//!
//! Standard logical operations such as:
//!
//! ```text
//! logical.x
//! logical.y
//! logical.z
//! logical.h
//! logical.s
//! logical.t
//! logical.cx
//! logical.cz
//! logical.swap
//! logical.measure
//! logical.reset
//! ```
//!
//! are conventions, not architectural limits.
//!
//! Future logical operations can be represented without modifying this file.
//!
//! # Determinism
//!
//! Semantic collections that have no meaningful order use deterministic
//! ordering through `BTreeSet`/`BTreeMap`.
//!
//! Operation sequences preserve explicit program order through `Vec`.
//!
//! No semantic behavior depends on hash-map iteration order.
//!
//! # Serialization
//!
//! This file does not own the repository-wide serialization format.
//!
//! The serialization layer must serialize the public semantic fields in a
//! deterministic order.
//!
//! No semantic information may be discarded during serialization.
//!
//! Unknown encoding parameters and operation attributes must remain
//! representable by this model.
//!
//! # Hashing
//!
//! Cryptographic hashing is owned by the canonical IR hashing subsystem.
//!
//! This file provides deterministic equality and ordering where practical but
//! does not implement a cryptographic hash algorithm.
//!
//! # Validation
//!
//! Local validation is performed here.
//!
//! Whole-program validation remains the responsibility of the repository-wide
//! validation layer.
//!
//! Local validation guarantees:
//!
//! - logical IDs are unique within a register;
//! - logical operation names are non-empty;
//! - namespaces are non-empty when present;
//! - operation targets are non-empty when required;
//! - operation target IDs are unique;
//! - operation arity constraints are respected;
//! - encoding identifiers are valid;
//! - code parameters have valid names;
//! - no duplicate code parameters exist;
//! - measurement destinations are semantically distinct;
//! - logical resource requirements are internally consistent.
//!
//! # Rust contract
//!
//! Supported:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no external dependencies;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Dependency contract
//!
//! This file intentionally has a very small dependency surface:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! It does not depend on:
//!
//! - circuit;
//! - operation;
//! - hardware;
//! - routing;
//! - scheduling;
//! - simulator;
//! - optimization;
//! - QEC implementation;
//! - backend.
//!
//! This allows the logical model to be implemented and frozen independently.
//!
//! # Integration contract
//!
//! Upstream:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Downstream consumers may include:
//!
//! ```text
//! model::circuit
//! model::logical
//! model::analog
//! optimization
//! qec
//! routing
//! scheduling
//! hardware
//! simulator
//! serialization
//! hashing
//! validation
//! analysis
//! algorithms
//! ```
//!
//! None of those modules may redefine the logical identity types contained
//! here.
//!
//! # Important repository rule
//!
//! New code must use:
//!
//! ```rust
//! crate::quantum::ir::qubit::QubitId
//! ```
//!
//! Do not introduce:
//!
//! ```rust
//! struct LogicalQubitId(usize);
//! ```
//!
//! because that would create a second quantum identity system.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::super::qubit::QubitId;

// =============================================================================
// Result
// =============================================================================

/// Result type used by the logical quantum model.
pub type LogicalResult<T> = Result<T, LogicalError>;

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by the logical quantum model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalError {
    /// A logical identifier was duplicated.
    DuplicateLogicalQubit {
        /// Duplicated logical identifier.
        qubit: QubitId,
    },

    /// An operation contained a duplicate target.
    DuplicateOperationTarget {
        /// Duplicated logical identifier.
        qubit: QubitId,
    },

    /// An operation references a logical qubit not declared by the model.
    UnknownLogicalQubit {
        /// Referenced logical identifier.
        qubit: QubitId,
    },

    /// An operation name has no content.
    EmptyOperationName,

    /// An operation namespace has no content.
    EmptyNamespace,

    /// An encoding namespace has no content.
    EmptyEncodingNamespace,

    /// An encoding name has no content.
    EmptyEncodingName,

    /// A code parameter name has no content.
    EmptyParameterName,

    /// A code parameter was defined twice.
    DuplicateParameter {
        /// Duplicated parameter name.
        name: String,
    },

    /// A logical operation has an invalid target count.
    InvalidArity {
        /// Operation name.
        operation: String,

        /// Minimum required number of targets.
        minimum: usize,

        /// Maximum number of targets when one exists.
        maximum: Option<usize>,

        /// Actual target count.
        actual: usize,
    },

    /// An operation requires at least one target.
    MissingTargets {
        /// Operation name.
        operation: String,
    },

    /// A measurement has no destination identifier.
    EmptyMeasurementDestination,

    /// A register name has no content.
    EmptyRegisterName,

    /// A logical register contains no qubits when at least one is required.
    EmptyRegister,

    /// A resource count cannot be represented by the host representation.
    ResourceCountOverflow {
        /// Resource being counted.
        resource: &'static str,
    },

    /// A logical requirement is internally contradictory.
    InvalidRequirement {
        /// Stable description of the violation.
        message: &'static str,
    },

    /// A logical operation references a disabled resource.
    DisabledLogicalQubit {
        /// Disabled logical identifier.
        qubit: QubitId,
    },
}

impl fmt::Display for LogicalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is declared more than once"
                )
            }

            Self::DuplicateOperationTarget { qubit } => {
                write!(
                    formatter,
                    "logical operation contains duplicate target {qubit}"
                )
            }

            Self::UnknownLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical operation references undeclared qubit {qubit}"
                )
            }

            Self::EmptyOperationName => {
                formatter.write_str("logical operation name cannot be empty")
            }

            Self::EmptyNamespace => {
                formatter.write_str("logical namespace cannot be empty")
            }

            Self::EmptyEncodingNamespace => {
                formatter.write_str(
                    "logical encoding namespace cannot be empty",
                )
            }

            Self::EmptyEncodingName => {
                formatter.write_str(
                    "logical encoding name cannot be empty",
                )
            }

            Self::EmptyParameterName => {
                formatter.write_str(
                    "logical encoding parameter name cannot be empty",
                )
            }

            Self::DuplicateParameter { name } => {
                write!(
                    formatter,
                    "logical encoding parameter `{name}` is duplicated"
                )
            }

            Self::InvalidArity {
                operation,
                minimum,
                maximum,
                actual,
            } => {
                match maximum {
                    Some(maximum) if minimum == *maximum => {
                        write!(
                            formatter,
                            "logical operation `{operation}` requires \
                             exactly {minimum} target(s), got {actual}"
                        )
                    }

                    Some(maximum) => {
                        write!(
                            formatter,
                            "logical operation `{operation}` requires \
                             {minimum}..={maximum} target(s), got {actual}"
                        )
                    }

                    None => {
                        write!(
                            formatter,
                            "logical operation `{operation}` requires \
                             at least {minimum} target(s), got {actual}"
                        )
                    }
                }
            }

            Self::MissingTargets { operation } => {
                write!(
                    formatter,
                    "logical operation `{operation}` requires targets"
                )
            }

            Self::EmptyMeasurementDestination => {
                formatter.write_str(
                    "logical measurement destination cannot be empty",
                )
            }

            Self::EmptyRegisterName => {
                formatter.write_str("logical register name cannot be empty")
            }

            Self::EmptyRegister => {
                formatter.write_str(
                    "logical register must contain at least one qubit",
                )
            }

            Self::ResourceCountOverflow { resource } => {
                write!(
                    formatter,
                    "overflow while counting logical resource `{resource}`"
                )
            }

            Self::InvalidRequirement { message } => {
                write!(
                    formatter,
                    "invalid logical resource requirement: {message}"
                )
            }

            Self::DisabledLogicalQubit { qubit } => {
                write!(
                    formatter,
                    "logical operation references disabled qubit {qubit}"
                )
            }
        }
    }
}

impl std::error::Error for LogicalError {}

// =============================================================================
// Logical qubit status
// =============================================================================

/// Semantic lifecycle status for a logical qubit.
///
/// This is IR bookkeeping only.
///
/// It does not represent a simulator state vector or physical QEC state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalQubitStatus {
    /// The logical resource is available for semantic operations.
    Available,

    /// The logical resource has been measured.
    Measured,

    /// The logical resource has been reset.
    Reset,

    /// The logical resource is explicitly unavailable.
    Disabled,
}

impl Default for LogicalQubitStatus {
    fn default() -> Self {
        Self::Available
    }
}

impl LogicalQubitStatus {
    /// Returns whether the resource can be referenced by ordinary logical
    /// operations.
    #[must_use]
    pub const fn is_usable(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns whether this resource is disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }
}

// =============================================================================
// Encoding parameter
// =============================================================================

/// A symbolic parameter of a logical encoding.
///
/// Parameters are intentionally represented as strings rather than fixed
/// numeric fields because different quantum codes expose different parameter
/// spaces.
///
/// Examples include:
///
/// ```text
/// distance = 7
/// rounds = 5
/// levels = 3
/// gauge = "x"
/// basis = "rotated"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EncodingParameter {
    name: String,
    value: String,
}

impl EncodingParameter {
    /// Creates an encoding parameter.
    pub fn new(
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> LogicalResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyParameterName);
        }

        Ok(Self {
            name,
            value: value.into(),
        })
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the parameter value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

// =============================================================================
// Logical encoding
// =============================================================================

/// Declarative description of a logical-qubit encoding.
///
/// The encoding is identified by an extensible namespace/name pair.
///
/// This deliberately does not enumerate specific quantum error-correcting
/// codes. New codes can therefore be introduced without modifying this file.
///
/// Physical realization is resolved by the QEC and target-compilation layers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalEncoding {
    namespace: String,
    name: String,
    parameters: BTreeMap<String, String>,
}

impl LogicalEncoding {
    /// Creates an encoding descriptor without parameters.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> LogicalResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(LogicalError::EmptyEncodingNamespace);
        }

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyEncodingName);
        }

        Ok(Self {
            namespace,
            name,
            parameters: BTreeMap::new(),
        })
    }

    /// Returns the encoding namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the encoding name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all encoding parameters in deterministic order.
    #[must_use]
    pub fn parameters(&self) -> &BTreeMap<String, String> {
        &self.parameters
    }

    /// Returns a parameter by name.
    #[must_use]
    pub fn parameter(&self, name: &str) -> Option<&str> {
        self.parameters.get(name).map(String::as_str)
    }

    /// Adds or replaces a parameter.
    ///
    /// Parameter names must not be empty.
    pub fn set_parameter(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> LogicalResult<()> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyParameterName);
        }

        self.parameters.insert(name, value.into());
        Ok(())
    }

    /// Adds a parameter and rejects duplicate names.
    pub fn add_parameter(
        &mut self,
        parameter: EncodingParameter,
    ) -> LogicalResult<()> {
        if self
            .parameters
            .contains_key(parameter.name())
        {
            return Err(LogicalError::DuplicateParameter {
                name: parameter.name().to_owned(),
            });
        }

        self.parameters.insert(
            parameter.name().to_owned(),
            parameter.value().to_owned(),
        );

        Ok(())
    }

    /// Validates the encoding descriptor.
    pub fn validate(&self) -> LogicalResult<()> {
        if self.namespace.trim().is_empty() {
            return Err(LogicalError::EmptyEncodingNamespace);
        }

        if self.name.trim().is_empty() {
            return Err(LogicalError::EmptyEncodingName);
        }

        for parameter in self.parameters.keys() {
            if parameter.trim().is_empty() {
                return Err(LogicalError::EmptyParameterName);
            }
        }

        Ok(())
    }
}

impl fmt::Display for LogicalEncoding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Logical qubit
// =============================================================================

/// Canonical logical qubit.
///
/// The identity is the repository-wide `QubitId`.
///
/// An optional encoding describes how the logical abstraction is protected,
/// but does not prescribe its physical realization.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalQubit {
    id: QubitId,
    encoding: Option<LogicalEncoding>,
    status: LogicalQubitStatus,
}

impl LogicalQubit {
    /// Creates an unencoded logical qubit.
    #[must_use]
    pub const fn new(id: QubitId) -> Self {
        Self {
            id,
            encoding: None,
            status: LogicalQubitStatus::Available,
        }
    }

    /// Creates a logical qubit with an explicit encoding.
    pub fn encoded(
        id: QubitId,
        encoding: LogicalEncoding,
    ) -> LogicalResult<Self> {
        encoding.validate()?;

        Ok(Self {
            id,
            encoding: Some(encoding),
            status: LogicalQubitStatus::Available,
        })
    }

    /// Returns the canonical logical identity.
    #[must_use]
    pub const fn id(&self) -> QubitId {
        self.id
    }

    /// Returns the encoding, if any.
    #[must_use]
    pub fn encoding(&self) -> Option<&LogicalEncoding> {
        self.encoding.as_ref()
    }

    /// Returns the current semantic bookkeeping status.
    #[must_use]
    pub const fn status(&self) -> LogicalQubitStatus {
        self.status
    }

    /// Returns whether the qubit is encoded.
    #[must_use]
    pub fn is_encoded(&self) -> bool {
        self.encoding.is_some()
    }

    /// Returns whether it can be used by logical operations.
    #[must_use]
    pub const fn is_usable(&self) -> bool {
        self.status.is_usable()
    }

    /// Changes the encoding descriptor.
    pub fn set_encoding(
        &mut self,
        encoding: Option<LogicalEncoding>,
    ) -> LogicalResult<()> {
        if let Some(ref value) = encoding {
            value.validate()?;
        }

        self.encoding = encoding;
        Ok(())
    }

    /// Marks the logical qubit as measured.
    pub const fn mark_measured(&mut self) {
        self.status = LogicalQubitStatus::Measured;
    }

    /// Marks the logical qubit as reset.
    pub const fn mark_reset(&mut self) {
        self.status = LogicalQubitStatus::Reset;
    }

    /// Marks the logical qubit as available.
    pub const fn mark_available(&mut self) {
        self.status = LogicalQubitStatus::Available;
    }

    /// Marks the logical qubit as disabled.
    pub const fn mark_disabled(&mut self) {
        self.status = LogicalQubitStatus::Disabled;
    }
}

// =============================================================================
// Logical register
// =============================================================================

/// Named logical-qubit register.
///
/// A register materializes only the logical identities actually declared by
/// the program. It does not allocate physical resources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalRegister {
    name: String,
    qubits: Vec<LogicalQubit>,
}

impl LogicalRegister {
    /// Creates an empty register.
    ///
    /// Empty registers are useful during incremental construction.
    pub fn new(name: impl Into<String>) -> LogicalResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyRegisterName);
        }

        Ok(Self {
            name,
            qubits: Vec::new(),
        })
    }

    /// Creates a register from logical qubits.
    pub fn from_qubits(
        name: impl Into<String>,
        qubits: Vec<LogicalQubit>,
    ) -> LogicalResult<Self> {
        let mut register = Self::new(name)?;

        for qubit in qubits {
            register.insert(qubit)?;
        }

        Ok(register)
    }

    /// Returns the register name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns all logical qubits in explicit declaration order.
    #[must_use]
    pub fn qubits(&self) -> &[LogicalQubit] {
        &self.qubits
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn len(&self) -> usize {
        self.qubits.len()
    }

    /// Returns whether the register is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.qubits.is_empty()
    }

    /// Finds a logical qubit by canonical ID.
    #[must_use]
    pub fn get(&self, id: QubitId) -> Option<&LogicalQubit> {
        self.qubits.iter().find(|qubit| qubit.id() == id)
    }

    /// Finds a mutable logical qubit by canonical ID.
    pub fn get_mut(
        &mut self,
        id: QubitId,
    ) -> Option<&mut LogicalQubit> {
        self.qubits
            .iter_mut()
            .find(|qubit| qubit.id() == id)
    }

    /// Inserts a logical qubit.
    pub fn insert(
        &mut self,
        qubit: LogicalQubit,
    ) -> LogicalResult<()> {
        if self.get(qubit.id()).is_some() {
            return Err(LogicalError::DuplicateLogicalQubit {
                qubit: qubit.id(),
            });
        }

        self.qubits.push(qubit);
        Ok(())
    }

    /// Validates the register.
    pub fn validate(&self) -> LogicalResult<()> {
        if self.name.trim().is_empty() {
            return Err(LogicalError::EmptyRegisterName);
        }

        let mut ids = BTreeSet::new();

        for qubit in &self.qubits {
            if !ids.insert(qubit.id()) {
                return Err(LogicalError::DuplicateLogicalQubit {
                    qubit: qubit.id(),
                });
            }

            if let Some(encoding) = qubit.encoding() {
                encoding.validate()?;
            }
        }

        Ok(())
    }
}

// =============================================================================
// Logical operation arity
// =============================================================================

/// Target-count constraint for a logical operation.
///
/// This is intentionally more general than one/two/three-qubit assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicalArity {
    minimum: usize,
    maximum: Option<usize>,
}

impl LogicalArity {
    /// Creates an exact target count.
    #[must_use]
    pub const fn exact(count: usize) -> Self {
        Self {
            minimum: count,
            maximum: Some(count),
        }
    }

    /// Creates a minimum target count with no semantic maximum.
    #[must_use]
    pub const fn at_least(minimum: usize) -> Self {
        Self {
            minimum,
            maximum: None,
        }
    }

    /// Creates a bounded inclusive target count.
    pub const fn range(
        minimum: usize,
        maximum: usize,
    ) -> LogicalResult<Self> {
        if minimum > maximum {
            return Err(LogicalError::InvalidRequirement {
                message: "logical operation arity minimum exceeds maximum",
            });
        }

        Ok(Self {
            minimum,
            maximum: Some(maximum),
        })
    }

    /// Returns the minimum number of targets.
    #[must_use]
    pub const fn minimum(self) -> usize {
        self.minimum
    }

    /// Returns the maximum number of targets.
    #[must_use]
    pub const fn maximum(self) -> Option<usize> {
        self.maximum
    }

    /// Returns whether a target count satisfies this arity.
    #[must_use]
    pub const fn accepts(self, count: usize) -> bool {
        if count < self.minimum {
            return false;
        }

        match self.maximum {
            Some(maximum) => count <= maximum,
            None => true,
        }
    }
}

// =============================================================================
// Logical operation kind
// =============================================================================

/// Extensible identity of a logical operation.
///
/// The operation is identified by a namespace and name rather than by a
/// closed enum. This prevents the logical IR from having a permanently fixed
/// set of quantum operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LogicalOperationKind {
    namespace: String,
    name: String,
}

impl LogicalOperationKind {
    /// Creates an operation kind.
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
    ) -> LogicalResult<Self> {
        let namespace = namespace.into();
        let name = name.into();

        if namespace.trim().is_empty() {
            return Err(LogicalError::EmptyNamespace);
        }

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyOperationName);
        }

        Ok(Self { namespace, name })
    }

    /// Returns the namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the fully-qualified operation identity.
    #[must_use]
    pub fn qualified_name(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

impl fmt::Display for LogicalOperationKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}::{}",
            self.namespace,
            self.name
        )
    }
}

// =============================================================================
// Logical operation
// =============================================================================

/// Canonical semantic logical operation.
///
/// Operations contain logical qubit targets only.
///
/// Any physical realization is deliberately absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOperation {
    kind: LogicalOperationKind,
    targets: Vec<QubitId>,
    controls: Vec<QubitId>,
    arity: LogicalArity,
}

impl LogicalOperation {
    /// Creates an operation with explicit target arity.
    pub fn new(
        kind: LogicalOperationKind,
        targets: Vec<QubitId>,
        arity: LogicalArity,
    ) -> LogicalResult<Self> {
        if !arity.accepts(targets.len()) {
            return Err(LogicalError::InvalidArity {
                operation: kind.qualified_name(),
                minimum: arity.minimum(),
                maximum: arity.maximum(),
                actual: targets.len(),
            });
        }

        Self::validate_unique_targets(&targets)?;

        Ok(Self {
            kind,
            targets,
            controls: Vec::new(),
            arity,
        })
    }

    /// Creates an unrestricted logical operation requiring at least one
    /// target.
    pub fn at_least_one(
        kind: LogicalOperationKind,
        targets: Vec<QubitId>,
    ) -> LogicalResult<Self> {
        Self::new(
            kind,
            targets,
            LogicalArity::at_least(1),
        )
    }

    /// Creates an operation with no targets.
    ///
    /// This is useful for purely classical/logical markers or operations whose
    /// semantic operands are represented elsewhere.
    pub fn targetless(
        kind: LogicalOperationKind,
    ) -> LogicalResult<Self> {
        Self::new(
            kind,
            Vec::new(),
            LogicalArity::exact(0),
        )
    }

    /// Adds a logical control.
    ///
    /// Controls are kept separate from targets so later lowering passes can
    /// distinguish semantic roles.
    pub fn add_control(
        &mut self,
        control: QubitId,
    ) -> LogicalResult<()> {
        if self.targets.contains(&control)
            || self.controls.contains(&control)
        {
            return Err(LogicalError::DuplicateOperationTarget {
                qubit: control,
            });
        }

        self.controls.push(control);
        Ok(())
    }

    /// Returns the operation kind.
    #[must_use]
    pub fn kind(&self) -> &LogicalOperationKind {
        &self.kind
    }

    /// Returns operation targets in explicit semantic order.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns logical controls in explicit semantic order.
    #[must_use]
    pub fn controls(&self) -> &[QubitId] {
        &self.controls
    }

    /// Returns the operation's target-count contract.
    #[must_use]
    pub const fn arity(&self) -> LogicalArity {
        self.arity
    }

    /// Returns the total number of logical resources directly referenced by
    /// the operation.
    pub fn resource_count(&self) -> LogicalResult<usize> {
        self.targets
            .len()
            .checked_add(self.controls.len())
            .ok_or(LogicalError::ResourceCountOverflow {
                resource: "logical operation resources",
            })
    }

    /// Validates local operation invariants.
    pub fn validate(&self) -> LogicalResult<()> {
        if self.kind.namespace().trim().is_empty() {
            return Err(LogicalError::EmptyNamespace);
        }

        if self.kind.name().trim().is_empty() {
            return Err(LogicalError::EmptyOperationName);
        }

        if !self.arity.accepts(self.targets.len()) {
            return Err(LogicalError::InvalidArity {
                operation: self.kind.qualified_name(),
                minimum: self.arity.minimum(),
                maximum: self.arity.maximum(),
                actual: self.targets.len(),
            });
        }

        Self::validate_unique_targets(&self.targets)?;

        let mut resources = BTreeSet::new();

        for target in &self.targets {
            resources.insert(*target);
        }

        for control in &self.controls {
            if !resources.insert(*control) {
                return Err(LogicalError::DuplicateOperationTarget {
                    qubit: *control,
                });
            }
        }

        Ok(())
    }

    fn validate_unique_targets(
        targets: &[QubitId],
    ) -> LogicalResult<()> {
        let mut seen = BTreeSet::new();

        for target in targets {
            if !seen.insert(*target) {
                return Err(LogicalError::DuplicateOperationTarget {
                    qubit: *target,
                });
            }
        }

        Ok(())
    }
}

// =============================================================================
// Logical measurement
// =============================================================================

/// Measurement basis used by a logical measurement.
///
/// `Named` keeps the model extensible beyond X/Y/Z.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalMeasurementBasis {
    /// Computational/Z basis.
    Z,

    /// X basis.
    X,

    /// Y basis.
    Y,

    /// Named observable/basis.
    Named(String),
}

impl LogicalMeasurementBasis {
    /// Creates a named basis.
    pub fn named(name: impl Into<String>) -> LogicalResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyOperationName);
        }

        Ok(Self::Named(name))
    }
}

/// Logical measurement description.
///
/// The destination is a symbolic classical result name. Classical storage
/// itself belongs to the classical/program IR.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalMeasurement {
    targets: Vec<QubitId>,
    basis: LogicalMeasurementBasis,
    destination: String,
}

impl LogicalMeasurement {
    /// Creates a logical measurement.
    pub fn new(
        targets: Vec<QubitId>,
        basis: LogicalMeasurementBasis,
        destination: impl Into<String>,
    ) -> LogicalResult<Self> {
        if targets.is_empty() {
            return Err(LogicalError::MissingTargets {
                operation: "logical.measure".to_owned(),
            });
        }

        let destination = destination.into();

        if destination.trim().is_empty() {
            return Err(LogicalError::EmptyMeasurementDestination);
        }

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(LogicalError::DuplicateOperationTarget {
                    qubit: *target,
                });
            }
        }

        Ok(Self {
            targets,
            basis,
            destination,
        })
    }

    /// Returns measurement targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }

    /// Returns the measurement basis.
    #[must_use]
    pub fn basis(&self) -> &LogicalMeasurementBasis {
        &self.basis
    }

    /// Returns the classical destination.
    #[must_use]
    pub fn destination(&self) -> &str {
        &self.destination
    }
}

// =============================================================================
// Logical initialization
// =============================================================================

/// Semantic logical initialization kind.
///
/// Arbitrary named states are supported so the IR does not assume that every
/// future logical architecture begins with computational-basis states.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LogicalInitialization {
    /// Computational zero state.
    Zero,

    /// Computational one state.
    One,

    /// Named logical state preparation.
    Named(String),
}

impl LogicalInitialization {
    /// Creates a named initialization.
    pub fn named(name: impl Into<String>) -> LogicalResult<Self> {
        let name = name.into();

        if name.trim().is_empty() {
            return Err(LogicalError::EmptyOperationName);
        }

        Ok(Self::Named(name))
    }
}

// =============================================================================
// Logical reset
// =============================================================================

/// Logical reset operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalReset {
    targets: Vec<QubitId>,
}

impl LogicalReset {
    /// Creates a reset over logical qubits.
    pub fn new(targets: Vec<QubitId>) -> LogicalResult<Self> {
        if targets.is_empty() {
            return Err(LogicalError::MissingTargets {
                operation: "logical.reset".to_owned(),
            });
        }

        let mut seen = BTreeSet::new();

        for target in &targets {
            if !seen.insert(*target) {
                return Err(LogicalError::DuplicateOperationTarget {
                    qubit: *target,
                });
            }
        }

        Ok(Self { targets })
    }

    /// Returns reset targets.
    #[must_use]
    pub fn targets(&self) -> &[QubitId] {
        &self.targets
    }
}

// =============================================================================
// Logical resource requirement
// =============================================================================

/// Declarative resource requirement for logical computation.
///
/// This describes semantic requirements rather than hardware capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicalResourceRequirement {
    minimum_logical_qubits: Option<usize>,
    minimum_logical_depth: Option<usize>,
    requires_fault_tolerance: bool,
    requires_mid_circuit_measurement: bool,
    requires_dynamic_control: bool,
    required_encodings: BTreeSet<LogicalEncoding>,
    required_operations: BTreeSet<LogicalOperationKind>,
}

impl LogicalResourceRequirement {
    /// Creates an empty requirement.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            minimum_logical_qubits: None,
            minimum_logical_depth: None,
            requires_fault_tolerance: false,
            requires_mid_circuit_measurement: false,
            requires_dynamic_control: false,
            required_encodings: BTreeSet::new(),
            required_operations: BTreeSet::new(),
        }
    }

    /// Sets the minimum logical-qubit count.
    pub const fn with_minimum_logical_qubits(
        mut self,
        count: usize,
    ) -> Self {
        self.minimum_logical_qubits = Some(count);
        self
    }

    /// Sets the minimum logical depth.
    pub const fn with_minimum_logical_depth(
        mut self,
        depth: usize,
    ) -> Self {
        self.minimum_logical_depth = Some(depth);
        self
    }

    /// Requires fault-tolerant execution.
    pub const fn requiring_fault_tolerance(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_fault_tolerance = required;
        self
    }

    /// Requires mid-circuit measurement.
    pub const fn requiring_mid_circuit_measurement(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_mid_circuit_measurement = required;
        self
    }

    /// Requires dynamic classical control.
    pub const fn requiring_dynamic_control(
        mut self,
        required: bool,
    ) -> Self {
        self.requires_dynamic_control = required;
        self
    }

    /// Adds a required encoding.
    pub fn require_encoding(
        &mut self,
        encoding: LogicalEncoding,
    ) -> LogicalResult<()> {
        encoding.validate()?;
        self.required_encodings.insert(encoding);
        Ok(())
    }

    /// Adds a required logical operation.
    pub fn require_operation(
        &mut self,
        operation: LogicalOperationKind,
    ) {
        self.required_operations.insert(operation);
    }

    /// Returns the minimum logical-qubit requirement.
    #[must_use]
    pub const fn minimum_logical_qubits(&self) -> Option<usize> {
        self.minimum_logical_qubits
    }

    /// Returns the minimum logical-depth requirement.
    #[must_use]
    pub const fn minimum_logical_depth(&self) -> Option<usize> {
        self.minimum_logical_depth
    }

    /// Returns whether fault tolerance is required.
    #[must_use]
    pub const fn requires_fault_tolerance(&self) -> bool {
        self.requires_fault_tolerance
    }

    /// Returns whether mid-circuit measurement is required.
    #[must_use]
    pub const fn requires_mid_circuit_measurement(&self) -> bool {
        self.requires_mid_circuit_measurement
    }

    /// Returns whether dynamic control is required.
    #[must_use]
    pub const fn requires_dynamic_control(&self) -> bool {
        self.requires_dynamic_control
    }

    /// Returns required encodings.
    #[must_use]
    pub fn required_encodings(&self) -> &BTreeSet<LogicalEncoding> {
        &self.required_encodings
    }

    /// Returns required logical operations.
    #[must_use]
    pub fn required_operations(
        &self,
    ) -> &BTreeSet<LogicalOperationKind> {
        &self.required_operations
    }

    /// Validates the requirement.
    pub fn validate(&self) -> LogicalResult<()> {
        if let Some(count) = self.minimum_logical_qubits {
            if count == 0 {
                return Err(LogicalError::InvalidRequirement {
                    message:
                        "a minimum logical-qubit requirement must be \
                         meaningful when specified",
                });
            }
        }

        for encoding in &self.required_encodings {
            encoding.validate()?;
        }

        Ok(())
    }
}

// =============================================================================
// Logical program
// =============================================================================

/// Canonical logical quantum model.
///
/// This is a semantic container for logical qubits, encodings, requirements,
/// operations, measurements, initializations and resets.
///
/// It intentionally does not own physical mapping or execution.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicalProgram {
    registers: Vec<LogicalRegister>,
    qubits: BTreeMap<QubitId, LogicalQubit>,
    operations: Vec<LogicalOperation>,
    measurements: Vec<LogicalMeasurement>,
    initializations: BTreeMap<QubitId, LogicalInitialization>,
    resets: Vec<LogicalReset>,
    requirement: LogicalResourceRequirement,
}

impl LogicalProgram {
    /// Creates an empty logical program.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all logical registers in declaration order.
    #[must_use]
    pub fn registers(&self) -> &[LogicalRegister] {
        &self.registers
    }

    /// Returns all logical qubits in deterministic ID order.
    #[must_use]
    pub fn qubits(&self) -> &BTreeMap<QubitId, LogicalQubit> {
        &self.qubits
    }

    /// Returns logical operations in semantic program order.
    #[must_use]
    pub fn operations(&self) -> &[LogicalOperation] {
        &self.operations
    }

    /// Returns logical measurements in program order.
    #[must_use]
    pub fn measurements(&self) -> &[LogicalMeasurement] {
        &self.measurements
    }

    /// Returns logical initializations keyed by qubit identity.
    #[must_use]
    pub fn initializations(
        &self,
    ) -> &BTreeMap<QubitId, LogicalInitialization> {
        &self.initializations
    }

    /// Returns logical resets in program order.
    #[must_use]
    pub fn resets(&self) -> &[LogicalReset] {
        &self.resets
    }

    /// Returns logical resource requirements.
    #[must_use]
    pub fn requirement(&self) -> &LogicalResourceRequirement {
        &self.requirement
    }

    /// Returns the number of logical qubits.
    #[must_use]
    pub fn logical_qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of logical operations.
    #[must_use]
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Adds a logical register atomically.
    ///
    /// All qubits are validated before the register is committed.
    pub fn add_register(
        &mut self,
        register: LogicalRegister,
    ) -> LogicalResult<()> {
        register.validate()?;

        for qubit in register.qubits() {
            if self.qubits.contains_key(&qubit.id()) {
                return Err(LogicalError::DuplicateLogicalQubit {
                    qubit: qubit.id(),
                });
            }
        }

        let register_qubits =
            register.qubits().iter().cloned().collect::<Vec<_>>();

        self.registers.push(register);

        for qubit in register_qubits {
            self.qubits.insert(qubit.id(), qubit);
        }

        Ok(())
    }

    /// Adds a single logical qubit.
    pub fn add_qubit(
        &mut self,
        qubit: LogicalQubit,
    ) -> LogicalResult<()> {
        if self.qubits.contains_key(&qubit.id()) {
            return Err(LogicalError::DuplicateLogicalQubit {
                qubit: qubit.id(),
            });
        }

        if let Some(encoding) = qubit.encoding() {
            encoding.validate()?;
        }

        self.qubits.insert(qubit.id(), qubit);
        Ok(())
    }

    /// Adds a logical operation atomically.
    pub fn add_operation(
        &mut self,
        operation: LogicalOperation,
    ) -> LogicalResult<()> {
        operation.validate()?;

        self.validate_operation_resources(&operation)?;

        self.operations.push(operation);

        Ok(())
    }

    /// Adds a logical measurement.
    pub fn add_measurement(
        &mut self,
        measurement: LogicalMeasurement,
    ) -> LogicalResult<()> {
        for qubit in measurement.targets() {
            self.require_usable_qubit(*qubit)?;
        }

        self.measurements.push(measurement);

        Ok(())
    }

    /// Adds a logical initialization.
    pub fn initialize(
        &mut self,
        qubit: QubitId,
        initialization: LogicalInitialization,
    ) -> LogicalResult<()> {
        self.require_usable_qubit(qubit)?;

        self.initializations.insert(qubit, initialization);

        Ok(())
    }

    /// Adds a logical reset.
    pub fn add_reset(
        &mut self,
        reset: LogicalReset,
    ) -> LogicalResult<()> {
        for qubit in reset.targets() {
            self.require_usable_qubit(*qubit)?;
        }

        self.resets.push(reset);

        Ok(())
    }

    /// Replaces the resource requirements.
    pub fn set_requirement(
        &mut self,
        requirement: LogicalResourceRequirement,
    ) -> LogicalResult<()> {
        requirement.validate()?;
        self.requirement = requirement;
        Ok(())
    }

    /// Returns a logical qubit by ID.
    #[must_use]
    pub fn qubit(
        &self,
        id: QubitId,
    ) -> Option<&LogicalQubit> {
        self.qubits.get(&id)
    }

    /// Returns a mutable logical qubit by ID.
    pub fn qubit_mut(
        &mut self,
        id: QubitId,
    ) -> Option<&mut LogicalQubit> {
        self.qubits.get_mut(&id)
    }

    /// Returns whether a logical qubit exists.
    #[must_use]
    pub fn contains_qubit(&self, id: QubitId) -> bool {
        self.qubits.contains_key(&id)
    }

    /// Performs complete local validation.
    pub fn validate(&self) -> LogicalResult<()> {
        self.requirement.validate()?;

        let mut register_ids = BTreeSet::new();

        for register in &self.registers {
            register.validate()?;

            for qubit in register.qubits() {
                if !register_ids.insert(qubit.id()) {
                    return Err(LogicalError::DuplicateLogicalQubit {
                        qubit: qubit.id(),
                    });
                }
            }
        }

        for (id, qubit) in &self.qubits {
            if *id != qubit.id() {
                return Err(LogicalError::InvalidRequirement {
                    message:
                        "logical-qubit map key does not match logical-qubit \
                         identity",
                });
            }

            if let Some(encoding) = qubit.encoding() {
                encoding.validate()?;
            }
        }

        for operation in &self.operations {
            operation.validate()?;
            self.validate_operation_resources(operation)?;
        }

        for measurement in &self.measurements {
            for qubit in measurement.targets() {
                self.require_usable_qubit(*qubit)?;
            }
        }

        for qubit in self.initializations.keys() {
            self.require_usable_qubit(*qubit)?;
        }

        for reset in &self.resets {
            for qubit in reset.targets() {
                self.require_usable_qubit(*qubit)?;
            }
        }

        Ok(())
    }

    fn validate_operation_resources(
        &self,
        operation: &LogicalOperation,
    ) -> LogicalResult<()> {
        for qubit in operation.targets() {
            self.require_usable_qubit(*qubit)?;
        }

        for qubit in operation.controls() {
            self.require_usable_qubit(*qubit)?;
        }

        Ok(())
    }

    fn require_usable_qubit(
        &self,
        qubit: QubitId,
    ) -> LogicalResult<()> {
        match self.qubits.get(&qubit) {
            Some(resource) if resource.is_usable() => Ok(()),

            Some(_) => Err(LogicalError::DisabledLogicalQubit { qubit }),

            None => Err(LogicalError::UnknownLogicalQubit { qubit }),
        }
    }
}

// =============================================================================
// Logical model statistics
// =============================================================================

/// Read-only deterministic statistics for a logical program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LogicalStatistics {
    logical_qubits: usize,
    encoded_logical_qubits: usize,
    operations: usize,
    measurements: usize,
    resets: usize,
    initializations: usize,
    distinct_operation_kinds: usize,
}

impl LogicalStatistics {
    /// Computes statistics without modifying the program.
    pub fn from_program(
        program: &LogicalProgram,
    ) -> LogicalResult<Self> {
        let encoded_logical_qubits =
            program
                .qubits()
                .values()
                .filter(|qubit| qubit.is_encoded())
                .count();

        let mut kinds = BTreeSet::new();

        for operation in program.operations() {
            kinds.insert(operation.kind().clone());
        }

        Ok(Self {
            logical_qubits: program.logical_qubit_count(),
            encoded_logical_qubits,
            operations: program.operation_count(),
            measurements: program.measurements().len(),
            resets: program.resets().len(),
            initializations: program.initializations().len(),
            distinct_operation_kinds: kinds.len(),
        })
    }

    /// Returns logical-qubit count.
    #[must_use]
    pub const fn logical_qubits(self) -> usize {
        self.logical_qubits
    }

    /// Returns encoded logical-qubit count.
    #[must_use]
    pub const fn encoded_logical_qubits(self) -> usize {
        self.encoded_logical_qubits
    }

    /// Returns operation count.
    #[must_use]
    pub const fn operations(self) -> usize {
        self.operations
    }

    /// Returns measurement count.
    #[must_use]
    pub const fn measurements(self) -> usize {
        self.measurements
    }

    /// Returns reset count.
    #[must_use]
    pub const fn resets(self) -> usize {
        self.resets
    }

    /// Returns initialization count.
    #[must_use]
    pub const fn initializations(self) -> usize {
        self.initializations
    }

    /// Returns distinct logical-operation-kind count.
    #[must_use]
    pub const fn distinct_operation_kinds(self) -> usize {
        self.distinct_operation_kinds
    }
}

// =============================================================================
// Standard logical operation constructors
// =============================================================================

/// Standard logical operation helpers.
///
/// These helpers are convenience constructors only. They do not define the
/// complete logical operation universe.
pub mod standard {
    use super::{
        LogicalArity,
        LogicalOperation,
        LogicalOperationKind,
        LogicalResult,
    };
    use crate::quantum::ir::qubit::QubitId;

    fn single(
        name: &'static str,
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        LogicalOperation::new(
            LogicalOperationKind::new("logical", name)?,
            vec![target],
            LogicalArity::exact(1),
        )
    }

    fn pair(
        name: &'static str,
        control: QubitId,
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        let mut operation = LogicalOperation::new(
            LogicalOperationKind::new("logical", name)?,
            vec![target],
            LogicalArity::exact(1),
        )?;

        operation.add_control(control)?;

        Ok(operation)
    }

    /// Logical X.
    pub fn x(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("x", target)
    }

    /// Logical Y.
    pub fn y(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("y", target)
    }

    /// Logical Z.
    pub fn z(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("z", target)
    }

    /// Logical H.
    pub fn h(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("h", target)
    }

    /// Logical S.
    pub fn s(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("s", target)
    }

    /// Logical T.
    pub fn t(
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        single("t", target)
    }

    /// Logical controlled-X.
    pub fn cx(
        control: QubitId,
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        pair("cx", control, target)
    }

    /// Logical controlled-Z.
    pub fn cz(
        control: QubitId,
        target: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        pair("cz", control, target)
    }

    /// Logical SWAP.
    ///
    /// SWAP is represented as a two-target logical operation rather than
    /// being decomposed into physical gates.
    pub fn swap(
        first: QubitId,
        second: QubitId,
    ) -> LogicalResult<LogicalOperation> {
        LogicalOperation::new(
            LogicalOperationKind::new("logical", "swap")?,
            vec![first, second],
            LogicalArity::exact(2),
        )
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uses_canonical_qubit_identity() {
        let id = QubitId::new(7);
        let qubit = LogicalQubit::new(id);

        assert_eq!(qubit.id(), id);
    }

    #[test]
    fn encoding_is_extensible() {
        let mut encoding =
            LogicalEncoding::new(
                "quantum.error_correction",
                "surface",
            )
            .expect("encoding must be valid");

        encoding
            .set_parameter("distance", "7")
            .expect("parameter must be valid");

        encoding
            .set_parameter("basis", "rotated")
            .expect("parameter must be valid");

        assert_eq!(encoding.parameter("distance"), Some("7"));
        assert_eq!(encoding.parameter("basis"), Some("rotated"));
    }

    #[test]
    fn duplicate_encoding_parameter_is_rejected() {
        let parameter =
            EncodingParameter::new("distance", "7")
                .expect("parameter must be valid");

        let mut encoding =
            LogicalEncoding::new(
                "quantum.error_correction",
                "surface",
            )
            .expect("encoding must be valid");

        encoding
            .add_parameter(parameter.clone())
            .expect("first parameter must succeed");

        assert_eq!(
            encoding.add_parameter(parameter),
            Err(LogicalError::DuplicateParameter {
                name: "distance".to_owned(),
            })
        );
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let id = QubitId::new(0);

        let mut register =
            LogicalRegister::new("q")
                .expect("register must be valid");

        register
            .insert(LogicalQubit::new(id))
            .expect("first insertion must succeed");

        assert_eq!(
            register.insert(LogicalQubit::new(id)),
            Err(LogicalError::DuplicateLogicalQubit {
                qubit: id,
            })
        );
    }

    #[test]
    fn operation_rejects_duplicate_targets() {
        let id = QubitId::new(0);

        let kind =
            LogicalOperationKind::new("logical", "test")
                .expect("operation kind must be valid");

        assert_eq!(
            LogicalOperation::new(
                kind,
                vec![id, id],
                LogicalArity::exact(2),
            ),
            Err(LogicalError::DuplicateOperationTarget {
                qubit: id,
            })
        );
    }

    #[test]
    fn operation_accepts_arbitrary_target_counts() {
        let targets = vec![
            QubitId::new(0),
            QubitId::new(1),
            QubitId::new(2),
            QubitId::new(3),
            QubitId::new(4),
        ];

        let kind =
            LogicalOperationKind::new(
                "future.logical",
                "global_operation",
            )
            .expect("operation kind must be valid");

        let operation =
            LogicalOperation::new(
                kind,
                targets,
                LogicalArity::at_least(1),
            )
            .expect("operation must be valid");

        assert_eq!(operation.targets().len(), 5);
    }

    #[test]
    fn program_rejects_unknown_qubits() {
        let mut program = LogicalProgram::new();

        program
            .add_qubit(LogicalQubit::new(QubitId::new(0)))
            .expect("qubit must be added");

        let operation =
            standard::x(QubitId::new(1))
                .expect("operation construction must succeed");

        assert_eq!(
            program.add_operation(operation),
            Err(LogicalError::UnknownLogicalQubit {
                qubit: QubitId::new(1),
            })
        );
    }

    #[test]
    fn standard_operations_remain_extensible() {
        let control = QubitId::new(0);
        let target = QubitId::new(1);

        let operation =
            standard::cx(control, target)
                .expect("CX must be constructible");

        assert_eq!(
            operation.kind().qualified_name(),
            "logical::cx"
        );

        assert_eq!(operation.controls(), &[control]);
        assert_eq!(operation.targets(), &[target]);
    }

    #[test]
    fn program_validates() {
        let mut program = LogicalProgram::new();

        program
            .add_qubit(Qubit::new(QubitId::new(0)))
            .expect("qubit must be added");

        program
            .add_qubit(Qubit::new(QubitId::new(1)))
            .expect("qubit must be added");

        program
            .add_operation(
                standard::h(QubitId::new(0))
                    .expect("H must be constructible"),
            )
            .expect("H must be added");

        program
            .add_operation(
                standard::cx(
                    QubitId::new(0),
                    QubitId::new(1),
                )
                .expect("CX must be constructible"),
            )
            .expect("CX must be added");

        assert!(program.validate().is_ok());
    }

    #[test]
    fn statistics_are_deterministic() {
        let mut program = LogicalProgram::new();

        program
            .add_qubit(Qubit::new(QubitId::new(0)))
            .expect("qubit must be added");

        program
            .add_qubit(Qubit::new(QubitId::new(1)))
            .expect("qubit must be added");

        program
            .add_operation(
                standard::h(QubitId::new(0))
                    .expect("H must be constructible"),
            )
            .expect("H must be added");

        program
            .add_operation(
                standard::cx(
                    QubitId::new(0),
                    QubitId::new(1),
                )
                .expect("CX must be constructible"),
            )
            .expect("CX must be added");

        let statistics =
            LogicalStatistics::from_program(&program)
                .expect("statistics must be computable");

        assert_eq!(statistics.logical_qubits(), 2);
        assert_eq!(statistics.operations(), 2);
        assert_eq!(statistics.distinct_operation_kinds(), 2);
    }

    #[test]
    fn no_fixed_machine_size_is_encoded() {
        let large_id = QubitId::new(usize::MAX);

        let qubit = LogicalQubit::new(large_id);

        assert_eq!(qubit.id(), large_id);
    }

    #[test]
    fn logical_encoding_does_not_allocate_physical_qubits() {
        let encoding =
            LogicalEncoding::new(
                "quantum.error_correction",
                "future_code",
            )
            .expect("encoding must be valid");

        let qubit =
            LogicalQubit::encoded(
                QubitId::new(1000),
                encoding,
            )
            .expect("encoded qubit must be valid");

        assert!(qubit.is_encoded());
        assert_eq!(qubit.id(), QubitId::new(1000));
    }
}