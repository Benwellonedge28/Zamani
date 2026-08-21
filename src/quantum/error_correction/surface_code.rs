//! Zamani Quantum Error Correction — Rotated Planar Surface Code.
//!
//! Production-grade mathematical/topological model of the canonical rotated
//! planar surface code.
//!
//! # Ownership
//!
//! This module owns:
//!
//! - rotated planar surface-code topology;
//! - physical data-qubit coordinates;
//! - deterministic qubit indexing;
//! - stabilizer topology;
//! - logical X/Z topology;
//! - topology-to-stabilizer algebra conversion;
//! - topology validation;
//! - resource preflight;
//! - exact-distance verification delegation.
//!
//! This module does NOT own:
//!
//! - generic Pauli algebra;
//! - generic stabilizer algebra;
//! - decoder algorithms;
//! - MWPM;
//! - Union-Find;
//! - QPU execution;
//! - streaming;
//! - distributed execution;
//! - checkpoint persistence;
//! - telemetry transport;
//! - capability authorization.
//!
//! Those responsibilities belong to their respective modules.
//!
//! # Code family
//!
//! For odd distance `d >= 3`:
//!
//! ```text
//! physical data qubits = d²
//! stabilizer generators = d² - 1
//! logical qubits = 1
//! code distance = d
//! ```
//!
//! Data qubits use deterministic row-major indexing:
//!
//! ```text
//! q(r,c) = r*d + c
//! ```
//!
//! # Construction contract
//!
//! Construction is always:
//!
//! ```text
//! untrusted distance
//!        ↓
//! policy validation
//!        ↓
//! checked resource calculation
//!        ↓
//! memory preflight
//!        ↓
//! stabilizer/logical-weight preflight
//!        ↓
//! allocation
//!        ↓
//! deterministic topology construction
//!        ↓
//! mathematical validation
//! ```
//!
//! No topology allocation is performed before the requested workload has
//! passed the configured QEC policy.
//!
//! # Integration
//!
//! ```text
//! arithmetic.rs
//!      │
//!      ▼
//! surface_code.rs
//!      │
//!      ├──────────────► stabilizer.rs
//!      │
//!      ├──────────────► distance.rs
//!      │
//!      ├──────────────► decoding_graph.rs
//!      │
//!      ├──────────────► surface_coder.rs
//!      │
//!      └──────────────► decoder.rs
//! ```
//!
//! `limits.rs` remains the single source of declarative resource policy.
//!
//! `errors.rs` remains the canonical QEC error boundary.
//!
//! `stabilizer.rs` owns generic binary-symplectic algebra.
//!
//! `distance.rs` owns exact distance verification.
//!
//! `surface_coder.rs` owns circuit/encoding/decoding integration.
//!
//! # Rust compatibility
//!
//! This implementation targets Rust 1.97.1 and uses stable standard-library
//! facilities only.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::arithmetic::{
    checked_add_usize,
    checked_mul_add_usize,
    checked_mul_usize,
    checked_sub_usize,
};
use super::distance;
use super::errors::{
    QecError,
    ResourceKind,
};
use super::limits::{
    LimitError,
    LimitKind,
    QecLimits,
};
use super::stabilizer::{
    Pauli,
    PauliString,
    QubitIndex,
    StabilizerError,
    StabilizerGenerator,
    StabilizerGroup,
    Syndrome,
};

// ============================================================================
// Coordinate
// ============================================================================

/// Two-dimensional coordinate of a physical data qubit.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    #[must_use]
    pub const fn new(
        row: usize,
        column: usize,
    ) -> Self {
        Self {
            row,
            column,
        }
    }

    #[must_use]
    pub const fn row(
        self,
    ) -> usize {
        self.row
    }

    #[must_use]
    pub const fn column(
        self,
    ) -> usize {
        self.column
    }
}

impl fmt::Display for Coordinate {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "({}, {})",
            self.row,
            self.column
        )
    }
}

// ============================================================================
// Data qubit
// ============================================================================

/// A physical data qubit in the surface-code lattice.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct DataQubit {
    index: QubitIndex,
    coordinate: Coordinate,
}

impl DataQubit {
    #[must_use]
    pub const fn new(
        index: QubitIndex,
        coordinate: Coordinate,
    ) -> Self {
        Self {
            index,
            coordinate,
        }
    }

    #[must_use]
    pub const fn index(
        self,
    ) -> QubitIndex {
        self.index
    }

    #[must_use]
    pub const fn coordinate(
        self,
    ) -> Coordinate {
        self.coordinate
    }
}

// ============================================================================
// Stabilizer kind
// ============================================================================

/// CSS stabilizer type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum StabilizerKind {
    X,
    Z,
}

impl StabilizerKind {
    #[must_use]
    pub const fn pauli(
        self,
    ) -> Pauli {
        match self {
            Self::X => Pauli::X,
            Self::Z => Pauli::Z,
        }
    }
}

impl fmt::Display for StabilizerKind {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::Z => write!(f, "Z"),
        }
    }
}

// ============================================================================
// Boundary
// ============================================================================

/// Physical boundary on which a weight-2 stabilizer resides.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum Boundary {
    Top,
    Bottom,
    Left,
    Right,
}

impl fmt::Display for Boundary {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Bottom => write!(f, "bottom"),
            Self::Left => write!(f, "left"),
            Self::Right => write!(f, "right"),
        }
    }
}

// ============================================================================
// Resource estimate
// ============================================================================

/// Deterministic resource estimate for a surface-code topology.
///
/// This is a preflight estimate. Runtime consumption belongs to
/// `resources.rs`.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
)]
pub struct SurfaceCodeResourceEstimate {
    num_qubits: usize,
    num_stabilizers: usize,
    estimated_memory_bytes: u64,
}

impl SurfaceCodeResourceEstimate {
    #[must_use]
    pub const fn num_qubits(
        self,
    ) -> usize {
        self.num_qubits
    }

    #[must_use]
    pub const fn num_stabilizers(
        self,
    ) -> usize {
        self.num_stabilizers
    }

    #[must_use]
    pub const fn estimated_memory_bytes(
        self,
    ) -> u64 {
        self.estimated_memory_bytes
    }
}

// ============================================================================
// Surface-code errors
// ============================================================================

/// Errors produced by the surface-code topology/model layer.
#[derive(Debug)]
pub enum SurfaceCodeError {
    InvalidDistance {
        distance: usize,
    },

    Limit(LimitError),

    ArithmeticOverflow {
        resource: &'static str,
    },

    MemoryPreflightExceeded {
        estimated: u64,
        maximum: u64,
    },

    EmptyStabilizer {
        id: usize,
    },

    DuplicateQubitInStabilizer {
        id: usize,
        qubit: QubitIndex,
    },

    InvalidStabilizerWeight {
        id: usize,
        expected: usize,
        actual: usize,
    },

    NonexistentQubit {
        stabilizer: usize,
        qubit: QubitIndex,
    },

    DuplicateStabilizerSupport {
        first: usize,
        second: usize,
    },

    DuplicateStabilizerId {
        id: usize,
    },

    InvalidStabilizerCount {
        expected: usize,
        actual: usize,
    },

    InvalidDataQubitCount {
        expected: usize,
        actual: usize,
    },

    CoordinateOutOfRange {
        coordinate: Coordinate,
        distance: usize,
    },

    CoordinateIndexMismatch {
        qubit: QubitIndex,
        expected: QubitIndex,
    },

    FaceOutOfRange {
        row: usize,
        column: usize,
        distance: usize,
    },

    InvalidBoundaryTopology {
        id: usize,
        boundary: Boundary,
        kind: StabilizerKind,
    },

    InvalidBoundaryParity {
        id: usize,
        boundary: Boundary,
    },

    InvalidBulkTopology {
        id: usize,
    },

    NonCommutingStabilizers {
        first: usize,
        second: usize,
    },

    IdentityLogicalOperator {
        name: &'static str,
    },

    InvalidLogicalWeight {
        name: &'static str,
        expected: usize,
        actual: usize,
    },

    LogicalOperatorDoesNotCommute {
        logical: &'static str,
        stabilizer: usize,
    },

    LogicalOperatorsDoNotAnticommute,

    InvalidLogicalQubitCount {
        expected: usize,
        actual: usize,
    },

    Stabilizer(StabilizerError),

    DistanceVerification(String),

    DimensionMismatch {
        expected: usize,
        actual: usize,
    },
}

impl From<StabilizerError> for SurfaceCodeError {
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

impl From<SurfaceCodeError> for QecError {
    fn from(
        error: SurfaceCodeError,
    ) -> Self {
        match error {
            SurfaceCodeError::InvalidDistance { distance } => {
                QecError::invalid_topology(
                    format!(
                        "invalid surface-code distance {distance}; \
                         distance must be odd and >= 3"
                    ),
                )
            }

            SurfaceCodeError::Limit(error) => {
                limit_error_to_qec_error(error)
            }

            SurfaceCodeError::ArithmeticOverflow {
                resource,
            } => {
                QecError::NumericalFailure {
                    operation:
                        super::errors::NumericalOperation::IndexCalculation,
                    message:
                        format!(
                            "surface-code arithmetic overflow while \
                             calculating {resource}"
                        ),
                }
            }

            SurfaceCodeError::MemoryPreflightExceeded {
                estimated,
                maximum,
            } => {
                QecError::MemoryLimitExceeded {
                    requested_bytes: estimated,
                    current_bytes: 0,
                    limit_bytes: maximum,
                    message:
                        "surface-code construction memory preflight \
                         exceeded the configured memory limit"
                            .to_string(),
                }
            }

            SurfaceCodeError::Stabilizer(error) => {
                QecError::invalid_stabilizer(
                    error.to_string(),
                )
            }

            SurfaceCodeError::DistanceVerification(error) => {
                QecError::invalid_topology(error)
            }

            SurfaceCodeError::DimensionMismatch {
                expected,
                actual,
            } => {
                QecError::invalid_input(
                    format!(
                        "surface-code Pauli dimension mismatch: \
                         expected {expected}, actual {actual}"
                    ),
                )
            }

            other => {
                QecError::invalid_topology(
                    other.to_string(),
                )
            }
        }
    }
}

impl fmt::Display for SurfaceCodeError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDistance { distance } => {
                write!(
                    f,
                    "invalid surface-code distance {}; \
                     distance must be odd and >= 3",
                    distance
                )
            }

            Self::Limit(error) => {
                write!(
                    f,
                    "QEC resource limit exceeded: {error}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "surface-code resource calculation \
                     overflowed for {resource}"
                )
            }

            Self::MemoryPreflightExceeded {
                estimated,
                maximum,
            } => {
                write!(
                    f,
                    "surface-code memory preflight {} bytes \
                     exceeds configured maximum {} bytes",
                    estimated,
                    maximum
                )
            }

            Self::EmptyStabilizer { id } => {
                write!(
                    f,
                    "stabilizer {id} has empty support"
                )
            }

            Self::DuplicateQubitInStabilizer {
                id,
                qubit,
            } => {
                write!(
                    f,
                    "stabilizer {id} contains duplicate qubit {qubit}"
                )
            }

            Self::InvalidStabilizerWeight {
                id,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "stabilizer {id} has weight {actual}; \
                     expected {expected}"
                )
            }

            Self::NonexistentQubit {
                stabilizer,
                qubit,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} references \
                     nonexistent qubit {qubit}"
                )
            }

            Self::DuplicateStabilizerSupport {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} \
                     have identical support"
                )
            }

            Self::DuplicateStabilizerId { id } => {
                write!(
                    f,
                    "duplicate stabilizer id {id}"
                )
            }

            Self::InvalidStabilizerCount {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "invalid stabilizer count {actual}; \
                     expected {expected}"
                )
            }

            Self::InvalidDataQubitCount {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "invalid data-qubit count {actual}; \
                     expected {expected}"
                )
            }

            Self::CoordinateOutOfRange {
                coordinate,
                distance,
            } => {
                write!(
                    f,
                    "coordinate {coordinate} is outside \
                     distance-{distance} lattice"
                )
            }

            Self::CoordinateIndexMismatch {
                qubit,
                expected,
            } => {
                write!(
                    f,
                    "qubit {qubit} has incorrect expected index {expected}"
                )
            }

            Self::FaceOutOfRange {
                row,
                column,
                distance,
            } => {
                write!(
                    f,
                    "face ({row}, {column}) is outside \
                     distance-{distance} lattice"
                )
            }

            Self::InvalidBoundaryTopology {
                id,
                boundary,
                kind,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid \
                     {boundary} boundary type {kind}"
                )
            }

            Self::InvalidBoundaryParity {
                id,
                boundary,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid alternating \
                     placement on {boundary} boundary"
                )
            }

            Self::InvalidBulkTopology { id } => {
                write!(
                    f,
                    "bulk stabilizer {id} has invalid \
                     checkerboard topology"
                )
            }

            Self::NonCommutingStabilizers {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} anticommute"
                )
            }

            Self::IdentityLogicalOperator { name } => {
                write!(
                    f,
                    "logical operator {name} is identity"
                )
            }

            Self::InvalidLogicalWeight {
                name,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "logical operator {name} has weight {actual}; \
                     expected {expected}"
                )
            }

            Self::LogicalOperatorDoesNotCommute {
                logical,
                stabilizer,
            } => {
                write!(
                    f,
                    "logical operator {logical} does not commute \
                     with stabilizer {stabilizer}"
                )
            }

            Self::LogicalOperatorsDoNotAnticommute => {
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::InvalidLogicalQubitCount {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "encoded logical-qubit count {actual}; \
                     expected {expected}"
                )
            }

            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::DistanceVerification(error) => {
                write!(
                    f,
                    "distance verification failed: {error}"
                )
            }

            Self::DimensionMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Pauli operator has {actual} qubits; \
                     expected {expected}"
                )
            }
        }
    }
}

impl std::error::Error for SurfaceCodeError {}

// ============================================================================
// Surface stabilizer
// ============================================================================

/// Explicit surface-code stabilizer topology.
///
/// The support contains exactly:
///
/// - four qubits for a bulk stabilizer;
/// - two qubits for a boundary stabilizer.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct SurfaceStabilizer {
    id: usize,
    kind: StabilizerKind,
    support: Vec<QubitIndex>,
    boundary: Option<Boundary>,
}

impl SurfaceStabilizer {
    pub fn new(
        id: usize,
        kind: StabilizerKind,
        mut support: Vec<QubitIndex>,
        boundary: Option<Boundary>,
    ) -> Result<Self, SurfaceCodeError> {
        if support.is_empty() {
            return Err(
                SurfaceCodeError::EmptyStabilizer {
                    id,
                },
            );
        }

        support.sort_unstable();

        for pair in support.windows(2) {
            if pair[0] == pair[1] {
                return Err(
                    SurfaceCodeError::DuplicateQubitInStabilizer {
                        id,
                        qubit: pair[0],
                    },
                );
            }
        }

        let expected_weight =
            if boundary.is_some() {
                2
            } else {
                4
            };

        if support.len() != expected_weight {
            return Err(
                SurfaceCodeError::InvalidStabilizerWeight {
                    id,
                    expected: expected_weight,
                    actual: support.len(),
                },
            );
        }

        Ok(Self {
            id,
            kind,
            support,
            boundary,
        })
    }

    #[must_use]
    pub const fn id(
        &self,
    ) -> usize {
        self.id
    }

    #[must_use]
    pub const fn kind(
        &self,
    ) -> StabilizerKind {
        self.kind
    }

    #[must_use]
    pub fn support(
        &self,
    ) -> &[QubitIndex] {
        &self.support
    }

    #[must_use]
    pub const fn boundary(
        &self,
    ) -> Option<Boundary> {
        self.boundary
    }

    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.support.len()
    }

    #[must_use]
    pub const fn is_boundary(
        &self,
    ) -> bool {
        self.boundary.is_some()
    }

    /// Converts this topology into a generic Pauli representation.
    pub fn pauli_string(
        &self,
        num_qubits: usize,
    ) -> Result<PauliString, SurfaceCodeError> {
        let mut paulis =
            vec![Pauli::I; num_qubits];

        for &qubit in &self.support {
            let index = qubit.index();

            if index >= num_qubits {
                return Err(
                    SurfaceCodeError::NonexistentQubit {
                        stabilizer: self.id,
                        qubit,
                    },
                );
            }

            paulis[index] =
                self.kind.pauli();
        }

        Ok(
            PauliString::from_paulis(
                &paulis,
            )
        )
    }

    /// Converts the topology into the generic stabilizer IR.
    pub fn generator(
        &self,
        num_qubits: usize,
    ) -> Result<StabilizerGenerator, SurfaceCodeError> {
        Ok(
            StabilizerGenerator::new(
                self.id,
                self.pauli_string(num_qubits)?,
            )?
        )
    }
}

// ============================================================================
// Logical operator
// ============================================================================

/// Canonical logical Pauli operator.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct LogicalOperator {
    name: &'static str,
    kind: StabilizerKind,
    operator: PauliString,
}

impl LogicalOperator {
    pub fn new(
        name: &'static str,
        kind: StabilizerKind,
        operator: PauliString,
    ) -> Result<Self, SurfaceCodeError> {
        if operator.is_identity() {
            return Err(
                SurfaceCodeError::IdentityLogicalOperator {
                    name,
                },
            );
        }

        Ok(Self {
            name,
            kind,
            operator,
        })
    }

    #[must_use]
    pub const fn name(
        &self,
    ) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn kind(
        &self,
    ) -> StabilizerKind {
        self.kind
    }

    #[must_use]
    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    #[must_use]
    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }
}

// ============================================================================
// Surface code
// ============================================================================

/// Canonical rotated planar surface-code topology.
#[derive(
    Debug,
    Clone,
)]
pub struct SurfaceCode {
    distance: usize,
    data_qubits: Vec<DataQubit>,
    stabilizers: Vec<SurfaceStabilizer>,
    logical_x: LogicalOperator,
    logical_z: LogicalOperator,
}

impl SurfaceCode {
    /// Constructs a surface code using the canonical QEC limits.
    pub fn new(
        distance: usize,
    ) -> Result<Self, SurfaceCodeError> {
        let limits =
            QecLimits::default();

        Self::new_with_limits(
            distance,
            &limits,
        )
    }

    /// Constructs a surface code under explicit QEC resource policy.
    ///
    /// This is the preferred production entry point when a validated
    /// `QecConfig` owns the policy.
    pub fn new_with_limits(
        distance: usize,
        limits: &QecLimits,
    ) -> Result<Self, SurfaceCodeError> {
        let estimate =
            preflight(
                distance,
                limits,
            )?;

        let data_qubits =
            build_data_qubits(
                distance,
                estimate.num_qubits,
            )?;

        let stabilizers =
            build_stabilizers(
                distance,
                limits,
            )?;

        let logical_x =
            build_logical_x(
                distance,
                limits,
            )?;

        let logical_z =
            build_logical_z(
                distance,
                limits,
            )?;

        let code = Self {
            distance,
            data_qubits,
            stabilizers,
            logical_x,
            logical_z,
        };

        code.validate_with_limits(
            limits,
        )?;

        Ok(code)
    }

    /// Compatibility constructor for explicit distance construction.
    pub fn from_distance(
        distance: usize,
    ) -> Result<Self, SurfaceCodeError> {
        Self::new(distance)
    }

    // ------------------------------------------------------------------------
    // Basic properties
    // ------------------------------------------------------------------------

    #[must_use]
    pub const fn distance(
        &self,
    ) -> usize {
        self.distance
    }

    #[must_use]
    pub fn num_data_qubits(
        &self,
    ) -> usize {
        self.data_qubits.len()
    }

    #[must_use]
    pub fn num_stabilizers(
        &self,
    ) -> usize {
        self.stabilizers.len()
    }

    /// The canonical rotated planar patch encodes one logical qubit.
    #[must_use]
    pub const fn num_logical_qubits(
        &self,
    ) -> usize {
        1
    }

    #[must_use]
    pub fn data_qubits(
        &self,
    ) -> &[DataQubit] {
        &self.data_qubits
    }

    #[must_use]
    pub fn stabilizers(
        &self,
    ) -> &[SurfaceStabilizer] {
        &self.stabilizers
    }

    #[must_use]
    pub fn logical_x(
        &self,
    ) -> &LogicalOperator {
        &self.logical_x
    }

    #[must_use]
    pub fn logical_z(
        &self,
    ) -> &LogicalOperator {
        &self.logical_z
    }

    /// Returns the conservative construction-memory estimate.
    #[must_use]
    pub fn estimated_memory_bytes(
        &self,
    ) -> u64 {
        estimate_memory_bytes(
            self.distance,
        )
        .unwrap_or(u64::MAX)
    }

    /// Returns the complete deterministic preflight estimate.
    pub fn resource_estimate(
        distance: usize,
    ) -> Result<
        SurfaceCodeResourceEstimate,
        SurfaceCodeError,
    > {
        let limits =
            QecLimits::default();

        preflight(
            distance,
            &limits,
        )
        .map(|estimate| {
            SurfaceCodeResourceEstimate {
                num_qubits:
                    estimate.num_qubits,
                num_stabilizers:
                    estimate.num_stabilizers,
                estimated_memory_bytes:
                    estimate.estimated_memory_bytes,
            }
        })
    }

    /// Returns a resource estimate under explicit limits.
    pub fn resource_estimate_with_limits(
        distance: usize,
        limits: &QecLimits,
    ) -> Result<
        SurfaceCodeResourceEstimate,
        SurfaceCodeError,
    > {
        preflight(
            distance,
            limits,
        )
        .map(|estimate| {
            SurfaceCodeResourceEstimate {
                num_qubits:
                    estimate.num_qubits,
                num_stabilizers:
                    estimate.num_stabilizers,
                estimated_memory_bytes:
                    estimate.estimated_memory_bytes,
            }
        })
    }

    // ------------------------------------------------------------------------
    // Coordinate topology
    // ------------------------------------------------------------------------

    /// Converts a lattice coordinate into a physical data qubit.
    pub fn qubit_at(
        &self,
        coordinate: Coordinate,
    ) -> Result<DataQubit, SurfaceCodeError> {
        if coordinate.row() >= self.distance
            || coordinate.column()
                >= self.distance
        {
            return Err(
                SurfaceCodeError::CoordinateOutOfRange {
                    coordinate,
                    distance: self.distance,
                },
            );
        }

        let index =
            checked_qubit_index(
                self.distance,
                coordinate.row(),
                coordinate.column(),
            )?;

        Ok(
            self.data_qubits[
                index.index()
            ],
        )
    }

    /// Returns the lattice coordinate of a physical qubit.
    pub fn coordinate_of(
        &self,
        qubit: QubitIndex,
    ) -> Result<Coordinate, SurfaceCodeError> {
        let index =
            qubit.index();

        if index >= self.data_qubits.len() {
            return Err(
                SurfaceCodeError::NonexistentQubit {
                    stabilizer: usize::MAX,
                    qubit,
                },
            );
        }

        Ok(
            self.data_qubits[index]
                .coordinate(),
        )
    }

    /// Returns the four data qubits surrounding a bulk plaquette.
    pub fn face_qubits(
        &self,
        row: usize,
        column: usize,
    ) -> Result<
        [QubitIndex; 4],
        SurfaceCodeError,
    > {
        let max_face =
            self.distance
                .checked_sub(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code face dimension",
                    },
                )?;

        if row >= max_face
            || column >= max_face
        {
            return Err(
                SurfaceCodeError::FaceOutOfRange {
                    row,
                    column,
                    distance: self.distance,
                },
            );
        }

        let d =
            self.distance;

        Ok([
            checked_qubit_index(
                d,
                row,
                column,
            )?,
            checked_qubit_index(
                d,
                row,
                checked_add_usize(
                    column,
                    1,
                )?,
            )?,
            checked_qubit_index(
                d,
                checked_add_usize(
                    row,
                    1,
                )?,
                column,
            )?,
            checked_qubit_index(
                d,
                checked_add_usize(
                    row,
                    1,
                )?,
                checked_add_usize(
                    column,
                    1,
                )?,
            )?,
        ])
    }

    // ------------------------------------------------------------------------
    // Stabilizer representation
    // ------------------------------------------------------------------------

    /// Converts the topology into the generic stabilizer group.
    ///
    /// `stabilizer.rs` owns the generic algebra. This method only performs
    /// the topology-to-algebra representation bridge.
    pub fn stabilizer_group(
        &self,
    ) -> Result<
        StabilizerGroup,
        SurfaceCodeError,
    > {
        let mut group =
            StabilizerGroup::new(
                self.num_data_qubits(),
            )?;

        for stabilizer
            in &self.stabilizers
        {
            group.add_generator(
                stabilizer.generator(
                    self.num_data_qubits(),
                )?,
            )?;
        }

        Ok(group)
    }

    /// Extracts the syndrome of a Pauli error.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<
        Syndrome,
        SurfaceCodeError,
    > {
        if error.num_qubits()
            != self.num_data_qubits()
        {
            return Err(
                SurfaceCodeError::DimensionMismatch {
                    expected:
                        self.num_data_qubits(),
                    actual:
                        error.num_qubits(),
                },
            );
        }

        let group =
            self.stabilizer_group()?;

        Ok(
            group.syndrome(error)?,
        )
    }

    // ------------------------------------------------------------------------
    // Mathematical validation
    // ------------------------------------------------------------------------

    /// Performs canonical surface-code validation.
    pub fn validate(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let limits =
            QecLimits::default();

        self.validate_with_limits(
            &limits,
        )
    }

    /// Performs structural and mathematical validation under explicit policy.
    pub fn validate_with_limits(
        &self,
        limits: &QecLimits,
    ) -> Result<(), SurfaceCodeError> {
        let expected_qubits =
            checked_square(
                self.distance,
                "surface-code qubit count",
            )?;

        let expected_stabilizers =
            checked_sub_usize(
                expected_qubits,
                1,
            )?;

        if self.num_data_qubits()
            != expected_qubits
        {
            return Err(
                SurfaceCodeError::InvalidDataQubitCount {
                    expected:
                        expected_qubits,
                    actual:
                        self.num_data_qubits(),
                },
            );
        }

        if self.num_stabilizers()
            != expected_stabilizers
        {
            return Err(
                SurfaceCodeError::InvalidStabilizerCount {
                    expected:
                        expected_stabilizers,
                    actual:
                        self.num_stabilizers(),
                },
            );
        }

        validate_policy(
            self.distance,
            expected_qubits,
            expected_stabilizers,
            limits,
        )?;

        self.validate_data_qubits()?;
        self.validate_stabilizers()?;
        self.validate_commutation()?;
        self.validate_logical_operators()?;

        Ok(())
    }

    fn validate_data_qubits(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let mut coordinates =
            BTreeSet::new();

        for (
            expected_index,
            qubit,
        ) in self.data_qubits.iter().enumerate()
        {
            let expected =
                QubitIndex::new(
                    expected_index,
                );

            if qubit.index()
                != expected
            {
                return Err(
                    SurfaceCodeError::CoordinateIndexMismatch {
                        qubit:
                            qubit.index(),
                        expected,
                    },
                );
            }

            let coordinate =
                qubit.coordinate();

            if coordinate.row()
                >= self.distance
                || coordinate.column()
                    >= self.distance
            {
                return Err(
                    SurfaceCodeError::CoordinateOutOfRange {
                        coordinate,
                        distance:
                            self.distance,
                    },
                );
            }

            if !coordinates.insert(
                coordinate,
            ) {
                return Err(
                    SurfaceCodeError::CoordinateIndexMismatch {
                        qubit:
                            qubit.index(),
                        expected,
                    },
                );
            }

            let expected_index =
                checked_qubit_index(
                    self.distance,
                    coordinate.row(),
                    coordinate.column(),
                )?;

            if expected_index
                != qubit.index()
            {
                return Err(
                    SurfaceCodeError::CoordinateIndexMismatch {
                        qubit:
                            qubit.index(),
                        expected:
                            expected_index,
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_stabilizers(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let mut ids =
            BTreeSet::new();

        let mut supports =
            BTreeMap::<
                Vec<QubitIndex>,
                usize,
            >::new();

        for stabilizer
            in &self.stabilizers
        {
            if !ids.insert(
                stabilizer.id(),
            ) {
                return Err(
                    SurfaceCodeError::DuplicateStabilizerId {
                        id:
                            stabilizer.id(),
                    },
                );
            }

            if let Some(previous) =
                supports.insert(
                    stabilizer
                        .support()
                        .to_vec(),
                    stabilizer.id(),
                )
            {
                return Err(
                    SurfaceCodeError::DuplicateStabilizerSupport {
                        first:
                            previous,
                        second:
                            stabilizer.id(),
                    },
                );
            }

            for &qubit
                in stabilizer.support()
            {
                if qubit.index()
                    >= self.num_data_qubits()
                {
                    return Err(
                        SurfaceCodeError::NonexistentQubit {
                            stabilizer:
                                stabilizer.id(),
                            qubit,
                        },
                    );
                }
            }

            if stabilizer.is_boundary() {
                if stabilizer.weight()
                    != 2
                {
                    return Err(
                        SurfaceCodeError::InvalidStabilizerWeight {
                            id:
                                stabilizer.id(),
                            expected: 2,
                            actual:
                                stabilizer.weight(),
                        },
                    );
                }

                self.validate_boundary_stabilizer(
                    stabilizer,
                )?;
            } else {
                if stabilizer.weight()
                    != 4
                {
                    return Err(
                        SurfaceCodeError::InvalidStabilizerWeight {
                            id:
                                stabilizer.id(),
                            expected: 4,
                            actual:
                                stabilizer.weight(),
                        },
                    );
                }

                self.validate_bulk_stabilizer(
                    stabilizer,
                )?;
            }
        }

        Ok(())
    }

    fn validate_bulk_stabilizer(
        &self,
        stabilizer: &SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        if stabilizer.support().len()
            != 4
        {
            return Err(
                SurfaceCodeError::InvalidBulkTopology {
                    id:
                        stabilizer.id(),
                },
            );
        }

        let coordinates:
            Vec<Coordinate> =
            stabilizer
                .support()
                .iter()
                .map(|qubit| {
                    self.coordinate_of(
                        *qubit,
                    )
                })
                .collect::<Result<_, _>>()?;

        let min_row =
            coordinates
                .iter()
                .map(|coordinate|
                    coordinate.row()
                )
                .min()
                .ok_or(
                    SurfaceCodeError::InvalidBulkTopology {
                        id:
                            stabilizer.id(),
                    },
                )?;

        let min_column =
            coordinates
                .iter()
                .map(|coordinate|
                    coordinate.column()
                )
                .min()
                .ok_or(
                    SurfaceCodeError::InvalidBulkTopology {
                        id:
                            stabilizer.id(),
                    },
                )?;

        let expected =
            self.face_qubits(
                min_row,
                min_column,
            )?;

        let expected_set:
            BTreeSet<QubitIndex> =
            expected
                .into_iter()
                .collect();

        let actual_set:
            BTreeSet<QubitIndex> =
            stabilizer
                .support()
                .iter()
                .copied()
                .collect();

        if expected_set
            != actual_set
        {
            return Err(
                SurfaceCodeError::InvalidBulkTopology {
                    id:
                        stabilizer.id(),
                },
            );
        }

        let expected_kind =
            if checked_add_usize(
                min_row,
                min_column,
            )? % 2
                == 1
            {
                StabilizerKind::X
            } else {
                StabilizerKind::Z
            };

        if stabilizer.kind()
            != expected_kind
        {
            return Err(
                SurfaceCodeError::InvalidBulkTopology {
                    id:
                        stabilizer.id(),
                },
            );
        }

        Ok(())
    }

    fn validate_boundary_stabilizer(
        &self,
        stabilizer: &SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        let boundary =
            stabilizer
                .boundary()
                .ok_or(
                    SurfaceCodeError::InvalidBoundaryTopology {
                        id:
                            stabilizer.id(),
                        boundary:
                            Boundary::Top,
                        kind:
                            stabilizer.kind(),
                    },
                )?;

        let coordinates =
            self.support_coordinates(
                stabilizer,
            )?;

        if coordinates.len()
            != 2
        {
            return Err(
                SurfaceCodeError::InvalidStabilizerWeight {
                    id:
                        stabilizer.id(),
                    expected: 2,
                    actual:
                        coordinates.len(),
                },
            );
        }

        match boundary {
            Boundary::Left
            | Boundary::Right => {
                if stabilizer.kind()
                    != StabilizerKind::X
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }

                let same_column =
                    coordinates[0]
                        .column()
                        == coordinates[1]
                            .column();

                let adjacent_rows =
                    coordinates[0]
                        .row()
                        .abs_diff(
                            coordinates[1]
                                .row(),
                        )
                        == 1;

                if !same_column
                    || !adjacent_rows
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }

                let row =
                    coordinates
                        .iter()
                        .map(|coordinate|
                            coordinate.row()
                        )
                        .min()
                        .ok_or(
                            SurfaceCodeError::InvalidBoundaryTopology {
                                id:
                                    stabilizer.id(),
                                boundary,
                                kind:
                                    stabilizer.kind(),
                            },
                        )?;

                let expected_boundary =
                    if row % 2 == 0 {
                        Boundary::Left
                    } else {
                        Boundary::Right
                    };

                if boundary
                    != expected_boundary
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryParity {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }

                let expected_column =
                    match boundary {
                        Boundary::Left => 0,
                        Boundary::Right => {
                            checked_sub_usize(
                                self.distance,
                                1,
                            )?
                        }
                        Boundary::Top
                        | Boundary::Bottom => {
                            return Err(
                                SurfaceCodeError::InvalidBoundaryTopology {
                                    id:
                                        stabilizer.id(),
                                    boundary,
                                    kind:
                                        stabilizer.kind(),
                                },
                            );
                        }
                    };

                if coordinates.iter().any(
                    |coordinate|
                        coordinate.column()
                            != expected_column,
                ) {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }
            }

            Boundary::Top
            | Boundary::Bottom => {
                if stabilizer.kind()
                    != StabilizerKind::Z
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }

                let same_row =
                    coordinates[0]
                        .row()
                        == coordinates[1]
                            .row();

                let adjacent_columns =
                    coordinates[0]
                        .column()
                        .abs_diff(
                            coordinates[1]
                                .column(),
                        )
                        == 1;

                if !same_row
                    || !adjacent_columns
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }

                let column =
                    coordinates
                        .iter()
                        .map(|coordinate|
                            coordinate.column()
                        )
                        .min()
                        .ok_or(
                            SurfaceCodeError::InvalidBoundaryTopology {
                                id:
                                    stabilizer.id(),
                                boundary,
                                kind:
                                    stabilizer.kind(),
                            },
                        )?;

                let expected_boundary =
                    if column % 2 == 1 {
                        Boundary::Top
                    } else {
                        Boundary::Bottom
                    };

                if boundary
                    != expected_boundary
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryParity {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }

                let expected_row =
                    match boundary {
                        Boundary::Top => 0,
                        Boundary::Bottom => {
                            checked_sub_usize(
                                self.distance,
                                1,
                            )?
                        }
                        Boundary::Left
                        | Boundary::Right => {
                            return Err(
                                SurfaceCodeError::InvalidBoundaryTopology {
                                    id:
                                        stabilizer.id(),
                                    boundary,
                                    kind:
                                        stabilizer.kind(),
                                },
                            );
                        }
                    };

                if coordinates.iter().any(
                    |coordinate|
                        coordinate.row()
                            != expected_row,
                ) {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id:
                                stabilizer.id(),
                            boundary,
                            kind:
                                stabilizer.kind(),
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn support_coordinates(
        &self,
        stabilizer: &SurfaceStabilizer,
    ) -> Result<
        Vec<Coordinate>,
        SurfaceCodeError,
    > {
        stabilizer
            .support()
            .iter()
            .map(|qubit|
                self.coordinate_of(*qubit)
            )
            .collect()
    }

    /// Validates all X/Z stabilizer commutation relationships.
    ///
    /// The implementation uses sparse incidence maps rather than an
    /// O(s²) all-pairs scan.
    fn validate_commutation(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let mut x_by_qubit =
            BTreeMap::<
                QubitIndex,
                Vec<usize>,
            >::new();

        let mut z_by_qubit =
            BTreeMap::<
                QubitIndex,
                Vec<usize>,
            >::new();

        for stabilizer
            in &self.stabilizers
        {
            let target =
                match stabilizer.kind() {
                    StabilizerKind::X => {
                        &mut x_by_qubit
                    }
                    StabilizerKind::Z => {
                        &mut z_by_qubit
                    }
                };

            for &qubit
                in stabilizer.support()
            {
                target
                    .entry(qubit)
                    .or_default()
                    .push(
                        stabilizer.id(),
                    );
            }
        }

        let mut parity =
            BTreeMap::<
                (usize, usize),
                bool,
            >::new();

        for (
            qubit,
            x_ids,
        ) in &x_by_qubit
        {
            let Some(z_ids) =
                z_by_qubit.get(qubit)
            else {
                continue;
            };

            for &x_id
                in x_ids
            {
                for &z_id
                    in z_ids
                {
                    let entry =
                        parity
                            .entry(
                                (x_id, z_id),
                            )
                            .or_insert(false);

                    *entry = !*entry;
                }
            }
        }

        for (
            (x_id, z_id),
            anticommutes,
        ) in parity
        {
            if anticommutes {
                return Err(
                    SurfaceCodeError::NonCommutingStabilizers {
                        first:
                            x_id,
                        second:
                            z_id,
                    },
                );
            }
        }

        Ok(())
    }

    /// Validates the canonical logical X/Z operators.
    pub fn validate_logical_operators(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        if self.num_logical_qubits()
            != 1
        {
            return Err(
                SurfaceCodeError::InvalidLogicalQubitCount {
                    expected: 1,
                    actual:
                        self.num_logical_qubits(),
                },
            );
        }

        if self.logical_x.operator()
            .is_identity()
        {
            return Err(
                SurfaceCodeError::IdentityLogicalOperator {
                    name: "X_L",
                },
            );
        }

        if self.logical_z.operator()
            .is_identity()
        {
            return Err(
                SurfaceCodeError::IdentityLogicalOperator {
                    name: "Z_L",
                },
            );
        }

        if self.logical_x.weight()
            != self.distance
        {
            return Err(
                SurfaceCodeError::InvalidLogicalWeight {
                    name: "X_L",
                    expected:
                        self.distance,
                    actual:
                        self.logical_x.weight(),
                },
            );
        }

        if self.logical_z.weight()
            != self.distance
        {
            return Err(
                SurfaceCodeError::InvalidLogicalWeight {
                    name: "Z_L",
                    expected:
                        self.distance,
                    actual:
                        self.logical_z.weight(),
                },
            );
        }

        for stabilizer
            in &self.stabilizers
        {
            let operator =
                stabilizer.pauli_string(
                    self.num_data_qubits(),
                )?;

            if self.logical_x
                .operator()
                .anticommutes_with(
                    &operator,
                )?
            {
                return Err(
                    SurfaceCodeError::LogicalOperatorDoesNotCommute {
                        logical:
                            "X_L",
                        stabilizer:
                            stabilizer.id(),
                    },
                );
            }

            if self.logical_z
                .operator()
                .anticommutes_with(
                    &operator,
                )?
            {
                return Err(
                    SurfaceCodeError::LogicalOperatorDoesNotCommute {
                        logical:
                            "Z_L",
                        stabilizer:
                            stabilizer.id(),
                    },
                );
            }
        }

        if !self.logical_x
            .operator()
            .anticommutes_with(
                self.logical_z
                    .operator(),
            )?
        {
            return Err(
                SurfaceCodeError::LogicalOperatorsDoNotAnticommute,
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Exact distance
    // ------------------------------------------------------------------------

    /// Verifies the exact code distance using canonical QEC limits.
    pub fn verify_distance(
        &self,
    ) -> Result<usize, SurfaceCodeError> {
        let limits =
            QecLimits::default();

        self.verify_distance_with_limits(
            &limits,
        )
    }

    /// Verifies the exact code distance under explicit resource limits.
    pub fn verify_distance_with_limits(
        &self,
        limits: &QecLimits,
    ) -> Result<usize, SurfaceCodeError> {
        self.validate_with_limits(
            limits,
        )?;

        let group =
            self.stabilizer_group()?;

        let result =
            distance::compute_distance_with_limits(
                &group,
                limits,
            )
            .map_err(
                |error| {
                    SurfaceCodeError::DistanceVerification(
                        error.to_string(),
                    )
                },
            )?;

        if result.distance()
            != self.distance
        {
            return Err(
                SurfaceCodeError::DistanceVerification(
                    format!(
                        "topological distance {} disagrees \
                         with exact verified distance {}",
                        self.distance,
                        result.distance(),
                    ),
                ),
            );
        }

        Ok(
            result.distance(),
        )
    }

    // ------------------------------------------------------------------------
    // Static resource helpers
    // ------------------------------------------------------------------------

    /// Returns the number of physical data qubits required by a distance.
    pub fn required_qubits(
        distance: usize,
    ) -> Result<usize, SurfaceCodeError> {
        checked_square(
            distance,
            "surface-code qubit count",
        )
    }

    /// Returns the number of stabilizer generators required by a distance.
    pub fn required_stabilizers(
        distance: usize,
    ) -> Result<usize, SurfaceCodeError> {
        let qubits =
            Self::required_qubits(
                distance,
            )?;

        checked_sub_usize(
            qubits,
            1,
        )
    }
}

// ============================================================================
// Preflight
// ============================================================================

#[derive(
    Debug,
    Clone,
    Copy,
)]
struct ResourceCounts {
    num_qubits: usize,
    num_stabilizers: usize,
    estimated_memory_bytes: u64,
}

fn preflight(
    distance: usize,
    limits: &QecLimits,
) -> Result<
    ResourceCounts,
    SurfaceCodeError,
> {
    limits
        .validate()
        .map_err(
            SurfaceCodeError::Limit,
        )?;

    if distance < 3
        || distance % 2 == 0
    {
        return Err(
            SurfaceCodeError::InvalidDistance {
                distance,
            },
        );
    }

    if distance
        > limits.max_code_distance
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::CodeDistance,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_code_distance
                            as u128,
                },
            ),
        );
    }

    let num_qubits =
        checked_square(
            distance,
            "surface-code qubit count",
        )?;

    let num_stabilizers =
        checked_sub_usize(
            num_qubits,
            1,
        )?;

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::Qubits,
                    requested:
                        num_qubits as u128,
                    maximum:
                        limits.max_qubits
                            as u128,
                },
            ),
        );
    }

    if num_stabilizers
        > limits.max_stabilizers
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::Stabilizers,
                    requested:
                        num_stabilizers as u128,
                    maximum:
                        limits.max_stabilizers
                            as u128,
                },
            ),
        );
    }

    if limits.max_stabilizer_weight
        < 4
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::StabilizerWeight,
                    requested: 4,
                    maximum:
                        limits.max_stabilizer_weight
                            as u128,
                },
            ),
        );
    }

    if distance
        > limits.max_logical_operator_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::LogicalOperatorWeight,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_logical_operator_weight
                            as u128,
                },
            ),
        );
    }

    let estimated_memory =
        estimate_memory_bytes(
            distance,
        )?;

    if estimated_memory
        > limits.max_memory_bytes
    {
        return Err(
            SurfaceCodeError::MemoryPreflightExceeded {
                estimated:
                    estimated_memory,
                maximum:
                    limits.max_memory_bytes,
            },
        );
    }

    Ok(
        ResourceCounts {
            num_qubits,
            num_stabilizers,
            estimated_memory_bytes:
                estimated_memory,
        },
    )
}

fn validate_policy(
    distance: usize,
    num_qubits: usize,
    num_stabilizers: usize,
    limits: &QecLimits,
) -> Result<(), SurfaceCodeError> {
    limits
        .validate()
        .map_err(
            SurfaceCodeError::Limit,
        )?;

    if distance
        > limits.max_code_distance
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::CodeDistance,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_code_distance
                            as u128,
                },
            ),
        );
    }

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::Qubits,
                    requested:
                        num_qubits as u128,
                    maximum:
                        limits.max_qubits
                            as u128,
                },
            ),
        );
    }

    if num_stabilizers
        > limits.max_stabilizers
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::Stabilizers,
                    requested:
                        num_stabilizers as u128,
                    maximum:
                        limits.max_stabilizers
                            as u128,
                },
            ),
        );
    }

    if 4
        > limits.max_stabilizer_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::StabilizerWeight,
                    requested: 4,
                    maximum:
                        limits.max_stabilizer_weight
                            as u128,
                },
            ),
        );
    }

    if distance
        > limits.max_logical_operator_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::LogicalOperatorWeight,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_logical_operator_weight
                            as u128,
                },
            ),
        );
    }

    Ok(())
}

// ============================================================================
// Memory preflight
// ============================================================================

/// Conservative construction-memory estimate.
///
/// This is deliberately a policy estimate rather than runtime accounting.
/// Exact runtime allocation tracking remains the responsibility of
/// `memory.rs` and `resources.rs`.
fn estimate_memory_bytes(
    distance: usize,
) -> Result<u64, SurfaceCodeError> {
    let qubits =
        checked_square(
            distance,
            "surface-code memory qubit count",
        )?;

    let stabilizers =
        checked_sub_usize(
            qubits,
            1,
        )?;

    const BYTES_PER_DATA_QUBIT: u64 = 64;
    const BYTES_PER_STABILIZER: u64 = 512;
    const BYTES_PER_LOGICAL_OPERATOR: u64 = 1024;
    const FIXED_OVERHEAD: u64 = 4096;

    let q =
        u64::try_from(qubits)
            .map_err(
                |_| SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code memory qubit conversion",
                },
            )?;

    let s =
        u64::try_from(stabilizers)
            .map_err(
                |_| SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code memory stabilizer conversion",
                },
            )?;

    let data_bytes =
        q.checked_mul(
            BYTES_PER_DATA_QUBIT,
        )
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code data-qubit memory",
            },
        )?;

    let stabilizer_bytes =
        s.checked_mul(
            BYTES_PER_STABILIZER,
        )
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code stabilizer memory",
            },
        )?;

    data_bytes
        .checked_add(
            stabilizer_bytes,
        )
        .and_then(
            |value|
                value.checked_add(
                    BYTES_PER_LOGICAL_OPERATOR,
                ),
        )
        .and_then(
            |value|
                value.checked_add(
                    FIXED_OVERHEAD,
                ),
        )
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code total memory estimate",
            },
        )
}

// ============================================================================
// Construction
// ============================================================================

fn build_data_qubits(
    distance: usize,
    expected_count: usize,
) -> Result<
    Vec<DataQubit>,
    SurfaceCodeError,
> {
    let mut data =
        Vec::with_capacity(
            expected_count,
        );

    for row in 0..distance {
        for column in 0..distance {
            let index =
                checked_qubit_index(
                    distance,
                    row,
                    column,
                )?;

            data.push(
                DataQubit::new(
                    index,
                    Coordinate::new(
                        row,
                        column,
                    ),
                ),
            );
        }
    }

    if data.len()
        != expected_count
    {
        return Err(
            SurfaceCodeError::InvalidDataQubitCount {
                expected:
                    expected_count,
                actual:
                    data.len(),
            },
        );
    }

    Ok(data)
}

/// Builds the canonical rotated surface-code stabilizers.
///
/// Bulk checks:
///
/// ```text
/// X if (row + column) is odd
/// Z if (row + column) is even
/// ```
///
/// Boundary checks:
///
/// ```text
/// left/right  -> X
/// top/bottom  -> Z
/// ```
///
/// Only alternating boundary edges are populated.
fn build_stabilizers(
    distance: usize,
    limits: &QecLimits,
) -> Result<
    Vec<SurfaceStabilizer>,
    SurfaceCodeError> {
    if limits.max_stabilizer_weight
        < 4
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::StabilizerWeight,
                    requested: 4,
                    maximum:
                        limits.max_stabilizer_weight
                            as u128,
                },
            ),
        );
    }

    let expected =
        checked_sub_usize(
            checked_square(
                distance,
                "surface-code stabilizer construction",
            )?,
            1,
        )?;

    let mut stabilizers =
        Vec::with_capacity(
            expected,
        );

    let mut next_id =
        0usize;

    // ------------------------------------------------------------------------
    // Bulk checkerboard stabilizers
    // ------------------------------------------------------------------------

    let face_limit =
        checked_sub_usize(
            distance,
            1,
        )?;

    for row in 0..face_limit {
        for column in 0..face_limit {
            let support = vec![
                checked_qubit_index(
                    distance,
                    row,
                    column,
                )?,
                checked_qubit_index(
                    distance,
                    row,
                    checked_add_usize(
                        column,
                        1,
                    )?,
                )?,
                checked_qubit_index(
                    distance,
                    checked_add_usize(
                        row,
                        1,
                    )?,
                    column,
                )?,
                checked_qubit_index(
                    distance,
                    checked_add_usize(
                        row,
                        1,
                    )?,
                    checked_add_usize(
                        column,
                        1,
                    )?,
                )?,
            ];

            let kind =
                if checked_add_usize(
                    row,
                    column,
                )? % 2
                    == 1
                {
                    StabilizerKind::X
                } else {
                    StabilizerKind::Z
                };

            stabilizers.push(
                SurfaceStabilizer::new(
                    next_id,
                    kind,
                    support,
                    None,
                )?,
            );

            next_id =
                checked_add_usize(
                    next_id,
                    1,
                )?;
        }
    }

    // ------------------------------------------------------------------------
    // Left boundary
    // ------------------------------------------------------------------------

    for row in (0..face_limit).step_by(2) {
        let support = vec![
            checked_qubit_index(
                distance,
                row,
                0,
            )?,
            checked_qubit_index(
                distance,
                checked_add_usize(
                    row,
                    1,
                )?,
                0,
            )?,
        ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::X,
                support,
                Some(Boundary::Left),
            )?,
        );

        next_id =
            checked_add_usize(
                next_id,
                1,
            )?;
    }

    // ------------------------------------------------------------------------
    // Right boundary
    // ------------------------------------------------------------------------

    let right =
        checked_sub_usize(
            distance,
            1,
        )?;

    for row in (1..face_limit).step_by(2) {
        let support = vec![
            checked_qubit_index(
                distance,
                row,
                right,
            )?,
            checked_qubit_index(
                distance,
                checked_add_usize(
                    row,
                    1,
                )?,
                right,
            )?,
        ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::X,
                support,
                Some(Boundary::Right),
            )?,
        );

        next_id =
            checked_add_usize(
                next_id,
                1,
            )?;
    }

    // ------------------------------------------------------------------------
    // Top boundary
    // ------------------------------------------------------------------------

    for column in (1..face_limit).step_by(2) {
        let support = vec![
            checked_qubit_index(
                distance,
                0,
                column,
            )?,
            checked_qubit_index(
                distance,
                0,
                checked_add_usize(
                    column,
                    1,
                )?,
            )?,
        ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::Z,
                support,
                Some(Boundary::Top),
            )?,
        );

        next_id =
            checked_add_usize(
                next_id,
                1,
            )?;
    }

    // ------------------------------------------------------------------------
    // Bottom boundary
    // ------------------------------------------------------------------------

    for column in (0..face_limit).step_by(2) {
        let support = vec![
            checked_qubit_index(
                distance,
                right,
                column,
            )?,
            checked_qubit_index(
                distance,
                right,
                checked_add_usize(
                    column,
                    1,
                )?,
            )?,
        ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::Z,
                support,
                Some(Boundary::Bottom),
            )?,
        );

        next_id =
            checked_add_usize(
                next_id,
                1,
            )?;
    }

    if stabilizers.len()
        != expected
    {
        return Err(
            SurfaceCodeError::InvalidStabilizerCount {
                expected,
                actual:
                    stabilizers.len(),
            },
        );
    }

    Ok(stabilizers)
}

// ============================================================================
// Logical operators
// ============================================================================

/// Canonical horizontal logical-X string.
fn build_logical_x(
    distance: usize,
    limits: &QecLimits,
) -> Result<
    LogicalOperator,
    SurfaceCodeError> {
    if distance
        > limits.max_logical_operator_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::LogicalOperatorWeight,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_logical_operator_weight
                            as u128,
                },
            ),
        );
    }

    let num_qubits =
        checked_square(
            distance,
            "logical-X construction",
        )?;

    let mut paulis =
        vec![Pauli::I; num_qubits];

    for column in 0..distance {
        let index =
            checked_qubit_index(
                distance,
                0,
                column,
            )?
            .index();

        paulis[index] =
            Pauli::X;
    }

    LogicalOperator::new(
        "X_L",
        StabilizerKind::X,
        PauliString::from_paulis(
            &paulis,
        ),
    )
}

/// Canonical vertical logical-Z string.
fn build_logical_z(
    distance: usize,
    limits: &QecLimits,
) -> Result<
    LogicalOperator,
    SurfaceCodeError> {
    if distance
        > limits.max_logical_operator_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Exceeded {
                    resource:
                        LimitKind::LogicalOperatorWeight,
                    requested:
                        distance as u128,
                    maximum:
                        limits.max_logical_operator_weight
                            as u128,
                },
            ),
        );
    }

    let num_qubits =
        checked_square(
            distance,
            "logical-Z construction",
        )?;

    let mut paulis =
        vec![Pauli::I; num_qubits];

    for row in 0..distance {
        let index =
            checked_qubit_index(
                distance,
                row,
                0,
            )?
            .index();

        paulis[index] =
            Pauli::Z;
    }

    LogicalOperator::new(
        "Z_L",
        StabilizerKind::Z,
        PauliString::from_paulis(
            &paulis,
        ),
    )
}

// ============================================================================
// Arithmetic helpers
// ============================================================================

fn checked_square(
    value: usize,
    resource: &'static str,
) -> Result<
    usize,
    SurfaceCodeError> {
    checked_mul_usize(
        value,
        value,
    )
    .map_err(
        |_| SurfaceCodeError::ArithmeticOverflow {
            resource,
        },
    )
}

fn checked_qubit_index(
    distance: usize,
    row: usize,
    column: usize,
) -> Result<
    QubitIndex,
    SurfaceCodeError> {
    if row >= distance
        || column >= distance
    {
        return Err(
            SurfaceCodeError::CoordinateOutOfRange {
                coordinate:
                    Coordinate::new(
                        row,
                        column,
                    ),
                distance,
            },
        );
    }

    let index =
        checked_mul_add_usize(
            row,
            distance,
            column,
        )
        .map_err(
            |_| SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code qubit index",
            },
        )?;

    Ok(
        QubitIndex::new(
            index,
        ),
    )
}

// ============================================================================
// Canonical error conversion helpers
// ============================================================================

fn limit_error_to_qec_error(
    error: LimitError,
) -> QecError {
    match error {
        LimitError::Exceeded {
            resource,
            requested,
            maximum,
        } => {
            QecError::ResourceLimitExceeded {
                resource:
                    limit_kind_to_resource_kind(
                        resource,
                    ),
                requested,
                current: 0,
                limit: maximum,
                message:
                    format!(
                        "QEC resource limit {resource} exceeded"
                    ),
            }
        }

        LimitError::InvalidLimit {
            resource,
            value,
        } => {
            QecError::invalid_input(
                format!(
                    "invalid QEC limit {resource}: {value}"
                ),
            )
        }

        LimitError::ArithmeticOverflow {
            resource,
        } => {
            QecError::NumericalFailure {
                operation:
                    super::errors::NumericalOperation::
                        MemorySizeCalculation,
                message:
                    format!(
                        "overflow while calculating \
                         QEC limit {resource}"
                    ),
            }
        }

        LimitError::InconsistentLimits {
            resource,
            related_resource,
            reason,
        } => {
            QecError::invalid_input(
                format!(
                    "inconsistent QEC limits: \
                     {resource} / {related_resource}: {reason}"
                ),
            )
        }

        LimitError::UnsupportedSchema {
            found,
            expected,
        } => {
            QecError::VersionMismatch {
                component:
                    "QecLimits".to_string(),
                expected:
                    expected.to_string(),
                actual:
                    found.to_string(),
                message:
                    "unsupported QEC limits schema".to_string(),
            }
        }
    }
}

fn limit_kind_to_resource_kind(
    kind: LimitKind,
) -> ResourceKind {
    match kind {
        LimitKind::CodeDistance =>
            ResourceKind::CodeDistance,

        LimitKind::Qubits =>
            ResourceKind::Qubits,

        LimitKind::Stabilizers =>
            ResourceKind::Stabilizers,

        LimitKind::SyndromeEvents =>
            ResourceKind::SyndromeEvents,

        LimitKind::MeasurementRounds =>
            ResourceKind::MeasurementRounds,

        LimitKind::GraphNodes =>
            ResourceKind::GraphNodes,

        LimitKind::GraphEdges =>
            ResourceKind::GraphEdges,

        LimitKind::MemoryBytes =>
            ResourceKind::MemoryBytes,

        LimitKind::DecoderTimeNs =>
            ResourceKind::Time,

        LimitKind::Parallelism =>
            ResourceKind::Parallelism,

        LimitKind::CheckpointSizeBytes =>
            ResourceKind::CheckpointSize,

        LimitKind::Partitions =>
            ResourceKind::Partitions,

        LimitKind::StreamBufferEvents =>
            ResourceKind::StreamBuffer,

        LimitKind::DecoderIterations =>
            ResourceKind::DecoderIterations,

        LimitKind::StabilizerWeight =>
            ResourceKind::StabilizerWeight,

        LimitKind::LogicalOperatorWeight =>
            ResourceKind::LogicalWeight,

        LimitKind::QubitsPerPartition =>
            ResourceKind::Qubits,

        LimitKind::QpuShots =>
            ResourceKind::QpuShots,

        LimitKind::QpuCircuits =>
            ResourceKind::QpuCircuits,

        LimitKind::VerificationOperations =>
            ResourceKind::Operations,
    }
}