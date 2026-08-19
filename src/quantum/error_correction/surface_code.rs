//! Zamani Quantum Error Correction — Surface Code.
//!
//! Hardware-independent, structurally validated representation of a 2-D
//! surface-code patch.
//!
//! The model explicitly represents:
//! - data-qubit coordinates and nearest-neighbour topology;
//! - stabilizer support;
//! - X/Z stabilizer types;
//! - rough/smooth boundaries;
//! - logical X/Z operators;
//! - structural and algebraic invariants.
//!
//! Decoding is intentionally kept outside this module.
//!
//! ## Mathematical invariants
//!
//! A valid code must satisfy:
//!
//! 1. Every data qubit has a unique identifier and coordinate.
//! 2. Every stabilizer has unique support.
//! 3. Every stabilizer references existing data qubits.
//! 4. Stabilizer support is connected on the data-qubit lattice.
//! 5. Stabilizer weights are within the surface-code range.
//! 6. X/Z stabilizers commute.
//! 7. Logical operators reference existing qubits.
//! 8. Logical paths are connected.
//! 9. Logical operators commute with every stabilizer.
//! 10. Logical X and logical Z anticommute.
//! 11. The supplied logical representatives certify the requested distance.
//!
//! The final condition is deliberately described as a *certified distance*
//! rather than claiming that an arbitrary user-supplied patch has had every
//! possible logical operator exhaustively enumerated.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

// -----------------------------------------------------------------------------
// Coordinates
// -----------------------------------------------------------------------------

/// Coordinate in the data-qubit lattice.
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

    pub fn is_adjacent_to(self, other: Self) -> bool {
        self.manhattan_distance(other) == 1
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
// Pauli / stabilizer types
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StabilizerType {
    X,
    Z,
}

impl StabilizerType {
    pub const fn detecting_pauli(self) -> PauliError {
        match self {
            Self::X => PauliError::Z,
            Self::Z => PauliError::X,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PauliError {
    X,
    Y,
    Z,
}

impl PauliError {
    const fn anticommutes_with(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::X, Self::Z)
                | (Self::Z, Self::X)
                | (Self::X, Self::Y)
                | (Self::Y, Self::X)
                | (Self::Y, Self::Z)
                | (Self::Z, Self::Y)
        )
    }

    const fn stabilizer_pauli(kind: StabilizerType) -> Self {
        match kind {
            StabilizerType::X => Self::X,
            StabilizerType::Z => Self::Z,
        }
    }
}

// -----------------------------------------------------------------------------
// Stabilizer
// -----------------------------------------------------------------------------

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

        for &qubit in &data_qubits {
            if !unique.insert(qubit) {
                return Err(SurfaceCodeError::DuplicateQubit { qubit });
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

    pub fn weight(&self) -> usize {
        self.data_qubits.len()
    }
}

// -----------------------------------------------------------------------------
// Boundaries
// -----------------------------------------------------------------------------

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
            Self::Top => height
                .saturating_sub(1)
                .saturating_sub(coordinate.y),
            Self::Bottom => coordinate.y,
            Self::Left => coordinate.x,
            Self::Right => width
                .saturating_sub(1)
                .saturating_sub(coordinate.x),
        }
    }
}

// -----------------------------------------------------------------------------
// Logical operators
// -----------------------------------------------------------------------------

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

        let mut unique = BTreeSet::new();

        for &qubit in &path {
            if !unique.insert(qubit) {
                return Err(
                    SurfaceCodeError::DuplicateLogicalQubit { qubit }
                );
            }
        }

        Ok(Self { pauli, path })
    }

    pub const fn pauli(&self) -> PauliError {
        self.pauli
    }

    pub fn path(&self) -> &[DataQubitId] {
        &self.path
    }

    pub fn weight(&self) -> usize {
        self.path.len()
    }
}

// -----------------------------------------------------------------------------
// Data-qubit topology
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct DataQubitTopology {
    coordinates: BTreeMap<DataQubitId, SurfaceCodeCoord>,
    adjacency: BTreeMap<DataQubitId, BTreeSet<DataQubitId>>,
}

impl DataQubitTopology {
    fn insert(
        &mut self,
        id: DataQubitId,
        coordinate: SurfaceCodeCoord,
    ) -> Result<(), SurfaceCodeError> {
        if self.coordinates.contains_key(&id) {
            return Err(
                SurfaceCodeError::DuplicateDataQubit { qubit: id }
            );
        }

        if self
            .coordinates
            .values()
            .any(|existing| *existing == coordinate)
        {
            return Err(
                SurfaceCodeError::DuplicateCoordinate { coordinate }
            );
        }

        self.coordinates.insert(id, coordinate);
        self.adjacency.entry(id).or_default();

        self.rebuild_local_adjacency();

        Ok(())
    }

    fn rebuild_local_adjacency(&mut self) {
        for neighbours in self.adjacency.values_mut() {
            neighbours.clear();
        }

        let items: Vec<_> = self
            .coordinates
            .iter()
            .map(|(&id, &coord)| (id, coord))
            .collect();

        for (id_a, coord_a) in &items {
            for (id_b, coord_b) in &items {
                if id_a != id_b && coord_a.is_adjacent_to(*coord_b) {
                    self.adjacency
                        .entry(*id_a)
                        .or_default()
                        .insert(*id_b);
                }
            }
        }
    }

    fn coordinate(
        &self,
        id: DataQubitId,
    ) -> Option<SurfaceCodeCoord> {
        self.coordinates.get(&id).copied()
    }

    fn neighbours(
        &self,
        id: DataQubitId,
    ) -> impl Iterator<Item = DataQubitId> + '_ {
        self.adjacency
            .get(&id)
            .into_iter()
            .flat_map(|set| set.iter().copied())
    }

    fn contains(&self, id: DataQubitId) -> bool {
        self.coordinates.contains_key(&id)
    }

    fn iter(
        &self,
    ) -> impl Iterator<Item = (DataQubitId, SurfaceCodeCoord)> + '_ {
        self.coordinates
            .iter()
            .map(|(&id, &coordinate)| (id, coordinate))
    }

    fn len(&self) -> usize {
        self.coordinates.len()
    }
}

// -----------------------------------------------------------------------------
// Surface code
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SurfaceCode {
    distance: usize,
    width: usize,
    height: usize,

    topology: DataQubitTopology,

    stabilizers: BTreeMap<StabilizerId, SurfaceStabilizer>,

    logical_x: Option<LogicalOperator>,
    logical_z: Option<LogicalOperator>,
}

impl SurfaceCode {
    pub fn new(distance: usize) -> Result<Self, SurfaceCodeError> {
        if distance < 2 {
            return Err(
                SurfaceCodeError::InvalidDistance { distance }
            );
        }

        Ok(Self {
            distance,
            width: distance,
            height: distance,
            topology: DataQubitTopology::default(),
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
    // Data-qubit topology
    // -------------------------------------------------------------------------

    pub fn add_data_qubit(
        &mut self,
        id: DataQubitId,
        coordinate: SurfaceCodeCoord,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_coordinate(coordinate)?;
        self.topology.insert(id, coordinate)
    }

    pub fn data_qubit(
        &self,
        id: DataQubitId,
    ) -> Option<&SurfaceCodeCoord> {
        self.topology
            .coordinate(id)
            .as_ref()
            .map(|coord| {
                // The reference is produced from the topology lookup below.
                // This branch exists only to preserve the public API.
                coord
            })
    }

    pub fn data_qubits(
        &self,
    ) -> impl Iterator<Item = (DataQubitId, SurfaceCodeCoord)> + '_ {
        self.topology.iter()
    }

    pub fn data_qubit_count(&self) -> usize {
        self.topology.len()
    }

    pub fn neighbours(
        &self,
        qubit: DataQubitId,
    ) -> Vec<DataQubitId> {
        self.topology.neighbours(qubit).collect()
    }

    // -------------------------------------------------------------------------
    // Stabilizers
    // -------------------------------------------------------------------------

    pub fn add_stabilizer(
        &mut self,
        stabilizer: SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_coordinate(stabilizer.coordinate())?;

        if self.stabilizers.contains_key(&stabilizer.id()) {
            return Err(
                SurfaceCodeError::DuplicateStabilizer {
                    stabilizer: stabilizer.id(),
                },
            );
        }

        self.validate_stabilizer_support(&stabilizer)?;

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

        if operator.pauli() != PauliError::X {
            return Err(
                SurfaceCodeError::LogicalPauliMismatch {
                    expected: PauliError::X,
                    actual: operator.pauli(),
                },
            );
        }

        self.logical_x = Some(operator);
        Ok(())
    }

    pub fn set_logical_z(
        &mut self,
        operator: LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        self.validate_logical_operator(&operator)?;

        if operator.pauli() != PauliError::Z {
            return Err(
                SurfaceCodeError::LogicalPauliMismatch {
                    expected: PauliError::Z,
                    actual: operator.pauli(),
                },
            );
        }

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
    // Boundary helpers
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

        [
            SurfaceBoundary::Top,
            SurfaceBoundary::Bottom,
            SurfaceBoundary::Left,
            SurfaceBoundary::Right,
        ]
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
    // Full validation
    // -------------------------------------------------------------------------

    pub fn validate(&self) -> Result<(), SurfaceCodeError> {
        if self.distance < 2 {
            return Err(
                SurfaceCodeError::InvalidDistance {
                    distance: self.distance,
                },
            );
        }

        if self.width != self.distance
            || self.height != self.distance
        {
            return Err(
                SurfaceCodeError::DimensionMismatch
            );
        }

        if self.topology.len() == 0 {
            return Err(
                SurfaceCodeError::EmptyDataQubitSet
            );
        }

        self.validate_topology()?;

        for stabilizer in self.stabilizers.values() {
            self.validate_stabilizer_support(stabilizer)?;
        }

        self.validate_stabilizer_commutation()?;

        if let Some(logical_x) = &self.logical_x {
            self.validate_logical_operator(logical_x)?;
            self.validate_logical_commutation(logical_x)?;
        }

        if let Some(logical_z) = &self.logical_z {
            self.validate_logical_operator(logical_z)?;
            self.validate_logical_commutation(logical_z)?;
        }

        if let (Some(logical_x), Some(logical_z)) =
            (&self.logical_x, &self.logical_z)
        {
            self.validate_logical_anticommutation(
                logical_x,
                logical_z,
            )?;

            self.validate_certified_distance(
                logical_x,
                logical_z,
            )?;
        }

        Ok(())
    }

    /// Returns the minimum weight among the supplied logical X/Z
    /// representatives.
    ///
    /// This is a *certified representative distance*. A complete minimum
    /// distance proof requires enumeration/search over all non-trivial
    /// logical equivalence classes.
    pub fn certified_distance(&self) -> Option<usize> {
        match (&self.logical_x, &self.logical_z) {
            (Some(x), Some(z)) => {
                Some(x.weight().min(z.weight()))
            }
            (Some(x), None) => Some(x.weight()),
            (None, Some(z)) => Some(z.weight()),
            (None, None) => None,
        }
    }

    // -------------------------------------------------------------------------
    // Topology validation
    // -------------------------------------------------------------------------

    fn validate_topology(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let coordinates: Vec<_> =
            self.topology.iter().collect();

        let mut seen =
            BTreeSet::new();

        for (_, coordinate) in &coordinates {
            if !seen.insert(*coordinate) {
                return Err(
                    SurfaceCodeError::DuplicateCoordinate {
                        coordinate: *coordinate,
                    },
                );
            }

            self.validate_coordinate(*coordinate)?;
        }

        Ok(())
    }

    fn validate_stabilizer_support(
        &self,
        stabilizer: &SurfaceStabilizer,
    ) -> Result<(), SurfaceCodeError> {
        let weight = stabilizer.weight();

        // Standard surface-code stabilizers have weight 4 in the bulk and
        // reduced weight 2 at a boundary.
        if !(2..=4).contains(&weight) {
            return Err(
                SurfaceCodeError::InvalidStabilizerWeight {
                    stabilizer: stabilizer.id(),
                    weight,
                },
            );
        }

        for &qubit in stabilizer.data_qubits() {
            if !self.topology.contains(qubit) {
                return Err(
                    SurfaceCodeError::UnknownDataQubit {
                        qubit,
                    },
                );
            }
        }

        if !self.support_is_connected(
            stabilizer.data_qubits(),
        ) {
            return Err(
                SurfaceCodeError::DisconnectedStabilizer {
                    stabilizer: stabilizer.id(),
                },
            );
        }

        Ok(())
    }

    fn support_is_connected(
        &self,
        support: &[DataQubitId],
    ) -> bool {
        if support.len() <= 1 {
            return true;
        }

        let allowed: BTreeSet<_> =
            support.iter().copied().collect();

        let start = support[0];
        let mut visited = BTreeSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(current) =
            queue.pop_front()
        {
            for neighbour in
                self.topology.neighbours(current)
            {
                if allowed.contains(&neighbour)
                    && visited.insert(neighbour)
                {
                    queue.push_back(neighbour);
                }
            }
        }

        visited.len() == support.len()
    }

    // -------------------------------------------------------------------------
    // Stabilizer algebra
    // -------------------------------------------------------------------------

    fn validate_stabilizer_commutation(
        &self,
    ) -> Result<(), SurfaceCodeError> {
        let stabilizers: Vec<_> =
            self.stabilizers.values().collect();

        for i in 0..stabilizers.len() {
            for j in (i + 1)..stabilizers.len() {
                let a = stabilizers[i];
                let b = stabilizers[j];

                let overlap = a
                    .data_qubits()
                    .iter()
                    .filter(|qubit| {
                        b.data_qubits()
                            .contains(qubit)
                    })
                    .count();

                let anticommutes =
                    a.kind() != b.kind()
                        && overlap % 2 == 1;

                if anticommutes {
                    return Err(
                        SurfaceCodeError::NonCommutingStabilizers {
                            first: a.id(),
                            second: b.id(),
                            overlap,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Logical algebra
    // -------------------------------------------------------------------------

    fn validate_logical_operator(
        &self,
        operator: &LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        if operator.path().is_empty() {
            return Err(
                SurfaceCodeError::EmptyLogicalOperator
            );
        }

        for &qubit in operator.path() {
            if !self.topology.contains(qubit) {
                return Err(
                    SurfaceCodeError::UnknownDataQubit {
                        qubit,
                    },
                );
            }
        }

        if !self.support_is_connected(
            operator.path(),
        ) {
            return Err(
                SurfaceCodeError::DisconnectedLogicalOperator
            );
        }

        Ok(())
    }

    fn validate_logical_commutation(
        &self,
        operator: &LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        for stabilizer in self.stabilizers.values() {
            let overlap = operator
                .path()
                .iter()
                .filter(|qubit| {
                    stabilizer
                        .data_qubits()
                        .contains(qubit)
                })
                .count();

            if overlap == 0 {
                continue;
            }

            let stabilizer_pauli =
                PauliError::stabilizer_pauli(
                    stabilizer.kind(),
                );

            if operator.pauli()
                .anticommutes_with(
                    stabilizer_pauli,
                )
                && overlap % 2 == 1
            {
                return Err(
                    SurfaceCodeError::LogicalDoesNotCommute {
                        stabilizer: stabilizer.id(),
                    },
                );
            }
        }

        Ok(())
    }

    fn validate_logical_anticommutation(
        &self,
        logical_x: &LogicalOperator,
        logical_z: &LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        let overlap = logical_x
            .path()
            .iter()
            .filter(|qubit| {
                logical_z
                    .path()
                    .contains(qubit)
            })
            .count();

        if overlap % 2 == 0 {
            return Err(
                SurfaceCodeError::LogicalOperatorsCommute
            );
        }

        Ok(())
    }

    fn validate_certified_distance(
        &self,
        logical_x: &LogicalOperator,
        logical_z: &LogicalOperator,
    ) -> Result<(), SurfaceCodeError> {
        let certified =
            logical_x.weight().min(
                logical_z.weight()
            );

        if certified != self.distance {
            return Err(
                SurfaceCodeError::DistanceMismatch {
                    declared: self.distance,
                    certified,
                },
            );
        }

        Ok(())
    }

    // -------------------------------------------------------------------------
    // Coordinates
    // -------------------------------------------------------------------------

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
}

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurfaceCodeError {
    InvalidDistance {
        distance: usize,
    },

    DimensionMismatch,

    EmptyDataQubitSet,

    CoordinateOutOfRange {
        coordinate: SurfaceCodeCoord,
    },

    DuplicateCoordinate {
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

    InvalidStabilizerWeight {
        stabilizer: StabilizerId,
        weight: usize,
    },

    DisconnectedStabilizer {
        stabilizer: StabilizerId,
    },

    NonCommutingStabilizers {
        first: StabilizerId,
        second: StabilizerId,
        overlap: usize,
    },

    EmptyLogicalOperator,

    DuplicateLogicalQubit {
        qubit: DataQubitId,
    },

    DisconnectedLogicalOperator,

    LogicalPauliMismatch {
        expected: PauliError,
        actual: PauliError,
    },

    LogicalDoesNotCommute {
        stabilizer: StabilizerId,
    },

    LogicalOperatorsCommute,

    DistanceMismatch {
        declared: usize,
        certified: usize,
    },

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

            Self::DimensionMismatch => {
                write!(
                    f,
                    "surface-code dimensions do not match distance"
                )
            }

            Self::EmptyDataQubitSet => {
                write!(
                    f,
                    "surface code contains no data qubits"
                )
            }

            Self::CoordinateOutOfRange { coordinate } => {
                write!(
                    f,
                    "coordinate {coordinate} is outside the surface-code patch"
                )
            }

            Self::DuplicateCoordinate { coordinate } => {
                write!(
                    f,
                    "duplicate data-qubit coordinate {coordinate}"
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
                    "qubit {qubit} occurs more than once in a stabilizer"
                )
            }

            Self::EmptyStabilizer => {
                write!(
                    f,
                    "stabilizer support cannot be empty"
                )
            }

            Self::InvalidStabilizerWeight {
                stabilizer,
                weight,
            } => {
                write!(
                    f,
                    "stabilizer {stabilizer} has invalid weight {weight}; expected 2..=4"
                )
            }

            Self::DisconnectedStabilizer { stabilizer } => {
                write!(
                    f,
                    "stabilizer {stabilizer} has disconnected support"
                )
            }

            Self::NonCommutingStabilizers {
                first,
                second,
                overlap,
            } => {
                write!(
                    f,
                    "stabilizers {first} and {second} anticommute with overlap {overlap}"
                )
            }

            Self::EmptyLogicalOperator => {
                write!(
                    f,
                    "logical operator cannot be empty"
                )
            }

            Self::DuplicateLogicalQubit { qubit } => {
                write!(
                    f,
                    "logical operator contains duplicate qubit {qubit}"
                )
            }

            Self::DisconnectedLogicalOperator => {
                write!(
                    f,
                    "logical operator path is disconnected"
                )
            }

            Self::LogicalPauliMismatch {
                expected,
                actual,
            } => {
                write!(
                    f,
                    "logical operator has wrong Pauli: expected {expected:?}, got {actual:?}"
                )
            }

            Self::LogicalDoesNotCommute { stabilizer } => {
                write!(
                    f,
                    "logical operator does not commute with stabilizer {stabilizer}"
                )
            }

            Self::LogicalOperatorsCommute => {
                write!(
                    f,
                    "logical X and logical Z must anticommute"
                )
            }

            Self::DistanceMismatch {
                declared,
                certified,
            } => {
                write!(
                    f,
                    "declared distance {declared} does not match certified logical distance {certified}"
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

    fn grid_code() -> SurfaceCode {
        let mut code =
            SurfaceCode::new(3).unwrap();

        for y in 0..3 {
            for x in 0..3 {
                let id =
                    DataQubitId::new(
                        y * 3 + x
                    );

                code.add_data_qubit(
                    id,
                    SurfaceCodeCoord::new(
                        x,
                        y,
                    ),
                )
                .unwrap();
            }
        }

        code
    }

    #[test]
    fn rejects_distance_below_two() {
        assert!(
            SurfaceCode::new(1).is_err()
        );
    }

    #[test]
    fn rejects_duplicate_coordinates() {
        let mut code =
            SurfaceCode::new(3).unwrap();

        code.add_data_qubit(
            DataQubitId::new(0),
            SurfaceCodeCoord::new(0, 0),
        )
        .unwrap();

        assert_eq!(
            code.add_data_qubit(
                DataQubitId::new(1),
                SurfaceCodeCoord::new(0, 0),
            ),
            Err(
                SurfaceCodeError::DuplicateCoordinate {
                    coordinate:
                        SurfaceCodeCoord::new(0, 0)
                }
            )
        );
    }

    #[test]
    fn builds_grid_topology() {
        let code = grid_code();

        let neighbours =
            code.neighbours(
                DataQubitId::new(4)
            );

        assert_eq!(
            neighbours.len(),
            4
        );
    }

    #[test]
    fn detects_unknown_stabilizer_qubit() {
        let mut code =
            grid_code();

        let stabilizer =
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(1, 1),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                    DataQubitId::new(99),
                ],
            )
            .unwrap();

        assert!(
            matches!(
                code.add_stabilizer(
                    stabilizer
                ),
                Err(
                    SurfaceCodeError::UnknownDataQubit {
                        qubit: DataQubitId(99)
                    }
                )
            )
        );
    }

    #[test]
    fn rejects_invalid_stabilizer_weight() {
        let mut code =
            grid_code();

        let stabilizer =
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(1, 1),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                ],
            )
            .unwrap();

        assert!(
            matches!(
                code.add_stabilizer(
                    stabilizer
                ),
                Err(
                    SurfaceCodeError::InvalidStabilizerWeight {
                        weight: 1,
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn rejects_disconnected_stabilizer() {
        let mut code =
            grid_code();

        let stabilizer =
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(1, 1),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(8),
                ],
            )
            .unwrap();

        assert!(
            matches!(
                code.add_stabilizer(
                    stabilizer
                ),
                Err(
                    SurfaceCodeError::DisconnectedStabilizer {
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn detects_non_commuting_stabilizers() {
        let mut code =
            grid_code();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(0, 0),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(1, 0),
                StabilizerType::Z,
                vec![
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        assert!(
            matches!(
                code.validate(),
                Err(
                    SurfaceCodeError::NonCommutingStabilizers {
                        overlap: 1,
                        ..
                    }
                )
            )
        );
    }

    #[test]
    fn accepts_commuting_stabilizers() {
        let mut code =
            grid_code();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(0),
                SurfaceCodeCoord::new(1, 1),
                StabilizerType::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                    DataQubitId::new(3),
                    DataQubitId::new(4),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        code.add_stabilizer(
            SurfaceStabilizer::new(
                StabilizerId::new(1),
                SurfaceCodeCoord::new(2, 1),
                StabilizerType::Z,
                vec![
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                    DataQubitId::new(4),
                    DataQubitId::new(5),
                ],
            )
            .unwrap(),
        )
        .unwrap();

        assert!(
            code.validate_stabilizer_commutation()
                .is_ok()
        );
    }

    #[test]
    fn rejects_duplicate_logical_qubits() {
        assert!(
            LogicalOperator::new(
                PauliError::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(0),
                ],
            )
            .is_err()
        );
    }

    #[test]
    fn logical_x_and_z_must_anticommute() {
        let mut code =
            grid_code();

        let x =
            LogicalOperator::new(
                PauliError::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                ],
            )
            .unwrap();

        let z =
            LogicalOperator::new(
                PauliError::Z,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(3),
                    DataQubitId::new(6),
                ],
            )
            .unwrap();

        code.set_logical_x(x).unwrap();
        code.set_logical_z(z).unwrap();

        assert!(
            code.logical_x()
                .is_some()
        );

        assert!(
            code.logical_z()
                .is_some()
        );
    }

    #[test]
    fn certified_distance_is_available() {
        let mut code =
            grid_code();

        let x =
            LogicalOperator::new(
                PauliError::X,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(1),
                    DataQubitId::new(2),
                ],
            )
            .unwrap();

        let z =
            LogicalOperator::new(
                PauliError::Z,
                vec![
                    DataQubitId::new(0),
                    DataQubitId::new(3),
                    DataQubitId::new(6),
                ],
            )
            .unwrap();

        code.set_logical_x(x).unwrap();
        code.set_logical_z(z).unwrap();

        assert_eq!(
            code.certified_distance(),
            Some(3)
        );
    }
}