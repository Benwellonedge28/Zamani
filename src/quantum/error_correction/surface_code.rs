//! Zamani Quantum Error Correction — Rotated Planar Surface Code.
//!
//! This module provides an explicit, validated representation of a
//! rotated planar surface code.
//!
//! Mathematical model
//! ------------------
//!
//! For an odd code distance d >= 3:
//!
//!     data qubits = d²
//!     stabilizers = d² - 1
//!     logical qubits = 1
//!     code distance = d
//!
//! Data qubits are arranged on a d × d square grid:
//!
//!     q00 ─ q01 ─ q02 ─ ... ─ q0(d-1)
//!      │     │     │
//!     q10 ─ q11 ─ q12 ─ ... ─ q1(d-1)
//!      │     │     │
//!     ...
//!
//! Bulk stabilizers act on four data qubits.
//! Boundary stabilizers act on two data qubits.
//!
//! The stabilizers form a checkerboard of X- and Z-type operators.
//!
//! Canonical logical operators are:
//!
//!     X_L = X on one complete row
//!     Z_L = Z on one complete column
//!
//! These have weight d and anticommute exactly once.
//!
//! The implementation deliberately keeps topology explicit rather than
//! deriving it from qubit counts. This makes invalid geometries detectable
//! before a decoder or circuit generator is invoked.

use std::collections::BTreeSet;
use std::fmt;

use super::distance;
use super::stabilizer::{
    logical_operators_anticommute,
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

/// Two-dimensional coordinate of a data qubit.
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
    pub const fn new(
        row: usize,
        column: usize,
    ) -> Self {
        Self {
            row,
            column,
        }
    }

    pub const fn row(
        self,
    ) -> usize {
        self.row
    }

    pub const fn column(
        self,
    ) -> usize {
        self.column
    }
}

impl fmt::Display
    for Coordinate
{
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
    pub const fn new(
        index: QubitIndex,
        coordinate: Coordinate,
    ) -> Self {
        Self {
            index,
            coordinate,
        }
    }

    pub const fn index(
        self,
    ) -> QubitIndex {
        self.index
    }

    pub const fn coordinate(
        self,
    ) -> Coordinate {
        self.coordinate
    }
}

// ============================================================================
// Stabilizer kind
// ============================================================================

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
    pub const fn pauli(
        self,
    ) -> Pauli {
        match self {
            Self::X => Pauli::X,
            Self::Z => Pauli::Z,
        }
    }
}

impl fmt::Display
    for StabilizerKind
{
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

/// Boundary classification for a surface-code stabilizer.
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

impl fmt::Display
    for Boundary
{
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
// Stabilizer
// ============================================================================

/// Explicit surface-code stabilizer topology.
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
        support: Vec<QubitIndex>,
        boundary: Option<Boundary>,
    ) -> Result<Self, SurfaceCodeError> {
        if support.is_empty() {
            return Err(
                SurfaceCodeError::EmptyStabilizer {
                    id,
                },
            );
        }

        let mut unique =
            BTreeSet::new();

        for &qubit in
            &support
        {
            if !unique.insert(qubit) {
                return Err(
                    SurfaceCodeError::DuplicateQubitInStabilizer {
                        id,
                        qubit,
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

        if support.len()
            != expected_weight
        {
            return Err(
                SurfaceCodeError::InvalidStabilizerWeight {
                    id,
                    expected:
                        expected_weight,
                    actual:
                        support.len(),
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

    pub const fn id(
        &self,
    ) -> usize {
        self.id
    }

    pub const fn kind(
        &self,
    ) -> StabilizerKind {
        self.kind
    }

    pub fn support(
        &self,
    ) -> &[QubitIndex] {
        &self.support
    }

    pub fn boundary(
        &self,
    ) -> Option<Boundary> {
        self.boundary
    }

    pub fn weight(
        &self,
    ) -> usize {
        self.support.len()
    }

    pub fn is_boundary(
        &self,
    ) -> bool {
        self.boundary.is_some()
    }

    /// Converts this explicit topology into the generic Pauli-string IR.
    pub fn pauli_string(
        &self,
        num_qubits: usize,
    ) -> Result<PauliString, SurfaceCodeError> {
        let mut paulis =
            vec![Pauli::I; num_qubits];

        for &qubit in
            &self.support
        {
            let index =
                qubit.index();

            if index >= num_qubits {
                return Err(
                    SurfaceCodeError::NonexistentQubit {
                        stabilizer:
                            self.id,
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

    pub fn generator(
        &self,
        num_qubits: usize,
    ) -> Result<StabilizerGenerator, SurfaceCodeError> {
        let operator =
            self.pauli_string(
                num_qubits,
            )?;

        Ok(
            StabilizerGenerator::new(
                self.id,
                operator,
            )?
        )
    }
}

// ============================================================================
// Logical operator
// ============================================================================

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

    pub const fn name(
        &self,
    ) -> &'static str {
        self.name
    }

    pub const fn kind(
        &self,
    ) -> StabilizerKind {
        self.kind
    }

    pub fn operator(
        &self,
    ) -> &PauliString {
        &self.operator
    }

    pub fn weight(
        &self,
    ) -> usize {
        self.operator.weight()
    }
}

// ============================================================================
// Surface code
// ============================================================================

/// Explicit rotated planar surface-code topology.
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
    /// Constructs a canonical rotated planar surface code.
    ///
    /// The implementation currently requires an odd distance:
    ///
    ///     d = 3, 5, 7, ...
    ///
    /// This corresponds to the standard single-logical-qubit rotated planar
    /// surface-code family.
    pub fn new(
        distance: usize,
    ) -> Result<Self, SurfaceCodeError> {
        validate_distance_parameter(
            distance,
        )?;

        let data_qubits =
            build_data_qubits(
                distance,
            );

        let stabilizers =
            build_stabilizers(
                distance,
            )?;

        let logical_x =
            build_logical_x(
                distance,
            )?;

        let logical_z =
            build_logical_z(
                distance,
            )?;

        let code = Self {
            distance,
            data_qubits,
            stabilizers,
            logical_x,
            logical_z,
        };

        code.validate()?;

        Ok(code)
    }

    /// Alias for explicit construction from a code distance.
    pub fn from_distance(
        distance: usize,
    ) -> Result<Self, SurfaceCodeError> {
        Self::new(distance)
    }

    // ------------------------------------------------------------------------
    // Basic properties
    // ------------------------------------------------------------------------

    pub const fn distance(
        &self,
    ) -> usize {
        self.distance
    }

    pub fn num_data_qubits(
        &self,
    ) -> usize {
        self.data_qubits.len()
    }

    pub fn num_stabilizers(
        &self,
    ) -> usize {
        self.stabilizers.len()
    }

    pub const fn num_logical_qubits(
        &self,
    ) -> usize {
        1
    }

    pub fn data_qubits(
        &self,
    ) -> &[DataQubit] {
        &self.data_qubits
    }

    pub fn stabilizers(
        &self,
    ) -> &[SurfaceStabilizer] {
        &self.stabilizers
    }

    pub fn logical_x(
        &self,
    ) -> &LogicalOperator {
        &self.logical_x
    }

    pub fn logical_z(
        &self,
    ) -> &LogicalOperator {
        &self.logical_z
    }

    // ------------------------------------------------------------------------
    // Topology
    // ------------------------------------------------------------------------

    /// Converts a grid coordinate into a data-qubit index.
    pub fn qubit_at(
        &self,
        coordinate: Coordinate,
    ) -> Result<DataQubit, SurfaceCodeError> {
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

        let index =
            coordinate.row()
                * self.distance
                + coordinate.column();

        Ok(
            self.data_qubits[index]
        )
    }

    /// Returns the coordinate of a data qubit.
    pub fn coordinate_of(
        &self,
        qubit: QubitIndex,
    ) -> Result<Coordinate, SurfaceCodeError> {
        if qubit.index()
            >= self.data_qubits.len()
        {
            return Err(
                SurfaceCodeError::NonexistentQubit {
                    stabilizer:
                        usize::MAX,
                    qubit,
                },
            );
        }

        Ok(
            self.data_qubits
                [qubit.index()]
                .coordinate()
        )
    }

    /// Returns the four data-qubit coordinates surrounding a bulk face.
    pub fn face_qubits(
        &self,
        row: usize,
        column: usize,
    ) -> Result<[QubitIndex; 4], SurfaceCodeError> {
        if row + 1
            >= self.distance
            || column + 1
                >= self.distance
        {
            return Err(
                SurfaceCodeError::FaceOutOfRange {
                    row,
                    column,
                    distance:
                        self.distance,
                },
            );
        }

        let d =
            self.distance;

        Ok([
            QubitIndex(
                row * d + column,
            ),
            QubitIndex(
                row * d + column + 1,
            ),
            QubitIndex(
                (row + 1) * d + column,
            ),
            QubitIndex(
                (row + 1) * d + column + 1,
            ),
        ])
    }

    // ------------------------------------------------------------------------
    // Generic stabilizer representation
    // ------------------------------------------------------------------------

    /// Builds the generic stabilizer group used by the decoder,
    /// syndrome engine, and distance verifier.
    pub fn stabilizer_group(
        &self,
    ) -> Result<StabilizerGroup, SurfaceCodeError> {
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

        group.validate()?;

        Ok(group)
    }

    /// Computes a syndrome for a Pauli error.
    pub fn syndrome(
        &self,
        error: &PauliString,
    ) -> Result<Syndrome, SurfaceCodeError> {
        let group =
            self.stabilizer_group()?;

        Ok(
            group.syndrome(error)?
        )
    }

    // ------------------------------------------------------------------------
    // Logical operators
    // ------------------------------------------------------------------------

    /// Validates the logical operators against the stabilizer group.
    pub fn validate_logical_operators(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let group =
            self.stabilizer_group()?;

        validate_logical_operator(
            &self.logical_x,
            &group,
            self.distance,
        )?;

        validate_logical_operator(
            &self.logical_z,
            &group,
            self.distance,
        )?;

        if !logical_operators_anticommute(
            self.logical_x.operator(),
            self.logical_z.operator(),
        )? {
            return Err(
                SurfaceCodeError::LogicalOperatorsMustAnticommute,
            );
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Full validation
    // ------------------------------------------------------------------------

    /// Performs structural and mathematical validation.
    ///
    /// This does NOT perform the expensive exhaustive distance search.
    /// Use `verify_distance()` when an independent distance calculation is
    /// required.
    pub fn validate(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        validate_distance_parameter(
            self.distance,
        )?;

        self.validate_data_qubits()?;

        self.validate_stabilizer_topology()?;

        let group =
            self.stabilizer_group()?;

        group.validate()?;

        self.validate_logical_operators()?;

        Ok(())
    }

    /// Independently computes the exact distance using the generic
    /// stabilizer-distance implementation.
    ///
    /// This is intentionally separate from `validate()` because exact
    /// distance calculation can become expensive as the code grows.
    pub fn verify_distance(
        &self,
    ) -> Result<usize, SurfaceCodeError> {
        let group =
            self.stabilizer_group()?;

        let result =
            distance::compute_distance(
                &group,
            )?;

        if result.distance()
            != self.distance
        {
            return Err(
                SurfaceCodeError::DistanceMismatch {
                    expected:
                        self.distance,
                    actual:
                        result.distance(),
                },
            );
        }

        distance::validate_distance(
            &group,
            self.distance,
            self.logical_x.operator(),
        )?;

        Ok(
            result.distance()
        )
    }

    // ------------------------------------------------------------------------
    // Data-qubit validation
    // ------------------------------------------------------------------------

    fn validate_data_qubits(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let expected =
            self.distance
                * self.distance;

        if self.data_qubits.len()
            != expected
        {
            return Err(
                SurfaceCodeError::DataQubitCountMismatch {
                    expected,
                    actual:
                        self.data_qubits.len(),
                },
            );
        }

        let mut indices =
            BTreeSet::new();

        let mut coordinates =
            BTreeSet::new();

        for qubit
            in &self.data_qubits
        {
            if !indices.insert(
                qubit.index(),
            ) {
                return Err(
                    SurfaceCodeError::DuplicateDataQubit {
                        qubit:
                            qubit.index(),
                    },
                );
            }

            if !coordinates.insert(
                qubit.coordinate(),
            ) {
                return Err(
                    SurfaceCodeError::DuplicateCoordinate {
                        coordinate:
                            qubit.coordinate(),
                    },
                );
            }

            if qubit.coordinate().row()
                >= self.distance
                || qubit
                    .coordinate()
                    .column()
                    >= self.distance
            {
                return Err(
                    SurfaceCodeError::CoordinateOutOfRange {
                        coordinate:
                            qubit.coordinate(),
                        distance:
                            self.distance,
                    },
                );
            }
        }

        for index in
            0..expected
        {
            if !indices.contains(
                &QubitIndex(index),
            ) {
                return Err(
                    SurfaceCodeError::MissingDataQubit {
                        qubit:
                            QubitIndex(index),
                    },
                );
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Stabilizer topology validation
    // ------------------------------------------------------------------------

    fn validate_stabilizer_topology(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let expected =
            self.distance
                * self.distance
                - 1;

        if self.stabilizers.len()
            != expected
        {
            return Err(
                SurfaceCodeError::StabilizerCountMismatch {
                    expected,
                    actual:
                        self.stabilizers.len(),
                },
            );
        }

        let mut ids =
            BTreeSet::new();

        let mut supports =
            BTreeSet::new();

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

            let canonical_support =
                canonical_support(
                    stabilizer.support(),
                );

            if !supports.insert(
                (
                    stabilizer.kind(),
                    canonical_support,
                ),
            ) {
                return Err(
                    SurfaceCodeError::DuplicateStabilizerSupport {
                        id:
                            stabilizer.id(),
                    },
                );
            }

            for &qubit in
                stabilizer.support()
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

            validate_boundary_geometry(
                stabilizer,
                self.distance,
            )?;
        }

        // IDs are deliberately contiguous and deterministic.
        for id in
            0..expected
        {
            if !ids.contains(&id) {
                return Err(
                    SurfaceCodeError::MissingStabilizerId {
                        id,
                    },
                );
            }
        }

        // This invokes the symplectic commutation validation.
        let group =
            self.stabilizer_group()?;

        group.validate()?;

        Ok(())
    }
}

// ============================================================================
// Construction
// ============================================================================

fn validate_distance_parameter(
    distance: usize,
) -> Result<(), SurfaceCodeError> {
    if distance < 3 {
        return Err(
            SurfaceCodeError::DistanceTooSmall {
                distance,
            },
        );
    }

    if distance % 2 == 0 {
        return Err(
            SurfaceCodeError::DistanceMustBeOdd {
                distance,
            },
        );
    }

    Ok(())
}

fn build_data_qubits(
    distance: usize,
) -> Vec<DataQubit> {
    let mut qubits =
        Vec::with_capacity(
            distance * distance,
        );

    for row in
        0..distance
    {
        for column in
            0..distance
        {
            let index =
                row * distance
                    + column;

            qubits.push(
                DataQubit::new(
                    QubitIndex(index),
                    Coordinate::new(
                        row,
                        column,
                    ),
                ),
            );
        }
    }

    qubits
}

/// Builds the canonical rotated planar surface-code stabilizers.
///
/// Bulk:
///
///     checkerboard X/Z faces
///
/// Boundary:
///
///     X checks on left/right boundaries
///     Z checks on top/bottom boundaries
///
/// For odd d this produces:
///
///     (d² - 1) / 2 X stabilizers
///     (d² - 1) / 2 Z stabilizers
///
/// and therefore d² - 1 total stabilizers.
fn build_stabilizers(
    distance: usize,
) -> Result<Vec<SurfaceStabilizer>, SurfaceCodeError> {
    let mut stabilizers =
        Vec::with_capacity(
            distance * distance
                - 1,
        );

    let mut id =
        0usize;

    // ------------------------------------------------------------------------
    // Bulk four-body stabilizers
    // ------------------------------------------------------------------------

    for row in
        0..(distance - 1)
    {
        for column in
            0..(distance - 1)
        {
            let kind =
                if (row + column) % 2
                    == 1
                {
                    StabilizerKind::X
                } else {
                    StabilizerKind::Z
                };

            let d =
                distance;

            let support =
                vec![
                    QubitIndex(
                        row * d + column,
                    ),
                    QubitIndex(
                        row * d
                            + column
                            + 1,
                    ),
                    QubitIndex(
                        (row + 1)
                            * d
                            + column,
                    ),
                    QubitIndex(
                        (row + 1)
                            * d
                            + column
                            + 1,
                    ),
                ];

            stabilizers.push(
                SurfaceStabilizer::new(
                    id,
                    kind,
                    support,
                    None,
                )?,
            );

            id += 1;
        }
    }

    // ------------------------------------------------------------------------
    // X-type boundary stabilizers
    // ------------------------------------------------------------------------
    //
    // Alternating between left and right boundaries prevents duplicate
    // endpoint interactions while maintaining the CSS commutation rule.

    for row in
        0..(distance - 1)
    {
        if row % 2 == 0 {
            // Left boundary.
            let support =
                vec![
                    QubitIndex(
                        row * distance,
                    ),
                    QubitIndex(
                        (row + 1)
                            * distance,
                    ),
                ];

            stabilizers.push(
                SurfaceStabilizer::new(
                    id,
                    StabilizerKind::X,
                    support,
                    Some(
                        Boundary::Left,
                    ),
                )?,
            );
        } else {
            // Right boundary.
            let right =
                distance - 1;

            let support =
                vec![
                    QubitIndex(
                        row * distance
                            + right,
                    ),
                    QubitIndex(
                        (row + 1)
                            * distance
                            + right,
                    ),
                ];

            stabilizers.push(
                SurfaceStabilizer::new(
                    id,
                    StabilizerKind::X,
                    support,
                    Some(
                        Boundary::Right,
                    ),
                )?,
            );
        }

        id += 1;
    }

    // ------------------------------------------------------------------------
    // Z-type boundary stabilizers
    // ------------------------------------------------------------------------
    //
    // Top boundary pairs are shifted by one position relative to the bottom
    // boundary. This is required by the rotated-code checkerboard geometry.

    for column in
        (0..(distance - 1))
            .step_by(2)
    {
        // Top boundary.
        let support_top =
            vec![
                QubitIndex(
                    column + 1,
                ),
                QubitIndex(
                    column + 2,
                ),
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                id,
                StabilizerKind::Z,
                support_top,
                Some(Boundary::Top),
            )?,
        );

        id += 1;

        // Bottom boundary.
        let bottom =
            distance - 1;

        let support_bottom =
            vec![
                QubitIndex(
                    bottom * distance
                        + column,
                ),
                QubitIndex(
                    bottom * distance
                        + column
                        + 1,
                ),
            ];

        stabilizers.push(
            SurfaceStabilizer::new(
                id,
                StabilizerKind::Z,
                support_bottom,
                Some(
                    Boundary::Bottom,
                ),
            )?,
        );

        id += 1;
    }

    Ok(stabilizers)
}

// ============================================================================
// Logical operators
// ============================================================================

fn build_logical_x(
    distance: usize,
) -> Result<LogicalOperator, SurfaceCodeError> {
    // Canonical logical X is a horizontal string.
    let mut paulis =
        vec![Pauli::I; distance * distance];

    for column in
        0..distance
    {
        paulis[column] =
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

fn build_logical_z(
    distance: usize,
) -> Result<LogicalOperator, SurfaceCodeError> {
    // Canonical logical Z is a vertical string.
    let mut paulis =
        vec![Pauli::I; distance * distance];

    for row in
        0..distance
    {
        paulis[
            row * distance
        ] = Pauli::Z;
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
// Geometry validation
// ============================================================================

fn validate_boundary_geometry(
    stabilizer: &SurfaceStabilizer,
    distance: usize,
) -> Result<(), SurfaceCodeError> {
    let support =
        stabilizer.support();

    if stabilizer.is_boundary() {
        if support.len() != 2 {
            return Err(
                SurfaceCodeError::InvalidBoundaryWeight {
                    id:
                        stabilizer.id(),
                    weight:
                        support.len(),
                },
            );
        }

        let coordinates: Vec<Coordinate> =
            support
                .iter()
                .map(|qubit| {
                    let index =
                        qubit.index();

                    Coordinate::new(
                        index / distance,
                        index % distance,
                    )
                })
                .collect();

        let boundary =
            stabilizer
                .boundary()
                .expect(
                    "boundary stabilizer must have a boundary",
                );

        match boundary {
            Boundary::Top => {
                if !coordinates
                    .iter()
                    .all(|coordinate| {
                        coordinate.row()
                            == 0
                    })
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryGeometry {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }
            }

            Boundary::Bottom => {
                if !coordinates
                    .iter()
                    .all(|coordinate| {
                        coordinate.row()
                            == distance - 1
                    })
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryGeometry {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }
            }

            Boundary::Left => {
                if !coordinates
                    .iter()
                    .all(|coordinate| {
                        coordinate
                            .column()
                            == 0
                    })
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryGeometry {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }
            }

            Boundary::Right => {
                if !coordinates
                    .iter()
                    .all(|coordinate| {
                        coordinate
                            .column()
                            == distance - 1
                    })
                {
                    return Err(
                        SurfaceCodeError::InvalidBoundaryGeometry {
                            id:
                                stabilizer.id(),
                            boundary,
                        },
                    );
                }
            }
        }
    } else if support.len() != 4 {
        return Err(
            SurfaceCodeError::InvalidBulkWeight {
                id:
                    stabilizer.id(),
                weight:
                    support.len(),
            },
        );
    }

    Ok(())
}

fn canonical_support(
    support: &[QubitIndex],
) -> Vec<usize> {
    let mut result: Vec<usize> =
        support
            .iter()
            .map(
                |qubit| qubit.index(),
            )
            .collect();

    result.sort_unstable();

    result
}

// ============================================================================
// Logical validation
// ============================================================================

fn validate_logical_operator(
    logical: &LogicalOperator,
    group: &StabilizerGroup,
    expected_distance: usize,
) -> Result<(), SurfaceCodeError> {
    if logical.weight()
        != expected_distance
    {
        return Err(
            SurfaceCodeError::LogicalOperatorWeightMismatch {
                name:
                    logical.name(),
                expected:
                    expected_distance,
                actual:
                    logical.weight(),
            },
        );
    }

    if logical.operator().is_identity() {
        return Err(
            SurfaceCodeError::IdentityLogicalOperator {
                name:
                    logical.name(),
            },
        );
    }

    for stabilizer
        in group.generators()
    {
        if logical
            .operator()
            .anticommutes_with(
                stabilizer.operator(),
            )?
        {
            return Err(
                SurfaceCodeError::LogicalOperatorDoesNotCommute {
                    name:
                        logical.name(),
                    stabilizer:
                        stabilizer.id(),
                },
            );
        }
    }

    if group.contains(
        logical.operator(),
    )? {
        return Err(
            SurfaceCodeError::LogicalOperatorIsStabilizer {
                name:
                    logical.name(),
            },
        );
    }

    Ok(())
}

// ============================================================================
// Errors
// ============================================================================

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub enum SurfaceCodeError {
    Stabilizer(
        StabilizerError,
    ),

    DistanceTooSmall {
        distance: usize,
    },

    DistanceMustBeOdd {
        distance: usize,
    },

    DataQubitCountMismatch {
        expected: usize,
        actual: usize,
    },

    StabilizerCountMismatch {
        expected: usize,
        actual: usize,
    },

    DuplicateDataQubit {
        qubit: QubitIndex,
    },

    MissingDataQubit {
        qubit: QubitIndex,
    },

    DuplicateCoordinate {
        coordinate: Coordinate,
    },

    CoordinateOutOfRange {
        coordinate: Coordinate,
        distance: usize,
    },

    FaceOutOfRange {
        row: usize,
        column: usize,
        distance: usize,
    },

    DuplicateStabilizerId {
        id: usize,
    },

    MissingStabilizerId {
        id: usize,
    },

    DuplicateStabilizerSupport {
        id: usize,
    },

    EmptyStabilizer {
        id: usize,
    },

    DuplicateQubitInStabilizer {
        id: usize,
        qubit: QubitIndex,
    },

    NonexistentQubit {
        stabilizer: usize,
        qubit: QubitIndex,
    },

    InvalidStabilizerWeight {
        id: usize,
        expected: usize,
        actual: usize,
    },

    InvalidBoundaryWeight {
        id: usize,
        weight: usize,
    },

    InvalidBulkWeight {
        id: usize,
        weight: usize,
    },

    InvalidBoundaryGeometry {
        id: usize,
        boundary: Boundary,
    },

    IdentityLogicalOperator {
        name: &'static str,
    },

    LogicalOperatorWeightMismatch {
        name: &'static str,
        expected: usize,
        actual: usize,
    },

    LogicalOperatorDoesNotCommute {
        name: &'static str,
        stabilizer: usize,
    },

    LogicalOperatorIsStabilizer {
        name: &'static str,
    },

    LogicalOperatorsMustAnticommute,

    DistanceMismatch {
        expected: usize,
        actual: usize,
    },

    Distance(
        distance::DistanceError,
    ),
}

impl fmt::Display
    for SurfaceCodeError
{
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Stabilizer(error) => {
                write!(
                    f,
                    "stabilizer error: {error}"
                )
            }

            Self::DistanceTooSmall {
                distance,
            } => {
                write!(
                    f,
                    "surface-code distance {distance} is too small; minimum supported distance is 3"
                )
            }

            Self::DistanceMustBeOdd {
                distance,
            } => {
                write!(
                    f,
                    "surface-code distance {distance} must be odd for this canonical rotated planar construction"
                )
            }

            Self::DataQubitCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "data-qubit count mismatch: expected {expected}, found {actual}"
                )
            }

            Self::StabilizerCountMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "stabilizer count mismatch: expected {expected}, found {actual}"
                )
            }

            Self::DuplicateDataQubit {
                qubit,
            } => {
                write!(
                    f,
                    "duplicate data qubit: {qubit}"
                )
            }

            Self::MissingDataQubit {
                qubit,
            } => {
                write!(
                    f,
                    "missing data qubit: {qubit}"
                )
            }

            Self::DuplicateCoordinate {
                coordinate,
            } => {
                write!(
                    f,
                    "duplicate data-qubit coordinate: {coordinate}"
                )
            }

            Self::CoordinateOutOfRange {
                coordinate,
                distance,
            } => {
                write!(
                    f,
                    "coordinate {coordinate} is outside the {distance}×{distance} lattice"
                )
            }

            Self::FaceOutOfRange {
                row,
                column,
                distance,
            } => {
                write!(
                    f,
                    "face ({row}, {column}) is outside the {distance}×{distance} data lattice"
                )
            }

            Self::DuplicateStabilizerId {
                id,
            } => {
                write!(
                    f,
                    "duplicate stabilizer ID: {id}"
                )
            }

            Self::MissingStabilizerId {
                id,
            } => {
                write!(
                    f,
                    "missing stabilizer ID: {id}"
                )
            }

            Self::DuplicateStabilizerSupport {
                id,
            } => {
                write!(
                    f,
                    "duplicate stabilizer support for stabilizer {id}"
                )
            }

            Self::EmptyStabilizer {
                id,
            } => {
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
                    "stabilizer {id} references qubit {qubit} more than once"
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

            Self::InvalidStabilizerWeight {
                id,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "stabilizer {id} has invalid weight {actual}; expected {expected}"
                )
            }

            Self::InvalidBoundaryWeight {
                id,
                weight,
            } => {
                write!(
                    f,
                    "boundary stabilizer {id} has invalid weight {weight}; expected 2"
                )
            }

            Self::InvalidBulkWeight {
                id,
                weight,
            } => {
                write!(
                    f,
                    "bulk stabilizer {id} has invalid weight {weight}; expected 4"
                )
            }

            Self::InvalidBoundaryGeometry {
                id,
                boundary,
            } => {
                write!(
                    f,
                    "stabilizer {id} is not geometrically valid for the {boundary} boundary"
                )
            }

            Self::IdentityLogicalOperator {
                name,
            } => {
                write!(
                    f,
                    "logical operator {name} cannot be identity"
                )
            }

            Self::LogicalOperatorWeightMismatch {
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
                name,
                stabilizer,
            } => {
                write!(
                    f,
                    "logical operator {name} anticommutes with stabilizer {stabilizer}"
                )
            }

            Self::LogicalOperatorIsStabilizer {
                name,
            } => {
                write!(
                    f,
                    "logical operator {name} is contained in the stabilizer group"
                )
            }

            Self::LogicalOperatorsMustAnticommute => {
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::DistanceMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "code-distance mismatch: expected {expected}, independently calculated {actual}"
                )
            }

            Self::Distance(error) => {
                write!(
                    f,
                    "distance error: {error}"
                )
            }
        }
    }
}

impl std::error::Error
    for SurfaceCodeError
{
}

impl From<StabilizerError>
    for SurfaceCodeError
{
    fn from(
        error: StabilizerError,
    ) -> Self {
        Self::Stabilizer(error)
    }
}

impl From<distance::DistanceError>
    for SurfaceCodeError
{
    fn from(
        error: distance::DistanceError,
    ) -> Self {
        Self::Distance(error)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_three_has_nine_data_qubits() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        assert_eq!(
            code.num_data_qubits(),
            9
        );

        assert_eq!(
            code.num_stabilizers(),
            8
        );

        assert_eq!(
            code.num_logical_qubits(),
            1
        );
    }

    #[test]
    fn distance_five_has_twenty_five_data_qubits() {
        let code =
            SurfaceCode::new(5)
                .unwrap();

        assert_eq!(
            code.num_data_qubits(),
            25
        );

        assert_eq!(
            code.num_stabilizers(),
            24
        );
    }

    #[test]
    fn stabilizer_counts_are_balanced() {
        let code =
            SurfaceCode::new(5)
                .unwrap();

        let x_count =
            code.stabilizers()
                .iter()
                .filter(|s| {
                    s.kind()
                        == StabilizerKind::X
                })
                .count();

        let z_count =
            code.stabilizers()
                .iter()
                .filter(|s| {
                    s.kind()
                        == StabilizerKind::Z
                })
                .count();

        assert_eq!(
            x_count,
            12
        );

        assert_eq!(
            z_count,
            12
        );
    }

    #[test]
    fn bulk_stabilizers_have_weight_four() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        for stabilizer
            in code.stabilizers()
        {
            if !stabilizer.is_boundary() {
                assert_eq!(
                    stabilizer.weight(),
                    4
                );
            }
        }
    }

    #[test]
    fn boundary_stabilizers_have_weight_two() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        for stabilizer
            in code.stabilizers()
        {
            if stabilizer.is_boundary() {
                assert_eq!(
                    stabilizer.weight(),
                    2
                );
            }
        }
    }

    #[test]
    fn all_stabilizers_commute() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        let group =
            code.stabilizer_group()
                .unwrap();

        assert!(
            group.validate().is_ok()
        );
    }

    #[test]
    fn logical_operators_are_valid() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        assert!(
            code
                .validate_logical_operators()
                .is_ok()
        );
    }

    #[test]
    fn logical_operators_have_distance_weight() {
        let code =
            SurfaceCode::new(5)
                .unwrap();

        assert_eq!(
            code.logical_x().weight(),
            5
        );

        assert_eq!(
            code.logical_z().weight(),
            5
        );
    }

    #[test]
    fn logical_operators_anticommute() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        assert!(
            code.logical_x()
                .operator()
                .anticommutes_with(
                    code.logical_z()
                        .operator()
                )
                .unwrap()
        );
    }

    #[test]
    fn coordinate_mapping_is_correct() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        let qubit =
            code.qubit_at(
                Coordinate::new(
                    2,
                    1,
                ),
            )
            .unwrap();

        assert_eq!(
            qubit.index(),
            QubitIndex(7)
        );
    }

    #[test]
    fn topology_validation_succeeds() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        assert!(
            code.validate().is_ok()
        );
    }

    #[test]
    fn rejects_even_distance() {
        assert!(matches!(
            SurfaceCode::new(4),
            Err(
                SurfaceCodeError::
                    DistanceMustBeOdd {
                        distance: 4
                    }
            )
        ));
    }

    #[test]
    fn rejects_distance_two() {
        assert!(matches!(
            SurfaceCode::new(2),
            Err(
                SurfaceCodeError::
                    DistanceTooSmall {
                        distance: 2
                    }
            )
        ));
    }

    #[test]
    fn distance_three_can_be_verified_independently() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        assert_eq!(
            code.verify_distance()
                .unwrap(),
            3
        );
    }

    #[test]
    fn syndrome_has_one_bit_per_stabilizer() {
        let code =
            SurfaceCode::new(3)
                .unwrap();

        let error =
            PauliString::from_paulis(
                &[
                    Pauli::X,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                    Pauli::I,
                ],
            );

        let syndrome =
            code.syndrome(
                &error,
            )
            .unwrap();

        assert_eq!(
            syndrome.len(),
            8
        );
    }
}