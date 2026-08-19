//! Zamani Quantum — Hardware Topology
//!
//! Production-grade representation of physical quantum hardware connectivity.
//!
//! This module deliberately contains topology information only:
//!
//!     logical qubits
//!          ↓
//!     routing / transpilation
//!          ↓
//!     physical qubits
//!          ↓
//!     hardware backend
//!
//! Hardware-specific calibration data belongs in `calibration.rs`, while
//! backend execution belongs in `backend.rs`.

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// Physical qubit identifier.
pub type QubitId = usize;

/// Directed or undirected connectivity between physical qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connectivity {
    /// Operations are allowed in both directions.
    Bidirectional,

    /// Operations are only natively supported from `source` to `target`.
    Directed,
}

/// A physical coupling between two qubits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coupling {
    pub source: QubitId,
    pub target: QubitId,
    pub connectivity: Connectivity,
}

impl Coupling {
    pub fn bidirectional(a: QubitId, b: QubitId) -> Self {
        Self {
            source: a,
            target: b,
            connectivity: Connectivity::Bidirectional,
        }
    }

    pub fn directed(source: QubitId, target: QubitId) -> Self {
        Self {
            source,
            target,
            connectivity: Connectivity::Directed,
        }
    }

    fn connects(self, a: QubitId, b: QubitId) -> bool {
        match self.connectivity {
            Connectivity::Bidirectional => {
                (self.source == a && self.target == b)
                    || (self.source == b && self.target == a)
            }
            Connectivity::Directed => {
                self.source == a && self.target == b
            }
        }
    }
}

/// Errors produced by topology operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyError {
    ZeroQubits,

    InvalidQubit {
        qubit: QubitId,
        qubit_count: usize,
    },

    DuplicateCoupling {
        source: QubitId,
        target: QubitId,
    },

    SelfCoupling {
        qubit: QubitId,
    },

    MissingCoupling {
        source: QubitId,
        target: QubitId,
    },

    EmptyTopology,

    NoPath {
        source: QubitId,
        target: QubitId,
    },
}

impl fmt::Display for TopologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroQubits => {
                write!(f, "quantum topology must contain at least one qubit")
            }

            Self::InvalidQubit {
                qubit,
                qubit_count,
            } => {
                write!(
                    f,
                    "qubit {} is outside topology containing {} qubits",
                    qubit, qubit_count
                )
            }

            Self::DuplicateCoupling { source, target } => {
                write!(
                    f,
                    "coupling between qubits {} and {} already exists",
                    source, target
                )
            }

            Self::SelfCoupling { qubit } => {
                write!(
                    f,
                    "qubit {} cannot be coupled to itself",
                    qubit
                )
            }

            Self::MissingCoupling { source, target } => {
                write!(
                    f,
                    "no coupling exists from qubit {} to {}",
                    source, target
                )
            }

            Self::EmptyTopology => {
                write!(f, "topology contains no qubits")
            }

            Self::NoPath { source, target } => {
                write!(
                    f,
                    "no connectivity path exists from qubit {} to {}",
                    source, target
                )
            }
        }
    }
}

impl std::error::Error for TopologyError {}

/// Quantum hardware connectivity graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareTopology {
    qubit_count: usize,
    couplings: Vec<Coupling>,
    adjacency: HashMap<QubitId, Vec<QubitId>>,
}

impl HardwareTopology {
    /// Creates a topology with no couplings.
    pub fn new(qubit_count: usize) -> Result<Self, TopologyError> {
        if qubit_count == 0 {
            return Err(TopologyError::ZeroQubits);
        }

        let mut adjacency = HashMap::with_capacity(qubit_count);

        for qubit in 0..qubit_count {
            adjacency.insert(qubit, Vec::new());
        }

        Ok(Self {
            qubit_count,
            couplings: Vec::new(),
            adjacency,
        })
    }

    /// Creates a fully connected topology.
    pub fn fully_connected(
        qubit_count: usize,
    ) -> Result<Self, TopologyError> {
        let mut topology = Self::new(qubit_count)?;

        for source in 0..qubit_count {
            for target in (source + 1)..qubit_count {
                topology.add_bidirectional_coupling(source, target)?;
            }
        }

        Ok(topology)
    }

    /// Creates a linear nearest-neighbour topology.
    pub fn linear(qubit_count: usize) -> Result<Self, TopologyError> {
        let mut topology = Self::new(qubit_count)?;

        for qubit in 0..qubit_count.saturating_sub(1) {
            topology.add_bidirectional_coupling(qubit, qubit + 1)?;
        }

        Ok(topology)
    }

    /// Creates a ring topology.
    pub fn ring(qubit_count: usize) -> Result<Self, TopologyError> {
        let mut topology = Self::linear(qubit_count)?;

        if qubit_count > 2 {
            topology.add_bidirectional_coupling(
                qubit_count - 1,
                0,
            )?;
        }

        Ok(topology)
    }

    /// Number of physical qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubit_count
    }

    /// Number of coupling edges.
    pub fn coupling_count(&self) -> usize {
        self.couplings.len()
    }

    /// Returns all registered couplings.
    pub fn couplings(&self) -> &[Coupling] {
        &self.couplings
    }

    /// Returns neighbouring qubits reachable from a qubit.
    pub fn neighbours(
        &self,
        qubit: QubitId,
    ) -> Result<&[QubitId], TopologyError> {
        self.validate_qubit(qubit)?;

        Ok(self
            .adjacency
            .get(&qubit)
            .map(Vec::as_slice)
            .unwrap_or(&[]))
    }

    /// Adds bidirectional connectivity.
    pub fn add_bidirectional_coupling(
        &mut self,
        a: QubitId,
        b: QubitId,
    ) -> Result<(), TopologyError> {
        self.validate_qubit(a)?;
        self.validate_qubit(b)?;

        if a == b {
            return Err(TopologyError::SelfCoupling { qubit: a });
        }

        if self.has_connection(a, b) || self.has_connection(b, a) {
            return Err(TopologyError::DuplicateCoupling {
                source: a,
                target: b,
            });
        }

        self.couplings
            .push(Coupling::bidirectional(a, b));

        self.adjacency
            .get_mut(&a)
            .expect("validated qubit must exist")
            .push(b);

        self.adjacency
            .get_mut(&b)
            .expect("validated qubit must exist")
            .push(a);

        Ok(())
    }

    /// Adds directed connectivity.
    pub fn add_directed_coupling(
        &mut self,
        source: QubitId,
        target: QubitId,
    ) -> Result<(), TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        if source == target {
            return Err(TopologyError::SelfCoupling {
                qubit: source,
            });
        }

        if self.has_connection(source, target) {
            return Err(TopologyError::DuplicateCoupling {
                source,
                target,
            });
        }

        self.couplings
            .push(Coupling::directed(source, target));

        self.adjacency
            .get_mut(&source)
            .expect("validated qubit must exist")
            .push(target);

        Ok(())
    }

    /// Returns whether a native operation can move from `source` to `target`.
    pub fn is_connected(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<bool, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        Ok(self.has_connection(source, target))
    }

    /// Finds the shortest physical routing path between two qubits.
    ///
    /// Breadth-first search is used because every coupling currently has
    /// equal routing cost. Weighted routing can be layered on top of this
    /// topology using calibration data.
    pub fn shortest_path(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<Vec<QubitId>, TopologyError> {
        self.validate_qubit(source)?;
        self.validate_qubit(target)?;

        if source == target {
            return Ok(vec![source]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut predecessor: HashMap<QubitId, QubitId> =
            HashMap::new();

        queue.push_back(source);
        visited.insert(source);

        while let Some(current) = queue.pop_front() {
            let neighbours = self
                .adjacency
                .get(&current)
                .map(Vec::as_slice)
                .unwrap_or(&[]);

            for &next in neighbours {
                if !visited.insert(next) {
                    continue;
                }

                predecessor.insert(next, current);

                if next == target {
                    let mut path = vec![target];
                    let mut node = target;

                    while let Some(&previous) =
                        predecessor.get(&node)
                    {
                        path.push(previous);

                        if previous == source {
                            break;
                        }

                        node = previous;
                    }

                    path.reverse();
                    return Ok(path);
                }

                queue.push_back(next);
            }
        }

        Err(TopologyError::NoPath { source, target })
    }

    /// Returns the shortest-path distance.
    pub fn distance(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> Result<usize, TopologyError> {
        Ok(self.shortest_path(source, target)?.len() - 1)
    }

    /// Returns whether the topology is fully connected.
    pub fn is_fully_connected(&self) -> bool {
        if self.qubit_count == 0 {
            return false;
        }

        for source in 0..self.qubit_count {
            for target in 0..self.qubit_count {
                if source == target {
                    continue;
                }

                if self.shortest_path(source, target).is_err() {
                    return false;
                }
            }
        }

        true
    }

    /// Returns the maximum degree of any qubit.
    pub fn maximum_degree(&self) -> usize {
        self.adjacency
            .values()
            .map(Vec::len)
            .max()
            .unwrap_or(0)
    }

    /// Returns the average degree.
    pub fn average_degree(&self) -> f64 {
        if self.qubit_count == 0 {
            return 0.0;
        }

        let total: usize =
            self.adjacency.values().map(Vec::len).sum();

        total as f64 / self.qubit_count as f64
    }

    fn validate_qubit(
        &self,
        qubit: QubitId,
    ) -> Result<(), TopologyError> {
        if qubit >= self.qubit_count {
            return Err(TopologyError::InvalidQubit {
                qubit,
                qubit_count: self.qubit_count,
            });
        }

        Ok(())
    }

    fn has_connection(
        &self,
        source: QubitId,
        target: QubitId,
    ) -> bool {
        self.couplings
            .iter()
            .any(|coupling| coupling.connects(source, target))
    }
}

impl Default for HardwareTopology {
    fn default() -> Self {
        Self::new(1).expect("one-qubit topology is always valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_qubits() {
        assert_eq!(
            HardwareTopology::new(0),
            Err(TopologyError::ZeroQubits)
        );
    }

    #[test]
    fn creates_linear_topology() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.qubit_count(), 5);
        assert_eq!(topology.coupling_count(), 4);
    }

    #[test]
    fn bidirectional_connection_works_both_ways() {
        let mut topology =
            HardwareTopology::new(2).unwrap();

        topology
            .add_bidirectional_coupling(0, 1)
            .unwrap();

        assert!(
            topology.is_connected(0, 1).unwrap()
        );

        assert!(
            topology.is_connected(1, 0).unwrap()
        );
    }

    #[test]
    fn directed_connection_only_works_forward() {
        let mut topology =
            HardwareTopology::new(2).unwrap();

        topology
            .add_directed_coupling(0, 1)
            .unwrap();

        assert!(
            topology.is_connected(0, 1).unwrap()
        );

        assert!(
            !topology.is_connected(1, 0).unwrap()
        );
    }

    #[test]
    fn shortest_path_is_correct() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(
            topology.shortest_path(0, 4).unwrap(),
            vec![0, 1, 2, 3, 4]
        );

        assert_eq!(
            topology.distance(0, 4).unwrap(),
            4
        );
    }

    #[test]
    fn disconnected_qubits_report_no_path() {
        let topology =
            HardwareTopology::new(3).unwrap();

        assert_eq!(
            topology.shortest_path(0, 2),
            Err(TopologyError::NoPath {
                source: 0,
                target: 2
            })
        );
    }

    #[test]
    fn duplicate_coupling_is_rejected() {
        let mut topology =
            HardwareTopology::new(2).unwrap();

        topology
            .add_bidirectional_coupling(0, 1)
            .unwrap();

        assert!(
            topology
                .add_bidirectional_coupling(0, 1)
                .is_err()
        );
    }

    #[test]
    fn self_coupling_is_rejected() {
        let mut topology =
            HardwareTopology::new(2).unwrap();

        assert_eq!(
            topology.add_bidirectional_coupling(0, 0),
            Err(TopologyError::SelfCoupling { qubit: 0 })
        );
    }

    #[test]
    fn fully_connected_topology_is_detected() {
        let topology =
            HardwareTopology::fully_connected(4).unwrap();

        assert!(topology.is_fully_connected());
    }

    #[test]
    fn ring_topology_wraps_around() {
        let topology =
            HardwareTopology::ring(5).unwrap();

        assert_eq!(
            topology.distance(0, 4).unwrap(),
            1
        );
    }

    #[test]
    fn statistics_are_calculated() {
        let topology =
            HardwareTopology::linear(5).unwrap();

        assert_eq!(topology.maximum_degree(), 2);
        assert_eq!(topology.average_degree(), 1.6);
    }
}