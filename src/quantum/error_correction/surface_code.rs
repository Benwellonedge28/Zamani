//! Zamani Quantum Error Correction — Surface Code.
//!
//! Hardware-independent representation of a 2-D surface code.
//!
//! This module describes:
//! - surface-code dimensions;
//! - data qubits;
//! - X and Z stabilizers;
//! - stabilizer-to-data-qubit connectivity;
//! - logical operators;
//! - boundaries.
//!
//! Decoding is intentionally kept outside this module. See
//! `surface_coder.rs` and the generic `decoder.rs` abstraction.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

// -----------------------------------------------------------------------------
// Coordinates
// -----------------------------------------------------------------------------

/// Coordinate in the surface-code lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCodeCoord {
    pub x: usize,
    pub y: usize,
}

impl SurfaceCodeCoord {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl fmt::Display for SurfaceCodeCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// -----------------------------------------------------------------------------
// Qubit identifiers
// -----------------------------------------------------------------------------

/// Identifier for a surface-code data qubit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataQubitId(pub usize);

impl DataQubitId {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for DataQubitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dq{}", self.0)
    }
}

/// Identifier for a stabilizer measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StabilizerId(pub usize);

impl StabilizerId {
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}

impl fmt::Display for StabilizerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "s{}", self.0)
    }
}

// -----------------------------------------------------------------------------
// Stabilizers
// -----------------------------------------------------------------------------

/// Surface-code stabilizer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StabilizerType {
    X,
    Z,
}

impl StabilizerType {
    /// Returns the Pauli error that anticommutes with this stabilizer.
    pub const fn detecting_pauli(self) -> PauliError {
        match self {
            Self::X => PauliError::Z,
            Self::Z => PauliError::X,
        }
    }
}

/// Abstract Pauli error type used by the surface-code model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauliError {
    X,
    Y,
    Z,
}

/// A stabilizer in the surface-code lattice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceStabilizer {
    id: StabilizerId,
    coordinate: SurfaceCodeCoord,
    kind: StabilizerType,
    data_qubits: Vec<DataQubitId>,
}

impl SurfaceStabilizer {
    pub fn new(
        id: StabilizerId,
        coordinate: SurfaceCodeCoord,
        kind: StabilizerType,
        data_qubits: Vec<DataQubitId>,
    ) -> Result<Self, SurfaceCodeError> {
        if data_qubits.is_empty() {
            return Err(SurfaceCodeError::EmptyStabilizer);
        }

        let mut unique = BTreeSet::new();

        for qubit in &data_qubits {
            if !unique.insert(*qubit) {
                return Err(
                    SurfaceCodeError::DuplicateQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        Ok(Self {
            id,
            coordinate,
            kind,
            data_qubits,
        })
    }

    pub const fn id(&self) -> StabilizerId {
        self.id
    }

    pub const fn coordinate(&self) -> SurfaceCodeCoord {
        self.coordinate
    }

    pub const fn kind(&self) -> StabilizerType {
        self.kind
    }

    pub fn data_qubits(&self) -> &[DataQubitId] {
        &self.data_qubits
    }
}

// -----------------------------------------------------------------------------
// Boundaries
// -----------------------------------------------------------------------------

/// Boundary of the surface-code patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SurfaceBoundary {
    Top,
    Bottom,
    Left,
    Right,
}

impl SurfaceBoundary {
    pub const fn distance(
        self,
        coordinate: SurfaceCodeCoord,
        width: usize,
        height: usize,
    ) -> usize {
        match self {
            Self::Top => {
                height.saturating_sub(1).saturating_sub(coordinate.y)
            }
            Self::Bottom => coordinate.y,
            Self::Left => coordinate.x,
            Self::Right => {
                width.saturating_sub(1).saturating_sub(coordinate.x)
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Logical operators
// -----------------------------------------------------------------------------

/// Logical Pauli operator represented by a path through the code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOperator {
    pauli: PauliError,
    path: Vec<DataQubitId>,
}

impl LogicalOperator {
    pub fn new(
        pauli: PauliError,
        path: Vec<DataQubitId>,
    ) -> Result<Self, SurfaceCodeError> {
        if path.is_empty() {
            return Err(SurfaceCodeError::EmptyLogicalOperator);
        }

        Ok(Self { pauli, path })
    }

    pub const fn pauli(&self) -> PauliError {
        self.pauli
    }

    pub fn path(&self) -> &[DataQubitId] {
        &self.path
    }
}

// -----------------------------------------------------------------------------
// Surface code
// -----------------------------------------------------------------------------

/// Complete hardware-independent surface-code description.
#[derive(Debug, Clone)]
pub struct SurfaceCode {
    distance: usize,
    width: usize,
    height: usize,
    data_qubits: BTreeMap<DataQubitId, SurfaceCodeCoord>,
    stabilizers: BTreeMap<StabilizerId, SurfaceStabilizer>,
    logical_x: Option<LogicalOperator>,
    logical_z: Option<LogicalOperator>,
}

impl SurfaceCode {
    /// Constructs a square surface-code patch.
    ///
    /// `distance` must be at least 2.
    pub fn new(distance: usize) -> Result<Self, SurfaceCodeError> {
        if distance < 2 {
            return Err(SurfaceCodeError::InvalidDistance {
                distance,
            });
        }

        let width = distance;
        let height = distance;

        Ok(Self {
            distance,
            width,
            height,
            data_qubits: BTreeMap::new(),
            stabilizers: BTreeMap::new(),
            logical_x: None,
            logical_z: None,
        })
    }

    pub const fn distance(&self) -> usize {
        self.distance
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    // -------------------------------------------------------------------------
    // Data qubits
    // -------------------------------------------------------------------------

    pub fn add_data_qubit(
        &mut self,
        id: DataQubitId,
        coordinate: SurfaceCodeCoord,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_coordinate(coordinate)?;

        if self.data_qubits.contains_key(&id) {
            return Err(
                SurfaceCodeError::DuplicateDataQubit {
                    qubit: id,
                },
            );
        }

        self.data_qubits.insert(id, coordinate);

        Ok(())
    }

    pub fn data_qubit(
        &self,
        id: DataQubitId,
    ) -> Option<&SurfaceCodeCoord> {
        self.data_qubits.get(&id)
    }

    pub fn data_qubits(
        &self,
    ) -> impl Iterator<Item = (DataQubitId, SurfaceCodeCoord)> + '_ {
        self.data_qubits
            .iter()
            .map(|(id, coordinate)| (*id, *coordinate))
    }

    pub fn data_qubit_count(&self) -> usize {
        self.data_qubits.len()
    }

    // -------------------------------------------------------------------------
    // Stabilizers
    // -------------------------------------------------------------------------

    pub fn add_stabilizer(
        &mut self,
        stabilizer: SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_coordinate(
            stabilizer.coordinate(),
        )?;

        if self.stabilizers.contains_key(&stabilizer.id()) {
            return Err(
                SurfaceCodeError::DuplicateStabilizer {
                    stabilizer: stabilizer.id(),
                },
            );
        }

        for qubit in stabilizer.data_qubits() {
            if !self.data_qubits.contains_key(qubit) {
                return Err(
                    SurfaceCodeError::UnknownDataQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        self.stabilizers
            .insert(stabilizer.id(), stabilizer);

        Ok(())
    }

    pub fn stabilizer(
        &self,
        id: StabilizerId,
    ) -> Option<&SurfaceStabilizer> {
        self.stabilizers.get(&id)
    }

    pub fn stabilizers(
        &self,
    ) -> impl Iterator<Item = &SurfaceStabilizer> {
        self.stabilizers.values()
    }

    pub fn stabilizer_count(&self) -> usize {
        self.stabilizers.len()
    }

    pub fn x_stabilizer_count(&self) -> usize {
        self.stabilizers
            .values()
            .filter(|s| s.kind() == StabilizerType::X)
            .count()
    }

    pub fn z_stabilizer_count(&self) -> usize {
        self.stabilizers
            .values()
            .filter(|s| s.kind() == StabilizerType::Z)
            .count()
    }

    // -------------------------------------------------------------------------
    // Connectivity
    // -------------------------------------------------------------------------

    pub fn stabilizers_for_qubit(
        &self,
        qubit: DataQubitId,
    ) -> Vec<&SurfaceStabilizer> {
        self.stabilizers
            .values()
            .filter(|stabilizer| {
                stabilizer.data_qubits().contains(&qubit)
            })
            .collect()
    }

    // -------------------------------------------------------------------------
    // Logical operators
    // -------------------------------------------------------------------------

    pub fn set_logical_x(
        &mut self,
        operator: LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_logical_operator(&operator)?;
        self.logical_x = Some(operator);
        Ok(())
    }

    pub fn set_logical_z(
        &mut self,
        operator: LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_logical_operator(&operator)?;
        self.logical_z = Some(operator);
        Ok(())
    }

    pub fn logical_x(&self) -> Option<&LogicalOperator> {
        self.logical_x.as_ref()
    }

    pub fn logical_z(&self) -> Option<&LogicalOperator> {
        self.logical_z.as_ref()
    }

    // -------------------------------------------------------------------------
    // Boundary utilities
    // -------------------------------------------------------------------------

    pub fn distance_to_boundary(
        &self,
        coordinate: SurfaceCodeCoord,
        boundary: SurfaceBoundary,
    ) -> Result<usize, SurfaceCodeError> {
        self.validate_coordinate(coordinate)?;

        Ok(boundary.distance(
            coordinate,
            self.width,
            self.height,
        ))
    }

    pub fn nearest_boundary(
        &self,
        coordinate: SurfaceCodeCoord,
    ) -> Result<SurfaceBoundary, SurfaceCodeError> {
        self.validate_coordinate(coordinate)?;

        let boundaries = [
            SurfaceBoundary::Top,
            SurfaceBoundary::Bottom,
            SurfaceBoundary::Left,
            SurfaceBoundary::Right,
        ];

        boundaries
            .into_iter()
            .min_by_key(|boundary| {
                boundary.distance(
                    coordinate,
                    self.width,
                    self.height,
                )
            })
            .ok_or(SurfaceCodeError::InvalidBoundary)
    }

    // -------------------------------------------------------------------------
    // Validation
    // -------------------------------------------------------------------------

    pub fn validate(&self) -> Result<(), SurfaceCodeError> {
        if self.distance < 2 {
            return Err(SurfaceCodeError::InvalidDistance {
                distance: self.distance,
            });
        }

        for stabilizer in self.stabilizers.values() {
            self.validate_coordinate(
                stabilizer.coordinate(),
            )?;

            for qubit in stabilizer.data_qubits() {
                if !self.data_qubits.contains_key(qubit) {
                    return Err(
                        SurfaceCodeError::UnknownDataQubit {
                            qubit: *qubit,
                        },
                    );
                }
            }
        }

        if let Some(operator) = &self.logical_x {
            self.validate_logical_operator(operator)?;
        }

        if let Some(operator) = &self.logical_z {
            self.validate_logical_operator(operator)?;
        }

        Ok(())
    }

    fn validate_coordinate(
        &self,
        coordinate: SurfaceCodeCoord,
    ) -> Result<(), SurfaceCodeError> {
        if coordinate.x >= self.width
            || coordinate.y >= self.height
        {
            return Err(
                SurfaceCodeError::CoordinateOutOfRange {
                    coordinate,
                },
            );
        }

        Ok(())
    }

    fn validate_logical_operator(
        &self,
        operator: &LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        for qubit in operator.path() {
            if !self.data_qubits.contains_key(qubit) {
                return Err(
                    SurfaceCodeError::UnknownDataQubit {
                        qubit: *qubit,
                    },
                );
            }
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceCodeError {
    InvalidDistance {
        distance: usize,
    },

    CoordinateOutOfRange {
        coordinate: SurfaceCodeCoord,
    },

    DuplicateDataQubit {
        qubit: DataQubitId,
    },

    DuplicateStabilizer {
        stabilizer: StabilizerId,
    },

    UnknownDataQubit {
        qubit: DataQubitId,
    },

    DuplicateQubit {
        qubit: DataQubitId,
    },

    EmptyStabilizer,

    EmptyLogicalOperator,

    InvalidBoundary,
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
                    "surface-code distance must be at least 2, got {distance}"
                )
            }

            Self::CoordinateOutOfRange { coordinate } => {
                write!(
                    f,
                    "surface-code coordinate {coordinate} is out of range"
                )
            }

            Self::DuplicateDataQubit { qubit } => {
                write!(
                    f,
                    "data qubit {qubit} already exists"
                )
            }

            Self::DuplicateStabilizer { stabilizer } => {
                write!(
                    f,
                    "stabilizer {stabilizer} already exists"
                )
            }

            Self::UnknownDataQubit { qubit } => {
                write!(
                    f,
                    "unknown data qubit {qubit}"
                )
            }

            Self::DuplicateQubit { qubit } => {
                write!(
                    f,
                    "qubit {qubit} occurs more than once"
                )
            }

            Self::EmptyStabilizer => {
                write!(
                    f,
                    "stabilizer must contain at least one data qubit"
                )
            }

            Self::EmptyLogicalOperator => {
                write!(
                    f,
                    "logical operator must contain at least one data qubit"
                )
            }

            Self::InvalidBoundary => {
                write!(
                    f,
                    "unable to determine surface-code boundary"
                )
            }
        }
    }
}

impl std::error::Error for SurfaceCodeError {}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_surface_code() {
        let code =
            SurfaceCode::new(3).unwrap();

        assert_eq!(code.distance(), 3);
        assert_eq!(code.width(), 3);
        assert_eq!(code.height(), 3);
    }

    #[test]
    fn rejects_invalid_distance() {
        assert!(
            SurfaceCode::new(1).is_err()
        );
    }

    #[test]
    fn adds_data_qubits() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        code.add_data_qubit(
            DataQubitId::new(0),
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        assert_eq!(
            code.data_qubit_count(),
            1
        );
    }

    #[test]
    fn rejects_duplicate_data_qubit() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        let id =
            DataQubitId::new(0);

        code.add_data_qubit(
            id,
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        assert!(
            code.add_data_qubit(
                id,
                SurfaceCodeCoord::new(1, 0),
            )
            .is_err()
        );
    }

    #[test]
    fn adds_stabilizer() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        code.add_data_qubit(
            DataQubitId::new(0),
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        let stabilizer =
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(0, 0),
                StabilizerType::X,
                vec![DataQubitId::new(0)],
            )
            .unwrap();

        code.add_stabilizer(
            stabilizer,
        )
        .unwrap();

        assert_eq!(
            code.stabilizer_count(),
            1
        );

        assert_eq!(
            code.x_stabilizer_count(),
            1
        );
    }

    #[test]
    fn stabilizer_requires_known_qubits() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        let stabilizer =
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(0, 0),
                StabilizerType::X,
                vec![DataQubitId::new(99)],
            )
            .unwrap();

        assert!(
            code.add_stabilizer(
                stabilizer
            )
            .is_err()
        );
    }

    #[test]
    fn finds_stabilizers_for_qubit() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        let qubit =
            DataQubitId::new(0);

        code.add_data_qubit(
            qubit,
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(0, 0),
                StabilizerType::X,
                vec![qubit],
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            code.stabilizers_for_qubit(
                qubit
            )
            .len(),
            1
        );
    }

    #[test]
    fn creates_logical_operator() {
        let operator =
            LogicalOperator::new(
                PauliError::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                ],
            )
            .unwrap();

        assert_eq!(
            operator.pauli(),
            PauliError::X
        );

        assert_eq!(
            operator.path().len(),
            2
        );
    }

    #[test]
    fn finds_nearest_boundary() {
        let code =
            SurfaceCode::new(5).unwrap();

        let boundary =
            code.nearest_boundary(
                SurfaceCodeCoord::new(0, 2),
            )
            .unwrap();

        assert_eq!(
            boundary,
            SurfaceBoundary::Left
        );
    }

    #[test]
    fn validates_complete_code() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        code.add_data_qubit(
            DataQubitId::new(0),
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(0, 0),
                StabilizerType::Z,
                vec![DataQubitId::new(0)],
            )
            .unwrap(),
        )
        .unwrap();

        assert!(
            code.validate().is_ok()
        );
    }
}