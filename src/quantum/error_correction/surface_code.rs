//! Zamani Quantum Error Correction — Rotated Planar Surface Code.
//!
//! Production-grade mathematical/topological model of the canonical rotated
//! planar surface code.
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
//! Bulk stabilizers occupy the `(d-1) × (d-1)` plaquettes in a checkerboard
//! pattern. Bulk stabilizers have weight four.
//!
//! Boundary stabilizers have weight two and are placed on alternating
//! boundary edges. Their placement is derived from the topology rather than
//! guessed from qubit counts.
//!
//! # Mathematical invariants
//!
//! The implementation validates:
//!
//! - distance validity;
//! - checked `d²` arithmetic;
//! - global `QecLimits` before allocation;
//! - estimated memory requirements before allocation;
//! - exact data-qubit topology;
//! - unique qubit coordinates;
//! - unique stabilizer identifiers;
//! - unique stabilizer supports;
//! - valid qubit references;
//! - correct bulk/boundary weights;
//! - correct boundary orientation;
//! - X/Z checkerboard structure;
//! - X/Z stabilizer commutation;
//! - logical X/Z non-identity;
//! - logical X/Z weight;
//! - logical X/Z commutation with all stabilizers;
//! - logical X/Z anticommutation;
//! - syndrome extraction;
//! - exact distance verification through `distance.rs`.
//!
//! # Resource model
//!
//! Construction is explicitly:
//!
//! ```text
//! untrusted distance
//!        ↓
//! QecLimits validation
//!        ↓
//! checked d²
//!        ↓
//! checked stabilizer count
//!        ↓
//! conservative memory preflight
//!        ↓
//! allocation
//!        ↓
//! topology construction
//!        ↓
//! mathematical validation
//! ```
//!
//! No large topology allocation is performed before the global QEC policy
//! has approved the requested workload.
//!
//! The memory calculation is intentionally conservative. It is a preflight
//! safety estimate, not an allocator accounting snapshot.
//!
//! # Representation boundary
//!
//! `surface_code.rs` owns the mathematical/topological representation.
//!
//! Generic Pauli algebra and stabilizer linear algebra remain in
//! `stabilizer.rs`.
//!
//! Exact code-distance verification remains in `distance.rs`.
//!
//! Decoder-specific behavior belongs in `surface_coder.rs`, `mwpm.rs`,
//! `union_find.rs`, and the standardized decoder layer.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::distance;
use super::limits::{LimitError, QecLimits};
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
        Self { row, column }
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

/// Physical boundary on which a weight-2 stabilizer may reside.
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

impl fmt::Display for SurfaceCodeError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidDistance { distance } => {
                write!(
                    f,
                    "invalid surface-code distance {}; distance must be odd and >= 3",
                    distance
                )
            }

            Self::Limit(error) => {
                write!(f, "QEC resource limit exceeded: {error}")
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "surface-code resource calculation overflowed for {resource}"
                )
            }

            Self::MemoryPreflightExceeded {
                estimated,
                maximum,
            } => {
                write!(
                    f,
                    "surface-code memory preflight {} bytes exceeds configured maximum {} bytes",
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
                    "stabilizer {id} has weight {actual}; expected {expected}"
                )
            }

            Self::NonexistentQubit {
                stabilizer,
                qubit,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} references nonexistent qubit {qubit}"
                )
            }

            Self::DuplicateStabilizerSupport {
                first,
                second,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} have identical support"
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
                    "invalid stabilizer count {actual}; expected {expected}"
                )
            }

            Self::InvalidDataQubitCount {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "invalid data-qubit count {actual}; expected {expected}"
                )
            }

            Self::CoordinateOutOfRange {
                coordinate,
                distance,
            } => {
                write!(
                    f,
                    "coordinate {coordinate} is outside distance-{distance} lattice"
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
                    "face ({row}, {column}) is outside distance-{distance} lattice"
                )
            }

            Self::InvalidBoundaryTopology {
                id,
                boundary,
                kind,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid {boundary} boundary type {kind}"
                )
            }

            Self::InvalidBoundaryParity {
                id,
                boundary,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid alternating placement on {boundary} boundary"
                )
            }

            Self::InvalidBulkTopology { id } => {
                write!(
                    f,
                    "bulk stabilizer {id} has invalid checkerboard topology"
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
                    "logical operator {name} has weight {actual}; expected {expected}"
                )
            }

            Self::LogicalOperatorDoesNotCommute {
                logical,
                stabilizer,
            } => {
                write!(
                    f,
                    "logical operator {logical} does not commute with stabilizer {stabilizer}"
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
                    "encoded logical-qubit count {actual}; expected {expected}"
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
                    "Pauli operator has {actual} qubits; expected {expected}"
                )
            }
        }
    }
}

impl std::error::Error for SurfaceCodeError {}

// ============================================================================
// Surface stabilizer
// ============================================================================

/// Explicit stabilizer topology.
///
/// The support is always deterministic and contains either:
///
/// * four qubits for a bulk stabilizer;
/// * two qubits for a boundary stabilizer.
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
                SurfaceCodeError::EmptyStabilizer { id }
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

    /// Converts the explicit topology into the generic Pauli representation.
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
                &paulis
            )
        )
    }

    /// Converts the stabilizer into the generic stabilizer-generator IR.
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
                }
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
    /// Constructs a surface code using the canonical global QEC limits.
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

    /// Constructs a surface code under an explicit resource policy.
    ///
    /// This is the preferred production entry point when the caller already
    /// owns a `QecConfig`/`QecLimits` policy.
    pub fn new_with_limits(
        distance: usize,
        limits: &QecLimits,
    ) -> Result<Self, SurfaceCodeError> {
        let counts =
            preflight(distance, limits)?;

        let data_qubits =
            build_data_qubits(
                distance,
                counts.num_qubits,
            )?;

        let stabilizers =
            build_stabilizers(distance)?;

        let logical_x =
            build_logical_x(distance)?;

        let logical_z =
            build_logical_z(distance)?;

        let code = Self {
            distance,
            data_qubits,
            stabilizers,
            logical_x,
            logical_z,
        };

        code.validate_with_limits(
            limits
        )?;

        Ok(code)
    }

    /// Compatibility constructor used by callers that refer to a code
    /// explicitly by its distance.
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

    /// Returns the conservative memory estimate used by construction.
    #[must_use]
    pub fn estimated_memory_bytes(
        &self,
    ) -> u64 {
        estimate_memory_bytes(
            self.distance
        )
        .unwrap_or(u64::MAX)
    }

    // ------------------------------------------------------------------------
    // Coordinate topology
    // ------------------------------------------------------------------------

    /// Converts a lattice coordinate to its physical qubit.
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
            self.data_qubits[index.index()]
        )
    }

    /// Returns the lattice coordinate for a physical qubit.
    pub fn coordinate_of(
        &self,
        qubit: QubitIndex,
    ) -> Result<Coordinate, SurfaceCodeError> {
        let index = qubit.index();

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
                .coordinate()
        )
    }

    /// Returns the four data qubits surrounding a bulk plaquette.
    pub fn face_qubits(
        &self,
        row: usize,
        column: usize,
    ) -> Result<[QubitIndex; 4], SurfaceCodeError> {
        if row >= self.distance.saturating_sub(1)
            || column
                >= self.distance.saturating_sub(1)
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
                column + 1,
            )?,
            checked_qubit_index(
                d,
                row + 1,
                column,
            )?,
            checked_qubit_index(
                d,
                row + 1,
                column + 1,
            )?,
        ])
    }

    // ------------------------------------------------------------------------
    // Stabilizer representation
    // ------------------------------------------------------------------------

    /// Builds the generic stabilizer group.
    ///
    /// `stabilizer.rs` remains the owner of the generic algebraic
    /// representation; this method only performs the topology-to-IR bridge.
    pub fn stabilizer_group(
        &self,
    ) -> Result<StabilizerGroup, SurfaceCodeError> {
        let mut group =
            StabilizerGroup::new(
                self.num_data_qubits()
            )?;

        for stabilizer
            in &self.stabilizers
        {
            group.add_generator(
                stabilizer.generator(
                    self.num_data_qubits()
                )?
            )?;
        }

        Ok(group)
    }

    /// Extracts the syndrome of a Pauli error.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, SurfaceCodeError> {
        if error.num_qubits()
            != self.num_data_qubits()
        {
            return Err(
                SurfaceCodeError::DimensionMismatch {
                    expected: self.num_data_qubits(),
                    actual: error.num_qubits(),
                },
            );
        }

        let group =
            self.stabilizer_group()?;

        Ok(
            group.syndrome(error)?
        )
    }

    // ------------------------------------------------------------------------
    // Mathematical validation
    // ------------------------------------------------------------------------

    /// Performs the canonical surface-code validation.
    pub fn validate(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let limits =
            QecLimits::default();

        self.validate_with_limits(
            &limits
        )
    }

    /// Performs structural and mathematical validation under an explicit
    /// resource policy.
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
            expected_qubits
                .checked_sub(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code stabilizer count",
                    }
                )?;

        if self.num_data_qubits()
            != expected_qubits
        {
            return Err(
                SurfaceCodeError::InvalidDataQubitCount {
                    expected: expected_qubits,
                    actual: self.num_data_qubits(),
                },
            );
        }

        if self.num_stabilizers()
            != expected_stabilizers
        {
            return Err(
                SurfaceCodeError::InvalidStabilizerCount {
                    expected: expected_stabilizers,
                    actual: self.num_stabilizers(),
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

        for (expected_index, qubit)
            in self.data_qubits.iter().enumerate()
        {
            let expected =
                QubitIndex::new(
                    expected_index
                );

            if qubit.index()
                != expected
            {
                return Err(
                    SurfaceCodeError::CoordinateIndexMismatch {
                        qubit: qubit.index(),
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
                        distance: self.distance,
                    },
                );
            }

            if !coordinates.insert(
                coordinate
            ) {
                return Err(
                    SurfaceCodeError::CoordinateIndexMismatch {
                        qubit: qubit.index(),
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
                        qubit: qubit.index(),
                        expected: expected_index,
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
                stabilizer.id()
            ) {
                return Err(
                    SurfaceCodeError::DuplicateStabilizerId {
                        id: stabilizer.id(),
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
                        first: previous,
                        second: stabilizer.id(),
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
                            id: stabilizer.id(),
                            expected: 2,
                            actual: stabilizer.weight(),
                        },
                    );
                }

                self.validate_boundary_stabilizer(
                    stabilizer
                )?;
            } else {
                if stabilizer.weight()
                    != 4
                {
                    return Err(
                        SurfaceCodeError::InvalidStabilizerWeight {
                            id: stabilizer.id(),
                            expected: 4,
                            actual: stabilizer.weight(),
                        },
                    );
                }

                self.validate_bulk_stabilizer(
                    stabilizer
                )?;
            }
        }

        Ok(())
    }

    fn validate_bulk_stabilizer(
        &self,
        stabilizer: &SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        let support =
            stabilizer.support();

        if support.len() != 4 {
            return Err(
                SurfaceCodeError::InvalidBulkTopology {
                    id: stabilizer.id(),
                },
            );
        }

        let coordinates: Vec<Coordinate> =
            support
                .iter()
                .map(|qubit| {
                    self.coordinate_of(*qubit)
                })
                .collect::<Result<_, _>>()?;

        let min_row =
            coordinates
                .iter()
                .map(|c| c.row())
                .min()
                .unwrap_or(usize::MAX);

        let min_column =
            coordinates
                .iter()
                .map(|c| c.column())
                .min()
                .unwrap_or(usize::MAX);

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
            support
                .iter()
                .copied()
                .collect();

        if expected_set
            != actual_set
        {
            return Err(
                SurfaceCodeError::InvalidBulkTopology {
                    id: stabilizer.id(),
                },
            );
        }

        let expected_kind =
            if (min_row
                + min_column)
                % 2
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
                    id: stabilizer.id(),
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
                        id: stabilizer.id(),
                        boundary: Boundary::Top,
                        kind: stabilizer.kind(),
                    }
                )?;

        match boundary {
            Boundary::Left
            | Boundary::Right => {
                if stabilizer.kind()
                    != StabilizerKind::X
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
                        },
                    );
                }

                let coordinates =
                    self.support_coordinates(
                        stabilizer
                    )?;

                let same_column =
                    coordinates[0].column()
                        == coordinates[1].column();

                let adjacent_rows =
                    coordinates[0]
                        .row()
                        .abs_diff(
                            coordinates[1]
                                .row()
                        )
                        == 1;

                if !same_column
                    || !adjacent_rows
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
                        },
                    );
                }

                let row =
                    coordinates
                        .iter()
                        .map(|c| c.row())
                        .min()
                        .unwrap_or(usize::MAX);

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
                            id: stabilizer.id(),
                            boundary,
                        },
                    );
                }

                let expected_column =
                    match boundary {
                        Boundary::Left => 0,
                        Boundary::Right =>
                            self.distance
                                .checked_sub(1)
                                .ok_or(
                                    SurfaceCodeError::ArithmeticOverflow {
                                        resource:
                                            "right boundary column",
                                    }
                                )?,
                        _ => unreachable!(),
                    };

                if coordinates
                    .iter()
                    .any(
                        |c| c.column()
                            != expected_column
                    )
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
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
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
                        },
                    );
                }

                let coordinates =
                    self.support_coordinates(
                        stabilizer
                    )?;

                let same_row =
                    coordinates[0].row()
                        == coordinates[1].row();

                let adjacent_columns =
                    coordinates[0]
                        .column()
                        .abs_diff(
                            coordinates[1]
                                .column()
                        )
                        == 1;

                if !same_row
                    || !adjacent_columns
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
                        },
                    );
                }

                let column =
                    coordinates
                        .iter()
                        .map(|c| c.column())
                        .min()
                        .unwrap_or(usize::MAX);

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
                            id: stabilizer.id(),
                            boundary,
                        },
                    );
                }

                let expected_row =
                    match boundary {
                        Boundary::Top => 0,
                        Boundary::Bottom =>
                            self.distance
                                .checked_sub(1)
                                .ok_or(
                                    SurfaceCodeError::ArithmeticOverflow {
                                        resource:
                                            "bottom boundary row",
                                    }
                                )?,
                        _ => unreachable!(),
                    };

                if coordinates
                    .iter()
                    .any(
                        |c| c.row()
                            != expected_row
                    )
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryTopology {
                            id: stabilizer.id(),
                            boundary,
                            kind: stabilizer.kind(),
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
    ) -> Result<Vec<Coordinate>, SurfaceCodeError> {
        stabilizer
            .support()
            .iter()
            .map(|qubit| {
                self.coordinate_of(*qubit)
            })
            .collect()
    }

    /// Validates all X/Z stabilizer commutation relationships.
    ///
    /// This is deliberately implemented using sparse incidence maps rather
    /// than an O(s²) all-pairs comparison. Each data qubit participates in a
    /// bounded number of local checks, so the generated topology is validated
    /// in approximately linear time in the number of physical qubits.
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
                    StabilizerKind::X =>
                        &mut x_by_qubit,
                    StabilizerKind::Z =>
                        &mut z_by_qubit,
                };

            for &qubit
                in stabilizer.support()
            {
                target
                    .entry(qubit)
                    .or_default()
                    .push(
                        stabilizer.id()
                    );
            }
        }

        // For CSS stabilizers, two operators anticommute exactly when their
        // support intersection has odd cardinality.
        //
        // Toggle parity for each X/Z pair at every shared data qubit.
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

            for &x_id in x_ids {
                for &z_id in z_ids {
                    let entry =
                        parity
                            .entry(
                                (x_id, z_id)
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
                        first: x_id,
                        second: z_id,
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
                    actual: self.num_logical_qubits(),
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
                    expected: self.distance,
                    actual: self.logical_x.weight(),
                },
            );
        }

        if self.logical_z.weight()
            != self.distance
        {
            return Err(
                SurfaceCodeError::InvalidLogicalWeight {
                    name: "Z_L",
                    expected: self.distance,
                    actual: self.logical_z.weight(),
                },
            );
        }

        for stabilizer
            in &self.stabilizers
        {
            let operator =
                stabilizer.pauli_string(
                    self.num_data_qubits()
                )?;

            if self.logical_x
                .operator()
                .anticommutes_with(
                    &operator
                )?
            {
                return Err(
                    SurfaceCodeError::LogicalOperatorDoesNotCommute {
                        logical: "X_L",
                        stabilizer:
                            stabilizer.id(),
                    },
                );
            }

            if self.logical_z
                .operator()
                .anticommutes_with(
                    &operator
                )?
            {
                return Err(
                    SurfaceCodeError::LogicalOperatorDoesNotCommute {
                        logical: "Z_L",
                        stabilizer:
                            stabilizer.id(),
                    },
                );
            }
        }

        let anticommutes =
            self.logical_x
                .operator()
                .anticommutes_with(
                    self.logical_z
                        .operator()
                )?;

        if !anticommutes {
            return Err(
                SurfaceCodeError::LogicalOperatorsDoNotAnticommute
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Exact distance
    // ------------------------------------------------------------------------

    /// Verifies the exact code distance using the canonical QEC limits.
    ///
    /// The topology's declared distance is not trusted. `distance.rs` performs
    /// the actual normalizer-minus-stabilizer search.
    pub fn verify_distance(
        &self,
    ) -> Result<usize, SurfaceCodeError> {
        let limits =
            QecLimits::default();

        self.verify_distance_with_limits(
            &limits
        )
    }

    /// Verifies the exact code distance under explicit resource limits.
    pub fn verify_distance_with_limits(
        &self,
        limits: &QecLimits,
    ) -> Result<usize, SurfaceCodeError> {
        self.validate_with_limits(
            limits
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
                        error.to_string()
                    )
                }
            )?;

        if result.distance()
            != self.distance
        {
            return Err(
                SurfaceCodeError::DistanceVerification(
                    format!(
                        "topological distance {} disagrees with exact verified distance {}",
                        self.distance,
                        result.distance()
                    )
                )
            );
        }

        Ok(
            result.distance()
        )
    }

    // ------------------------------------------------------------------------
    // Resource information
    // ------------------------------------------------------------------------

    /// Returns the number of data qubits required by a distance.
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
                distance
            )?;

        qubits
            .checked_sub(1)
            .ok_or(
                SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code stabilizer count",
                }
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
) -> Result<ResourceCounts, SurfaceCodeError> {
    limits
        .validate()
        .map_err(
            SurfaceCodeError::Limit
        )?;

    if distance < 3
        || distance % 2 == 0
    {
        return Err(
            SurfaceCodeError::InvalidDistance {
                distance,
            }
        );
    }

    if distance
        > limits.max_code_distance
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::CodeDistance {
                    requested: distance,
                    maximum:
                        limits.max_code_distance,
                }
            )
        );
    }

    let num_qubits =
        checked_square(
            distance,
            "surface-code qubit count",
        )?;

    let num_stabilizers =
        num_qubits
            .checked_sub(1)
            .ok_or(
                SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code stabilizer count",
                }
            )?;

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Qubits {
                    requested:
                        num_qubits,
                    maximum:
                        limits.max_qubits,
                }
            )
        );
    }

    if num_stabilizers
        > limits.max_stabilizers
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Stabilizers {
                    requested:
                        num_stabilizers,
                    maximum:
                        limits.max_stabilizers,
                }
            )
        );
    }

    let estimated_memory =
        estimate_memory_bytes(
            distance
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
            }
        );
    }

    Ok(
        ResourceCounts {
            num_qubits,
            num_stabilizers,
            estimated_memory_bytes:
                estimated_memory,
        }
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
            SurfaceCodeError::Limit
        )?;

    if distance
        > limits.max_code_distance
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::CodeDistance {
                    requested: distance,
                    maximum:
                        limits.max_code_distance,
                }
            )
        );
    }

    if num_qubits
        > limits.max_qubits
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Qubits {
                    requested:
                        num_qubits,
                    maximum:
                        limits.max_qubits,
                }
            )
        );
    }

    if num_stabilizers
        > limits.max_stabilizers
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::Stabilizers {
                    requested:
                        num_stabilizers,
                    maximum:
                        limits.max_stabilizers,
                }
            )
        );
    }

    if 4
        > limits.max_stabilizer_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::StabilizerWeight {
                    requested: 4,
                    maximum:
                        limits.max_stabilizer_weight,
                }
            )
        );
    }

    if distance
        > limits.max_logical_operator_weight
    {
        return Err(
            SurfaceCodeError::Limit(
                LimitError::LogicalOperatorWeight {
                    requested:
                        distance,
                    maximum:
                        limits.max_logical_operator_weight,
                }
            )
        );
    }

    Ok(())
}

/// Conservative construction-memory estimate.
///
/// The estimate intentionally over-approximates the main owned structures:
///
/// * data-qubit storage;
/// * stabilizer objects;
/// * support allocations;
/// * logical Pauli storage;
/// * topology bookkeeping.
///
/// This is a policy preflight, not a replacement for `ResourceManager`.
fn estimate_memory_bytes(
    distance: usize,
) -> Result<u64, SurfaceCodeError> {
    let qubits =
        checked_square(
            distance,
            "surface-code memory qubit count",
        )?;

    let stabilizers =
        qubits
            .checked_sub(1)
            .ok_or(
                SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code memory stabilizer count",
                }
            )?;

    // Deliberately conservative constants. The actual allocator/accounting
    // layer can provide exact runtime measurements later.
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
                }
            )?;

    let s =
        u64::try_from(stabilizers)
            .map_err(
                |_| SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code memory stabilizer conversion",
                }
            )?;

    q.checked_mul(
        BYTES_PER_DATA_QUBIT
    )
    .and_then(
        |value| {
            s.checked_mul(
                BYTES_PER_STABILIZER
            )
            .and_then(
                |stabilizer_bytes| {
                    value.checked_add(
                        stabilizer_bytes
                    )
                }
            )
        }
    )
    .and_then(
        |value| {
            value.checked_add(
                BYTES_PER_LOGICAL_OPERATOR
            )
        }
    )
    .and_then(
        |value| {
            value.checked_add(
                FIXED_OVERHEAD
            )
        }
    )
    .ok_or(
        SurfaceCodeError::ArithmeticOverflow {
            resource:
                "surface-code memory estimate",
        }
    )
}

// ============================================================================
// Construction
// ============================================================================

fn build_data_qubits(
    distance: usize,
    expected_count: usize,
) -> Result<Vec<DataQubit>, SurfaceCodeError> {
    let mut data =
        Vec::with_capacity(
            expected_count
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
                )
            );
        }
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
/// Boundary checks are the alternating weight-2 checks:
///
/// ```text
/// left/right  -> X
/// top/bottom  -> Z
/// ```
///
/// Only every second boundary edge is populated. This is essential: placing
/// a weight-2 stabilizer on every boundary edge would generally violate the
/// X/Z commutation constraints.
fn build_stabilizers(
    distance: usize,
) -> Result<
    Vec<SurfaceStabilizer>,
    SurfaceCodeError,
> {
    let expected =
        checked_square(
            distance,
            "surface-code stabilizer construction",
        )?
        .checked_sub(1)
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code stabilizer construction count",
            }
        )?;

    let mut stabilizers =
        Vec::with_capacity(
            expected
        );

    let mut next_id = 0usize;

    // ------------------------------------------------------------------------
    // Bulk checkerboard stabilizers
    // ------------------------------------------------------------------------

    for row in 0..distance - 1 {
        for column in 0..distance - 1 {
            let support =
                [
                    checked_qubit_index(
                        distance,
                        row,
                        column,
                    )?,
                    checked_qubit_index(
                        distance,
                        row,
                        column + 1,
                    )?,
                    checked_qubit_index(
                        distance,
                        row + 1,
                        column,
                    )?,
                    checked_qubit_index(
                        distance,
                        row + 1,
                        column + 1,
                    )?,
                ]
                .to_vec();

            let kind =
                if (row + column) % 2 == 1 {
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
                )?
            );

            next_id =
                next_id
                    .checked_add(1)
                    .ok_or(
                        SurfaceCodeError::ArithmeticOverflow {
                            resource:
                                "surface-code stabilizer id",
                        }
                    )?;
        }
    }

    // ------------------------------------------------------------------------
    // Alternating boundary stabilizers
    // ------------------------------------------------------------------------

    //
    // Left boundary:
    //   X on even row segments.
    //
    for row in (0..distance - 1).step_by(2) {
        let support =
            vec![
                checked_qubit_index(
                    distance,
                    row,
                    0,
                )?,
                checked_qubit_index(
                    distance,
                    row + 1,
                    0,
                )?,
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::X,
                support,
                Some(Boundary::Left),
            )?
        );

        next_id =
            next_id
                .checked_add(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code left-boundary stabilizer id",
                    }
                )?;
    }

    //
    // Right boundary:
    //   X on odd row segments.
    //
    for row in (1..distance - 1).step_by(2) {
        let right =
            distance
                .checked_sub(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code right boundary coordinate",
                    }
                )?;

        let support =
            vec![
                checked_qubit_index(
                    distance,
                    row,
                    right,
                )?,
                checked_qubit_index(
                    distance,
                    row + 1,
                    right,
                )?,
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::X,
                support,
                Some(Boundary::Right),
            )?
        );

        next_id =
            next_id
                .checked_add(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code right-boundary stabilizer id",
                    }
                )?;
    }

    //
    // Top boundary:
    //   Z on odd column segments.
    //
    for column in (1..distance - 1).step_by(2) {
        let support =
            vec![
                checked_qubit_index(
                    distance,
                    0,
                    column,
                )?,
                checked_qubit_index(
                    distance,
                    0,
                    column + 1,
                )?,
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::Z,
                support,
                Some(Boundary::Top),
            )?
        );

        next_id =
            next_id
                .checked_add(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code top-boundary stabilizer id",
                    }
                )?;
    }

    //
    // Bottom boundary:
    //   Z on even column segments.
    //
    for column in (0..distance - 1).step_by(2) {
        let bottom =
            distance
                .checked_sub(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code bottom boundary coordinate",
                    }
                )?;

        let support =
            vec![
                checked_qubit_index(
                    distance,
                    bottom,
                    column,
                )?,
                checked_qubit_index(
                    distance,
                    bottom,
                    column + 1,
                )?,
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                next_id,
                StabilizerKind::Z,
                support,
                Some(Boundary::Bottom),
            )?
        );

        next_id =
            next_id
                .checked_add(1)
                .ok_or(
                    SurfaceCodeError::ArithmeticOverflow {
                        resource:
                            "surface-code bottom-boundary stabilizer id",
                    }
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
            }
        );
    }

    Ok(stabilizers)
}

// ============================================================================
// Logical operators
// ============================================================================

/// Canonical horizontal X logical string.
fn build_logical_x(
    distance: usize,
) -> Result<
    LogicalOperator,
    SurfaceCodeError,
> {
    let mut paulis =
        vec![Pauli::I; checked_square(
            distance,
            "logical-X construction",
        )?];

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
            &paulis
        ),
    )
}

/// Canonical vertical Z logical string.
fn build_logical_z(
    distance: usize,
) -> Result<
    LogicalOperator,
    SurfaceCodeError,
> {
    let mut paulis =
        vec![Pauli::I; checked_square(
            distance,
            "logical-Z construction",
        )?];

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
            &paulis
        ),
    )
}

// ============================================================================
// Arithmetic helpers
// ============================================================================

fn checked_square(
    value: usize,
    resource: &'static str,
) -> Result<usize, SurfaceCodeError> {
    value
        .checked_mul(value)
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource,
            }
        )
}

fn checked_qubit_index(
    distance: usize,
    row: usize,
    column: usize,
) -> Result<QubitIndex, SurfaceCodeError> {
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
            }
        );
    }

    let row_offset =
        row.checked_mul(
            distance
        )
        .ok_or(
            SurfaceCodeError::ArithmeticOverflow {
                resource:
                    "surface-code row offset",
            }
        )?;

    let index =
        row_offset
            .checked_add(column)
            .ok_or(
                SurfaceCodeError::ArithmeticOverflow {
                    resource:
                        "surface-code qubit index",
                }
            )?;

    Ok(
        QubitIndex::new(index)
    )
}