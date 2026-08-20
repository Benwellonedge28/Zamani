//! Zamani Quantum Error Correction — Sparse Representations.
//!
//! This module contains sparse data structures used by the QEC execution
//! pipeline.
//!
//! # Responsibility
//!
//! ```text
//!                 QEC LIMITS
//!                     |
//!                     v
//!               SPARSE PREFLIGHT
//!                     |
//!          +----------+----------+
//!          |          |          |
//!          v          v          v
//!    SparsePauli  SparseGraph  SparseSyndrome
//!          |          |          |
//!          v          v          v
//! SparseStabilizerMatrix  SparseCorrection
//! ```
//!
//! This module is deliberately NOT a parser.
//!
//! The previous implementation of this file parsed textual QEC documents.
//! That responsibility does not belong in the sparse representation layer.
//!
//! # Design goals
//!
//! - deterministic ordering;
//! - sparse storage;
//! - checked arithmetic;
//! - no unchecked indexing;
//! - no implicit allocation bombs;
//! - canonical `QecLimits` integration;
//! - cheap structural validation;
//! - explicit memory estimation;
//! - duplicate suppression;
//! - deterministic serialization primitives;
//! - scalable support for large QEC workloads;
//! - no decoder-specific policy;
//! - no QPU access;
//! - no network access;
//! - no hidden global resource limits.
//!
//! # Representation model
//!
//! A sparse QEC workload is represented as:
//!
//! ```text
//! Pauli operator
//!     |
//!     +--> X support: {q1, q7, q100000}
//!     |
//!     +--> Z support: {q3, q7}
//!
//! Stabilizer matrix
//!     |
//!     +--> stabilizer 0 -> sparse Pauli
//!     +--> stabilizer 1 -> sparse Pauli
//!     +--> ...
//!
//! Graph
//!     |
//!     +--> node -> sparse neighbor set
//!
//! Syndrome
//!     |
//!     +--> round/stabilizer -> detection event
//!
//! Correction
//!     |
//!     +--> qubit -> Pauli operation
//! ```
//!
//! The representation is intentionally independent of a particular decoder.
//! MWPM, Union-Find, streaming decoders, partitioned decoders and distributed
//! decoders can consume these structures without requiring a dense expansion.
//!
//! # Resource architecture
//!
//! `QecLimits` is the declarative policy.
//!
//! ```text
//! QecLimits
//!     |
//!     v
//! preflight()
//!     |
//!     v
//! sparse allocation
//!     |
//!     v
//! ResourceManager
//! ```
//!
//! This module therefore performs *preflight* but does not attempt to replace
//! runtime accounting in `resources.rs`.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::limits::{LimitError, LimitKind, QecLimits};

// ============================================================================
// Constants
// ============================================================================

/// Number of bytes used by the sparse representation estimate for one
/// qubit-index entry.
///
/// This is an intentionally conservative accounting estimate rather than a
/// promise about the exact allocator representation.
pub const ESTIMATED_INDEX_BYTES: u64 = 16;

/// Number of bytes used by a sparse graph edge estimate.
pub const ESTIMATED_EDGE_BYTES: u64 = 24;

/// Number of bytes used by a sparse syndrome event estimate.
pub const ESTIMATED_SYNDROME_EVENT_BYTES: u64 = 24;

/// Number of bytes used by a sparse correction entry estimate.
pub const ESTIMATED_CORRECTION_BYTES: u64 = 24;

// ============================================================================
// Sparse errors
// ============================================================================

/// Errors produced by sparse QEC representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SparseError {
    /// A sparse representation was constructed with an invalid qubit count.
    InvalidQubitCount {
        qubits: usize,
    },

    /// A qubit index is outside the declared representation.
    QubitOutOfRange {
        qubit: usize,
        num_qubits: usize,
    },

    /// Two representations have incompatible dimensions.
    DimensionMismatch {
        left: usize,
        right: usize,
    },

    /// A requested sparse structure exceeds the canonical QEC policy.
    ResourceLimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    /// A derived-size calculation overflowed.
    ArithmeticOverflow {
        resource: LimitKind,
    },

    /// A graph node was already present.
    DuplicateNode {
        node: usize,
    },

    /// A graph edge was already present.
    DuplicateEdge {
        from: usize,
        to: usize,
    },

    /// A sparse structure contains an invalid internal state.
    InvalidInvariant {
        message: &'static str,
    },
}

impl fmt::Display for SparseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { qubits } => {
                write!(f, "invalid sparse qubit count: {qubits}")
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    f,
                    "qubit {qubit} is outside sparse representation \
                     with {num_qubits} qubits"
                )
            }

            Self::DimensionMismatch { left, right } => {
                write!(
                    f,
                    "sparse representation dimension mismatch: \
                     {left} != {right}"
                )
            }

            Self::ResourceLimitExceeded {
                resource,
                requested,
                maximum,
            } => {
                write!(
                    f,
                    "sparse {resource} limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ArithmeticOverflow { resource } => {
                write!(
                    f,
                    "arithmetic overflow while estimating sparse {resource}"
                )
            }

            Self::DuplicateNode { node } => {
                write!(f, "duplicate sparse graph node: {node}")
            }

            Self::DuplicateEdge { from, to } => {
                write!(
                    f,
                    "duplicate sparse graph edge: {from} -> {to}"
                )
            }

            Self::InvalidInvariant { message } => {
                write!(f, "invalid sparse representation invariant: {message}")
            }
        }
    }
}

impl std::error::Error for SparseError {}

/// Result type for sparse QEC operations.
pub type SparseResult<T> = Result<T, SparseError>;

// ============================================================================
// Resource helpers
// ============================================================================

fn checked_add(
    left: u64,
    right: u64,
    resource: LimitKind,
) -> SparseResult<u64> {
    left.checked_add(right)
        .ok_or(SparseError::ArithmeticOverflow { resource })
}

fn checked_mul(
    left: u64,
    right: u64,
    resource: LimitKind,
) -> SparseResult<u64> {
    left.checked_mul(right)
        .ok_or(SparseError::ArithmeticOverflow { resource })
}

fn usize_to_u64(
    value: usize,
    resource: LimitKind,
) -> SparseResult<u64> {
    u64::try_from(value).map_err(|_| {
        SparseError::ArithmeticOverflow { resource }
    })
}

fn limit_error(error: LimitError) -> SparseError {
    match error {
        LimitError::Exceeded {
            resource,
            requested,
            maximum,
        } => SparseError::ResourceLimitExceeded {
            resource,
            requested,
            maximum,
        },

        LimitError::ArithmeticOverflow { resource } => {
            SparseError::ArithmeticOverflow { resource }
        }

        LimitError::InvalidLimit { resource, value } => {
            SparseError::ResourceLimitExceeded {
                resource,
                requested: value,
                maximum: value,
            }
        }

        LimitError::InconsistentLimits {
            resource,
            ..
        } => SparseError::InvalidInvariant {
            message: match resource {
                LimitKind::QubitsPerPartition => {
                    "inconsistent QEC partition limits"
                }

                _ => "inconsistent QEC limits",
            },
        },
    }
}

// ============================================================================
// Sparse Pauli
// ============================================================================

/// Sparse binary-symplectic Pauli representation.
///
/// Instead of storing two dense `Vec<bool>` arrays, only the non-zero X and Z
/// supports are stored.
///
/// ```text
/// X support = {1, 4, 9000}
/// Z support = {4, 7}
/// ```
///
/// Qubit 4 therefore represents `Y`, because both X and Z components are set.
///
/// Global phase is intentionally ignored, matching the stabilizer layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparsePauli {
    num_qubits: usize,
    x_support: BTreeSet<usize>,
    z_support: BTreeSet<usize>,
}

impl SparsePauli {
    /// Creates the identity operator on `num_qubits`.
    pub fn identity(num_qubits: usize) -> SparseResult<Self> {
        if num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: num_qubits,
            });
        }

        Ok(Self {
            num_qubits,
            x_support: BTreeSet::new(),
            z_support: BTreeSet::new(),
        })
    }

    /// Creates a sparse Pauli from explicit X and Z supports.
    pub fn from_supports<I, J>(
        num_qubits: usize,
        x_support: I,
        z_support: J,
    ) -> SparseResult<Self>
    where
        I: IntoIterator<Item = usize>,
        J: IntoIterator<Item = usize>,
    {
        let mut result = Self::identity(num_qubits)?;

        for qubit in x_support {
            result.insert_x(qubit)?;
        }

        for qubit in z_support {
            result.insert_z(qubit)?;
        }

        Ok(result)
    }

    /// Returns the number of qubits represented.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the X support.
    #[must_use]
    pub fn x_support(&self) -> &BTreeSet<usize> {
        &self.x_support
    }

    /// Returns the Z support.
    #[must_use]
    pub fn z_support(&self) -> &BTreeSet<usize> {
        &self.z_support
    }

    /// Inserts an X component.
    pub fn insert_x(&mut self, qubit: usize) -> SparseResult<bool> {
        self.validate_qubit(qubit)?;
        Ok(self.x_support.insert(qubit))
    }

    /// Inserts a Z component.
    pub fn insert_z(&mut self, qubit: usize) -> SparseResult<bool> {
        self.validate_qubit(qubit)?;
        Ok(self.z_support.insert(qubit))
    }

    /// Removes an X component.
    pub fn remove_x(&mut self, qubit: usize) -> SparseResult<bool> {
        self.validate_qubit(qubit)?;
        Ok(self.x_support.remove(&qubit))
    }

    /// Removes a Z component.
    pub fn remove_z(&mut self, qubit: usize) -> SparseResult<bool> {
        self.validate_qubit(qubit)?;
        Ok(self.z_support.remove(&qubit))
    }

    /// Returns whether an X component exists at `qubit`.
    #[must_use]
    pub fn has_x(&self, qubit: usize) -> bool {
        self.x_support.contains(&qubit)
    }

    /// Returns whether a Z component exists at `qubit`.
    #[must_use]
    pub fn has_z(&self, qubit: usize) -> bool {
        self.z_support.contains(&qubit)
    }

    /// Returns whether the operator is identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.x_support.is_empty() && self.z_support.is_empty()
    }

    /// Returns the number of qubits on which the Pauli is non-identity.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.x_support
            .union(&self.z_support)
            .count()
    }

    /// Returns the number of stored binary components.
    #[must_use]
    pub fn support_size(&self) -> usize {
        self.x_support.len() + self.z_support.len()
    }

    /// Returns the single-qubit Pauli encoded at `qubit`.
    ///
    /// The result uses:
    ///
    /// ```text
    /// 00 = I
    /// 10 = X
    /// 11 = Y
    /// 01 = Z
    /// ```
    #[must_use]
    pub fn component(&self, qubit: usize) -> u8 {
        let x = self.x_support.contains(&qubit);
        let z = self.z_support.contains(&qubit);

        match (x, z) {
            (false, false) => 0,
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
        }
    }

    /// Computes the binary symplectic product.
    ///
    /// Returns `0` for commuting and `1` for anti-commuting operators.
    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> SparseResult<u8> {
        if self.num_qubits != other.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: self.num_qubits,
                right: other.num_qubits,
            });
        }

        let xz = self
            .x_support
            .intersection(&other.z_support)
            .count();

        let zx = self
            .z_support
            .intersection(&other.x_support)
            .count();

        Ok(((xz + zx) & 1) as u8)
    }

    /// Returns whether two sparse Paulis commute.
    pub fn commutes_with(
        &self,
        other: &Self,
    ) -> SparseResult<bool> {
        Ok(self.symplectic_product(other)? == 0)
    }

    /// Multiplies this Pauli by another Pauli modulo global phase.
    ///
    /// Binary symplectic multiplication is XOR of X and Z supports.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> SparseResult<Self> {
        if self.num_qubits != other.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: self.num_qubits,
                right: other.num_qubits,
            });
        }

        let x_support = self
            .x_support
            .symmetric_difference(&other.x_support)
            .copied()
            .collect();

        let z_support = self
            .z_support
            .symmetric_difference(&other.z_support)
            .copied()
            .collect();

        Ok(Self {
            num_qubits: self.num_qubits,
            x_support,
            z_support,
        })
    }

    /// Returns a deterministic iterator over non-identity qubits.
    pub fn support(&self) -> impl Iterator<Item = usize> + '_ {
        self.x_support.union(&self.z_support).copied()
    }

    /// Estimates memory required by this sparse representation.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let entries = usize_to_u64(
            self.support_size(),
            LimitKind::MemoryBytes,
        )?;

        checked_mul(
            entries,
            ESTIMATED_INDEX_BYTES,
            LimitKind::MemoryBytes,
        )
    }

    /// Performs structural validation.
    pub fn validate(&self) -> SparseResult<()> {
        if self.num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: self.num_qubits,
            });
        }

        if self
            .x_support
            .iter()
            .any(|qubit| *qubit >= self.num_qubits)
        {
            return Err(SparseError::InvalidInvariant {
                message: "X support contains an out-of-range qubit",
            });
        }

        if self
            .z_support
            .iter()
            .any(|qubit| *qubit >= self.num_qubits)
        {
            return Err(SparseError::InvalidInvariant {
                message: "Z support contains an out-of-range qubit",
            });
        }

        Ok(())
    }

    fn validate_qubit(&self, qubit: usize) -> SparseResult<()> {
        if qubit >= self.num_qubits {
            return Err(SparseError::QubitOutOfRange {
                qubit,
                num_qubits: self.num_qubits,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Sparse stabilizer matrix
// ============================================================================

/// Sparse stabilizer-generator matrix.
///
/// Each row is represented by a `SparsePauli`.
///
/// No dense `n × m` matrix is materialized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseStabilizerMatrix {
    num_qubits: usize,
    rows: Vec<SparsePauli>,
}

impl SparseStabilizerMatrix {
    /// Creates an empty sparse stabilizer matrix.
    pub fn new(
        num_qubits: usize,
        limits: &QecLimits,
    ) -> SparseResult<Self> {
        if num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: num_qubits,
            });
        }

        limits.validate().map_err(limit_error)?;

        if num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        Ok(Self {
            num_qubits,
            rows: Vec::new(),
        })
    }

    /// Returns the represented qubit count.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the stabilizer-generator count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Returns whether there are no generators.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns all rows in deterministic order.
    #[must_use]
    pub fn rows(&self) -> &[SparsePauli] {
        &self.rows
    }

    /// Appends one stabilizer generator.
    pub fn push(
        &mut self,
        generator: SparsePauli,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if generator.num_qubits() != self.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: self.num_qubits,
                right: generator.num_qubits(),
            });
        }

        if self.rows.len() >= limits.max_stabilizers {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: (self.rows.len() as u128) + 1,
                maximum: limits.max_stabilizers as u128,
            });
        }

        if generator.weight() > limits.max_stabilizer_weight {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::StabilizerWeight,
                requested: generator.weight() as u128,
                maximum: limits.max_stabilizer_weight as u128,
            });
        }

        generator.validate()?;

        self.rows.push(generator);

        Ok(())
    }

    /// Returns one generator by index without unchecked indexing.
    pub fn get(&self, index: usize) -> Option<&SparsePauli> {
        self.rows.get(index)
    }

    /// Computes the total sparse support size.
    #[must_use]
    pub fn total_support_size(&self) -> usize {
        self.rows
            .iter()
            .map(SparsePauli::support_size)
            .sum()
    }

    /// Estimates memory required by the matrix.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let mut total = 0u64;

        for row in &self.rows {
            total = checked_add(
                total,
                row.estimated_memory_bytes()?,
                LimitKind::MemoryBytes,
            )?;
        }

        Ok(total)
    }

    /// Performs structural validation.
    pub fn validate(&self, limits: &QecLimits) -> SparseResult<()> {
        limits.validate().map_err(limit_error)?;

        if self.num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.rows.len() > limits.max_stabilizers {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: self.rows.len() as u128,
                maximum: limits.max_stabilizers as u128,
            });
        }

        for row in &self.rows {
            if row.weight() > limits.max_stabilizer_weight {
                return Err(SparseError::ResourceLimitExceeded {
                    resource: LimitKind::StabilizerWeight,
                    requested: row.weight() as u128,
                    maximum: limits.max_stabilizer_weight as u128,
                });
            }

            row.validate()?;
        }

        Ok(())
    }

    /// Returns the maximum generator weight.
    #[must_use]
    pub fn max_weight(&self) -> usize {
        self.rows
            .iter()
            .map(SparsePauli::weight)
            .max()
            .unwrap_or(0)
    }
}

// ============================================================================
// Sparse adjacency
// ============================================================================

/// Sparse deterministic adjacency structure.
///
/// Each node owns an ordered `BTreeSet` of neighboring nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseAdjacency {
    nodes: BTreeMap<usize, BTreeSet<usize>>,
}

impl SparseAdjacency {
    /// Creates an empty adjacency structure.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
        }
    }

    /// Returns the number of nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of directed adjacency entries.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.nodes
            .values()
            .map(BTreeSet::len)
            .sum()
    }

    /// Inserts a node.
    pub fn insert_node(
        &mut self,
        node: usize,
        limits: &QecLimits,
    ) -> SparseResult<bool> {
        if self.nodes.contains_key(&node) {
            return Err(SparseError::DuplicateNode { node });
        }

        if self.nodes.len() >= limits.max_graph_nodes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                requested: (self.nodes.len() as u128) + 1,
                maximum: limits.max_graph_nodes as u128,
            });
        }

        self.nodes.insert(node, BTreeSet::new());

        Ok(true)
    }

    /// Inserts a directed edge.
    ///
    /// Both endpoints must already exist.
    pub fn insert_edge(
        &mut self,
        from: usize,
        to: usize,
        limits: &QecLimits,
    ) -> SparseResult<bool> {
        if !self.nodes.contains_key(&from) {
            self.insert_node(from, limits)?;
        }

        if !self.nodes.contains_key(&to) {
            self.insert_node(to, limits)?;
        }

        let exists = self
            .nodes
            .get(&from)
            .map(|neighbors| neighbors.contains(&to))
            .unwrap_or(false);

        if exists {
            return Err(SparseError::DuplicateEdge { from, to });
        }

        if self.edge_count() >= limits.max_graph_edges {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: (self.edge_count() as u128) + 1,
                maximum: limits.max_graph_edges as u128,
            });
        }

        let neighbors = self
            .nodes
            .get_mut(&from)
            .ok_or(SparseError::InvalidInvariant {
                message: "graph node disappeared during insertion",
            })?;

        neighbors.insert(to);

        Ok(true)
    }

    /// Inserts an undirected edge.
    pub fn insert_undirected_edge(
        &mut self,
        left: usize,
        right: usize,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if left == right {
            return Err(SparseError::InvalidInvariant {
                message: "self-loop is not a valid QEC adjacency edge",
            });
        }

        self.insert_edge(left, right, limits)?;
        self.insert_edge(right, left, limits)?;

        Ok(())
    }

    /// Returns neighbors in deterministic order.
    #[must_use]
    pub fn neighbors(
        &self,
        node: usize,
    ) -> Option<&BTreeSet<usize>> {
        self.nodes.get(&node)
    }

    /// Returns nodes in deterministic order.
    pub fn nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.nodes.keys().copied()
    }

    /// Estimates adjacency memory.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let nodes = usize_to_u64(
            self.node_count(),
            LimitKind::MemoryBytes,
        )?;

        let edges = usize_to_u64(
            self.edge_count(),
            LimitKind::MemoryBytes,
        )?;

        let node_bytes = checked_mul(
            nodes,
            ESTIMATED_INDEX_BYTES,
            LimitKind::MemoryBytes,
        )?;

        let edge_bytes = checked_mul(
            edges,
            ESTIMATED_EDGE_BYTES,
            LimitKind::MemoryBytes,
        )?;

        checked_add(
            node_bytes,
            edge_bytes,
            LimitKind::MemoryBytes,
        )
    }

    /// Validates the sparse adjacency structure against policy.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(limit_error)?;

        if self.node_count() > limits.max_graph_nodes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                requested: self.node_count() as u128,
                maximum: limits.max_graph_nodes as u128,
            });
        }

        if self.edge_count() > limits.max_graph_edges {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: self.edge_count() as u128,
                maximum: limits.max_graph_edges as u128,
            });
        }

        for (node, neighbors) in &self.nodes {
            if neighbors.contains(node) {
                return Err(SparseError::InvalidInvariant {
                    message: "sparse graph contains a self-loop",
                });
            }

            for neighbor in neighbors {
                if !self.nodes.contains_key(neighbor) {
                    return Err(SparseError::InvalidInvariant {
                        message: "sparse graph contains dangling edge",
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for SparseAdjacency {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Weighted sparse graph
// ============================================================================

/// Sparse weighted graph edge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SparseWeightedEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

impl SparseWeightedEdge {
    /// Creates a validated sparse weighted edge.
    pub fn new(
        from: usize,
        to: usize,
        weight: f64,
    ) -> SparseResult<Self> {
        if from == to {
            return Err(SparseError::InvalidInvariant {
                message: "QEC sparse weighted graph cannot contain self-loops",
            });
        }

        if !weight.is_finite() || weight < 0.0 {
            return Err(SparseError::InvalidInvariant {
                message: "QEC sparse edge weights must be finite and non-negative",
            });
        }

        Ok(Self {
            from,
            to,
            weight,
        })
    }
}

/// Sparse weighted graph.
///
/// This structure stores only edges that actually exist. It is intended for
/// decoding graphs where the full dense adjacency matrix would be wasteful.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseGraph {
    adjacency: SparseAdjacency,
    edges: BTreeMap<(usize, usize), f64>,
}

impl SparseGraph {
    /// Creates an empty sparse graph.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            adjacency: SparseAdjacency::new(),
            edges: BTreeMap::new(),
        }
    }

    /// Returns the number of graph nodes.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.adjacency.node_count()
    }

    /// Returns the number of directed weighted edges.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Inserts a graph node.
    pub fn insert_node(
        &mut self,
        node: usize,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        self.adjacency.insert_node(node, limits)?;
        Ok(())
    }

    /// Inserts a directed weighted edge.
    pub fn insert_edge(
        &mut self,
        edge: SparseWeightedEdge,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if self.edges.contains_key(&(edge.from, edge.to)) {
            return Err(SparseError::DuplicateEdge {
                from: edge.from,
                to: edge.to,
            });
        }

        self.adjacency
            .insert_edge(edge.from, edge.to, limits)?;

        self.edges.insert(
            (edge.from, edge.to),
            edge.weight,
        );

        Ok(())
    }

    /// Inserts an undirected weighted edge.
    pub fn insert_undirected_edge(
        &mut self,
        from: usize,
        to: usize,
        weight: f64,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        let first = SparseWeightedEdge::new(
            from,
            to,
            weight,
        )?;

        let second = SparseWeightedEdge::new(
            to,
            from,
            weight,
        )?;

        self.insert_edge(first, limits)?;
        self.insert_edge(second, limits)?;

        Ok(())
    }

    /// Returns a deterministic edge iterator.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = SparseWeightedEdge> + '_ {
        self.edges.iter().map(|(&(from, to), &weight)| {
            SparseWeightedEdge {
                from,
                to,
                weight,
            }
        })
    }

    /// Returns the weight of a specific edge.
    #[must_use]
    pub fn weight(
        &self,
        from: usize,
        to: usize,
    ) -> Option<f64> {
        self.edges.get(&(from, to)).copied()
    }

    /// Returns neighboring nodes.
    #[must_use]
    pub fn neighbors(
        &self,
        node: usize,
    ) -> Option<&BTreeSet<usize>> {
        self.adjacency.neighbors(node)
    }

    /// Validates the complete graph.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        self.adjacency.validate(limits)?;

        if self.edge_count() > limits.max_graph_edges {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: self.edge_count() as u128,
                maximum: limits.max_graph_edges as u128,
            });
        }

        for (&(from, to), weight) in &self.edges {
            if from == to {
                return Err(SparseError::InvalidInvariant {
                    message: "sparse weighted graph contains a self-loop",
                });
            }

            if !weight.is_finite() || *weight < 0.0 {
                return Err(SparseError::InvalidInvariant {
                    message: "sparse weighted graph contains invalid weight",
                });
            }

            if self
                .adjacency
                .neighbors(from)
                .map(|neighbors| neighbors.contains(&to))
                != Some(true)
            {
                return Err(SparseError::InvalidInvariant {
                    message: "weighted edge is missing from adjacency index",
                });
            }
        }

        Ok(())
    }

    /// Estimates memory used by the sparse graph.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        self.adjacency.estimated_memory_bytes()
    }
}

impl Default for SparseGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Sparse syndrome
// ============================================================================

/// Sparse detection-event identifier.
///
/// A syndrome event is identified by measurement round and stabilizer ID.
/// This avoids storing a dense `round × stabilizer` matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseSyndromeKey {
    pub round: u64,
    pub stabilizer: u64,
}

/// Sparse syndrome event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparseSyndromeEvent {
    pub key: SparseSyndromeKey,
    pub value: bool,
}

impl SparseSyndromeEvent {
    #[must_use]
    pub const fn new(
        round: u64,
        stabilizer: u64,
        value: bool,
    ) -> Self {
        Self {
            key: SparseSyndromeKey {
                round,
                stabilizer,
            },
            value,
        }
    }
}

/// Sparse syndrome representation.
///
/// Only non-trivial events are retained.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseSyndrome {
    events: BTreeSet<SparseSyndromeKey>,
    rounds: u64,
}

impl SparseSyndrome {
    /// Creates an empty sparse syndrome.
    pub fn new(
        rounds: u64,
        limits: &QecLimits,
    ) -> SparseResult<Self> {
        if rounds == 0 {
            return Err(SparseError::InvalidInvariant {
                message: "syndrome must contain at least one round",
            });
        }

        if rounds > limits.max_rounds as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MeasurementRounds,
                requested: rounds as u128,
                maximum: limits.max_rounds as u128,
            });
        }

        Ok(Self {
            events: BTreeSet::new(),
            rounds,
        })
    }

    /// Returns the number of rounds.
    #[must_use]
    pub const fn rounds(&self) -> u64 {
        self.rounds
    }

    /// Returns the number of detection events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns whether the syndrome is trivial.
    #[must_use]
    pub fn is_trivial(&self) -> bool {
        self.events.is_empty()
    }

    /// Inserts a non-trivial event.
    ///
    /// Re-inserting the same event is idempotent.
    pub fn insert(
        &mut self,
        event: SparseSyndromeEvent,
        limits: &QecLimits,
    ) -> SparseResult<bool> {
        if event.key.round >= self.rounds {
            return Err(SparseError::InvalidInvariant {
                message: "syndrome event round exceeds syndrome rounds",
            });
        }

        if !event.value {
            return Ok(false);
        }

        if self.events.contains(&event.key) {
            return Ok(false);
        }

        if self.events.len() >= limits.max_syndrome_events {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: (self.events.len() as u128) + 1,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        Ok(self.events.insert(event.key))
    }

    /// Removes a detection event.
    pub fn remove(
        &mut self,
        key: SparseSyndromeKey,
    ) -> bool {
        self.events.remove(&key)
    }

    /// Checks whether an event exists.
    #[must_use]
    pub fn contains(
        &self,
        key: SparseSyndromeKey,
    ) -> bool {
        self.events.contains(&key)
    }

    /// Returns deterministic event iteration.
    pub fn events(
        &self,
    ) -> impl Iterator<Item = SparseSyndromeEvent> + '_ {
        self.events.iter().copied().map(|key| {
            SparseSyndromeEvent {
                key,
                value: true,
            }
        })
    }

    /// Estimates memory required by the syndrome.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let count = usize_to_u64(
            self.event_count(),
            LimitKind::MemoryBytes,
        )?;

        checked_mul(
            count,
            ESTIMATED_SYNDROME_EVENT_BYTES,
            LimitKind::MemoryBytes,
        )
    }

    /// Validates the sparse syndrome against the canonical limits.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(limit_error)?;

        if self.rounds == 0 {
            return Err(SparseError::InvalidInvariant {
                message: "syndrome has zero rounds",
            });
        }

        if self.rounds > limits.max_rounds as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MeasurementRounds,
                requested: self.rounds as u128,
                maximum: limits.max_rounds as u128,
            });
        }

        if self.event_count() > limits.max_syndrome_events {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: self.event_count() as u128,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        for event in &self.events {
            if event.round >= self.rounds {
                return Err(SparseError::InvalidInvariant {
                    message: "syndrome contains an event outside its round range",
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Sparse correction
// ============================================================================

/// Sparse correction operator.
///
/// Each entry stores the binary-symplectic X/Z components for one physical
/// qubit. Identity entries are never stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseCorrection {
    num_qubits: usize,
    operators: BTreeMap<usize, u8>,
}

impl SparseCorrection {
    /// Creates an empty correction.
    pub fn new(
        num_qubits: usize,
        limits: &QecLimits,
    ) -> SparseResult<Self> {
        if num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: num_qubits,
            });
        }

        if num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        Ok(Self {
            num_qubits,
            operators: BTreeMap::new(),
        })
    }

    /// Returns the number of represented qubits.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of non-identity corrections.
    #[must_use]
    pub fn len(&self) -> usize {
        self.operators.len()
    }

    /// Returns whether the correction is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.operators.is_empty()
    }

    /// Applies a binary-symplectic Pauli component.
    ///
    /// The value must be:
    ///
    /// ```text
    /// 0 = I
    /// 1 = X
    /// 2 = Y
    /// 3 = Z
    /// ```
    ///
    /// Composition is performed modulo global phase.
    pub fn apply(
        &mut self,
        qubit: usize,
        pauli: u8,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if qubit >= self.num_qubits {
            return Err(SparseError::QubitOutOfRange {
                qubit,
                num_qubits: self.num_qubits,
            });
        }

        if pauli > 3 {
            return Err(SparseError::InvalidInvariant {
                message: "sparse correction Pauli component must be 0..=3",
            });
        }

        if pauli == 0 {
            return Ok(());
        }

        if !self.operators.contains_key(&qubit)
            && self.operators.len() >= limits.max_qubits
        {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: (self.operators.len() as u128) + 1,
                maximum: limits.max_qubits as u128,
            });
        }

        let current = self
            .operators
            .get(&qubit)
            .copied()
            .unwrap_or(0);

        let combined = pauli_multiply(current, pauli);

        if combined == 0 {
            self.operators.remove(&qubit);
        } else {
            self.operators.insert(qubit, combined);
        }

        Ok(())
    }

    /// Returns the stored Pauli component.
    #[must_use]
    pub fn get(&self, qubit: usize) -> u8 {
        self.operators
            .get(&qubit)
            .copied()
            .unwrap_or(0)
    }

    /// Returns deterministic correction entries.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (usize, u8)> + '_ {
        self.operators
            .iter()
            .map(|(&qubit, &pauli)| (qubit, pauli))
    }

    /// Returns whether this correction is the identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.operators.is_empty()
    }

    /// Estimates memory used by this correction.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let count = usize_to_u64(
            self.len(),
            LimitKind::MemoryBytes,
        )?;

        checked_mul(
            count,
            ESTIMATED_CORRECTION_BYTES,
            LimitKind::MemoryBytes,
        )
    }

    /// Validates the correction.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if self.num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: self.num_qubits,
            });
        }

        if self.num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        for (&qubit, &pauli) in &self.operators {
            if qubit >= self.num_qubits {
                return Err(SparseError::QubitOutOfRange {
                    qubit,
                    num_qubits: self.num_qubits,
                });
            }

            if pauli == 0 || pauli > 3 {
                return Err(SparseError::InvalidInvariant {
                    message: "sparse correction contains invalid Pauli",
                });
            }
        }

        Ok(())
    }
}

// ============================================================================
// Pauli composition
// ============================================================================

/// Multiplies binary-symplectic single-qubit Pauli values modulo global phase.
///
/// ```text
/// I = 0
/// X = 1
/// Y = 2
/// Z = 3
/// ```
#[must_use]
pub const fn pauli_multiply(left: u8, right: u8) -> u8 {
    let x_left = (left == 1) || (left == 2);
    let z_left = (left == 2) || (left == 3);

    let x_right = (right == 1) || (right == 2);
    let z_right = (right == 2) || (right == 3);

    let x = x_left ^ x_right;
    let z = z_left ^ z_right;

    match (x, z) {
        (false, false) => 0,
        (true, false) => 1,
        (true, true) => 2,
        (false, true) => 3,
    }
}

// ============================================================================
// Sparse resource estimate
// ============================================================================

/// Aggregate estimate for a sparse QEC workload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparseResourceEstimate {
    /// Estimated memory in bytes.
    pub memory_bytes: u64,

    /// Number of sparse Pauli support entries.
    pub pauli_support_entries: u64,

    /// Number of stabilizer generators.
    pub stabilizers: u64,

    /// Number of graph nodes.
    pub graph_nodes: u64,

    /// Number of graph edges.
    pub graph_edges: u64,

    /// Number of syndrome events.
    pub syndrome_events: u64,

    /// Number of correction entries.
    pub correction_entries: u64,
}

impl SparseResourceEstimate {
    /// Returns a zero estimate.
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            memory_bytes: 0,
            pauli_support_entries: 0,
            stabilizers: 0,
            graph_nodes: 0,
            graph_edges: 0,
            syndrome_events: 0,
            correction_entries: 0,
        }
    }

    /// Adds another estimate using checked arithmetic.
    pub fn checked_add(
        self,
        other: Self,
    ) -> SparseResult<Self> {
        Ok(Self {
            memory_bytes: checked_add(
                self.memory_bytes,
                other.memory_bytes,
                LimitKind::MemoryBytes,
            )?,

            pauli_support_entries: checked_add(
                self.pauli_support_entries,
                other.pauli_support_entries,
                LimitKind::StabilizerWeight,
            )?,

            stabilizers: checked_add(
                self.stabilizers,
                other.stabilizers,
                LimitKind::Stabilizers,
            )?,

            graph_nodes: checked_add(
                self.graph_nodes,
                other.graph_nodes,
                LimitKind::GraphNodes,
            )?,

            graph_edges: checked_add(
                self.graph_edges,
                other.graph_edges,
                LimitKind::GraphEdges,
            )?,

            syndrome_events: checked_add(
                self.syndrome_events,
                other.syndrome_events,
                LimitKind::SyndromeEvents,
            )?,

            correction_entries: checked_add(
                self.correction_entries,
                other.correction_entries,
                LimitKind::Qubits,
            )?,
        })
    }

    /// Validates the memory estimate against the canonical policy.
    pub fn validate(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        if self.memory_bytes > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: self.memory_bytes as u128,
                maximum: limits.max_memory_bytes as u128,
            });
        }

        if self.stabilizers > limits.max_stabilizers as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: self.stabilizers as u128,
                maximum: limits.max_stabilizers as u128,
            });
        }

        if self.graph_nodes > limits.max_graph_nodes as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                requested: self.graph_nodes as u128,
                maximum: limits.max_graph_nodes as u128,
            });
        }

        if self.graph_edges > limits.max_graph_edges as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: self.graph_edges as u128,
                maximum: limits.max_graph_edges as u128,
            });
        }

        if self.syndrome_events
            > limits.max_syndrome_events as u64
        {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: self.syndrome_events as u128,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        Ok(())
    }
}

// ============================================================================
// Sparse preflight
// ============================================================================

/// Performs a no-allocation preflight for a sparse QEC workload.
///
/// This function is intended to be called before constructing a large sparse
/// object.
///
/// The function deliberately does not allocate.
pub fn preflight_sparse_workload(
    limits: &QecLimits,
    num_qubits: usize,
    stabilizers: usize,
    graph_nodes: usize,
    graph_edges: usize,
    syndrome_events: usize,
    correction_entries: usize,
) -> SparseResult<SparseResourceEstimate> {
    limits.validate().map_err(limit_error)?;

    if num_qubits == 0 {
        return Err(SparseError::InvalidQubitCount {
            qubits: num_qubits,
        });
    }

    if num_qubits > limits.max_qubits {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::Qubits,
            requested: num_qubits as u128,
            maximum: limits.max_qubits as u128,
        });
    }

    if stabilizers > limits.max_stabilizers {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::Stabilizers,
            requested: stabilizers as u128,
            maximum: limits.max_stabilizers as u128,
        });
    }

    if graph_nodes > limits.max_graph_nodes {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::GraphNodes,
            requested: graph_nodes as u128,
            maximum: limits.max_graph_nodes as u128,
        });
    }

    if graph_edges > limits.max_graph_edges {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::GraphEdges,
            requested: graph_edges as u128,
            maximum: limits.max_graph_edges as u128,
        });
    }

    if syndrome_events > limits.max_syndrome_events {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::SyndromeEvents,
            requested: syndrome_events as u128,
            maximum: limits.max_syndrome_events as u128,
        });
    }

    if correction_entries > limits.max_qubits {
        return Err(SparseError::ResourceLimitExceeded {
            resource: LimitKind::Qubits,
            requested: correction_entries as u128,
            maximum: limits.max_qubits as u128,
        });
    }

    let graph_node_count = usize_to_u64(
        graph_nodes,
        LimitKind::GraphNodes,
    )?;

    let graph_edge_count = usize_to_u64(
        graph_edges,
        LimitKind::GraphEdges,
    )?;

    let syndrome_count = usize_to_u64(
        syndrome_events,
        LimitKind::SyndromeEvents,
    )?;

    let correction_count = usize_to_u64(
        correction_entries,
        LimitKind::Qubits,
    )?;

    let stabilizer_count = usize_to_u64(
        stabilizers,
        LimitKind::Stabilizers,
    )?;

    let graph_memory = checked_add(
        checked_mul(
            graph_node_count,
            ESTIMATED_INDEX_BYTES,
            LimitKind::MemoryBytes,
        )?,
        checked_mul(
            graph_edge_count,
            ESTIMATED_EDGE_BYTES,
            LimitKind::MemoryBytes,
        )?,
        LimitKind::MemoryBytes,
    )?;

    let syndrome_memory = checked_mul(
        syndrome_count,
        ESTIMATED_SYNDROME_EVENT_BYTES,
        LimitKind::MemoryBytes,
    )?;

    let correction_memory = checked_mul(
        correction_count,
        ESTIMATED_CORRECTION_BYTES,
        LimitKind::MemoryBytes,
    )?;

    let stabilizer_memory = checked_mul(
        stabilizer_count,
        ESTIMATED_INDEX_BYTES,
        LimitKind::MemoryBytes,
    )?;

    let memory_bytes = checked_add(
        graph_memory,
        syndrome_memory,
        LimitKind::MemoryBytes,
    )?;

    let memory_bytes = checked_add(
        memory_bytes,
        correction_memory,
        LimitKind::MemoryBytes,
    )?;

    let memory_bytes = checked_add(
        memory_bytes,
        stabilizer_memory,
        LimitKind::MemoryBytes,
    )?;

    let estimate = SparseResourceEstimate {
        memory_bytes,
        pauli_support_entries: 0,
        stabilizers: stabilizer_count,
        graph_nodes: graph_node_count,
        graph_edges: graph_edge_count,
        syndrome_events: syndrome_count,
        correction_entries: correction_count,
    };

    estimate.validate(limits)?;

    Ok(estimate)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> QecLimits {
        QecLimits::default()
    }

    #[test]
    fn sparse_identity_is_empty() {
        let identity = SparsePauli::identity(16)
            .expect("valid sparse identity");

        assert!(identity.is_identity());
        assert_eq!(identity.weight(), 0);
        assert_eq!(identity.support_size(), 0);
    }

    #[test]
    fn sparse_pauli_tracks_only_non_identity_support() {
        let mut pauli =
            SparsePauli::identity(100_000)
                .expect("valid sparse Pauli");

        pauli.insert_x(2)
            .expect("valid qubit");

        pauli.insert_z(2)
            .expect("valid qubit");

        pauli.insert_x(99_999)
            .expect("valid qubit");

        assert_eq!(pauli.weight(), 2);
        assert_eq!(pauli.component(2), 2);
        assert_eq!(pauli.component(99_999), 1);
        assert_eq!(pauli.component(0), 0);
    }

    #[test]
    fn sparse_paulis_use_symplectic_intersections() {
        let a = SparsePauli::from_supports(
            8,
            [1usize],
            [2usize],
        )
        .expect("valid sparse Pauli");

        let b = SparsePauli::from_supports(
            8,
            [2usize],
            [1usize],
        )
        .expect("valid sparse Pauli");

        assert_eq!(
            a.symplectic_product(&b)
                .expect("matching dimensions"),
            0
        );
    }

    #[test]
    fn sparse_pauli_detects_anticommutation() {
        let x = SparsePauli::from_supports(
            8,
            [2usize],
            [],
        )
        .expect("valid sparse Pauli");

        let z = SparsePauli::from_supports(
            8,
            [],
            [2usize],
        )
        .expect("valid sparse Pauli");

        assert_eq!(
            x.symplectic_product(&z)
                .expect("matching dimensions"),
            1
        );

        assert!(
            !x.commutes_with(&z)
                .expect("matching dimensions")
        );
    }

    #[test]
    fn sparse_pauli_multiplication_is_xor_based() {
        let x = SparsePauli::from_supports(
            4,
            [1usize],
            [],
        )
        .expect("valid sparse Pauli");

        let z = SparsePauli::from_supports(
            4,
            [],
            [1usize],
        )
        .expect("valid sparse Pauli");

        let y = x.multiply(&z)
            .expect("matching dimensions");

        assert_eq!(y.component(1), 2);
    }

    #[test]
    fn sparse_stabilizer_matrix_enforces_weight_limit() {
        let mut matrix =
            SparseStabilizerMatrix::new(16, &limits())
                .expect("valid matrix");

        let generator =
            SparsePauli::from_supports(
                16,
                [0, 1, 2],
                [3, 4],
            )
            .expect("valid generator");

        matrix
            .push(generator, &limits())
            .expect("generator should fit");

        assert_eq!(matrix.len(), 1);
        assert_eq!(matrix.max_weight(), 5);
    }

    #[test]
    fn sparse_adjacency_is_deterministic() {
        let mut graph = SparseAdjacency::new();
        let policy = limits();

        graph
            .insert_undirected_edge(4, 1, &policy)
            .expect("edge");

        graph
            .insert_undirected_edge(4, 2, &policy)
            .expect("edge");

        let nodes: Vec<_> = graph.nodes().collect();

        assert_eq!(nodes, vec![1, 2, 4]);

        let neighbors = graph
            .neighbors(4)
            .expect("node must exist");

        let neighbors: Vec<_> =
            neighbors.iter().copied().collect();

        assert_eq!(neighbors, vec![1, 2]);
    }

    #[test]
    fn sparse_graph_rejects_invalid_weight() {
        assert!(
            SparseWeightedEdge::new(1, 2, f64::NAN)
                .is_err()
        );

        assert!(
            SparseWeightedEdge::new(1, 2, -1.0)
                .is_err()
        );
    }

    #[test]
    fn sparse_syndrome_stores_only_detection_events() {
        let policy = limits();

        let mut syndrome =
            SparseSyndrome::new(10, &policy)
                .expect("valid syndrome");

        syndrome
            .insert(
                SparseSyndromeEvent::new(2, 7, true),
                &policy,
            )
            .expect("event");

        syndrome
            .insert(
                SparseSyndromeEvent::new(3, 8, true),
                &policy,
            )
            .expect("event");

        syndrome
            .insert(
                SparseSyndromeEvent::new(4, 9, false),
                &policy,
            )
            .expect("trivial event");

        assert_eq!(syndrome.event_count(), 2);
        assert!(!syndrome.is_trivial());
    }

    #[test]
    fn sparse_correction_composes_paulis() {
        let policy = limits();

        let mut correction =
            SparseCorrection::new(32, &policy)
                .expect("valid correction");

        correction
            .apply(7, 1, &policy)
            .expect("X correction");

        correction
            .apply(7, 3, &policy)
            .expect("Z correction");

        assert_eq!(correction.get(7), 2);

        correction
            .apply(7, 2, &policy)
            .expect("Y correction");

        assert_eq!(correction.get(7), 0);
        assert!(correction.is_identity());
    }

    #[test]
    fn preflight_performs_no_sparse_allocation() {
        let estimate =
            preflight_sparse_workload(
                &limits(),
                1_000,
                500,
                2_000,
                4_000,
                5_000,
                100,
            )
            .expect("workload should fit");

        assert_eq!(estimate.stabilizers, 500);
        assert_eq!(estimate.graph_nodes, 2_000);
        assert_eq!(estimate.graph_edges, 4_000);
        assert_eq!(estimate.syndrome_events, 5_000);
        assert_eq!(estimate.correction_entries, 100);
        assert!(estimate.memory_bytes > 0);
    }

    #[test]
    fn preflight_rejects_excessive_graphs() {
        let mut policy = limits();

        policy.max_graph_nodes = 4;

        let result =
            preflight_sparse_workload(
                &policy,
                16,
                1,
                5,
                1,
                1,
                1,
            );

        assert!(matches!(
            result,
            Err(
                SparseError::ResourceLimitExceeded {
                    resource: LimitKind::GraphNodes,
                    ..
                }
            )
        ));
    }

    #[test]
    fn out_of_range_qubits_never_panic() {
        let mut pauli =
            SparsePauli::identity(8)
                .expect("valid Pauli");

        assert!(
            pauli.insert_x(8).is_err()
        );

        assert!(
            pauli.insert_z(usize::MAX).is_err()
        );
    }

    #[test]
    fn sparse_structures_have_no_local_production_ceiling() {
        // The implementation deliberately does not define arbitrary
        // MAX_* resource ceilings. QecLimits is the canonical policy.
        let policy = limits();

        let mut pauli =
            SparsePauli::identity(
                policy.max_qubits.min(1_000),
            )
            .expect("valid Pauli");

        pauli
            .insert_x(0)
            .expect("valid qubit");

        assert_eq!(pauli.weight(), 1);
    }
}