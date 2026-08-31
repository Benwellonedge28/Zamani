//! Zamani Quantum IR — Program Results
//!
//! Canonical representation of values/results produced by operations in the
//! Zamani Quantum IR program layer.
//!
//! # Architectural role
//!
//! This module defines WHAT an operation produces at the program/IR level.
//!
//! It does NOT define:
//!
//! - simulator state;
//! - measurement sampling implementation;
//! - backend execution results;
//! - hardware result packets;
//! - network transport;
//! - optimization results;
//! - routing results;
//! - scheduling results;
//! - frontend AST values;
//! - host-language runtime values.
//!
//! The architectural boundary is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! canonical Quantum IR
//!      |
//!      +--> Operation
//!      |       |
//!      |       +--> ResultBinding
//!      |               |
//!      |               +--> ValueId
//!      |               +--> result type/category
//!      |               +--> producer OperationId
//!      |               +--> logical resource information
//!      |
//!      v
//! optimization / routing / scheduling / lowering
//!      |
//!      v
//! backend
//!      |
//!      v
//! execution result
//! ```
//!
//! # Why this file exists
//!
//! An operation and the value it produces are different semantic concepts.
//!
//! For example:
//!
//! ```text
//! measure q[0] -> c[0]
//! ```
//!
//! has:
//!
//! - an operation identity;
//! - a logical qubit operand;
//! - a produced IR value;
//! - a classical destination;
//! - a relationship between the producer and the result.
//!
//! The result itself must therefore have a canonical representation rather
//! than being encoded implicitly in operation-specific code.
//!
//! # Important distinction: IR result vs execution result
//!
//! This module represents an IR-level result declaration/binding.
//!
//! It does NOT represent a concrete sampled result such as:
//!
//! ```text
//! 0
//! 1
//! 011010
//! probability distribution
//! expectation value
//! waveform samples
//! ```
//!
//! Concrete execution data belongs to the execution/backend/result layers.
//!
//! This distinction is essential for the universal Zamani architecture:
//!
//! ```text
//! IR result
//!     = what a program produces
//!
//! execution result
//!     = what a particular execution produced
//! ```
//!
//! # Universal-program principle
//!
//! The same result model must work for:
//!
//! - one qubit;
//! - thousands of qubits;
//! - millions of qubits;
//! - distributed quantum machines;
//! - logical/fault-tolerant machines;
//! - pulse-level systems;
//! - analog systems;
//! - annealing systems;
//! - continuous-variable systems;
//! - future quantum architectures.
//!
//! There is deliberately no fixed result count, qubit count, register size,
//! operation count, machine size, or result width in this module.
//!
//! Concrete memory/security limits belong to explicit IR policy such as
//! `QuantumIrLimits`.
//!
//! "Infinity" is therefore represented correctly:
//!
//! ```text
//! no architectural ceiling
//! +
//! finite concrete programs
//! +
//! explicit resource limits
//! ```
//!
//! # Canonical identity rule
//!
//! Results use the canonical identity types from `identity.rs`.
//!
//! In particular:
//!
//! - `OperationId` identifies the producer;
//! - `ValueId` identifies the produced IR value;
//! - `QubitId` identifies a logical qubit;
//! - `ClassicalBitId` identifies a classical storage destination.
//!
//! The canonical logical qubit type is:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! This module must never use the historical `quantum::ir::qubits::QubitId`
//! spelling.
//!
//! # Single-source-of-truth rule
//!
//! This module does not redefine:
//!
//! - `OperationId`;
//! - `ValueId`;
//! - `QubitId`;
//! - `ClassicalBitId`;
//! - operation semantics;
//! - gate semantics;
//! - measurement semantics;
//! - classical expression semantics.
//!
//! Those remain owned by their canonical modules.
//!
//! # Integration contract
//!
//! `identity.rs`
//!     Owns stable `OperationId` and `ValueId`.
//!
//! `qubit.rs`
//!     Owns canonical `QubitId`.
//!
//! `classical.rs`
//!     Owns canonical `ClassicalBitId`.
//!
//! `operation.rs`
//!     Owns the operation that produces a result.
//!
//! `program.rs`
//!     Owns the program containing operations and result bindings.
//!
//! `region.rs` / `block.rs`
//!     Own structured program control flow.
//!
//! `types.rs`
//!     Owns canonical IR type vocabulary where available.
//!
//! `value.rs`
//!     Owns semantic value representation where available.
//!
//! `measurement.rs`
//!     Owns measurement semantics.
//!
//! `validation.rs`
//!     Performs whole-program producer/destination/reference validation.
//!
//! `analysis.rs`
//!     Reads result definitions for dependency, liveness, and resource
//!     analysis.
//!
//! `serialization.rs`
//!     Serializes this representation through the canonical IR schema.
//!
//! `hash.rs`
//!     Includes semantic result information in canonical hashing.
//!
//! `provenance.rs`
//!     Tracks result lineage when transformations create or replace results.
//!
//! `optimization/`
//!     May replace result bindings as part of a valid transformation but must
//!     not redefine this result model.
//!
//! `routing/`
//!     Must not alter result identity merely because logical qubits are mapped.
//!
//! `scheduling/`
//!     Must not alter semantic result identity.
//!
//! `hardware/`
//!     Converts semantic results into target-specific execution/result forms.
//!
//! `backend/`
//!     Owns concrete execution results, not this IR representation.
//!
//! # Error ownership
//!
//! `ResultError` contains errors that can be detected locally by this module.
//!
//! Whole-program errors such as:
//!
//! - producer does not exist;
//! - destination was never declared;
//! - value is used before definition;
//! - result type conflicts with a consumer;
//! - result is produced in an unreachable block;
//!
//! belong to whole-program validation.
//!
//! # Determinism
//!
//! Result collections preserve explicit semantic order.
//!
//! `Vec<ResultBinding>` is used where result order is meaningful.
//!
//! A result's position in a collection is NOT its identity.
//!
//! `ValueId` remains stable when result ordering changes.
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
//! - no unsafe code.
//!
//! This file deliberately uses only the Rust standard library and canonical
//! Zamani IR modules.
//!
//! `#![forbid(unsafe_code)]` makes the no-unsafe requirement compiler-enforced.
//!
//! # Ownership contract
//!
//! OWNS:
//!
//! - result categories;
//! - result bindings;
//! - producer/result relationships;
//! - optional logical-qubit association;
//! - optional classical destination association;
//! - deterministic result collections;
//! - local result invariants.
//!
//! DOES NOT OWN:
//!
//! - operation semantics;
//! - concrete execution data;
//! - measurement sampling;
//! - hardware result packets;
//! - backend result decoding;
//! - simulator state;
//! - routing;
//! - scheduling;
//! - optimization;
//! - QEC decoding.
//!
//! # Stability contract
//!
//! Once this module is integrated, downstream modules should consume these
//! types rather than inventing local result representations.
//!
//! The following must therefore remain one semantic contract:
//!
//! ```text
//! quantum::ir::program::result::ResultBinding
//! ```
//!
//! No optimization, routing, scheduler, simulator, hardware backend, or
//! frontend should introduce a competing result-binding type.
//!
//! # Scaling contract
//!
//! No fixed limits are embedded in this file.
//!
//! In particular, this file does NOT define:
//!
//! - maximum results;
//! - maximum result width;
//! - maximum qubits;
//! - maximum classical bits;
//! - maximum operations;
//! - maximum program size.
//!
//! `usize` is used only for host collection indexing/lengths.
//!
//! Semantic identities use canonical strongly typed IR identities.
//!
//! # Memory model
//!
//! This module uses owned Rust values and ordinary collections.
//!
//! It does not use:
//!
//! - unsafe pointers;
//! - global mutable state;
//! - reference-counted global registries;
//! - hidden allocators;
//! - platform-sized semantic identities.
//!
//! Large programs can therefore be partitioned by higher-level program/region
//! infrastructure without changing this semantic result contract.
//!
//! # Transformation rule
//!
//! If an optimization pass replaces:
//!
//! ```text
//! operation A -> result R
//! ```
//!
//! with:
//!
//! ```text
//! operation B -> result R'
//! ```
//!
//! the pass must explicitly update the producer/result relationship and
//! preserve semantic uses.
//!
//! It must not silently mutate `ResultBinding` internals or reinterpret an
//! existing `ValueId` as another semantic value.
//!
//! # Serialization rule
//!
//! Serialization belongs to `serialization.rs`.
//!
//! This module therefore does not define a second serialization format.
//!
//! The canonical serializer must preserve:
//!
//! - result identity;
//! - producer identity;
//! - result category;
//! - destination;
//! - logical-qubit association;
//! - semantic type information;
//! - explicit ordering.
//!
//! Unknown future result extensions must be handled according to the canonical
//! IR extension/compatibility policy rather than silently discarded.
//!
//! # Hashing rule
//!
//! Canonical hashing must include semantic result information.
//!
//! It must include at least:
//!
//! - result identity;
//! - producer identity;
//! - result category;
//! - destination;
//! - logical resource association;
//! - type/category information;
//! - extension metadata where semantically relevant.
//!
//! It must exclude nondeterministic implementation details such as:
//!
//! - memory addresses;
//! - allocator state;
//! - process IDs;
//! - host pointer values;
//! - unordered map iteration order.
//!
//! # Thread-safety
//!
//! This module contains no global mutable state.
//!
//! It introduces no custom `Send` or `Sync` implementations.
//!
//! Normal Rust ownership rules determine thread safety.
//!
//! # No hardware assumptions
//!
//! This module must not contain:
//!
//! - vendor identifiers;
//! - physical qubit numbers;
//! - backend names;
//! - DAC addresses;
//! - network endpoints;
//! - device handles;
//! - simulator state;
//! - execution job IDs.
//!
//! A semantic result must remain meaningful before hardware selection.
//!
//! # Dynamic circuits
//!
//! Dynamic quantum computation commonly has:
//!
//! ```text
//! measurement
//!     |
//!     v
//! result value
//!     |
//!     v
//! classical condition
//!     |
//!     v
//! later operation
//! ```
//!
//! `ResultBinding` represents the result node in this dependency chain.
//!
//! The actual control-flow semantics remain owned by the control/program
//! layers.
//!
//! # Pulse and analog integration
//!
//! Results are not restricted to Boolean measurements.
//!
//! The result category system therefore supports semantic classes for:
//!
//! - classical bit;
//! - integer;
//! - floating point;
//! - angle;
//! - complex value;
//! - bit vector;
//! - measurement;
//! - waveform/capture data;
//! - analog value;
//! - logical/QEC data;
//! - opaque extension-defined values.
//!
//! Concrete representation remains target-independent.
//!
//! # Future architectures
//!
//! A new quantum architecture must be able to introduce a new result category
//! or extension without modifying the fundamental producer/result relationship.
//!
//! This is why `ResultKind::Extension` exists.
//!
//! Future architecture-specific semantics should normally be introduced as a
//! dialect/extension rather than by repeatedly expanding the core result
//! representation.
//!
//! # Example
//!
//! A measurement:
//!
//! ```text
//! measure q[0] -> c[0]
//! ```
//!
//! can be represented conceptually as:
//!
//! ```text
//! ResultBinding {
//!     value: ValueId(...),
//!     producer: OperationId(...),
//!     kind: ResultKind::MeasurementBit,
//!     qubit: Some(QubitId(...)),
//!     classical_destination: Some(ClassicalBitId(...)),
//! }
//! ```
//!
//! This does not contain the sampled `0` or `1`.
//!
//! # Example: pulse capture
//!
//! A pulse capture operation can produce:
//!
//! ```text
//! ResultKind::CaptureSamples
//! ```
//!
//! without embedding a concrete DAC/sample-buffer implementation.
//!
//! # Example: logical measurement
//!
//! A fault-tolerant operation may produce:
//!
//! ```text
//! ResultKind::LogicalMeasurement
//! ```
//!
//! while physical syndrome decoding remains outside this module.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use std::fmt;

use super::classical::ClassicalBitId;
use super::identity::{OperationId, ValueId};
use super::qubit::QubitId;

// =============================================================================
// Result error
// =============================================================================

/// Errors detectable while constructing one result binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultError {
    /// A result must have a producer operation.
    MissingProducer,

    /// A result must have a stable IR value identity.
    MissingValueId,

    /// The result cannot be associated with more than one semantic
    /// destination of the same kind.
    ConflictingDestination,

    /// A result kind requires a logical qubit association.
    MissingQubit,

    /// A result kind requires a classical destination.
    MissingClassicalDestination,

    /// A result kind cannot have a classical destination.
    UnexpectedClassicalDestination,

    /// A result kind cannot have a logical qubit association.
    UnexpectedQubit,

    /// A result cannot be its own producer in an invalid self-referential
    /// configuration.
    InvalidSelfReference,

    /// An explicitly supplied result category is not structurally valid.
    InvalidKind,

    /// A result collection contains the same `ValueId` more than once.
    DuplicateValue {
        value: ValueId,
    },

    /// A result collection contains two bindings for the same producer and
    /// destination where uniqueness is required.
    DuplicateBinding {
        producer: OperationId,
    },
}

impl fmt::Display for ResultError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingProducer => {
                formatter.write_str("result binding requires a producer operation")
            }

            Self::MissingValueId => {
                formatter.write_str("result binding requires a ValueId")
            }

            Self::ConflictingDestination => {
                formatter.write_str(
                    "result binding contains conflicting semantic destinations",
                )
            }

            Self::MissingQubit => {
                formatter.write_str(
                    "this result kind requires a logical qubit association",
                )
            }

            Self::MissingClassicalDestination => {
                formatter.write_str(
                    "this result kind requires a classical destination",
                )
            }

            Self::UnexpectedClassicalDestination => {
                formatter.write_str(
                    "this result kind cannot have a classical destination",
                )
            }

            Self::UnexpectedQubit => {
                formatter.write_str(
                    "this result kind cannot have a logical qubit association",
                )
            }

            Self::InvalidSelfReference => {
                formatter.write_str(
                    "result producer cannot be an invalid self-reference",
                )
            }

            Self::InvalidKind => {
                formatter.write_str("result kind is structurally invalid")
            }

            Self::DuplicateValue { value } => {
                write!(
                    formatter,
                    "result value {value} occurs more than once"
                )
            }

            Self::DuplicateBinding { producer } => {
                write!(
                    formatter,
                    "duplicate result binding for producer {producer}"
                )
            }
        }
    }
}

impl std::error::Error for ResultError {}

// =============================================================================
// Result kind
// =============================================================================

/// Semantic category of a value produced by a quantum IR operation.
///
/// This is deliberately a category rather than a complete closed type system.
/// Detailed type semantics belong to the canonical IR type/value layers.
///
/// The `Extension` variant provides a future-proof escape hatch without
/// requiring every future quantum architecture to modify the core result
/// representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResultKind {
    /// A single classical Boolean/bit result.
    Bit,

    /// A classical integer result.
    Integer,

    /// A classical floating-point result.
    Float,

    /// An exact or symbolic angle result.
    Angle,

    /// A complex-valued result.
    Complex,

    /// A classical bit-vector result.
    BitVector,

    /// A measurement result associated with a quantum system.
    Measurement,

    /// A computational-basis measurement bit.
    MeasurementBit,

    /// A generalized/observable measurement result.
    MeasurementValue,

    /// A probability or expectation-like scalar.
    Probability,

    /// An expectation value.
    Expectation,

    /// Raw or processed pulse-capture samples.
    CaptureSamples,

    /// A pulse-capture scalar or integrated measurement.
    CaptureValue,

    /// An analog-program result.
    Analog,

    /// An annealing/Ising/QUBO result.
    Annealing,

    /// A logical/fault-tolerant quantum result.
    Logical,

    /// A syndrome/diagnostic result associated with QEC.
    Syndrome,

    /// A state/resource handle whose complete representation is owned by
    /// another IR layer.
    Resource,

    /// A result defined by a registered extension/dialect.
    Extension,
}

impl ResultKind {
    /// Returns whether this result represents classical information.
    #[must_use]
    pub const fn is_classical(self) -> bool {
        matches!(
            self,
            Self::Bit
                | Self::Integer
                | Self::Float
                | Self::Angle
                | Self::Complex
                | Self::BitVector
                | Self::Measurement
                | Self::MeasurementBit
                | Self::MeasurementValue
                | Self::Probability
                | Self::Expectation
                | Self::CaptureValue
                | Self::Syndrome
        )
    }

    /// Returns whether this result normally originates from a quantum
    /// measurement or quantum-state observation.
    #[must_use]
    pub const fn is_measurement_related(self) -> bool {
        matches!(
            self,
            Self::Measurement
                | Self::MeasurementBit
                | Self::MeasurementValue
                | Self::Probability
                | Self::Expectation
                | Self::CaptureSamples
                | Self::CaptureValue
        )
    }

    /// Returns whether the result is naturally associated with a logical
    /// qubit or quantum resource.
    #[must_use]
    pub const fn is_quantum_related(self) -> bool {
        matches!(
            self,
            Self::Measurement
                | Self::MeasurementBit
                | Self::MeasurementValue
                | Self::Logical
                | Self::Syndrome
                | Self::Resource
        )
    }

    /// Returns whether this result is naturally a classical storage value.
    #[must_use]
    pub const fn is_storable_classically(self) -> bool {
        self.is_classical()
    }

    /// Returns whether this result may require an extension-specific type
    /// interpretation.
    #[must_use]
    pub const fn is_extension_defined(self) -> bool {
        matches!(self, Self::Extension)
    }
}

impl fmt::Display for ResultKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Bit => "bit",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::Angle => "angle",
            Self::Complex => "complex",
            Self::BitVector => "bit-vector",
            Self::Measurement => "measurement",
            Self::MeasurementBit => "measurement-bit",
            Self::MeasurementValue => "measurement-value",
            Self::Probability => "probability",
            Self::Expectation => "expectation",
            Self::CaptureSamples => "capture-samples",
            Self::CaptureValue => "capture-value",
            Self::Analog => "analog",
            Self::Annealing => "annealing",
            Self::Logical => "logical",
            Self::Syndrome => "syndrome",
            Self::Resource => "resource",
            Self::Extension => "extension",
        };

        formatter.write_str(name)
    }
}

// =============================================================================
// Result binding
// =============================================================================

/// Canonical semantic binding between an operation and one value it produces.
///
/// A `ResultBinding` identifies the relationship:
///
/// ```text
/// OperationId
///      |
///      v
/// ResultBinding
///      |
///      +--> ValueId
///      +--> ResultKind
///      +--> optional logical qubit
///      +--> optional classical destination
/// ```
///
/// The binding is semantic metadata. It does not contain a concrete runtime
/// value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResultBinding {
    /// Stable identity of the produced IR value.
    value: ValueId,

    /// Stable identity of the operation producing the value.
    producer: OperationId,

    /// Semantic category of the produced result.
    kind: ResultKind,

    /// Optional logical quantum resource associated with the result.
    ///
    /// This always uses the canonical `quantum::ir::qubit::QubitId`.
    qubit: Option<QubitId>,

    /// Optional classical destination.
    ///
    /// This is useful for dynamic-circuit constructs such as:
    ///
    /// ```text
    /// measure q[0] -> c[0]
    /// ```
    classical_destination: Option<ClassicalBitId>,
}

impl ResultBinding {
    /// Creates a result binding.
    ///
    /// Local structural validation is performed before the value is returned.
    pub fn new(
        value: ValueId,
        producer: OperationId,
        kind: ResultKind,
    ) -> Result<Self, ResultError> {
        let binding = Self {
            value,
            producer,
            kind,
            qubit: None,
            classical_destination: None,
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Creates a result associated with a logical qubit.
    ///
    /// This is useful for measurement and logical quantum results.
    pub fn for_qubit(
        value: ValueId,
        producer: OperationId,
        kind: ResultKind,
        qubit: QubitId,
    ) -> Result<Self, ResultError> {
        let binding = Self {
            value,
            producer,
            kind,
            qubit: Some(qubit),
            classical_destination: None,
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Creates a result written into a classical bit.
    ///
    /// This is the canonical representation for a result that is explicitly
    /// stored in a classical destination.
    pub fn to_classical(
        value: ValueId,
        producer: OperationId,
        kind: ResultKind,
        destination: ClassicalBitId,
    ) -> Result<Self, ResultError> {
        let binding = Self {
            value,
            producer,
            kind,
            qubit: None,
            classical_destination: Some(destination),
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Creates a measurement result associated with both a logical qubit and a
    /// classical destination.
    ///
    /// This represents the semantic relationship:
    ///
    /// ```text
    /// quantum resource
    ///       |
    ///       v
    /// measurement
    ///       |
    ///       v
    /// result value
    ///       |
    ///       v
    /// classical destination
    /// ```
    pub fn measurement(
        value: ValueId,
        producer: OperationId,
        qubit: QubitId,
        destination: ClassicalBitId,
    ) -> Result<Self, ResultError> {
        let binding = Self {
            value,
            producer,
            kind: ResultKind::MeasurementBit,
            qubit: Some(qubit),
            classical_destination: Some(destination),
        };

        binding.validate()?;

        Ok(binding)
    }

    /// Returns the produced IR value identity.
    #[must_use]
    pub const fn value(&self) -> ValueId {
        self.value
    }

    /// Returns the producer operation identity.
    #[must_use]
    pub const fn producer(&self) -> OperationId {
        self.producer
    }

    /// Returns the semantic result category.
    #[must_use]
    pub const fn kind(&self) -> ResultKind {
        self.kind
    }

    /// Returns the associated logical qubit, when one exists.
    #[must_use]
    pub const fn qubit(&self) -> Option<QubitId> {
        self.qubit
    }

    /// Returns the classical destination, when one exists.
    #[must_use]
    pub const fn classical_destination(
        &self,
    ) -> Option<ClassicalBitId> {
        self.classical_destination
    }

    /// Returns whether this result has a logical-qubit association.
    #[must_use]
    pub const fn has_qubit(&self) -> bool {
        self.qubit.is_some()
    }

    /// Returns whether this result has a classical destination.
    #[must_use]
    pub const fn has_classical_destination(&self) -> bool {
        self.classical_destination.is_some()
    }

    /// Returns whether this is measurement-related.
    #[must_use]
    pub const fn is_measurement(&self) -> bool {
        self.kind.is_measurement_related()
    }

    /// Returns whether this is classically storable.
    #[must_use]
    pub const fn is_classical(&self) -> bool {
        self.kind.is_storable_classically()
    }

    /// Validates local invariants.
    ///
    /// This does not validate whether the producer operation actually exists
    /// in a particular `QuantumProgram`. That is a whole-program validation
    /// responsibility.
    pub fn validate(&self) -> Result<(), ResultError> {
        match self.kind {
            ResultKind::Measurement
            | ResultKind::MeasurementBit
            | ResultKind::MeasurementValue
            | ResultKind::Logical
            | ResultKind::Syndrome => {
                if self.qubit.is_none() {
                    return Err(ResultError::MissingQubit);
                }
            }

            _ => {}
        }

        if matches!(
            self.kind,
            ResultKind::Bit
                | ResultKind::Integer
                | ResultKind::Float
                | ResultKind::Angle
                | ResultKind::Complex
                | ResultKind::BitVector
                | ResultKind::Probability
                | ResultKind::Expectation
                | ResultKind::CaptureValue
                | ResultKind::Analog
                | ResultKind::Annealing
        ) && self.qubit.is_some()
        {
            return Err(ResultError::UnexpectedQubit);
        }

        if matches!(
            self.kind,
            ResultKind::MeasurementBit
                | ResultKind::Bit
                | ResultKind::Integer
                | ResultKind::Float
                | ResultKind::Angle
                | ResultKind::Complex
                | ResultKind::BitVector
        ) && self.classical_destination.is_none()
        {
            return Err(ResultError::MissingClassicalDestination);
        }

        if matches!(
            self.kind,
            ResultKind::Measurement
                | ResultKind::MeasurementValue
                | ResultKind::Probability
                | ResultKind::Expectation
                | ResultKind::CaptureSamples
                | ResultKind::CaptureValue
                | ResultKind::Analog
                | ResultKind::Annealing
                | ResultKind::Logical
                | ResultKind::Syndrome
                | ResultKind::Resource
                | ResultKind::Extension
        ) && matches!(
            self.classical_destination,
            Some(_)
        ) && !matches!(
            self.kind,
            ResultKind::Measurement
                | ResultKind::MeasurementValue
                | ResultKind::Logical
                | ResultKind::Syndrome
        )
        {
            return Err(ResultError::UnexpectedClassicalDestination);
        }

        Ok(())
    }

    /// Returns a copy of this result with a different logical-qubit
    /// association.
    ///
    /// The producer and value identities remain unchanged.
    pub fn with_qubit(
        mut self,
        qubit: QubitId,
    ) -> Result<Self, ResultError> {
        self.qubit = Some(qubit);
        self.validate()?;
        Ok(self)
    }

    /// Returns a copy of this result with a classical destination.
    ///
    /// The producer and value identities remain unchanged.
    pub fn with_classical_destination(
        mut self,
        destination: ClassicalBitId,
    ) -> Result<Self, ResultError> {
        self.classical_destination = Some(destination);
        self.validate()?;
        Ok(self)
    }

    /// Removes the classical destination.
    ///
    /// This is useful when a transformation changes a result from an explicit
    /// classical assignment into an SSA-like value.
    pub fn without_classical_destination(
        mut self,
    ) -> Result<Self, ResultError> {
        self.classical_destination = None;
        self.validate()?;
        Ok(self)
    }

    /// Returns a copy of the result with another semantic category.
    ///
    /// This method always validates the resulting binding.
    pub fn with_kind(
        mut self,
        kind: ResultKind,
    ) -> Result<Self, ResultError> {
        self.kind = kind;
        self.validate()?;
        Ok(self)
    }
}

impl fmt::Display for ResultBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{} <- {}",
            self.value,
            self.kind,
            self.producer
        )?;

        if let Some(qubit) = self.qubit {
            write!(formatter, " [qubit={qubit}]")?;
        }

        if let Some(destination) = self.classical_destination {
            write!(
                formatter,
                " [classical={destination}]"
            )?;
        }

        Ok(())
    }
}

// =============================================================================
// Result collection
// =============================================================================

/// Deterministically ordered collection of result bindings.
///
/// Result order is semantic ordering, not identity.
///
/// This collection deliberately uses `Vec` because:
///
/// - multiple results from one operation are valid;
/// - result ordering can be meaningful;
/// - `ValueId` remains the stable identity;
/// - callers can efficiently iterate in program order.
///
/// Duplicate `ValueId`s are rejected.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ResultBindings {
    results: Vec<ResultBinding>,
}

impl ResultBindings {
    /// Creates an empty result collection.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Creates an empty result collection with requested host capacity.
    ///
    /// Capacity is only an allocation hint.
    ///
    /// It is NOT a semantic IR limit.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            results: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of result bindings.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.results.len()
    }

    /// Returns whether there are no results.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns the host allocation capacity.
    ///
    /// This value must never be interpreted as an IR limit.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.results.capacity()
    }

    /// Reserves host collection capacity.
    ///
    /// This does not change semantic program meaning.
    pub fn reserve(
        &mut self,
        additional: usize,
    ) {
        self.results.reserve(additional);
    }

    /// Adds one result binding.
    ///
    /// The operation is transactional: if validation fails, the collection is
    /// unchanged.
    pub fn push(
        &mut self,
        result: ResultBinding,
    ) -> Result<(), ResultError> {
        result.validate()?;

        if self
            .results
            .iter()
            .any(|existing| existing.value() == result.value())
        {
            return Err(ResultError::DuplicateValue {
                value: result.value(),
            });
        }

        self.results.push(result);
        Ok(())
    }

    /// Inserts a result at an explicit position.
    ///
    /// The caller owns semantic ordering.
    pub fn insert(
        &mut self,
        index: usize,
        result: ResultBinding,
    ) -> Result<(), ResultError> {
        result.validate()?;

        if self
            .results
            .iter()
            .any(|existing| existing.value() == result.value())
        {
            return Err(ResultError::DuplicateValue {
                value: result.value(),
            });
        }

        if index > self.results.len() {
            return Err(ResultError::InvalidKind);
        }

        self.results.insert(index, result);
        Ok(())
    }

    /// Removes the result at `index`.
    ///
    /// Returns the removed binding when the index is valid.
    pub fn remove(
        &mut self,
        index: usize,
    ) -> Option<ResultBinding> {
        if index < self.results.len() {
            Some(self.results.remove(index))
        } else {
            None
        }
    }

    /// Returns the result at `index`.
    #[must_use]
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&ResultBinding> {
        self.results.get(index)
    }

    /// Returns a mutable result at `index`.
    ///
    /// Mutation is intentionally not exposed directly because changing a
    /// binding can violate local invariants or duplicate an existing
    /// `ValueId`. Use the transformation methods on `ResultBinding` and
    /// replace the value through `replace`.
    #[must_use]
    pub fn get_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut ResultBinding> {
        self.results.get_mut(index)
    }

    /// Replaces one result atomically.
    ///
    /// If validation fails, the existing result remains unchanged.
    pub fn replace(
        &mut self,
        index: usize,
        replacement: ResultBinding,
    ) -> Result<(), ResultError> {
        replacement.validate()?;

        if index >= self.results.len() {
            return Err(ResultError::InvalidKind);
        }

        if self
            .results
            .iter()
            .enumerate()
            .any(|(position, existing)| {
                position != index
                    && existing.value() == replacement.value()
            })
        {
            return Err(ResultError::DuplicateValue {
                value: replacement.value(),
            });
        }

        self.results[index] = replacement;
        Ok(())
    }

    /// Returns the first result with the requested `ValueId`.
    #[must_use]
    pub fn by_value(
        &self,
        value: ValueId,
    ) -> Option<&ResultBinding> {
        self.results
            .iter()
            .find(|result| result.value() == value)
    }

    /// Returns the first result produced by the requested operation.
    #[must_use]
    pub fn by_producer(
        &self,
        producer: OperationId,
    ) -> Option<&ResultBinding> {
        self.results
            .iter()
            .find(|result| result.producer() == producer)
    }

    /// Returns all results produced by the requested operation.
    ///
    /// Multiple results are valid and therefore this method returns an
    /// iterator rather than assuming one result per operation.
    pub fn by_producer_iter(
        &self,
        producer: OperationId,
    ) -> impl Iterator<Item = &ResultBinding> {
        self.results
            .iter()
            .filter(move |result| result.producer() == producer)
    }

    /// Returns the first result associated with a logical qubit.
    #[must_use]
    pub fn by_qubit(
        &self,
        qubit: QubitId,
    ) -> Option<&ResultBinding> {
        self.results
            .iter()
            .find(|result| result.qubit() == Some(qubit))
    }

    /// Returns all results associated with a logical qubit.
    pub fn by_qubit_iter(
        &self,
        qubit: QubitId,
    ) -> impl Iterator<Item = &ResultBinding> {
        self.results
            .iter()
            .filter(move |result| result.qubit() == Some(qubit))
    }

    /// Returns the first result written to a classical destination.
    #[must_use]
    pub fn by_classical_destination(
        &self,
        destination: ClassicalBitId,
    ) -> Option<&ResultBinding> {
        self.results.iter().find(|result| {
            result.classical_destination() == Some(destination)
        })
    }

    /// Returns all results written to a classical destination.
    pub fn by_classical_destination_iter(
        &self,
        destination: ClassicalBitId,
    ) -> impl Iterator<Item = &ResultBinding> {
        self.results.iter().filter(move |result| {
            result.classical_destination() == Some(destination)
        })
    }

    /// Returns an iterator over all results in semantic order.
    pub fn iter(&self) -> impl Iterator<Item = &ResultBinding> {
        self.results.iter()
    }

    /// Returns a mutable iterator.
    ///
    /// This is provided for internal program transformations. Callers that
    /// mutate through it must preserve `ResultBinding::validate()` invariants.
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut ResultBinding> {
        self.results.iter_mut()
    }

    /// Returns the results as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[ResultBinding] {
        self.results.as_slice()
    }

    /// Consumes the collection and returns its underlying vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<ResultBinding> {
        self.results
    }

    /// Validates every result and verifies unique `ValueId`s.
    pub fn validate(&self) -> Result<(), ResultError> {
        for result in &self.results {
            result.validate()?;
        }

        let mut seen = std::collections::BTreeSet::new();

        for result in &self.results {
            if !seen.insert(result.value()) {
                return Err(ResultError::DuplicateValue {
                    value: result.value(),
                });
            }
        }

        Ok(())
    }
}

impl From<Vec<ResultBinding>> for ResultBindings {
    fn from(results: Vec<ResultBinding>) -> Self {
        Self { results }
    }
}

impl IntoIterator for ResultBindings {
    type Item = ResultBinding;
    type IntoIter = std::vec::IntoIter<ResultBinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.into_iter()
    }
}

impl<'a> IntoIterator for &'a ResultBindings {
    type Item = &'a ResultBinding;
    type IntoIter = std::slice::Iter<'a, ResultBinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter()
    }
}

impl<'a> IntoIterator for &'a mut ResultBindings {
    type Item = &'a mut ResultBinding;
    type IntoIter = std::slice::IterMut<'a, ResultBinding>;

    fn into_iter(self) -> Self::IntoIter {
        self.results.iter_mut()
    }
}

// =============================================================================
// Result set utilities
// =============================================================================

/// Returns the number of distinct result values in a result collection.
///
/// This helper is intentionally independent of any machine-size assumptions.
#[must_use]
pub fn distinct_value_count(
    results: &ResultBindings,
) -> usize {
    let mut values = std::collections::BTreeSet::new();

    for result in results {
        values.insert(result.value());
    }

    values.len()
}

/// Returns whether all result values are unique.
#[must_use]
pub fn has_unique_values(
    results: &ResultBindings,
) -> bool {
    distinct_value_count(results) == results.len()
}

/// Returns whether the collection contains at least one measurement result.
#[must_use]
pub fn contains_measurement(
    results: &ResultBindings,
) -> bool {
    results.iter().any(ResultBinding::is_measurement)
}

/// Returns whether the collection contains at least one classical result.
#[must_use]
pub fn contains_classical_result(
    results: &ResultBindings,
) -> bool {
    results.iter().any(ResultBinding::is_classical)
}

/// Returns whether the collection contains a result produced by an operation.
#[must_use]
pub fn contains_producer(
    results: &ResultBindings,
    producer: OperationId,
) -> bool {
    results
        .iter()
        .any(|result| result.producer() == producer)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn value(value: u64) -> ValueId {
        ValueId::new(value)
    }

    fn operation(value: u64) -> OperationId {
        OperationId::new(value)
    }

    fn qubit(value: u64) -> QubitId {
        QubitId::new(value)
    }

    fn classical(value: u64) -> ClassicalBitId {
        ClassicalBitId::new(value)
    }

    #[test]
    fn creates_basic_bit_result() {
        let result = ResultBinding::to_classical(
            value(1),
            operation(10),
            ResultKind::Bit,
            classical(0),
        )
        .expect("bit result should be valid");

        assert_eq!(result.value(), value(1));
        assert_eq!(result.producer(), operation(10));
        assert_eq!(result.kind(), ResultKind::Bit);
        assert_eq!(
            result.classical_destination(),
            Some(classical(0))
        );
        assert!(!result.has_qubit());
    }

    #[test]
    fn creates_measurement_result_with_qubit_and_classical_destination() {
        let result = ResultBinding::measurement(
            value(1),
            operation(10),
            qubit(0),
            classical(0),
        )
        .expect("measurement result should be valid");

        assert_eq!(result.kind(), ResultKind::MeasurementBit);
        assert_eq!(result.qubit(), Some(qubit(0)));
        assert_eq!(
            result.classical_destination(),
            Some(classical(0))
        );
        assert!(result.is_measurement());
        assert!(result.is_classical());
    }

    #[test]
    fn logical_result_requires_qubit() {
        let result = ResultBinding::new(
            value(1),
            operation(10),
            ResultKind::Logical,
        );

        assert_eq!(
            result,
            Err(ResultError::MissingQubit)
        );
    }

    #[test]
    fn measurement_requires_qubit() {
        let result = ResultBinding::to_classical(
            value(1),
            operation(10),
            ResultKind::MeasurementBit,
            classical(0),
        );

        assert_eq!(
            result,
            Err(ResultError::MissingQubit)
        );
    }

    #[test]
    fn measurement_requires_classical_destination() {
        let result = ResultBinding::for_qubit(
            value(1),
            operation(10),
            ResultKind::MeasurementBit,
            qubit(0),
        );

        assert_eq!(
            result,
            Err(ResultError::MissingClassicalDestination)
        );
    }

    #[test]
    fn result_collection_rejects_duplicate_values() {
        let first = ResultBinding::to_classical(
            value(1),
            operation(10),
            ResultKind::Bit,
            classical(0),
        )
        .expect("first result should be valid");

        let second = ResultBinding::to_classical(
            value(1),
            operation(11),
            ResultKind::Bit,
            classical(1),
        )
        .expect("second result should be locally valid");

        let mut results = ResultBindings::new();

        results
            .push(first)
            .expect("first insertion should succeed");

        assert_eq!(
            results.push(second),
            Err(ResultError::DuplicateValue {
                value: value(1),
            })
        );

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn collection_is_deterministically_ordered() {
        let first = ResultBinding::to_classical(
            value(20),
            operation(2),
            ResultKind::Bit,
            classical(0),
        )
        .expect("first result should be valid");

        let second = ResultBinding::to_classical(
            value(10),
            operation(1),
            ResultKind::Bit,
            classical(1),
        )
        .expect("second result should be valid");

        let mut results = ResultBindings::new();

        results.push(first).expect("first insertion");
        results.push(second).expect("second insertion");

        assert_eq!(
            results
                .get(0)
                .expect("first result")
                .value(),
            value(20)
        );

        assert_eq!(
            results
                .get(1)
                .expect("second result")
                .value(),
            value(10)
        );
    }

    #[test]
    fn collection_lookup_by_producer_supports_multiple_results() {
        let first = ResultBinding::to_classical(
            value(1),
            operation(10),
            ResultKind::Bit,
            classical(0),
        )
        .expect("first result");

        let second = ResultBinding::to_classical(
            value(2),
            operation(10),
            ResultKind::Bit,
            classical(1),
        )
        .expect("second result");

        let mut results = ResultBindings::new();

        results.push(first).expect("first insertion");
        results.push(second).expect("second insertion");

        assert_eq!(
            results.by_producer_iter(operation(10)).count(),
            2
        );
    }

    #[test]
    fn collection_lookup_by_qubit_uses_canonical_qubit_type() {
        let result = ResultBinding::for_qubit(
            value(1),
            operation(10),
            ResultKind::Measurement,
            qubit(7),
        )
        .expect("measurement result should be valid");

        let mut results = ResultBindings::new();

        results.push(result).expect("result insertion");

        assert_eq!(
            results
                .by_qubit(qubit(7))
                .expect("result for q7")
                .qubit(),
            Some(qubit(7))
        );
    }

    #[test]
    fn replacement_is_atomic_on_failure() {
        let original = ResultBinding::to_classical(
            value(1),
            operation(10),
            ResultKind::Bit,
            classical(0),
        )
        .expect("original result");

        let duplicate = ResultBinding::to_classical(
            value(1),
            operation(20),
            ResultKind::Bit,
            classical(1),
        )
        .expect("duplicate result");

        let mut results = ResultBindings::new();

        results
            .push(original)
            .expect("original insertion");

        let replacement = results.replace(0, duplicate);

        assert_eq!(
            replacement,
            Err(ResultError::DuplicateValue {
                value: value(1),
            })
        );

        assert_eq!(
            results
                .get(0)
                .expect("original must remain")
                .producer(),
            operation(10)
        );
    }

    #[test]
    fn result_identity_is_independent_of_collection_position() {
        let first = ResultBinding::to_classical(
            value(100),
            operation(1),
            ResultKind::Bit,
            classical(0),
        )
        .expect("first result");

        let second = ResultBinding::to_classical(
            value(200),
            operation(2),
            ResultKind::Bit,
            classical(1),
        )
        .expect("second result");

        let mut results = ResultBindings::new();

        results.push(first).expect("first insertion");
        results.push(second).expect("second insertion");

        let first_id = results
            .get(0)
            .expect("first result")
            .value();

        let second_id = results
            .get(1)
            .expect("second result")
            .value();

        results
            .insert(
                0,
                ResultBinding::to_classical(
                    value(300),
                    operation(3),
                    ResultKind::Bit,
                    classical(2),
                )
                .expect("third result"),
            )
            .expect("insertion should succeed");

        assert_eq!(
            results.by_value(first_id)
                .expect("first result by identity")
                .value(),
            first_id
        );

        assert_eq!(
            results.by_value(second_id)
                .expect("second result by identity")
                .value(),
            second_id
        );
    }

    #[test]
    fn no_fixed_machine_size_is_encoded() {
        let large_qubit = QubitId::new(u64::MAX);
        let large_value = ValueId::new(u64::MAX - 1);
        let large_operation = OperationId::new(u64::MAX - 2);

        let result = ResultBinding::for_qubit(
            large_value,
            large_operation,
            ResultKind::Logical,
            large_qubit,
        )
        .expect("large identity values must remain valid");

        assert_eq!(result.qubit(), Some(large_qubit));
        assert_eq!(result.value(), large_value);
        assert_eq!(result.producer(), large_operation);
    }

    #[test]
    fn helper_functions_are_consistent() {
        let first = ResultBinding::to_classical(
            value(1),
            operation(1),
            ResultKind::Bit,
            classical(0),
        )
        .expect("first");

        let second = ResultBinding::measurement(
            value(2),
            operation(2),
            qubit(0),
            classical(1),
        )
        .expect("second");

        let mut results = ResultBindings::new();

        results.push(first).expect("first insertion");
        results.push(second).expect("second insertion");

        assert_eq!(distinct_value_count(&results), 2);
        assert!(has_unique_values(&results));
        assert!(contains_measurement(&results));
        assert!(contains_classical_result(&results));
        assert!(contains_producer(&results, operation(1)));
        assert!(contains_producer(&results, operation(2)));
    }

    #[test]
    fn validation_is_repeatable() {
        let result = ResultBinding::measurement(
            value(1),
            operation(1),
            qubit(0),
            classical(0),
        )
        .expect("measurement");

        assert!(result.validate().is_ok());
        assert!(result.validate().is_ok());
        assert!(result.validate().is_ok());
    }
}