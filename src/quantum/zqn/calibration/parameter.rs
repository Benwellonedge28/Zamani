//! Zamani Quantum Noise (ZQN) — Calibration Parameters.
//!
//! Path:
//!
//!     src/quantum/zqn/calibration/parameter.rs
//!
//! ============================================================================
//! PURPOSE
//! ============================================================================
//!
//! This module defines the canonical, backend-independent representation of a
//! single calibration parameter and its associated semantic information.
//!
//! A calibration parameter answers:
//!
//! > "What calibrated value is associated with this named physical quantity,
//! > for which quantum resource scope, with what uncertainty, unit, revision,
//! > provenance and semantic status?"
//!
//! This module owns parameter semantics.
//!
//! It does NOT own:
//!
//! - calibration snapshot lifetime;
//! - calibration registries;
//! - hardware discovery;
//! - hardware credentials;
//! - calibration experiments;
//! - statistical fitting;
//! - drift algorithms;
//! - interpolation algorithms;
//! - gate calibration semantics;
//! - readout calibration semantics;
//! - noise-channel mathematics;
//! - simulation;
//! - routing;
//! - scheduling;
//! - QEC;
//! - vendor APIs;
//! - serialization formats;
//! - global mutable state.
//!
//! Those responsibilities belong to their respective ZQN or repository
//! subsystems.
//!
//! ============================================================================
//! ARCHITECTURAL POSITION
//! ============================================================================
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              │ canonical resource identity
//!                              ▼
//!                    calibration::parameter
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!       snapshot            device             gate/readout
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                       noise::model
//!                              │
//!              ┌───────────────┼────────────────┐
//!              ▼               ▼                ▼
//!          simulation         QEC            hardware
//! ```
//!
//! ============================================================================
//! CANONICAL IDENTITIES
//! ============================================================================
//!
//! ZQN MUST NOT define another QubitId or PhysicalQubitId.
//!
//! Quantum resource identity remains owned by:
//!
//!     crate::quantum::ir::qubit
//!
//! Therefore this module directly uses:
//!
//!     crate::quantum::ir::qubit::QubitId
//!     crate::quantum::ir::qubit::PhysicalQubitId
//!
//! when a calibration parameter is scoped to a logical or physical qubit.
//!
//! Operation identity remains owned by the canonical Quantum IR identity
//! subsystem.
//!
//! ZQN-specific parameter identity uses:
//!
//!     crate::quantum::zqn::core::ids::NoiseParameterId
//!
//! ============================================================================
//! WRITE-ONCE / SCALE-EVERYWHERE CONTRACT
//! ============================================================================
//!
//! This module contains NO semantic machine-size ceiling.
//!
//! In particular, it does not define:
//!
//!     MAX_QUBITS
//!     MAX_PARAMETERS
//!     MAX_VECTOR_LENGTH
//!     MAX_MATRIX_SIZE
//!     MAX_RESOURCES
//!     MAX_CALIBRATION_ENTRIES
//!
//! A parameter can therefore describe a calibration object of any size
//! representable by the selected host representation and permitted by the
//! surrounding resource policy.
//!
//! "Infinity" means that ZQN does not encode an artificial finite machine-size
//! ceiling. It does not mean that physical hardware, memory, address space,
//! storage, network bandwidth or execution time are infinite.
//!
//! Large values must be handled by higher-level resource policies and, where
//! appropriate, streaming/chunked representations.
//!
//! ============================================================================
//! DETERMINISM
//! ============================================================================
//!
//! Parameter construction is deterministic.
//!
//! This file does NOT:
//!
//! - read the wall clock;
//! - generate random IDs;
//! - access process IDs;
//! - access thread IDs;
//! - inspect memory addresses;
//! - use global mutable state;
//! - use a global RNG;
//! - depend on hash-map iteration order;
//! - perform implicit I/O.
//!
//! ============================================================================
//! IMMUTABILITY
//! ============================================================================
//!
//! CalibrationParameter is an immutable value object.
//!
//! A changed calibration value must create a new parameter value or a new
//! parameter revision. Existing parameter values must never be silently
//! mutated while they are being used by execution.
//!
//! This is required for:
//!
//! - reproducibility;
//! - concurrent execution;
//! - deterministic benchmarking;
//! - calibration snapshot integrity;
//! - distributed execution.
//!
//! ============================================================================
//! NUMERICAL SEMANTICS
//! ============================================================================
//!
//! Floating-point values are accepted only when finite.
//!
//! NaN and ±infinity are rejected at construction/validation boundaries.
//!
//! This module deliberately does not claim that f64 is the universal numerical
//! representation for every future quantum technology. It provides a stable
//! interchange representation while allowing future numerical subsystems to
//! introduce richer representations behind explicit extension boundaries.
//!
//! ============================================================================
//! SERIALIZATION
//! ============================================================================
//!
//! This module does NOT define a wire format.
//!
//! The canonical external representation belongs to:
//!
//!     crate::quantum::zqn::io
//!
//! Serialization must preserve:
//!
//! - parameter identity;
//! - parameter name;
//! - revision;
//! - value;
//! - unit;
//! - uncertainty;
//! - scope;
//! - calibration lineage;
//! - semantic status;
//! - provenance.
//!
//! Rust memory layout is NOT a serialization contract.
//!
//! ============================================================================
//! SECURITY
//! ============================================================================
//!
//! Calibration parameters are data, not capabilities.
//!
//! A parameter MUST NOT contain:
//!
//! - API keys;
//! - QPU credentials;
//! - authentication tokens;
//! - executable code;
//! - file paths requiring implicit access;
//! - network handles;
//! - provider SDK objects.
//!
//! An ID identifying a resource does not grant access to that resource.
//!
//! ============================================================================
//! RUST COMPATIBILITY
//! ============================================================================
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021 edition;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code;
//! - standard library only.
//!
//! ============================================================================

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cmp::Ordering;
use core::fmt;

use crate::quantum::ir::identity::OperationId;
use crate::quantum::ir::qubit::{PhysicalQubitId, QubitId};
use crate::quantum::zqn::core::errors::ZqnResult;
use crate::quantum::zqn::core::ids::{CalibrationId, NoiseParameterId, ZqnObjectId};

// ============================================================================
// CONSTANTS
// ============================================================================

/// Current semantic revision of the calibration-parameter representation.
///
/// This is a semantic representation revision and is NOT a machine-size
/// limit.
pub const CALIBRATION_PARAMETER_SCHEMA_VERSION: u16 = 1;

// ============================================================================
// PARAMETER REVISION
// ============================================================================

/// Semantic revision of one calibration parameter definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalibrationParameterRevision {
    major: u32,
    minor: u32,
    patch: u32,
}

impl CalibrationParameterRevision {
    /// Creates an explicit parameter revision.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major semantic revision.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor semantic revision.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns the patch semantic revision.
    #[must_use]
    pub const fn patch(self) -> u32 {
        self.patch
    }
}

impl Default for CalibrationParameterRevision {
    fn default() -> Self {
        Self::new(1, 0, 0)
    }
}

impl fmt::Display for CalibrationParameterRevision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// ============================================================================
// PARAMETER STATUS
// ============================================================================

/// Lifecycle/semantic status of a calibration parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationParameterStatus {
    /// Parameter has been validated and is usable.
    Valid,

    /// Parameter was observed but has not yet passed validation.
    Unvalidated,

    /// Parameter is known to be stale.
    Stale,

    /// Parameter is known to be invalid.
    Invalid,

    /// Parameter is intentionally disabled.
    Disabled,

    /// Parameter was superseded by another revision.
    Superseded,
}

impl Default for CalibrationParameterStatus {
    fn default() -> Self {
        Self::Unvalidated
    }
}

impl fmt::Display for CalibrationParameterStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => formatter.write_str("valid"),
            Self::Unvalidated => formatter.write_str("unvalidated"),
            Self::Stale => formatter.write_str("stale"),
            Self::Invalid => formatter.write_str("invalid"),
            Self::Disabled => formatter.write_str("disabled"),
            Self::Superseded => formatter.write_str("superseded"),
        }
    }
}

// ============================================================================
// VALUE TYPE
// ============================================================================

/// A calibration parameter value.
///
/// The representation deliberately supports more than one scalar numerical
/// form because calibration data is not universally scalar.
///
/// Examples include:
///
/// - frequency;
/// - T1/T2;
/// - gate amplitude;
/// - phase;
/// - readout threshold;
/// - complex response;
/// - vectors of coefficients;
/// - matrices of correction coefficients;
/// - categorical calibration states.
///
/// Vector and matrix dimensions are data-driven rather than fixed.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationValue {
    /// Finite real scalar.
    Scalar(f64),

    /// Signed integer value.
    Integer(i128),

    /// Boolean calibration state.
    Boolean(bool),

    /// Human-readable/categorical value.
    Text(String),

    /// Finite real vector.
    Vector(Vec<f64>),

    /// Finite real matrix stored row-major.
    ///
    /// `rows * columns == values.len()` is an invariant.
    Matrix {
        rows: usize,
        columns: usize,
        values: Vec<f64>,
    },

    /// Finite complex scalar.
    Complex {
        real: f64,
        imaginary: f64,
    },

    /// Finite complex vector stored as interleaved real/imaginary values:
    ///
    /// `[re0, im0, re1, im1, ...]`
    ComplexVector(Vec<f64>),

    /// Opaque structured value represented by an explicit schema identifier
    /// and UTF-8 payload.
    ///
    /// This is intentionally not executable data.
    Structured {
        schema: String,
        payload: String,
    },
}

impl CalibrationValue {
    /// Creates a finite scalar.
    pub fn scalar(value: f64) -> Result<Self, CalibrationParameterError> {
        ensure_finite(value, "scalar")?;
        Ok(Self::Scalar(value))
    }

    /// Creates a signed integer value.
    #[must_use]
    pub const fn integer(value: i128) -> Self {
        Self::Integer(value)
    }

    /// Creates a boolean value.
    #[must_use]
    pub const fn boolean(value: bool) -> Self {
        Self::Boolean(value)
    }

    /// Creates a textual value.
    pub fn text(value: impl Into<String>) -> Result<Self, CalibrationParameterError> {
        let value = value.into();

        if value.is_empty() {
            return Err(CalibrationParameterError::EmptyTextValue);
        }

        Ok(Self::Text(value))
    }

    /// Creates a finite real vector.
    pub fn vector(values: Vec<f64>) -> Result<Self, CalibrationParameterError> {
        validate_finite_slice(&values, "vector")?;
        Ok(Self::Vector(values))
    }

    /// Creates a finite real matrix.
    pub fn matrix(
        rows: usize,
        columns: usize,
        values: Vec<f64>,
    ) -> Result<Self, CalibrationParameterError> {
        let expected = rows.checked_mul(columns).ok_or(
            CalibrationParameterError::DimensionOverflow,
        )?;

        if expected != values.len() {
            return Err(CalibrationParameterError::MatrixShapeMismatch {
                rows,
                columns,
                values: values.len(),
            });
        }

        validate_finite_slice(&values, "matrix")?;

        Ok(Self::Matrix {
            rows,
            columns,
            values,
        })
    }

    /// Creates a finite complex scalar.
    pub fn complex(
        real: f64,
        imaginary: f64,
    ) -> Result<Self, CalibrationParameterError> {
        ensure_finite(real, "complex real component")?;
        ensure_finite(imaginary, "complex imaginary component")?;

        Ok(Self::Complex { real, imaginary })
    }

    /// Creates a finite complex vector.
    ///
    /// The values are stored as:
    ///
    /// `real0, imaginary0, real1, imaginary1, ...`
    pub fn complex_vector(
        values: Vec<f64>,
    ) -> Result<Self, CalibrationParameterError> {
        if values.len() % 2 != 0 {
            return Err(CalibrationParameterError::ComplexVectorShapeMismatch {
                values: values.len(),
            });
        }

        validate_finite_slice(&values, "complex vector")?;

        Ok(Self::ComplexVector(values))
    }

    /// Creates an opaque structured value.
    ///
    /// The payload is data only. It is never interpreted as executable code.
    pub fn structured(
        schema: impl Into<String>,
        payload: impl Into<String>,
    ) -> Result<Self, CalibrationParameterError> {
        let schema = schema.into();
        let payload = payload.into();

        if schema.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyStructuredSchema);
        }

        Ok(Self::Structured { schema, payload })
    }

    /// Returns whether the value contains only finite numerical values.
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match self {
            Self::Scalar(value) => value.is_finite(),

            Self::Integer(_) | Self::Boolean(_) | Self::Text(_) => true,

            Self::Vector(values) => values.iter().all(|value| value.is_finite()),

            Self::Matrix { values, .. } => {
                values.iter().all(|value| value.is_finite())
            }

            Self::Complex { real, imaginary } => {
                real.is_finite() && imaginary.is_finite()
            }

            Self::ComplexVector(values) => {
                values.iter().all(|value| value.is_finite())
            }

            Self::Structured { .. } => true,
        }
    }

    /// Returns the number of scalar storage elements represented by this
    /// value.
    ///
    /// This is descriptive metadata, not an architectural limit.
    #[must_use]
    pub fn element_count(&self) -> usize {
        match self {
            Self::Scalar(_) => 1,
            Self::Integer(_) => 1,
            Self::Boolean(_) => 1,
            Self::Text(_) => 1,
            Self::Vector(values) => values.len(),
            Self::Matrix { values, .. } => values.len(),
            Self::Complex { .. } => 2,
            Self::ComplexVector(values) => values.len(),
            Self::Structured { .. } => 1,
        }
    }

    /// Validates all internal numerical invariants.
    pub fn validate(&self) -> Result<(), CalibrationParameterError> {
        match self {
            Self::Scalar(value) => ensure_finite(*value, "scalar"),

            Self::Integer(_) | Self::Boolean(_) | Self::Text(_) => Ok(()),

            Self::Vector(values) => validate_finite_slice(values, "vector"),

            Self::Matrix {
                rows,
                columns,
                values,
            } => {
                let expected = rows.checked_mul(*columns).ok_or(
                    CalibrationParameterError::DimensionOverflow,
                )?;

                if expected != values.len() {
                    return Err(
                        CalibrationParameterError::MatrixShapeMismatch {
                            rows: *rows,
                            columns: *columns,
                            values: values.len(),
                        },
                    );
                }

                validate_finite_slice(values, "matrix")
            }

            Self::Complex { real, imaginary } => {
                ensure_finite(*real, "complex real component")?;
                ensure_finite(*imaginary, "complex imaginary component")
            }

            Self::ComplexVector(values) => {
                if values.len() % 2 != 0 {
                    return Err(
                        CalibrationParameterError::ComplexVectorShapeMismatch {
                            values: values.len(),
                        },
                    );
                }

                validate_finite_slice(values, "complex vector")
            }

            Self::Structured { schema, .. } => {
                if schema.trim().is_empty() {
                    return Err(
                        CalibrationParameterError::EmptyStructuredSchema,
                    );
                }

                Ok(())
            }
        }
    }
}

// ============================================================================
// VALUE KIND
// ============================================================================

/// Lightweight description of the representation used by a calibration value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationValueKind {
    Scalar,
    Integer,
    Boolean,
    Text,
    Vector,
    Matrix,
    Complex,
    ComplexVector,
    Structured,
}

impl CalibrationValue {
    /// Returns the representation kind.
    #[must_use]
    pub const fn kind(&self) -> CalibrationValueKind {
        match self {
            Self::Scalar(_) => CalibrationValueKind::Scalar,
            Self::Integer(_) => CalibrationValueKind::Integer,
            Self::Boolean(_) => CalibrationValueKind::Boolean,
            Self::Text(_) => CalibrationValueKind::Text,
            Self::Vector(_) => CalibrationValueKind::Vector,
            Self::Matrix { .. } => CalibrationValueKind::Matrix,
            Self::Complex { .. } => CalibrationValueKind::Complex,
            Self::ComplexVector(_) => CalibrationValueKind::ComplexVector,
            Self::Structured { .. } => CalibrationValueKind::Structured,
        }
    }
}

// ============================================================================
// UNIT
// ============================================================================

/// Provider-neutral unit description.
///
/// `symbol` is the canonical display/interchange symbol.
///
/// `dimension` is an optional semantic dimension identifier. It is intentionally
/// opaque because the complete dimensional-analysis system belongs outside
/// this file.
///
/// `scale_to_base` and `offset_to_base` allow an owning unit subsystem to
/// describe conversion to its selected base representation.
///
/// No unit names are hard-coded into ZQN.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationUnit {
    symbol: String,
    dimension: Option<String>,
    scale_to_base: f64,
    offset_to_base: f64,
}

impl CalibrationUnit {
    /// Creates a unit.
    ///
    /// `scale_to_base` must be finite and non-zero.
    /// `offset_to_base` must be finite.
    pub fn new(
        symbol: impl Into<String>,
        dimension: Option<String>,
        scale_to_base: f64,
        offset_to_base: f64,
    ) -> Result<Self, CalibrationParameterError> {
        let symbol = symbol.into();

        if symbol.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyUnitSymbol);
        }

        ensure_finite(scale_to_base, "unit scale")?;
        ensure_finite(offset_to_base, "unit offset")?;

        if scale_to_base == 0.0 {
            return Err(CalibrationParameterError::ZeroUnitScale);
        }

        Ok(Self {
            symbol,
            dimension,
            scale_to_base,
            offset_to_base,
        })
    }

    /// Creates a dimensionless unit.
    pub fn dimensionless() -> Self {
        Self {
            symbol: String::from("1"),
            dimension: None,
            scale_to_base: 1.0,
            offset_to_base: 0.0,
        }
    }

    /// Returns the unit symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Returns the optional semantic dimension.
    #[must_use]
    pub fn dimension(&self) -> Option<&str> {
        self.dimension.as_deref()
    }

    /// Returns the scale to the owning base representation.
    #[must_use]
    pub const fn scale_to_base(&self) -> f64 {
        self.scale_to_base
    }

    /// Returns the offset to the owning base representation.
    #[must_use]
    pub const fn offset_to_base(&self) -> f64 {
        self.offset_to_base
    }

    /// Validates the unit.
    pub fn validate(&self) -> Result<(), CalibrationParameterError> {
        if self.symbol.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyUnitSymbol);
        }

        ensure_finite(self.scale_to_base, "unit scale")?;
        ensure_finite(self.offset_to_base, "unit offset")?;

        if self.scale_to_base == 0.0 {
            return Err(CalibrationParameterError::ZeroUnitScale);
        }

        Ok(())
    }
}

impl Eq for CalibrationUnit {}

impl std::hash::Hash for CalibrationUnit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        self.dimension.hash(state);
        self.scale_to_base.to_bits().hash(state);
        self.offset_to_base.to_bits().hash(state);
    }
}

// ============================================================================
// UNCERTAINTY
// ============================================================================

/// Explicit uncertainty attached to a calibration parameter.
///
/// The interpretation is intentionally explicit:
///
/// - `Absolute(x)` means absolute uncertainty in the parameter's unit.
/// - `Relative(x)` means fractional uncertainty.
/// - `Interval { lower, upper }` defines an explicit interval around the
///   parameter's nominal value.
///
/// None of these variants silently imply a particular statistical distribution.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationUncertainty {
    /// No uncertainty was supplied.
    None,

    /// Absolute uncertainty.
    Absolute(f64),

    /// Relative uncertainty represented as a fraction.
    Relative(f64),

    /// Explicit lower/upper uncertainty bounds.
    Interval {
        lower: f64,
        upper: f64,
    },
}

impl Default for CalibrationUncertainty {
    fn default() -> Self {
        Self::None
    }
}

impl CalibrationUncertainty {
    /// Creates absolute uncertainty.
    pub fn absolute(value: f64) -> Result<Self, CalibrationParameterError> {
        ensure_finite(value, "absolute uncertainty")?;

        if value < 0.0 {
            return Err(CalibrationParameterError::NegativeUncertainty);
        }

        Ok(Self::Absolute(value))
    }

    /// Creates relative uncertainty.
    ///
    /// `value = 0.01` means 1%.
    pub fn relative(value: f64) -> Result<Self, CalibrationParameterError> {
        ensure_finite(value, "relative uncertainty")?;

        if value < 0.0 {
            return Err(CalibrationParameterError::NegativeUncertainty);
        }

        Ok(Self::Relative(value))
    }

    /// Creates explicit uncertainty bounds.
    pub fn interval(
        lower: f64,
        upper: f64,
    ) -> Result<Self, CalibrationParameterError> {
        ensure_finite(lower, "uncertainty lower bound")?;
        ensure_finite(upper, "uncertainty upper bound")?;

        if lower > upper {
            return Err(CalibrationParameterError::InvalidUncertaintyInterval);
        }

        Ok(Self::Interval { lower, upper })
    }

    /// Validates uncertainty.
    pub fn validate(&self) -> Result<(), CalibrationParameterError> {
        match self {
            Self::None => Ok(()),

            Self::Absolute(value) => {
                ensure_finite(*value, "absolute uncertainty")?;

                if *value < 0.0 {
                    return Err(CalibrationParameterError::NegativeUncertainty);
                }

                Ok(())
            }

            Self::Relative(value) => {
                ensure_finite(*value, "relative uncertainty")?;

                if *value < 0.0 {
                    return Err(CalibrationParameterError::NegativeUncertainty);
                }

                Ok(())
            }

            Self::Interval { lower, upper } => {
                ensure_finite(*lower, "uncertainty lower bound")?;
                ensure_finite(*upper, "uncertainty upper bound")?;

                if lower > upper {
                    return Err(
                        CalibrationParameterError::InvalidUncertaintyInterval,
                    );
                }

                Ok(())
            }
        }
    }
}

impl Eq for CalibrationUncertainty {}

impl std::hash::Hash for CalibrationUncertainty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::None => {}

            Self::Absolute(value) | Self::Relative(value) => {
                value.to_bits().hash(state);
            }

            Self::Interval { lower, upper } => {
                lower.to_bits().hash(state);
                upper.to_bits().hash(state);
            }
        }
    }
}

// ============================================================================
// PARAMETER SCOPE
// ============================================================================

/// Scope of a calibration parameter.
///
/// The scope is intentionally data-driven.
///
/// There is no fixed assumption that calibration applies only to one or two
/// qubits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CalibrationScope {
    /// Parameter applies globally to the calibration target.
    Global,

    /// Parameter applies to one canonical logical qubit.
    LogicalQubit(QubitId),

    /// Parameter applies to one canonical physical qubit.
    PhysicalQubit(PhysicalQubitId),

    /// Parameter applies to an arbitrary set of logical qubits.
    LogicalQubits(Vec<QubitId>),

    /// Parameter applies to an arbitrary set of physical qubits.
    PhysicalQubits(Vec<PhysicalQubitId>),

    /// Parameter applies to one canonical IR operation.
    Operation(OperationId),

    /// Parameter applies to an arbitrary ZQN-owned resource.
    Resource(ZqnObjectId),

    /// Parameter applies to a composite set of scopes.
    Composite(Vec<Self>),
}

impl Default for CalibrationScope {
    fn default() -> Self {
        Self::Global
    }
}

impl CalibrationScope {
    /// Creates a logical-qubit scope.
    #[must_use]
    pub fn logical_qubit(qubit: QubitId) -> Self {
        Self::LogicalQubit(qubit)
    }

    /// Creates a physical-qubit scope.
    #[must_use]
    pub fn physical_qubit(qubit: PhysicalQubitId) -> Self {
        Self::PhysicalQubit(qubit)
    }

    /// Creates an arbitrary logical-qubit scope.
    ///
    /// The order supplied by the caller is retained. Consumers requiring a
    /// canonical ordering should call `canonicalize`.
    #[must_use]
    pub fn logical_qubits(qubits: Vec<QubitId>) -> Self {
        Self::LogicalQubits(qubits)
    }

    /// Creates an arbitrary physical-qubit scope.
    #[must_use]
    pub fn physical_qubits(qubits: Vec<PhysicalQubitId>) -> Self {
        Self::PhysicalQubits(qubits)
    }

    /// Creates an operation scope.
    #[must_use]
    pub fn operation(operation: OperationId) -> Self {
        Self::Operation(operation)
    }

    /// Creates a ZQN-resource scope.
    #[must_use]
    pub fn resource(resource: ZqnObjectId) -> Self {
        Self::Resource(resource)
    }

    /// Creates a composite scope.
    #[must_use]
    pub fn composite(scopes: Vec<Self>) -> Self {
        Self::Composite(scopes)
    }

    /// Returns the number of directly referenced resources.
    ///
    /// This is descriptive only and does not impose a maximum.
    #[must_use]
    pub fn resource_count(&self) -> usize {
        match self {
            Self::Global => 0,

            Self::LogicalQubit(_) |
            Self::PhysicalQubit(_) |
            Self::Operation(_) |
            Self::Resource(_) => 1,

            Self::LogicalQubits(qubits) => qubits.len(),

            Self::PhysicalQubits(qubits) => qubits.len(),

            Self::Composite(scopes) => {
                scopes.iter().map(Self::resource_count).sum()
            }
        }
    }

    /// Validates the structural scope.
    pub fn validate(&self) -> Result<(), CalibrationParameterError> {
        match self {
            Self::Global |
            Self::LogicalQubit(_) |
            Self::PhysicalQubit(_) |
            Self::Operation(_) |
            Self::Resource(_) => Ok(()),

            Self::LogicalQubits(qubits) => {
                validate_unique(qubits)
            }

            Self::PhysicalQubits(qubits) => {
                validate_unique(qubits)
            }

            Self::Composite(scopes) => {
                for scope in scopes {
                    scope.validate()?;
                }

                Ok(())
            }
        }
    }

    /// Returns a deterministic canonicalized copy of the scope.
    ///
    /// Resource identity ordering is semantic-independent and is used only
    /// for deterministic representation, hashing and serialization.
    #[must_use]
    pub fn canonicalized(&self) -> Self {
        match self {
            Self::Global => Self::Global,

            Self::LogicalQubit(id) => Self::LogicalQubit(*id),

            Self::PhysicalQubit(id) => Self::PhysicalQubit(*id),

            Self::Operation(id) => Self::Operation(*id),

            Self::Resource(id) => Self::Resource(*id),

            Self::LogicalQubits(qubits) => {
                let mut result = qubits.clone();
                result.sort();
                result.dedup();
                Self::LogicalQubits(result)
            }

            Self::PhysicalQubits(qubits) => {
                let mut result = qubits.clone();
                result.sort();
                result.dedup();
                Self::PhysicalQubits(result)
            }

            Self::Composite(scopes) => {
                let mut result: Vec<Self> =
                    scopes.iter().map(Self::canonicalized).collect();

                result.sort();
                result.dedup();

                Self::Composite(result)
            }
        }
    }
}

// ============================================================================
// PARAMETER NAME
// ============================================================================

/// Provider-neutral parameter name.
///
/// Names are semantic identifiers, not executable expressions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalibrationParameterName(String);

impl CalibrationParameterName {
    /// Creates a parameter name.
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, CalibrationParameterError> {
        let value = value.into();

        if value.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyParameterName);
        }

        Ok(Self(value))
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CalibrationParameterName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

// ============================================================================
// PROVENANCE REFERENCE
// ============================================================================

/// Explicit provenance reference for a calibration parameter.
///
/// The actual provenance document/data remains owned by the provenance layer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CalibrationParameterProvenance {
    /// Upstream calibration identity, when available.
    calibration_id: Option<CalibrationId>,

    /// Human/provider-neutral source identifier.
    source: Option<String>,

    /// Optional experiment/observation identifier.
    observation: Option<String>,
}

impl CalibrationParameterProvenance {
    /// Creates an empty provenance reference.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            calibration_id: None,
            source: None,
            observation: None,
        }
    }

    /// Creates provenance referencing a calibration identity.
    #[must_use]
    pub const fn from_calibration(calibration_id: CalibrationId) -> Self {
        Self {
            calibration_id: Some(calibration_id),
            source: None,
            observation: None,
        }
    }

    /// Adds a source identifier.
    pub fn with_source(
        mut self,
        source: impl Into<String>,
    ) -> Result<Self, CalibrationParameterError> {
        let source = source.into();

        if source.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyProvenanceField);
        }

        self.source = Some(source);
        Ok(self)
    }

    /// Adds an observation identifier.
    pub fn with_observation(
        mut self,
        observation: impl Into<String>,
    ) -> Result<Self, CalibrationParameterError> {
        let observation = observation.into();

        if observation.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyProvenanceField);
        }

        self.observation = Some(observation);
        Ok(self)
    }

    /// Returns the calibration identity.
    #[must_use]
    pub const fn calibration_id(&self) -> Option<CalibrationId> {
        self.calibration_id
    }

    /// Returns the source identifier.
    #[must_use]
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Returns the observation identifier.
    #[must_use]
    pub fn observation(&self) -> Option<&str> {
        self.observation.as_deref()
    }
}

// ============================================================================
// CALIBRATION PARAMETER
// ============================================================================

/// Immutable production calibration parameter.
///
/// This is the principal public type of this file.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationParameter {
    id: NoiseParameterId,
    name: CalibrationParameterName,
    revision: CalibrationParameterRevision,
    value: CalibrationValue,
    unit: CalibrationUnit,
    uncertainty: CalibrationUncertainty,
    scope: CalibrationScope,
    status: CalibrationParameterStatus,
    provenance: CalibrationParameterProvenance,
    description: Option<String>,
}

impl CalibrationParameter {
    /// Creates a new calibration parameter.
    ///
    /// The constructor validates all local invariants.
    pub fn new(
        id: NoiseParameterId,
        name: CalibrationParameterName,
        value: CalibrationValue,
        unit: CalibrationUnit,
        scope: CalibrationScope,
    ) -> ZqnResult<Self> {
        value
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        unit.validate()
            .map_err(|error| error.into_zqn_error())?;

        scope
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        Ok(Self {
            id,
            name,
            revision: CalibrationParameterRevision::default(),
            value,
            unit,
            uncertainty: CalibrationUncertainty::None,
            scope: scope.canonicalized(),
            status: CalibrationParameterStatus::Unvalidated,
            provenance: CalibrationParameterProvenance::empty(),
            description: None,
        })
    }

    /// Returns the parameter identity.
    #[must_use]
    pub const fn id(&self) -> NoiseParameterId {
        self.id
    }

    /// Returns the parameter name.
    #[must_use]
    pub fn name(&self) -> &CalibrationParameterName {
        &self.name
    }

    /// Returns the parameter revision.
    #[must_use]
    pub const fn revision(&self) -> CalibrationParameterRevision {
        self.revision
    }

    /// Returns the value.
    #[must_use]
    pub fn value(&self) -> &CalibrationValue {
        &self.value
    }

    /// Returns the unit.
    #[must_use]
    pub fn unit(&self) -> &CalibrationUnit {
        &self.unit
    }

    /// Returns uncertainty.
    #[must_use]
    pub fn uncertainty(&self) -> &CalibrationUncertainty {
        &self.uncertainty
    }

    /// Returns the parameter scope.
    #[must_use]
    pub fn scope(&self) -> &CalibrationScope {
        &self.scope
    }

    /// Returns parameter status.
    #[must_use]
    pub const fn status(&self) -> CalibrationParameterStatus {
        self.status
    }

    /// Returns provenance.
    #[must_use]
    pub fn provenance(&self) -> &CalibrationParameterProvenance {
        &self.provenance
    }

    /// Returns the optional description.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns a new parameter with an explicit revision.
    #[must_use]
    pub fn with_revision(
        mut self,
        revision: CalibrationParameterRevision,
    ) -> Self {
        self.revision = revision;
        self
    }

    /// Returns a new parameter with uncertainty.
    pub fn with_uncertainty(
        mut self,
        uncertainty: CalibrationUncertainty,
    ) -> Result<Self, CalibrationParameterError> {
        uncertainty.validate()?;
        self.uncertainty = uncertainty;
        Ok(self)
    }

    /// Returns a new parameter with provenance.
    #[must_use]
    pub fn with_provenance(
        mut self,
        provenance: CalibrationParameterProvenance,
    ) -> Self {
        self.provenance = provenance;
        self
    }

    /// Returns a new parameter with description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, CalibrationParameterError> {
        let description = description.into();

        if description.trim().is_empty() {
            return Err(CalibrationParameterError::EmptyDescription);
        }

        self.description = Some(description);
        Ok(self)
    }

    /// Returns a new parameter marked valid.
    #[must_use]
    pub fn validated(mut self) -> Self {
        self.status = CalibrationParameterStatus::Valid;
        self
    }

    /// Returns a new parameter marked stale.
    #[must_use]
    pub fn stale(mut self) -> Self {
        self.status = CalibrationParameterStatus::Stale;
        self
    }

    /// Returns a new parameter marked invalid.
    #[must_use]
    pub fn invalid(mut self) -> Self {
        self.status = CalibrationParameterStatus::Invalid;
        self
    }

    /// Returns a new parameter marked disabled.
    #[must_use]
    pub fn disabled(mut self) -> Self {
        self.status = CalibrationParameterStatus::Disabled;
        self
    }

    /// Returns a new parameter marked superseded.
    #[must_use]
    pub fn superseded(mut self) -> Self {
        self.status = CalibrationParameterStatus::Superseded;
        self
    }

    /// Validates every invariant owned by this module.
    pub fn validate(&self) -> ZqnResult<()> {
        self.value
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        self.unit
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        self.uncertainty
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        self.scope
            .validate()
            .map_err(|error| error.into_zqn_error())?;

        if self.name.as_str().trim().is_empty() {
            return Err(
                CalibrationParameterError::EmptyParameterName.into_zqn_error()
            );
        }

        Ok(())
    }

    /// Returns the representation kind of the parameter value.
    #[must_use]
    pub fn value_kind(&self) -> CalibrationValueKind {
        self.value.kind()
    }

    /// Returns the number of scalar storage elements in the value.
    #[must_use]
    pub fn element_count(&self) -> usize {
        self.value.element_count()
    }

    /// Returns whether this parameter is usable for execution.
    ///
    /// A parameter is executable only when it is structurally valid and has
    /// status `Valid`.
    pub fn is_usable(&self) -> ZqnResult<bool> {
        self.validate()?;

        Ok(self.status == CalibrationParameterStatus::Valid)
    }
}

// ============================================================================
// ERROR MODEL
// ============================================================================

/// Domain-specific construction/validation failures.
///
/// These are converted into the canonical ZQN error model at public ZQN
/// boundaries.
///
/// This error enum exists to make local constructors precise without making it
/// a second system-wide error hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibrationParameterError {
    EmptyParameterName,
    EmptyUnitSymbol,
    ZeroUnitScale,
    NonFiniteValue {
        field: &'static str,
    },
    NegativeUncertainty,
    InvalidUncertaintyInterval,
    DimensionOverflow,
    MatrixShapeMismatch {
        rows: usize,
        columns: usize,
        values: usize,
    },
    ComplexVectorShapeMismatch {
        values: usize,
    },
    EmptyTextValue,
    EmptyStructuredSchema,
    EmptyProvenanceField,
    EmptyDescription,
    DuplicateScopeResource,
}

impl CalibrationParameterError {
    /// Converts this local diagnostic into the canonical ZQN error model.
    ///
    /// The conversion intentionally uses the central ZQN error vocabulary
    /// rather than exposing a competing top-level error system.
    pub fn into_zqn_error(self) -> crate::quantum::zqn::core::errors::ZqnError {
        use crate::quantum::zqn::core::errors::ZqnError;

        match self {
            Self::EmptyParameterName => {
                ZqnError::invalid_calibration("calibration parameter name is empty")
            }

            Self::EmptyUnitSymbol => {
                ZqnError::invalid_calibration("calibration unit symbol is empty")
            }

            Self::ZeroUnitScale => {
                ZqnError::invalid_calibration(
                    "calibration unit scale cannot be zero",
                )
            }

            Self::NonFiniteValue { field } => {
                ZqnError::invalid_calibration(format!(
                    "calibration parameter contains a non-finite value in {field}"
                ))
            }

            Self::NegativeUncertainty => {
                ZqnError::invalid_calibration(
                    "calibration uncertainty cannot be negative",
                )
            }

            Self::InvalidUncertaintyInterval => {
                ZqnError::invalid_calibration(
                    "calibration uncertainty interval has lower > upper",
                )
            }

            Self::DimensionOverflow => {
                ZqnError::invalid_calibration(
                    "calibration matrix dimensions overflow the host representation",
                )
            }

            Self::MatrixShapeMismatch {
                rows,
                columns,
                values,
            } => {
                ZqnError::invalid_calibration(format!(
                    "calibration matrix shape mismatch: rows={rows}, columns={columns}, values={values}"
                ))
            }

            Self::ComplexVectorShapeMismatch { values } => {
                ZqnError::invalid_calibration(format!(
                    "complex calibration vector requires an even number of scalar components; received {values}"
                ))
            }

            Self::EmptyTextValue => {
                ZqnError::invalid_calibration(
                    "calibration textual value cannot be empty",
                )
            }

            Self::EmptyStructuredSchema => {
                ZqnError::invalid_calibration(
                    "calibration structured value requires a non-empty schema",
                )
            }

            Self::EmptyProvenanceField => {
                ZqnError::invalid_calibration(
                    "calibration provenance field cannot be empty",
                )
            }

            Self::EmptyDescription => {
                ZqnError::invalid_calibration(
                    "calibration parameter description cannot be empty",
                )
            }

            Self::DuplicateScopeResource => {
                ZqnError::invalid_calibration(
                    "calibration parameter scope contains duplicate resources",
                )
            }
        }
    }
}

impl fmt::Display for CalibrationParameterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyParameterName => {
                formatter.write_str("calibration parameter name is empty")
            }

            Self::EmptyUnitSymbol => {
                formatter.write_str("calibration unit symbol is empty")
            }

            Self::ZeroUnitScale => {
                formatter.write_str("calibration unit scale cannot be zero")
            }

            Self::NonFiniteValue { field } => {
                write!(
                    formatter,
                    "calibration parameter contains non-finite value in {field}"
                )
            }

            Self::NegativeUncertainty => {
                formatter.write_str("calibration uncertainty cannot be negative")
            }

            Self::InvalidUncertaintyInterval => {
                formatter.write_str(
                    "calibration uncertainty interval is invalid",
                )
            }

            Self::DimensionOverflow => {
                formatter.write_str(
                    "calibration matrix dimension calculation overflowed",
                )
            }

            Self::MatrixShapeMismatch {
                rows,
                columns,
                values,
            } => {
                write!(
                    formatter,
                    "matrix shape mismatch: {rows}x{columns} requires {} values but received {values}",
                    rows.saturating_mul(*columns)
                )
            }

            Self::ComplexVectorShapeMismatch { values } => {
                write!(
                    formatter,
                    "complex vector requires an even number of values, received {values}"
                )
            }

            Self::EmptyTextValue => {
                formatter.write_str("calibration text value is empty")
            }

            Self::EmptyStructuredSchema => {
                formatter.write_str("structured calibration schema is empty")
            }

            Self::EmptyProvenanceField => {
                formatter.write_str("calibration provenance field is empty")
            }

            Self::EmptyDescription => {
                formatter.write_str("calibration parameter description is empty")
            }

            Self::DuplicateScopeResource => {
                formatter.write_str(
                    "calibration parameter scope contains duplicate resources",
                )
            }
        }
    }
}

impl std::error::Error for CalibrationParameterError {}

// ============================================================================
// HELPERS
// ============================================================================

fn ensure_finite(
    value: f64,
    field: &'static str,
) -> Result<(), CalibrationParameterError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(CalibrationParameterError::NonFiniteValue { field })
    }
}

fn validate_finite_slice(
    values: &[f64],
    field: &'static str,
) -> Result<(), CalibrationParameterError> {
    for &value in values {
        ensure_finite(value, field)?;
    }

    Ok(())
}

fn validate_unique<T>(
    values: &[T],
) -> Result<(), CalibrationParameterError>
where
    T: Ord,
{
    if values.len() < 2 {
        return Ok(());
    }

    let mut sorted = values.to_vec();
    sorted.sort();

    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(CalibrationParameterError::DuplicateScopeResource);
        }
    }

    Ok(())
}

// ============================================================================
// DETERMINISTIC HASH SUPPORT
// ============================================================================

impl Eq for CalibrationValue {}

impl std::hash::Hash for CalibrationValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Scalar(value) => value.to_bits().hash(state),

            Self::Integer(value) => value.hash(state),

            Self::Boolean(value) => value.hash(state),

            Self::Text(value) => value.hash(state),

            Self::Vector(values) => {
                values.len().hash(state);

                for value in values {
                    value.to_bits().hash(state);
                }
            }

            Self::Matrix {
                rows,
                columns,
                values,
            } => {
                rows.hash(state);
                columns.hash(state);
                values.len().hash(state);

                for value in values {
                    value.to_bits().hash(state);
                }
            }

            Self::Complex { real, imaginary } => {
                real.to_bits().hash(state);
                imaginary.to_bits().hash(state);
            }

            Self::ComplexVector(values) => {
                values.len().hash(state);

                for value in values {
                    value.to_bits().hash(state);
                }
            }

            Self::Structured { schema, payload } => {
                schema.hash(state);
                payload.hash(state);
            }
        }
    }
}

// ============================================================================
// DETERMINISTIC ORDERING
// ============================================================================

impl Ord for CalibrationValue {
    fn cmp(&self, other: &Self) -> Ordering {
        fn kind_rank(value: &CalibrationValue) -> u8 {
            match value {
                CalibrationValue::Scalar(_) => 0,
                CalibrationValue::Integer(_) => 1,
                CalibrationValue::Boolean(_) => 2,
                CalibrationValue::Text(_) => 3,
                CalibrationValue::Vector(_) => 4,
                CalibrationValue::Matrix { .. } => 5,
                CalibrationValue::Complex { .. } => 6,
                CalibrationValue::ComplexVector(_) => 7,
                CalibrationValue::Structured { .. } => 8,
            }
        }

        let rank_order = kind_rank(self).cmp(&kind_rank(other));

        if rank_order != Ordering::Equal {
            return rank_order;
        }

        match (self, other) {
            (Self::Scalar(a), Self::Scalar(b)) => {
                a.to_bits().cmp(&b.to_bits())
            }

            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),

            (Self::Boolean(a), Self::Boolean(b)) => a.cmp(b),

            (Self::Text(a), Self::Text(b)) => a.cmp(b),

            (Self::Vector(a), Self::Vector(b)) => compare_f64_slices(a, b),

            (
                Self::Matrix {
                    rows: a_rows,
                    columns: a_columns,
                    values: a_values,
                },
                Self::Matrix {
                    rows: b_rows,
                    columns: b_columns,
                    values: b_values,
                },
            ) => a_rows
                .cmp(b_rows)
                .then_with(|| a_columns.cmp(b_columns))
                .then_with(|| compare_f64_slices(a_values, b_values)),

            (
                Self::Complex {
                    real: a_real,
                    imaginary: a_imaginary,
                },
                Self::Complex {
                    real: b_real,
                    imaginary: b_imaginary,
                },
            ) => a_real
                .to_bits()
                .cmp(&b_real.to_bits())
                .then_with(|| a_imaginary.to_bits().cmp(&b_imaginary.to_bits())),

            (Self::ComplexVector(a), Self::ComplexVector(b)) => {
                compare_f64_slices(a, b)
            }

            (
                Self::Structured {
                    schema: a_schema,
                    payload: a_payload,
                },
                Self::Structured {
                    schema: b_schema,
                    payload: b_payload,
                },
            ) => a_schema.cmp(b_schema).then_with(|| a_payload.cmp(b_payload)),

            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for CalibrationValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_f64_slices(left: &[f64], right: &[f64]) -> Ordering {
    left.len()
        .cmp(&right.len())
        .then_with(|| {
            left.iter()
                .zip(right.iter())
                .map(|(a, b)| a.to_bits().cmp(&b.to_bits()))
                .find(|ordering| *ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_rejects_non_finite_values() {
        assert!(CalibrationValue::scalar(f64::NAN).is_err());
        assert!(CalibrationValue::scalar(f64::INFINITY).is_err());
        assert!(CalibrationValue::scalar(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn vector_rejects_non_finite_values() {
        assert!(
            CalibrationValue::vector(vec![1.0, f64::NAN]).is_err()
        );

        assert!(
            CalibrationValue::vector(vec![1.0, f64::INFINITY]).is_err()
        );
    }

    #[test]
    fn matrix_shape_is_checked_without_overflow() {
        assert!(
            CalibrationValue::matrix(2, 3, vec![1.0; 5]).is_err()
        );

        assert!(
            CalibrationValue::matrix(2, 3, vec![1.0; 6]).is_ok()
        );
    }

    #[test]
    fn complex_vector_requires_pairs() {
        assert!(
            CalibrationValue::complex_vector(vec![1.0]).is_err()
        );

        assert!(
            CalibrationValue::complex_vector(vec![1.0, 2.0]).is_ok()
        );
    }

    #[test]
    fn unit_rejects_zero_scale() {
        assert!(
            CalibrationUnit::new("x", None, 0.0, 0.0).is_err()
        );
    }

    #[test]
    fn uncertainty_rejects_negative_absolute_value() {
        assert!(
            CalibrationUncertainty::absolute(-1.0).is_err()
        );
    }

    #[test]
    fn uncertainty_interval_is_ordered() {
        assert!(
            CalibrationUncertainty::interval(2.0, 1.0).is_err()
        );

        assert!(
            CalibrationUncertainty::interval(1.0, 2.0).is_ok()
        );
    }

    #[test]
    fn scope_canonicalization_is_deterministic() {
        let scope = CalibrationScope::LogicalQubits(vec![
            QubitId::new(3),
            QubitId::new(1),
            QubitId::new(2),
        ]);

        let canonical = scope.canonicalized();

        assert_eq!(
            canonical,
            CalibrationScope::LogicalQubits(vec![
                QubitId::new(1),
                QubitId::new(2),
                QubitId::new(3),
            ])
        );
    }

    #[test]
    fn duplicate_scope_resources_are_rejected() {
        let scope = CalibrationScope::LogicalQubits(vec![
            QubitId::new(1),
            QubitId::new(1),
        ]);

        assert!(scope.validate().is_err());
    }

    #[test]
    fn parameter_is_immutable_by_construction() {
        let name =
            CalibrationParameterName::new("frequency").expect("valid name");

        let value =
            CalibrationValue::scalar(5.0e9).expect("finite value");

        let unit = CalibrationUnit::new(
            "Hz",
            Some(String::from("frequency")),
            1.0,
            0.0,
        )
        .expect("valid unit");

        let parameter = CalibrationParameter::new(
            NoiseParameterId::new(1),
            name,
            value,
            unit,
            CalibrationScope::physical_qubit(
                PhysicalQubitId::new(0),
            ),
        )
        .expect("valid parameter");

        assert_eq!(
            parameter.value_kind(),
            CalibrationValueKind::Scalar
        );

        assert_eq!(parameter.element_count(), 1);
        assert_eq!(
            parameter.status(),
            CalibrationParameterStatus::Unvalidated
        );
    }

    #[test]
    fn parameter_validation_is_deterministic() {
        let name =
            CalibrationParameterName::new("t1").expect("valid name");

        let value =
            CalibrationValue::scalar(100.0).expect("finite value");

        let unit = CalibrationUnit::new(
            "s",
            Some(String::from("time")),
            1.0,
            0.0,
        )
        .expect("valid unit");

        let parameter = CalibrationParameter::new(
            NoiseParameterId::new(7),
            name,
            value,
            unit,
            CalibrationScope::Global,
        )
        .expect("valid parameter");

        assert!(parameter.validate().is_ok());
        assert!(parameter.validate().is_ok());
    }

    #[test]
    fn provenance_is_explicit() {
        let provenance =
            CalibrationParameterProvenance::from_calibration(
                CalibrationId::new(42),
            )
            .with_source("characterization")
            .expect("valid source")
            .with_observation("experiment-1")
            .expect("valid observation");

        assert_eq!(
            provenance.calibration_id(),
            Some(CalibrationId::new(42))
        );

        assert_eq!(
            provenance.source(),
            Some("characterization")
        );

        assert_eq!(
            provenance.observation(),
            Some("experiment-1")
        );
    }

    #[test]
    fn structured_values_are_data_not_code() {
        let value = CalibrationValue::structured(
            "example.schema",
            "{\"value\": 1}",
        )
        .expect("valid structured value");

        assert_eq!(
            value.kind(),
            CalibrationValueKind::Structured
        );
    }

    #[test]
    fn no_artificial_qubit_limit_is_encoded() {
        let scope = CalibrationScope::physical_qubit(
            PhysicalQubitId::new(u64::MAX),
        );

        assert!(scope.validate().is_ok());
    }
}