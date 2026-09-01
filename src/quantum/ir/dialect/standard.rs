//! Zamani Quantum IR — Standard Quantum Dialect
//!
//! This module defines the canonical, target-independent **standard quantum
//! dialect** exposed by Zamani's Quantum IR.
//!
//! # Architectural role
//!
//! `dialect::standard` describes the standard semantic vocabulary of quantum
//! operations. It does NOT implement execution, simulation, decomposition,
//! routing, scheduling, calibration, pulse generation, hardware selection,
//! or backend lowering.
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
//!      ├── dialect::standard  ← this module
//!      │
//!      ├── optimization
//!      ├── analysis
//!      ├── validation
//!      ├── QEC
//!      ├── routing
//!      ├── scheduling
//!      └── backend lowering
//! ```
//!
//! The central architectural rule is:
//!
//! > A standard dialect operation describes WHAT the program means, never
//! > WHERE or HOW a particular quantum machine executes it.
//!
//! # Canonical ownership
//!
//! `gate.rs` owns the canonical [`GateKind`] representation and gate-level
//! semantic validation.
//!
//! This module owns:
//!
//! - the standard dialect identity;
//! - stable standard operation names;
//! - standard-gate descriptors;
//! - canonical name lookup;
//! - explicitly supported aliases;
//! - semantic classification metadata;
//! - standard-dialect operand/parameter contracts;
//! - logical-qubit validation helpers;
//! - deterministic enumeration of standard operations.
//!
//! This module does NOT own:
//!
//! - `QubitId` definition;
//! - gate execution;
//! - matrices;
//! - unitary simulation;
//! - physical qubits;
//! - topology;
//! - routing;
//! - scheduling;
//! - pulse implementation;
//! - calibration;
//! - hardware capabilities;
//! - vendor operations;
//! - optimization policy;
//! - decomposition algorithms;
//! - QEC implementation;
//! - backend APIs.
//!
//! # Universal-program principle
//!
//! Zamani programs must be capable of being written once and compiled to
//! machines of different sizes and architectures.
//!
//! Therefore this module deliberately contains:
//!
//! - no maximum qubit count;
//! - no maximum register size;
//! - no machine-specific topology;
//! - no vendor-specific qubit numbering;
//! - no physical-qubit assumptions;
//! - no hardware-native instruction assumptions;
//! - no fixed target architecture.
//!
//! `usize` appears only for Rust collection/cardinality APIs. It is never used
//! as the semantic identity of a quantum resource.
//!
//! # Standard dialect versus universal Zamani IR
//!
//! This file must NOT become the complete universe of quantum operations.
//!
//! The standard dialect is one dialect:
//!
//! ```text
//! canonical Zamani IR
//!        │
//!        ├── standard dialect
//!        ├── pulse dialect
//!        ├── analog dialect
//!        ├── fault-tolerant dialect
//!        ├── distributed dialect
//!        ├── vendor dialect
//!        └── future dialects
//! ```
//!
//! A future quantum operation must not require modifying this file merely
//! because a new architecture exists.
//!
//! # Existing IR integration
//!
//! The current canonical gate implementation defines [`GateKind`], including
//! standard operations such as:
//!
//! - I, X, Y, Z, H;
//! - S, Sdg, T, Tdg;
//! - V, Vdg;
//! - RX, RY, RZ, Phase, U1, U2, U3;
//! - CX, CY, CZ, CH;
//! - SWAP, ISWAP, ECR;
//! - CRX, CRY, CRZ;
//! - CCX, CSWAP;
//! - Measure, Barrier, Reset.
//!
//! This module deliberately reuses that type instead of creating a second
//! incompatible gate enum.
//!
//! # Qubit integration
//!
//! Logical operands use the canonical:
//!
//! ```text
//! quantum::ir::qubit::QubitId
//! ```
//!
//! Physical mapping remains outside this module.
//!
//! # Parameter integration
//!
//! Parameter counts are derived from [`GateKind`] and therefore remain
//! compatible with the canonical [`Parameter`] representation.
//!
//! Parameters may be concrete or symbolic. This module does not evaluate,
//! bind, or interpret them as hardware quantities.
//!
//! # Determinism
//!
//! Standard operation enumeration and alias resolution are deterministic.
//!
//! No `HashMap` or randomized hashing is used here.
//!
//! # Rust compatibility
//!
//! - Rust 1.97 / Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! `#![forbid(unsafe_code)]` makes the safety requirement compiler-enforced.
//!
//! # Integration contract
//!
//! `gate.rs` remains the canonical owner of gate semantics.
//!
//! `operation.rs` may use [`StandardGate`] and [`StandardDialect`] to identify
//! standard operations.
//!
//! `validation.rs` may use [`StandardGate::validate_qubits`] and
//! [`StandardGate::validate_parameter_count`] for dialect-level validation.
//!
//! `optimization` may inspect [`StandardGate`] metadata but must own all
//! transformation policy.
//!
//! `routing` may consume logical `QubitId` operands but must not be implemented
//! here.
//!
//! `scheduling` may consume operation/resource metadata but must not be
//! implemented here.
//!
//! serialization/hashing may use the stable dialect name and canonical
//! operation name.
//!
//! frontend importers may resolve source-level operation names through
//! [`StandardDialect::lookup`].
//!
//! Vendor/future dialects must not be forced into this registry.
//!
//! # Versioning
//!
//! The standard dialect has an explicit semantic version independent of the
//! Rust crate version. Adding a new standard operation is intentionally
//! different from changing the meaning of an existing operation.
//!
//! Existing operation names and their canonical `GateKind` mappings are part
//! of the serialized semantic contract and must not be silently renamed.

#![forbid(unsafe_code)]

use std::fmt;

use super::super::gate::{GateKind, OperandCount};
use super::super::parameter::Parameter;
use super::super::qubit::QubitId;

// =============================================================================
// Dialect identity
// =============================================================================

/// Stable namespace of the standard Zamani quantum dialect.
///
/// This identifier is semantic and serialization-facing. It must not depend
/// on a Rust module path.
pub const STANDARD_DIALECT_NAME: &str = "zamani.quantum.standard";

/// Major version of the standard dialect semantic contract.
///
/// A major-version change may alter the meaning or compatibility guarantees
/// of existing operations.
pub const STANDARD_DIALECT_MAJOR: u16 = 1;

/// Minor version of the standard dialect semantic contract.
///
/// New backwards-compatible standard operations or metadata may be introduced
/// in a minor version.
pub const STANDARD_DIALECT_MINOR: u16 = 0;

/// Patch version of the standard dialect semantic contract.
pub const STANDARD_DIALECT_PATCH: u16 = 0;

/// Complete semantic version of the standard dialect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StandardDialectVersion {
    /// Major semantic version.
    pub major: u16,

    /// Minor semantic version.
    pub minor: u16,

    /// Patch semantic version.
    pub patch: u16,
}

impl StandardDialectVersion {
    /// Creates a dialect version.
    #[must_use]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the current standard dialect version.
    #[must_use]
    pub const fn current() -> Self {
        Self::new(
            STANDARD_DIALECT_MAJOR,
            STANDARD_DIALECT_MINOR,
            STANDARD_DIALECT_PATCH,
        )
    }

    /// Returns whether this version has the same semantic major version.
    #[must_use]
    pub const fn same_major(self, other: Self) -> bool {
        self.major == other.major
    }
}

impl Default for StandardDialectVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl fmt::Display for StandardDialectVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// =============================================================================
// Semantic classification
// =============================================================================

/// High-level semantic class of a standard operation.
///
/// This classification is deliberately broader than "gate" because the
/// standard dialect also contains measurement, reset, and barrier operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardOperationClass {
    /// A state-preserving unitary gate.
    Unitary,

    /// A non-unitary measurement.
    Measurement,

    /// A state-preparation/reset operation.
    Reset,

    /// A synchronization/analysis marker with no state transformation.
    Barrier,
}

impl StandardOperationClass {
    /// Returns whether the operation changes the quantum state through a
    /// unitary transformation.
    #[must_use]
    pub const fn is_unitary(self) -> bool {
        matches!(self, Self::Unitary)
    }

    /// Returns whether the operation is a measurement.
    #[must_use]
    pub const fn is_measurement(self) -> bool {
        matches!(self, Self::Measurement)
    }

    /// Returns whether the operation resets quantum state.
    #[must_use]
    pub const fn is_reset(self) -> bool {
        matches!(self, Self::Reset)
    }

    /// Returns whether the operation is a barrier marker.
    #[must_use]
    pub const fn is_barrier(self) -> bool {
        matches!(self, Self::Barrier)
    }
}

// =============================================================================
// Semantic properties
// =============================================================================

/// Static semantic properties of a standard operation.
///
/// These flags describe the operation itself. They do not describe whether a
/// particular backend supports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardOperationProperties {
    /// Operation is unitary.
    pub unitary: bool,

    /// Operation is self-inverse without inspecting parameter values.
    pub self_inverse: bool,

    /// Operation accepts symbolic parameters.
    pub parameterized: bool,

    /// Operation is non-unitary measurement.
    pub measurement: bool,

    /// Operation resets quantum state.
    pub reset: bool,

    /// Operation is a synchronization barrier.
    pub barrier: bool,

    /// Operation has a classical destination as part of its canonical
    /// semantic form.
    pub has_classical_destination: bool,
}

impl StandardOperationProperties {
    /// Returns the properties for a canonical [`GateKind`].
    #[must_use]
    pub const fn for_gate(kind: GateKind) -> Self {
        Self {
            unitary: kind.is_unitary(),
            self_inverse: kind.is_self_inverse(),
            parameterized: kind.is_parameterized(),
            measurement: kind.is_measurement(),
            reset: kind.is_reset(),
            barrier: kind.is_barrier(),
            has_classical_destination: kind.requires_classical_target(),
        }
    }
}

// =============================================================================
// Standard gate descriptor
// =============================================================================

/// Immutable descriptor for one standard dialect operation.
///
/// A descriptor is metadata, not an executable instruction.
///
/// The descriptor intentionally contains no hardware information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StandardGate {
    /// Canonical semantic operation kind.
    kind: GateKind,

    /// Canonical serialized/source-independent operation name.
    canonical_name: &'static str,

    /// Additional accepted source spelling.
    ///
    /// `None` means there is no dedicated alias.
    alias: Option<&'static str>,

    /// Semantic operation class.
    class: StandardOperationClass,

    /// Static semantic properties.
    properties: StandardOperationProperties,

    /// Logical-qubit operand cardinality.
    operands: OperandCount,

    /// Number of scalar parameters.
    parameter_count: usize,
}

impl StandardGate {
    /// Creates a standard-gate descriptor.
    const fn new(
        kind: GateKind,
        canonical_name: &'static str,
        alias: Option<&'static str>,
        class: StandardOperationClass,
    ) -> Self {
        Self {
            kind,
            canonical_name,
            alias,
            class,
            properties: StandardOperationProperties::for_gate(kind),
            operands: kind.operand_count(),
            parameter_count: kind.parameter_count(),
        }
    }

    /// Returns the canonical [`GateKind`].
    #[must_use]
    pub const fn kind(self) -> GateKind {
        self.kind
    }

    /// Returns the canonical operation name.
    #[must_use]
    pub const fn canonical_name(self) -> &'static str {
        self.canonical_name
    }

    /// Returns the optional source alias.
    #[must_use]
    pub const fn alias(self) -> Option<&'static str> {
        self.alias
    }

    /// Returns the semantic operation class.
    #[must_use]
    pub const fn class(self) -> StandardOperationClass {
        self.class
    }

    /// Returns static semantic properties.
    #[must_use]
    pub const fn properties(self) -> StandardOperationProperties {
        self.properties
    }

    /// Returns the logical-qubit operand contract.
    #[must_use]
    pub const fn operand_count(self) -> OperandCount {
        self.operands
    }

    /// Returns the exact scalar-parameter count.
    #[must_use]
    pub const fn parameter_count(self) -> usize {
        self.parameter_count
    }

    /// Returns whether this operation accepts the supplied number of logical
    /// qubit operands.
    #[must_use]
    pub const fn accepts_operand_count(self, actual: usize) -> bool {
        self.operands.accepts(actual)
    }

    /// Returns whether this operation accepts the supplied number of
    /// parameters.
    #[must_use]
    pub const fn accepts_parameter_count(self, actual: usize) -> bool {
        self.parameter_count == actual
    }

    /// Validates the number of logical qubit operands.
    pub fn validate_qubit_count(
        self,
        qubits: &[QubitId],
    ) -> Result<(), StandardDialectError> {
        if !self.accepts_operand_count(qubits.len()) {
            return Err(StandardDialectError::InvalidOperandCount {
                operation: self.kind,
                expected: self.operands,
                actual: qubits.len(),
            });
        }

        Ok(())
    }

    /// Validates logical-qubit operands.
    ///
    /// Standard gate semantics require distinct logical operands for the
    /// standard operations represented here.
    ///
    /// This function only validates logical identity. It does not validate
    /// whether the qubits are physically connected.
    pub fn validate_qubits(
        self,
        qubits: &[QubitId],
    ) -> Result<(), StandardDialectError> {
        self.validate_qubit_count(qubits)?;

        for (index, qubit) in qubits.iter().enumerate() {
            if qubits[index + 1..].iter().any(|other| other == qubit) {
                return Err(StandardDialectError::DuplicateQubit {
                    operation: self.kind,
                    qubit: *qubit,
                });
            }
        }

        Ok(())
    }

    /// Validates the supplied parameter count.
    ///
    /// Individual [`Parameter`] validity remains owned by `parameter.rs`.
    pub fn validate_parameters(
        self,
        parameters: &[Parameter],
    ) -> Result<(), StandardDialectError> {
        if !self.accepts_parameter_count(parameters.len()) {
            return Err(StandardDialectError::InvalidParameterCount {
                operation: self.kind,
                expected: self.parameter_count,
                actual: parameters.len(),
            });
        }

        for (index, parameter) in parameters.iter().enumerate() {
            parameter.validate().map_err(|_| {
                StandardDialectError::InvalidParameter { index }
            })?;
        }

        Ok(())
    }

    /// Validates both logical operands and parameters.
    pub fn validate(
        self,
        qubits: &[QubitId],
        parameters: &[Parameter],
    ) -> Result<(), StandardDialectError> {
        self.validate_qubits(qubits)?;
        self.validate_parameters(parameters)?;
        Ok(())
    }
}

impl fmt::Display for StandardGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.canonical_name)
    }
}

// =============================================================================
// Standard operation registry
// =============================================================================
//
// IMPORTANT:
// This array is the standard dialect vocabulary, not a scalability limit.
// Its finite size describes the operations standardized by this dialect
// version. Future dialects and extensions must remain possible without
// changing the canonical IR architecture.

/// Complete standard-operation descriptor table.
///
/// The ordering is stable and deterministic.
///
/// Do not use the table length as a quantum-machine capacity.
pub static STANDARD_OPERATIONS: &[StandardGate] = &[
    // Single-qubit fixed operations.
    StandardGate::new(
        GateKind::I,
        "id",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::X,
        "x",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Y,
        "y",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Z,
        "z",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::H,
        "h",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::S,
        "s",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Sdg,
        "sdg",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::T,
        "t",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Tdg,
        "tdg",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::V,
        "v",
        Some("sqrt_x"),
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Vdg,
        "vdg",
        Some("sqrt_x_dag"),
        StandardOperationClass::Unitary,
    ),

    // Single-qubit parameterized operations.
    StandardGate::new(
        GateKind::RX,
        "rx",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::RY,
        "ry",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::RZ,
        "rz",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::Phase,
        "phase",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::U1,
        "u1",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::U2,
        "u2",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::U3,
        "u3",
        None,
        StandardOperationClass::Unitary,
    ),

    // Two-qubit fixed operations.
    StandardGate::new(
        GateKind::CX,
        "cx",
        Some("cnot"),
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CY,
        "cy",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CZ,
        "cz",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CH,
        "ch",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::SWAP,
        "swap",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::ISWAP,
        "iswap",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::ECR,
        "ecr",
        None,
        StandardOperationClass::Unitary,
    ),

    // Two-qubit parameterized operations.
    StandardGate::new(
        GateKind::CRX,
        "crx",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CRY,
        "cry",
        None,
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CRZ,
        "crz",
        None,
        StandardOperationClass::Unitary,
    ),

    // Three-qubit operations.
    StandardGate::new(
        GateKind::CCX,
        "ccx",
        Some("toffoli"),
        StandardOperationClass::Unitary,
    ),
    StandardGate::new(
        GateKind::CSWAP,
        "cswap",
        Some("fredkin"),
        StandardOperationClass::Unitary,
    ),

    // Non-unitary / structural operations.
    StandardGate::new(
        GateKind::Measure,
        "measure",
        Some("measure_z"),
        StandardOperationClass::Measurement,
    ),
    StandardGate::new(
        GateKind::Barrier,
        "barrier",
        None,
        StandardOperationClass::Barrier,
    ),
    StandardGate::new(
        GateKind::Reset,
        "reset",
        None,
        StandardOperationClass::Reset,
    ),
];

// =============================================================================
// Errors
// =============================================================================

/// Errors produced by standard-dialect lookup and validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardDialectError {
    /// The requested operation name is not part of the standard dialect.
    UnknownOperation {
        /// The caller-owned operation name is represented by a stable static
        /// description rather than retaining arbitrary input in the error.
        ///
        /// The actual source string should be retained by the frontend
        /// diagnostic layer.
        namespace: &'static str,
    },

    /// Operand cardinality is invalid.
    InvalidOperandCount {
        /// Operation that failed validation.
        operation: GateKind,

        /// Required cardinality.
        expected: OperandCount,

        /// Actual cardinality.
        actual: usize,
    },

    /// Two logical operands refer to the same logical qubit.
    DuplicateQubit {
        /// Operation containing the duplicate.
        operation: GateKind,

        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// Parameter cardinality is invalid.
    InvalidParameterCount {
        /// Operation that failed validation.
        operation: GateKind,

        /// Required number of parameters.
        expected: usize,

        /// Actual number of parameters.
        actual: usize,
    },

    /// A parameter failed canonical parameter validation.
    InvalidParameter {
        /// Position of the invalid parameter.
        index: usize,
    },
}

impl fmt::Display for StandardDialectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOperation { namespace } => {
                write!(
                    f,
                    "operation is not defined by dialect `{namespace}`"
                )
            }

            Self::InvalidOperandCount {
                operation,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "standard operation {operation:?} requires \
                     {expected} logical operand(s), received {actual}"
                )
            }

            Self::DuplicateQubit { operation, qubit } => {
                write!(
                    f,
                    "standard operation {operation:?} contains \
                     duplicate logical qubit {qubit}"
                )
            }

            Self::InvalidParameterCount {
                operation,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "standard operation {operation:?} requires \
                     {expected} parameter(s), received {actual}"
                )
            }

            Self::InvalidParameter { index } => {
                write!(
                    f,
                    "standard operation contains invalid parameter \
                     at index {index}"
                )
            }
        }
    }
}

impl std::error::Error for StandardDialectError {}

// =============================================================================
// Dialect
// =============================================================================

/// Stateless access point for the standard quantum dialect.
///
/// `StandardDialect` contains no mutable registry and no global runtime state.
/// The standard vocabulary is immutable and compiled into the dialect version.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct StandardDialect;

impl StandardDialect {
    /// Creates the standard dialect handle.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the stable dialect namespace.
    #[must_use]
    pub const fn name(self) -> &'static str {
        STANDARD_DIALECT_NAME
    }

    /// Returns the current dialect semantic version.
    #[must_use]
    pub const fn version(self) -> StandardDialectVersion {
        StandardDialectVersion::current()
    }

    /// Returns every operation in deterministic canonical order.
    #[must_use]
    pub const fn operations(self) -> &'static [StandardGate] {
        STANDARD_OPERATIONS
    }

    /// Returns the number of operations defined by this dialect version.
    ///
    /// This is the number of semantic operations in the standard vocabulary,
    /// not a quantum-machine capacity.
    #[must_use]
    pub const fn operation_count(self) -> usize {
        STANDARD_OPERATIONS.len()
    }

    /// Finds an operation by canonical name or explicitly supported alias.
    ///
    /// Lookup is case-sensitive. Source languages that provide case-insensitive
    /// syntax should normalize their spelling in the frontend rather than
    /// changing the canonical dialect namespace.
    #[must_use]
    pub fn lookup(self, name: &str) -> Option<StandardGate> {
        STANDARD_OPERATIONS.iter().copied().find(|operation| {
            operation.canonical_name() == name
                || operation.alias() == Some(name)
        })
    }

    /// Finds an operation by its canonical [`GateKind`].
    #[must_use]
    pub fn by_kind(self, kind: GateKind) -> Option<StandardGate> {
        STANDARD_OPERATIONS
            .iter()
            .copied()
            .find(|operation| operation.kind() == kind)
    }

    /// Returns whether a name belongs to the standard dialect.
    #[must_use]
    pub fn contains(self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Returns the canonical operation name for a [`GateKind`].
    ///
    /// Returns `None` if the `GateKind` exists in the canonical gate layer but
    /// is not part of this dialect version.
    #[must_use]
    pub fn canonical_name(
        self,
        kind: GateKind,
    ) -> Option<&'static str> {
        self.by_kind(kind)
            .map(StandardGate::canonical_name)
    }

    /// Validates a standard operation using its canonical [`GateKind`].
    pub fn validate(
        self,
        kind: GateKind,
        qubits: &[QubitId],
        parameters: &[Parameter],
    ) -> Result<(), StandardDialectError> {
        let operation = self
            .by_kind(kind)
            .ok_or(StandardDialectError::UnknownOperation {
                namespace: STANDARD_DIALECT_NAME,
            })?;

        operation.validate(qubits, parameters)
    }
}

// =============================================================================
// Compile-time standard-operation helpers
// =============================================================================

/// Returns the descriptor for a standard [`GateKind`].
///
/// This function is useful in contexts where a dialect handle would add
/// unnecessary ceremony.
#[must_use]
pub fn standard_gate(kind: GateKind) -> Option<StandardGate> {
    StandardDialect::new().by_kind(kind)
}

/// Resolves a canonical standard operation name or supported alias.
#[must_use]
pub fn lookup_standard_gate(name: &str) -> Option<StandardGate> {
    StandardDialect::new().lookup(name)
}

/// Returns whether the supplied name is part of the standard dialect.
#[must_use]
pub fn is_standard_operation(name: &str) -> bool {
    lookup_standard_gate(name).is_some()
}

// =============================================================================
// Stable semantic-name helpers
// =============================================================================

/// Returns the canonical semantic name of a standard gate.
///
/// This is intentionally a free function because serialization, diagnostics,
/// and hashing code frequently need a canonical spelling without constructing
/// a dialect object.
#[must_use]
pub fn canonical_standard_name(
    kind: GateKind,
) -> Option<&'static str> {
    standard_gate(kind).map(StandardGate::canonical_name)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn q(index: u64) -> QubitId {
        QubitId(index)
    }

    #[test]
    fn dialect_identity_is_stable() {
        let dialect = StandardDialect::new();

        assert_eq!(dialect.name(), "zamani.quantum.standard");
        assert_eq!(
            dialect.version(),
            StandardDialectVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn every_gate_kind_is_resolved_exactly_once() {
        let dialect = StandardDialect::new();

        for operation in STANDARD_OPERATIONS {
            let resolved = dialect.by_kind(operation.kind());

            assert_eq!(resolved, Some(*operation));
        }
    }

    #[test]
    fn canonical_names_round_trip() {
        let dialect = StandardDialect::new();

        for operation in STANDARD_OPERATIONS {
            let resolved =
                dialect.lookup(operation.canonical_name());

            assert_eq!(resolved, Some(*operation));
        }
    }

    #[test]
    fn aliases_resolve_to_the_same_semantic_operation() {
        let dialect = StandardDialect::new();

        assert_eq!(
            dialect.lookup("cnot").map(StandardGate::kind),
            Some(GateKind::CX)
        );

        assert_eq!(
            dialect.lookup("toffoli").map(StandardGate::kind),
            Some(GateKind::CCX)
        );

        assert_eq!(
            dialect.lookup("fredkin").map(StandardGate::kind),
            Some(GateKind::CSWAP)
        );

        assert_eq!(
            dialect.lookup("sqrt_x").map(StandardGate::kind),
            Some(GateKind::V)
        );
    }

    #[test]
    fn unknown_names_are_not_silently_accepted() {
        let dialect = StandardDialect::new();

        assert_eq!(dialect.lookup("unknown_gate"), None);
        assert!(!dialect.contains("unknown_gate"));
    }

    #[test]
    fn x_has_one_qubit_and_no_parameters() {
        let gate = standard_gate(GateKind::X).expect("X is standard");

        assert_eq!(gate.operand_count(), OperandCount::Exact(1));
        assert_eq!(gate.parameter_count(), 0);

        gate.validate(&[q(0)], &[]).expect("valid X");
    }

    #[test]
    fn cx_has_two_distinct_logical_qubits() {
        let gate = standard_gate(GateKind::CX).expect("CX is standard");

        assert_eq!(gate.operand_count(), OperandCount::Exact(2));
        assert_eq!(gate.parameter_count(), 0);

        gate.validate(&[q(0), q(1)], &[])
            .expect("valid CX");
    }

    #[test]
    fn duplicate_logical_qubits_are_rejected() {
        let gate = standard_gate(GateKind::CX).expect("CX is standard");

        let result = gate.validate(&[q(0), q(0)], &[]);

        assert!(matches!(
            result,
            Err(StandardDialectError::DuplicateQubit {
                operation: GateKind::CX,
                qubit: QubitId(0),
            })
        ));
    }

    #[test]
    fn wrong_operand_count_is_rejected() {
        let gate = standard_gate(GateKind::CX).expect("CX is standard");

        let result = gate.validate(&[q(0)], &[]);

        assert!(matches!(
            result,
            Err(StandardDialectError::InvalidOperandCount {
                operation: GateKind::CX,
                expected: OperandCount::Exact(2),
                actual: 1,
            })
        ));
    }

    #[test]
    fn parameterized_gate_requires_correct_parameter_count() {
        let gate = standard_gate(GateKind::RX).expect("RX is standard");

        let parameter = Parameter::constant(0.5)
            .expect("finite parameter");

        gate.validate(&[q(0)], &[parameter])
            .expect("valid RX");

        let result = gate.validate(&[q(0)], &[]);

        assert!(matches!(
            result,
            Err(StandardDialectError::InvalidParameterCount {
                operation: GateKind::RX,
                expected: 1,
                actual: 0,
            })
        ));
    }

    #[test]
    fn symbolic_parameters_are_accepted() {
        let gate = standard_gate(GateKind::RZ).expect("RZ is standard");

        let parameter =
            Parameter::symbol("theta").expect("valid symbol");

        gate.validate(&[q(0)], &[parameter])
            .expect("symbolic RZ is valid");
    }

    #[test]
    fn u3_requires_three_parameters() {
        let gate = standard_gate(GateKind::U3).expect("U3 is standard");

        let a = Parameter::constant(0.1).expect("finite");
        let b = Parameter::constant(0.2).expect("finite");
        let c = Parameter::constant(0.3).expect("finite");

        gate.validate(&[q(0)], &[a, b, c])
            .expect("valid U3");
    }

    #[test]
    fn barrier_is_variadic_but_not_empty() {
        let gate =
            standard_gate(GateKind::Barrier).expect("barrier is standard");

        gate.validate(&[q(0)], &[])
            .expect("one-qubit barrier");

        gate.validate(&[q(0), q(1), q(2)], &[])
            .expect("multi-qubit barrier");

        let result = gate.validate(&[], &[]);

        assert!(matches!(
            result,
            Err(StandardDialectError::InvalidOperandCount {
                operation: GateKind::Barrier,
                expected: OperandCount::AtLeast(1),
                actual: 0,
            })
        ));
    }

    #[test]
    fn operation_classification_is_correct() {
        assert_eq!(
            standard_gate(GateKind::X)
                .expect("X")
                .class(),
            StandardOperationClass::Unitary
        );

        assert_eq!(
            standard_gate(GateKind::Measure)
                .expect("measure")
                .class(),
            StandardOperationClass::Measurement
        );

        assert_eq!(
            standard_gate(GateKind::Reset)
                .expect("reset")
                .class(),
            StandardOperationClass::Reset
        );

        assert_eq!(
            standard_gate(GateKind::Barrier)
                .expect("barrier")
                .class(),
            StandardOperationClass::Barrier
        );
    }

    #[test]
    fn no_duplicate_canonical_names_exist() {
        for (index, left) in STANDARD_OPERATIONS.iter().enumerate() {
            for right in STANDARD_OPERATIONS.iter().skip(index + 1) {
                assert_ne!(
                    left.canonical_name(),
                    right.canonical_name()
                );
            }
        }
    }

    #[test]
    fn no_alias_collides_with_another_canonical_name() {
        for operation in STANDARD_OPERATIONS {
            if let Some(alias) = operation.alias() {
                assert!(
                    STANDARD_OPERATIONS
                        .iter()
                        .all(|other| other.canonical_name() != alias),
                    "alias `{alias}` collides with a canonical operation name"
                );
            }
        }
    }

    #[test]
    fn standard_operation_count_is_not_a_resource_limit() {
        let dialect = StandardDialect::new();

        assert_eq!(
            dialect.operation_count(),
            STANDARD_OPERATIONS.len()
        );

        // This test intentionally documents that the table describes the
        // vocabulary of one dialect version, not the capacity of a QPU.
        assert!(dialect.operation_count() > 0);
    }
}