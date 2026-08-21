//! Zamani Quantum Error Correction — Sparse Representations.
//!
//! This module owns sparse mathematical/data representations used throughout
//! the QEC pipeline.
//!
//! # Architectural contract
//!
//! ```text
//!                         QecLimits
//!                            |
//!                            v
//!                    sparse preflight
//!                            |
//!          +-----------------+-----------------+
//!          |                 |                 |
//!          v                 v                 v
//!      SparsePauli      SparseGraph      SparseSyndrome
//!          |                 |                 |
//!          v                 v                 v
//! SparseStabilizerMatrix  SparseCorrection  event streams
//!          |                 |                 |
//!          +-----------------+-----------------+
//!                            |
//!                            v
//!                    Decoder / QEC pipeline
//! ```
//!
//! # Responsibilities
//!
//! This module owns:
//!
//! - sparse Pauli representations;
//! - sparse stabilizer matrices;
//! - sparse graph representations;
//! - sparse syndrome/event representations;
//! - sparse correction representations;
//! - deterministic iteration;
//! - structural validation;
//! - checked memory estimation;
//! - canonical `QecLimits` preflight.
//!
//! This module does NOT own:
//!
//! - decoder algorithms;
//! - decoder-specific limits;
//! - QPU access;
//! - networking;
//! - runtime resource accounting;
//! - allocation enforcement;
//! - telemetry;
//! - configuration policy;
//! - parsing textual source documents.
//!
//! Runtime resource consumption belongs to `resources.rs`.
//! Allocation reservation belongs to `memory.rs`.
//! Declarative limits belong to `limits.rs`.
//!
//! # Integration contract
//!
//! `limits.rs`
//!     -> canonical resource policy and `LimitKind`.
//!
//! `memory.rs`
//!     -> actual memory reservation before large allocation.
//!
//! `resources.rs`
//!     -> runtime accounting after allocation/use.
//!
//! `stabilizer.rs`
//!     -> mathematical stabilizer operations.
//!
//! `syndrome.rs`
//!     -> higher-level syndrome semantics.
//!
//! `decoding_graph.rs`
//!     -> decoder-facing graph semantics.
//!
//! `decoder.rs`
//!     -> consumes sparse correction/graph/syndrome representations.
//!
//! `mwpm.rs` / `union_find.rs`
//!     -> consume sparse graph representations.
//!
//! `streaming.rs` / `partition.rs`
//!     -> consume bounded sparse event representations.
//!
//! The sparse layer deliberately does not depend on any of those higher-level
//! modules. This keeps the dependency direction acyclic and allows this file
//! to be completed independently.
//!
//! # Determinism
//!
//! All public collections use ordered maps/sets. Iteration therefore has a
//! stable order independent of hash randomization or insertion order.
//!
//! # Rust compatibility
//!
//! Target language/toolchain: Rust 1.97.1.
//!
//! The implementation uses stable standard-library APIs and does not require
//! nightly features.

use core::fmt;
use std::collections::{BTreeMap, BTreeSet};

use super::limits::{LimitError, LimitKind, QecLimits};

/* ========================================================================== */
/* Constants                                                                  */
/* ========================================================================== */

/// Conservative estimated bytes per sparse index/component.
pub const ESTIMATED_INDEX_BYTES: u64 = 16;

/// Conservative estimated bytes per sparse graph edge.
pub const ESTIMATED_EDGE_BYTES: u64 = 24;

/// Conservative estimated bytes per sparse syndrome event.
pub const ESTIMATED_SYNDROME_EVENT_BYTES: u64 = 32;

/// Conservative estimated bytes per sparse correction entry.
pub const ESTIMATED_CORRECTION_BYTES: u64 = 24;

/// Conservative estimated bytes per sparse graph node.
pub const ESTIMATED_GRAPH_NODE_BYTES: u64 = 32;

/// Conservative estimated bytes per stabilizer row.
pub const ESTIMATED_STABILIZER_ROW_BYTES: u64 = 32;

/* ========================================================================== */
/* Error type                                                                 */
/* ========================================================================== */

/// Errors produced by sparse QEC representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SparseError {
    /// A representation cannot contain zero qubits.
    InvalidQubitCount {
        qubits: usize,
    },

    /// A representation cannot contain zero nodes.
    InvalidNodeCount {
        nodes: usize,
    },

    /// An index is outside its declared domain.
    IndexOutOfRange {
        index: usize,
        upper_bound: usize,
        domain: &'static str,
    },

    /// Two representations have incompatible dimensions.
    DimensionMismatch {
        left: usize,
        right: usize,
        domain: &'static str,
    },

    /// A resource limit was exceeded during preflight.
    ResourceLimitExceeded {
        resource: LimitKind,
        requested: u128,
        maximum: u128,
    },

    /// A checked calculation overflowed.
    ArithmeticOverflow {
        resource: LimitKind,
    },

    /// An internal structural invariant is invalid.
    InvalidInvariant {
        message: &'static str,
    },

    /// An operation would introduce an invalid value.
    InvalidValue {
        message: &'static str,
    },

    /// An edge refers to itself when self-edges are prohibited.
    SelfEdge {
        node: usize,
    },
}

impl fmt::Display for SparseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubitCount { qubits } => {
                write!(f, "invalid sparse qubit count: {qubits}")
            }

            Self::InvalidNodeCount { nodes } => {
                write!(f, "invalid sparse node count: {nodes}")
            }

            Self::IndexOutOfRange {
                index,
                upper_bound,
                domain,
            } => {
                write!(
                    f,
                    "{domain} index {index} is outside \
                     valid range 0..{upper_bound}"
                )
            }

            Self::DimensionMismatch {
                left,
                right,
                domain,
            } => {
                write!(
                    f,
                    "{domain} dimension mismatch: {left} != {right}"
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

            Self::InvalidInvariant { message } => {
                write!(
                    f,
                    "invalid sparse representation invariant: {message}"
                )
            }

            Self::InvalidValue { message } => {
                write!(f, "invalid sparse value: {message}")
            }

            Self::SelfEdge { node } => {
                write!(f, "self-edge is not permitted for node {node}")
            }
        }
    }
}

impl std::error::Error for SparseError {}

/// Result type for sparse QEC operations.
pub type SparseResult<T> = Result<T, SparseError>;

/* ========================================================================== */
/* Checked arithmetic                                                         */
/* ========================================================================== */

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
    u64::try_from(value)
        .map_err(|_| SparseError::ArithmeticOverflow { resource })
}

fn map_limit_error(error: LimitError) -> SparseError {
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

        LimitError::InconsistentLimits { .. }
        | LimitError::UnsupportedSchema { .. } => {
            SparseError::InvalidInvariant {
                message: "invalid QEC limits supplied to sparse preflight",
            }
        }
    }
}

/* ========================================================================== */
/* Sparse resource estimate                                                   */
/* ========================================================================== */

/// Conservative preflight estimate for a sparse representation.
///
/// This is intentionally an estimate, not a replacement for runtime
/// accounting in `resources.rs` or allocation reservation in `memory.rs`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SparseResourceEstimate {
    /// Estimated bytes.
    pub memory_bytes: u64,

    /// Number of sparse entries.
    pub entries: u64,

    /// Number of graph nodes.
    pub graph_nodes: u64,

    /// Number of graph edges.
    pub graph_edges: u64,

    /// Number of syndrome events.
    pub syndrome_events: u64,

    /// Number of correction entries.
    pub correction_entries: u64,

    /// Number of stabilizer rows.
    pub stabilizer_rows: u64,
}

impl SparseResourceEstimate {
    /// Creates an empty estimate.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            memory_bytes: 0,
            entries: 0,
            graph_nodes: 0,
            graph_edges: 0,
            syndrome_events: 0,
            correction_entries: 0,
            stabilizer_rows: 0,
        }
    }

    /// Adds another estimate using checked arithmetic.
    pub fn checked_add(
        &self,
        other: &Self,
    ) -> SparseResult<Self> {
        Ok(Self {
            memory_bytes: checked_add(
                self.memory_bytes,
                other.memory_bytes,
                LimitKind::MemoryBytes,
            )?,

            entries: checked_add(
                self.entries,
                other.entries,
                LimitKind::MemoryBytes,
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
                LimitKind::MemoryBytes,
            )?,

            stabilizer_rows: checked_add(
                self.stabilizer_rows,
                other.stabilizer_rows,
                LimitKind::Stabilizers,
            )?,
        })
    }

    /// Validates the estimate against canonical QEC policy.
    pub fn validate_against(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(map_limit_error)?;

        if self.memory_bytes > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: u128::from(self.memory_bytes),
                maximum: u128::from(limits.max_memory_bytes),
            });
        }

        if self.graph_nodes > limits.max_graph_nodes as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                requested: u128::from(self.graph_nodes),
                maximum: u128::from(limits.max_graph_nodes as u64),
            });
        }

        if self.graph_edges > limits.max_graph_edges as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: u128::from(self.graph_edges),
                maximum: u128::from(limits.max_graph_edges as u64),
            });
        }

        if self.syndrome_events > limits.max_syndrome_events as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: u128::from(self.syndrome_events),
                maximum: u128::from(limits.max_syndrome_events as u64),
            });
        }

        if self.stabilizer_rows > limits.max_stabilizers as u64 {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: u128::from(self.stabilizer_rows),
                maximum: u128::from(limits.max_stabilizers as u64),
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Sparse Pauli                                                               */
/* ========================================================================== */

/// Sparse binary-symplectic Pauli representation.
///
/// Only non-zero X and Z components are stored.
///
/// ```text
/// X support = {1, 4, 9000}
/// Z support = {4, 7}
/// ```
///
/// Qubit `4` therefore represents `Y`.
///
/// Global phase is intentionally omitted because stabilizer/QEC correction
/// logic generally works modulo global phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparsePauli {
    num_qubits: usize,
    x_support: BTreeSet<usize>,
    z_support: BTreeSet<usize>,
}

impl SparsePauli {
    /// Creates the identity on `num_qubits`.
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

    /// Creates a Pauli from explicit X/Z supports.
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

    /// Number of represented qubits.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// X support.
    #[must_use]
    pub fn x_support(&self) -> &BTreeSet<usize> {
        &self.x_support
    }

    /// Z support.
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

    /// Returns whether an X component exists.
    #[must_use]
    pub fn has_x(&self, qubit: usize) -> bool {
        self.x_support.contains(&qubit)
    }

    /// Returns whether a Z component exists.
    #[must_use]
    pub fn has_z(&self, qubit: usize) -> bool {
        self.z_support.contains(&qubit)
    }

    /// Returns whether this is identity.
    #[must_use]
    pub fn is_identity(&self) -> bool {
        self.x_support.is_empty() && self.z_support.is_empty()
    }

    /// Returns Pauli weight.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.x_support
            .union(&self.z_support)
            .count()
    }

    /// Returns number of stored X/Z components.
    #[must_use]
    pub fn support_size(&self) -> usize {
        self.x_support.len() + self.z_support.len()
    }

    /// Returns a single-qubit component.
    ///
    /// ```text
    /// 0 = I
    /// 1 = X
    /// 2 = Y
    /// 3 = Z
    /// ```
    #[must_use]
    pub fn component(&self, qubit: usize) -> u8 {
        match (
            self.x_support.contains(&qubit),
            self.z_support.contains(&qubit),
        ) {
            (false, false) => 0,
            (true, false) => 1,
            (true, true) => 2,
            (false, true) => 3,
        }
    }

    /// Computes the binary symplectic product.
    ///
    /// `0` means commuting.
    /// `1` means anti-commuting.
    pub fn symplectic_product(
        &self,
        other: &Self,
    ) -> SparseResult<u8> {
        self.ensure_same_dimension(other)?;

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

    /// Multiplies modulo global phase.
    pub fn multiply(
        &self,
        other: &Self,
    ) -> SparseResult<Self> {
        self.ensure_same_dimension(other)?;

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

    /// Deterministically iterates over all non-identity qubits.
    pub fn support(&self) -> impl Iterator<Item = usize> + '_ {
        self.x_support.union(&self.z_support).copied()
    }

    /// Conservative memory estimate.
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

    /// Validates all invariants.
    pub fn validate(&self) -> SparseResult<()> {
        if self.num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: 0,
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

    /// Validates this Pauli against canonical QEC policy.
    pub fn preflight(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        self.validate()?;

        if self.num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.weight() > limits.max_logical_operator_weight {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::LogicalOperatorWeight,
                requested: self.weight() as u128,
                maximum: limits.max_logical_operator_weight as u128,
            });
        }

        self.estimated_memory_bytes()?;
        Ok(())
    }

    fn validate_qubit(&self, qubit: usize) -> SparseResult<()> {
        if qubit >= self.num_qubits {
            return Err(SparseError::IndexOutOfRange {
                index: qubit,
                upper_bound: self.num_qubits,
                domain: "qubit",
            });
        }

        Ok(())
    }

    fn ensure_same_dimension(
        &self,
        other: &Self,
    ) -> SparseResult<()> {
        if self.num_qubits != other.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: self.num_qubits,
                right: other.num_qubits,
                domain: "Pauli",
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Sparse stabilizer matrix                                                   */
/* ========================================================================== */

/// Sparse collection of stabilizer generators.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseStabilizerMatrix {
    num_qubits: usize,
    rows: BTreeMap<usize, SparsePauli>,
}

impl SparseStabilizerMatrix {
    /// Creates an empty matrix.
    pub fn new(num_qubits: usize) -> SparseResult<Self> {
        if num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: num_qubits,
            });
        }

        Ok(Self {
            num_qubits,
            rows: BTreeMap::new(),
        })
    }

    /// Creates a matrix from rows.
    pub fn from_rows<I>(
        num_qubits: usize,
        rows: I,
    ) -> SparseResult<Self>
    where
        I: IntoIterator<Item = (usize, SparsePauli)>,
    {
        let mut matrix = Self::new(num_qubits)?;

        for (row, pauli) in rows {
            matrix.insert(row, pauli)?;
        }

        Ok(matrix)
    }

    /// Number of qubits.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Number of stabilizer rows.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns whether the matrix has no rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns a row.
    #[must_use]
    pub fn get(&self, row: usize) -> Option<&SparsePauli> {
        self.rows.get(&row)
    }

    /// Deterministically iterates over rows.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&usize, &SparsePauli)> {
        self.rows.iter()
    }

    /// Inserts or replaces a stabilizer row.
    pub fn insert(
        &mut self,
        row: usize,
        pauli: SparsePauli,
    ) -> SparseResult<Option<SparsePauli>> {
        if pauli.num_qubits() != self.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: pauli.num_qubits(),
                right: self.num_qubits,
                domain: "stabilizer matrix",
            });
        }

        if row >= self.rows.len()
            && row > self.rows.len().saturating_add(1_000_000)
        {
            return Err(SparseError::InvalidValue {
                message: "stabilizer row index is unreasonably sparse",
            });
        }

        pauli.validate()?;

        Ok(self.rows.insert(row, pauli))
    }

    /// Removes a row.
    pub fn remove(
        &mut self,
        row: usize,
    ) -> Option<SparsePauli> {
        self.rows.remove(&row)
    }

    /// Validates all rows.
    pub fn validate(&self) -> SparseResult<()> {
        if self.num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: 0,
            });
        }

        for pauli in self.rows.values() {
            if pauli.num_qubits() != self.num_qubits {
                return Err(SparseError::DimensionMismatch {
                    left: pauli.num_qubits(),
                    right: self.num_qubits,
                    domain: "stabilizer matrix",
                });
            }

            pauli.validate()?;
        }

        Ok(())
    }

    /// Conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let rows = usize_to_u64(
            self.rows.len(),
            LimitKind::Stabilizers,
        )?;

        let mut total = checked_mul(
            rows,
            ESTIMATED_STABILIZER_ROW_BYTES,
            LimitKind::MemoryBytes,
        )?;

        for pauli in self.rows.values() {
            total = checked_add(
                total,
                pauli.estimated_memory_bytes()?,
                LimitKind::MemoryBytes,
            )?;
        }

        Ok(total)
    }

    /// Canonical QEC preflight.
    pub fn preflight(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(map_limit_error)?;
        self.validate()?;

        if self.num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.row_count() > limits.max_stabilizers {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: self.row_count() as u128,
                maximum: limits.max_stabilizers as u128,
            });
        }

        for pauli in self.rows.values() {
            if pauli.weight() > limits.max_stabilizer_weight {
                return Err(SparseError::ResourceLimitExceeded {
                    resource: LimitKind::StabilizerWeight,
                    requested: pauli.weight() as u128,
                    maximum: limits.max_stabilizer_weight as u128,
                });
            }
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: u128::from(memory),
                maximum: u128::from(limits.max_memory_bytes),
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Sparse graph                                                               */
/* ========================================================================== */

/// Sparse undirected graph.
///
/// The graph stores each edge canonically as `(min(a,b), max(a,b))`.
/// This prevents duplicate logical edges and guarantees deterministic
/// serialization/iteration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseGraph {
    node_count: usize,
    adjacency: BTreeMap<usize, BTreeSet<usize>>,
    edge_count: usize,
}

impl SparseGraph {
    /// Creates an empty graph with `node_count` nodes.
    pub fn new(node_count: usize) -> SparseResult<Self> {
        if node_count == 0 {
            return Err(SparseError::InvalidNodeCount {
                nodes: node_count,
            });
        }

        Ok(Self {
            node_count,
            adjacency: BTreeMap::new(),
            edge_count: 0,
        })
    }

    /// Number of nodes.
    #[must_use]
    pub const fn node_count(&self) -> usize {
        self.node_count
    }

    /// Number of undirected edges.
    #[must_use]
    pub const fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Returns neighbors of a node.
    #[must_use]
    pub fn neighbors(
        &self,
        node: usize,
    ) -> Option<&BTreeSet<usize>> {
        if node >= self.node_count {
            return None;
        }

        self.adjacency.get(&node)
    }

    /// Returns whether an edge exists.
    #[must_use]
    pub fn contains_edge(
        &self,
        from: usize,
        to: usize,
    ) -> bool {
        if from >= self.node_count || to >= self.node_count {
            return false;
        }

        self.adjacency
            .get(&from)
            .map_or(false, |neighbors| neighbors.contains(&to))
    }

    /// Adds an undirected edge.
    ///
    /// Adding an existing edge is idempotent and returns `false`.
    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
    ) -> SparseResult<bool> {
        self.validate_node(from)?;
        self.validate_node(to)?;

        if from == to {
            return Err(SparseError::SelfEdge { node: from });
        }

        if self.contains_edge(from, to) {
            return Ok(false);
        }

        self.adjacency
            .entry(from)
            .or_default()
            .insert(to);

        self.adjacency
            .entry(to)
            .or_default()
            .insert(from);

        self.edge_count = self
            .edge_count
            .checked_add(1)
            .ok_or(SparseError::ArithmeticOverflow {
                resource: LimitKind::GraphEdges,
            })?;

        Ok(true)
    }

    /// Removes an undirected edge.
    pub fn remove_edge(
        &mut self,
        from: usize,
        to: usize,
    ) -> SparseResult<bool> {
        self.validate_node(from)?;
        self.validate_node(to)?;

        if !self.contains_edge(from, to) {
            return Ok(false);
        }

        if let Some(neighbors) = self.adjacency.get_mut(&from) {
            neighbors.remove(&to);
            if neighbors.is_empty() {
                self.adjacency.remove(&from);
            }
        }

        if let Some(neighbors) = self.adjacency.get_mut(&to) {
            neighbors.remove(&from);
            if neighbors.is_empty() {
                self.adjacency.remove(&to);
            }
        }

        self.edge_count = self
            .edge_count
            .checked_sub(1)
            .ok_or(SparseError::InvalidInvariant {
                message: "graph edge count underflow",
            })?;

        Ok(true)
    }

    /// Deterministically iterates over graph edges.
    pub fn edges(
        &self,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.adjacency.iter().flat_map(|(&from, neighbors)| {
            neighbors
                .iter()
                .filter_map(move |&to| {
                    if from < to {
                        Some((from, to))
                    } else {
                        None
                    }
                })
        })
    }

    /// Validates graph invariants.
    pub fn validate(&self) -> SparseResult<()> {
        if self.node_count == 0 {
            return Err(SparseError::InvalidNodeCount {
                nodes: 0,
            });
        }

        let mut counted_edges = 0usize;

        for (&node, neighbors) in &self.adjacency {
            self.validate_node(node)?;

            for &neighbor in neighbors {
                self.validate_node(neighbor)?;

                if node == neighbor {
                    return Err(SparseError::SelfEdge { node });
                }

                if !self
                    .adjacency
                    .get(&neighbor)
                    .map_or(false, |set| set.contains(&node))
                {
                    return Err(SparseError::InvalidInvariant {
                        message: "graph adjacency is not symmetric",
                    });
                }

                if node < neighbor {
                    counted_edges = counted_edges
                        .checked_add(1)
                        .ok_or(
                            SparseError::ArithmeticOverflow {
                                resource: LimitKind::GraphEdges,
                            },
                        )?;
                }
            }
        }

        if counted_edges != self.edge_count {
            return Err(SparseError::InvalidInvariant {
                message: "stored graph edge count disagrees with adjacency",
            });
        }

        Ok(())
    }

    /// Conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let nodes = usize_to_u64(
            self.adjacency.len(),
            LimitKind::GraphNodes,
        )?;

        let edges = usize_to_u64(
            self.edge_count,
            LimitKind::GraphEdges,
        )?;

        let node_bytes = checked_mul(
            nodes,
            ESTIMATED_GRAPH_NODE_BYTES,
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

    /// Canonical QEC preflight.
    pub fn preflight(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(map_limit_error)?;
        self.validate()?;

        if self.node_count > limits.max_graph_nodes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                requested: self.node_count as u128,
                maximum: limits.max_graph_nodes as u128,
            });
        }

        if self.edge_count > limits.max_graph_edges {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphEdges,
                requested: self.edge_count as u128,
                maximum: limits.max_graph_edges as u128,
            });
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: u128::from(memory),
                maximum: u128::from(limits.max_memory_bytes),
            });
        }

        Ok(())
    }

    fn validate_node(&self, node: usize) -> SparseResult<()> {
        if node >= self.node_count {
            return Err(SparseError::IndexOutOfRange {
                index: node,
                upper_bound: self.node_count,
                domain: "graph node",
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Sparse syndrome                                                            */
/* ========================================================================== */

/// A single sparse detection event.
///
/// `round` identifies the measurement round.
/// `stabilizer` identifies the detector/stabilizer.
/// `value` is the binary detection value.
///
/// Optional confidence and timestamp metadata are kept separate from the
/// sparse identity so the representation remains usable by classical
/// decoders without requiring a particular backend.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SparseSyndromeEvent {
    /// Measurement round.
    pub round: usize,

    /// Stabilizer/detector index.
    pub stabilizer: usize,

    /// Detection value.
    pub value: bool,

    /// Optional confidence in the measurement.
    pub confidence: Option<f64>,

    /// Optional timestamp in nanoseconds.
    pub timestamp_ns: Option<u64>,
}

impl SparseSyndromeEvent {
    /// Creates a binary detection event.
    #[must_use]
    pub const fn new(
        round: usize,
        stabilizer: usize,
        value: bool,
    ) -> Self {
        Self {
            round,
            stabilizer,
            value,
            confidence: None,
            timestamp_ns: None,
        }
    }

    /// Sets confidence.
    pub fn with_confidence(
        mut self,
        confidence: f64,
    ) -> SparseResult<Self> {
        if !confidence.is_finite()
            || !(0.0..=1.0).contains(&confidence)
        {
            return Err(SparseError::InvalidValue {
                message: "confidence must be finite and in [0, 1]",
            });
        }

        self.confidence = Some(confidence);
        Ok(self)
    }

    /// Sets a timestamp.
    #[must_use]
    pub const fn with_timestamp(
        mut self,
        timestamp_ns: u64,
    ) -> Self {
        self.timestamp_ns = Some(timestamp_ns);
        self
    }
}

/// Sparse syndrome/event collection.
///
/// Events are indexed by `(round, stabilizer)` and therefore duplicate
/// insertion replaces the existing event rather than silently producing two
/// conflicting values.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseSyndrome {
    max_rounds: usize,
    max_stabilizers: usize,
    events: BTreeMap<(usize, usize), SparseSyndromeEvent>,
}

impl SparseSyndrome {
    /// Creates an empty syndrome representation.
    pub fn new(
        max_rounds: usize,
        max_stabilizers: usize,
    ) -> SparseResult<Self> {
        if max_rounds == 0 {
            return Err(SparseError::InvalidValue {
                message: "syndrome must support at least one round",
            });
        }

        if max_stabilizers == 0 {
            return Err(SparseError::InvalidValue {
                message: "syndrome must support at least one stabilizer",
            });
        }

        Ok(Self {
            max_rounds,
            max_stabilizers,
            events: BTreeMap::new(),
        })
    }

    /// Maximum number of supported rounds.
    #[must_use]
    pub const fn max_rounds(&self) -> usize {
        self.max_rounds
    }

    /// Maximum number of supported stabilizers.
    #[must_use]
    pub const fn max_stabilizers(&self) -> usize {
        self.max_stabilizers
    }

    /// Number of stored events.
    #[must_use]
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Inserts/replaces an event.
    pub fn insert(
        &mut self,
        event: SparseSyndromeEvent,
    ) -> SparseResult<Option<SparseSyndromeEvent>> {
        self.validate_event(&event)?;

        Ok(self
            .events
            .insert((event.round, event.stabilizer), event))
    }

    /// Removes an event.
    pub fn remove(
        &mut self,
        round: usize,
        stabilizer: usize,
    ) -> Option<SparseSyndromeEvent> {
        self.events.remove(&(round, stabilizer))
    }

    /// Gets an event.
    #[must_use]
    pub fn get(
        &self,
        round: usize,
        stabilizer: usize,
    ) -> Option<&SparseSyndromeEvent> {
        self.events.get(&(round, stabilizer))
    }

    /// Deterministically iterates over events.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &SparseSyndromeEvent> {
        self.events.values()
    }

    /// Returns only active detection events.
    pub fn detection_events(
        &self,
    ) -> impl Iterator<Item = &SparseSyndromeEvent> {
        self.events.values().filter(|event| event.value)
    }

    /// Validates all events.
    pub fn validate(&self) -> SparseResult<()> {
        if self.max_rounds == 0 || self.max_stabilizers == 0 {
            return Err(SparseError::InvalidInvariant {
                message: "syndrome dimensions must be non-zero",
            });
        }

        for event in self.events.values() {
            self.validate_event(event)?;
        }

        Ok(())
    }

    /// Conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let count = usize_to_u64(
            self.events.len(),
            LimitKind::SyndromeEvents,
        )?;

        checked_mul(
            count,
            ESTIMATED_SYNDROME_EVENT_BYTES,
            LimitKind::MemoryBytes,
        )
    }

    /// Canonical QEC preflight.
    pub fn preflight(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(map_limit_error)?;
        self.validate()?;

        if self.event_count() > limits.max_syndrome_events {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::SyndromeEvents,
                requested: self.event_count() as u128,
                maximum: limits.max_syndrome_events as u128,
            });
        }

        if self.max_rounds > limits.max_rounds {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MeasurementRounds,
                requested: self.max_rounds as u128,
                maximum: limits.max_rounds as u128,
            });
        }

        if self.max_stabilizers > limits.max_stabilizers {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Stabilizers,
                requested: self.max_stabilizers as u128,
                maximum: limits.max_stabilizers as u128,
            });
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: u128::from(memory),
                maximum: u128::from(limits.max_memory_bytes),
            });
        }

        Ok(())
    }

    fn validate_event(
        &self,
        event: &SparseSyndromeEvent,
    ) -> SparseResult<()> {
        if event.round >= self.max_rounds {
            return Err(SparseError::IndexOutOfRange {
                index: event.round,
                upper_bound: self.max_rounds,
                domain: "syndrome round",
            });
        }

        if event.stabilizer >= self.max_stabilizers {
            return Err(SparseError::IndexOutOfRange {
                index: event.stabilizer,
                upper_bound: self.max_stabilizers,
                domain: "syndrome stabilizer",
            });
        }

        if let Some(confidence) = event.confidence {
            if !confidence.is_finite()
                || !(0.0..=1.0).contains(&confidence)
            {
                return Err(SparseError::InvalidValue {
                    message: "syndrome confidence must be finite and in [0, 1]",
                });
            }
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Sparse correction                                                          */
/* ========================================================================== */

/// Sparse correction entry.
///
/// The operation uses the same compact encoding as `SparsePauli`:
///
/// ```text
/// 0 = I
/// 1 = X
/// 2 = Y
/// 3 = Z
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparseCorrectionEntry {
    /// Physical qubit.
    pub qubit: usize,

    /// Pauli operation.
    pub operation: u8,
}

impl SparseCorrectionEntry {
    /// Creates a correction entry.
    pub fn new(
        qubit: usize,
        operation: u8,
    ) -> SparseResult<Self> {
        if operation > 3 {
            return Err(SparseError::InvalidValue {
                message: "correction operation must be 0, 1, 2, or 3",
            });
        }

        Ok(Self { qubit, operation })
    }

    /// Returns whether the operation is identity.
    #[must_use]
    pub const fn is_identity(&self) -> bool {
        self.operation == 0
    }
}

/// Sparse correction map.
///
/// Identity corrections are not retained. This guarantees that the
/// representation remains sparse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseCorrection {
    num_qubits: usize,
    entries: BTreeMap<usize, u8>,
}

impl SparseCorrection {
    /// Creates an empty correction.
    pub fn new(num_qubits: usize) -> SparseResult<Self> {
        if num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: num_qubits,
            });
        }

        Ok(Self {
            num_qubits,
            entries: BTreeMap::new(),
        })
    }

    /// Number of represented qubits.
    #[must_use]
    pub const fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Number of non-identity correction entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns whether there are no corrections.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the operation at a qubit.
    #[must_use]
    pub fn get(&self, qubit: usize) -> Option<u8> {
        self.entries.get(&qubit).copied()
    }

    /// Inserts a correction.
    ///
    /// Identity removes an existing correction.
    pub fn insert(
        &mut self,
        entry: SparseCorrectionEntry,
    ) -> SparseResult<Option<u8>> {
        self.validate_qubit(entry.qubit)?;

        if entry.operation > 3 {
            return Err(SparseError::InvalidValue {
                message: "correction operation must be in 0..=3",
            });
        }

        if entry.is_identity() {
            Ok(self.entries.remove(&entry.qubit))
        } else {
            Ok(self.entries.insert(
                entry.qubit,
                entry.operation,
            ))
        }
    }

    /// Removes a correction.
    pub fn remove(
        &mut self,
        qubit: usize,
    ) -> SparseResult<Option<u8>> {
        self.validate_qubit(qubit)?;
        Ok(self.entries.remove(&qubit))
    }

    /// Deterministically iterates over corrections.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = SparseCorrectionEntry> + '_ {
        self.entries.iter().map(|(&qubit, &operation)| {
            SparseCorrectionEntry { qubit, operation }
        })
    }

    /// Composes another correction modulo global phase.
    ///
    /// Pauli composition is represented by XOR of the binary X/Z bits.
    pub fn compose(
        &mut self,
        other: &Self,
    ) -> SparseResult<()> {
        if self.num_qubits != other.num_qubits {
            return Err(SparseError::DimensionMismatch {
                left: self.num_qubits,
                right: other.num_qubits,
                domain: "correction",
            });
        }

        for entry in other.iter() {
            let current =
                self.entries.get(&entry.qubit).copied().unwrap_or(0);

            let composed = compose_pauli_codes(
                current,
                entry.operation,
            );

            if composed == 0 {
                self.entries.remove(&entry.qubit);
            } else {
                self.entries.insert(
                    entry.qubit,
                    composed,
                );
            }
        }

        Ok(())
    }

    /// Validates invariants.
    pub fn validate(&self) -> SparseResult<()> {
        if self.num_qubits == 0 {
            return Err(SparseError::InvalidQubitCount {
                qubits: 0,
            });
        }

        for (&qubit, &operation) in &self.entries {
            self.validate_qubit(qubit)?;

            if operation == 0 || operation > 3 {
                return Err(SparseError::InvalidInvariant {
                    message: "correction contains invalid operation code",
                });
            }
        }

        Ok(())
    }

    /// Conservative memory estimate.
    pub fn estimated_memory_bytes(&self) -> SparseResult<u64> {
        let entries = usize_to_u64(
            self.entries.len(),
            LimitKind::MemoryBytes,
        )?;

        checked_mul(
            entries,
            ESTIMATED_CORRECTION_BYTES,
            LimitKind::MemoryBytes,
        )
    }

    /// Canonical QEC preflight.
    pub fn preflight(
        &self,
        limits: &QecLimits,
    ) -> SparseResult<()> {
        limits.validate().map_err(map_limit_error)?;
        self.validate()?;

        if self.num_qubits > limits.max_qubits {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::Qubits,
                requested: self.num_qubits as u128,
                maximum: limits.max_qubits as u128,
            });
        }

        if self.len() > limits.max_logical_operator_weight {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::LogicalOperatorWeight,
                requested: self.len() as u128,
                maximum: limits.max_logical_operator_weight as u128,
            });
        }

        let memory = self.estimated_memory_bytes()?;

        if memory > limits.max_memory_bytes {
            return Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::MemoryBytes,
                requested: u128::from(memory),
                maximum: u128::from(limits.max_memory_bytes),
            });
        }

        Ok(())
    }

    fn validate_qubit(
        &self,
        qubit: usize,
    ) -> SparseResult<()> {
        if qubit >= self.num_qubits {
            return Err(SparseError::IndexOutOfRange {
                index: qubit,
                upper_bound: self.num_qubits,
                domain: "correction qubit",
            });
        }

        Ok(())
    }
}

/* ========================================================================== */
/* Pauli-code helpers                                                         */
/* ========================================================================== */

/// Composes compact Pauli codes modulo global phase.
///
/// ```text
/// 0 = I
/// 1 = X
/// 2 = Y
/// 3 = Z
/// ```
#[must_use]
pub const fn compose_pauli_codes(
    left: u8,
    right: u8,
) -> u8 {
    let left_x = (left == 1) || (left == 2);
    let left_z = (left == 2) || (left == 3);

    let right_x = (right == 1) || (right == 2);
    let right_z = (right == 2) || (right == 3);

    let x = left_x ^ right_x;
    let z = left_z ^ right_z;

    match (x, z) {
        (false, false) => 0,
        (true, false) => 1,
        (true, true) => 2,
        (false, true) => 3,
    }
}

/* ========================================================================== */
/* Aggregate preflight                                                        */
/* ========================================================================== */

/// Performs preflight for a collection of sparse representations.
///
/// This is useful to higher-level constructors that need to verify the
/// combined resource footprint before allocation.
///
/// The function deliberately does not reserve memory. `memory.rs` owns that
/// responsibility.
pub fn preflight(
    limits: &QecLimits,
    pauli: Option<&SparsePauli>,
    stabilizers: Option<&SparseStabilizerMatrix>,
    graph: Option<&SparseGraph>,
    syndrome: Option<&SparseSyndrome>,
    correction: Option<&SparseCorrection>,
) -> SparseResult<SparseResourceEstimate> {
    limits.validate().map_err(map_limit_error)?;

    let mut estimate = SparseResourceEstimate::new();

    if let Some(value) = pauli {
        value.preflight(limits)?;

        let memory = value.estimated_memory_bytes()?;

        estimate.memory_bytes = checked_add(
            estimate.memory_bytes,
            memory,
            LimitKind::MemoryBytes,
        )?;

        estimate.entries = checked_add(
            estimate.entries,
            usize_to_u64(
                value.support_size(),
                LimitKind::MemoryBytes,
            )?,
            LimitKind::MemoryBytes,
        )?;
    }

    if let Some(value) = stabilizers {
        value.preflight(limits)?;

        let memory = value.estimated_memory_bytes()?;

        estimate.memory_bytes = checked_add(
            estimate.memory_bytes,
            memory,
            LimitKind::MemoryBytes,
        )?;

        estimate.stabilizer_rows = checked_add(
            estimate.stabilizer_rows,
            usize_to_u64(
                value.row_count(),
                LimitKind::Stabilizers,
            )?,
            LimitKind::Stabilizers,
        )?;
    }

    if let Some(value) = graph {
        value.preflight(limits)?;

        let memory = value.estimated_memory_bytes()?;

        estimate.memory_bytes = checked_add(
            estimate.memory_bytes,
            memory,
            LimitKind::MemoryBytes,
        )?;

        estimate.graph_nodes = checked_add(
            estimate.graph_nodes,
            usize_to_u64(
                value.node_count(),
                LimitKind::GraphNodes,
            )?,
            LimitKind::GraphNodes,
        )?;

        estimate.graph_edges = checked_add(
            estimate.graph_edges,
            usize_to_u64(
                value.edge_count(),
                LimitKind::GraphEdges,
            )?,
            LimitKind::GraphEdges,
        )?;
    }

    if let Some(value) = syndrome {
        value.preflight(limits)?;

        let memory = value.estimated_memory_bytes()?;

        estimate.memory_bytes = checked_add(
            estimate.memory_bytes,
            memory,
            LimitKind::MemoryBytes,
        )?;

        estimate.syndrome_events = checked_add(
            estimate.syndrome_events,
            usize_to_u64(
                value.event_count(),
                LimitKind::SyndromeEvents,
            )?,
            LimitKind::SyndromeEvents,
        )?;
    }

    if let Some(value) = correction {
        value.preflight(limits)?;

        let memory = value.estimated_memory_bytes()?;

        estimate.memory_bytes = checked_add(
            estimate.memory_bytes,
            memory,
            LimitKind::MemoryBytes,
        )?;

        estimate.correction_entries = checked_add(
            estimate.correction_entries,
            usize_to_u64(
                value.len(),
                LimitKind::MemoryBytes,
            )?,
            LimitKind::MemoryBytes,
        )?;
    }

    estimate.validate_against(limits)?;

    Ok(estimate)
}

/* ========================================================================== */
/* Tests                                                                      */
/* ========================================================================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pauli_identity_is_sparse() {
        let pauli = SparsePauli::identity(9).unwrap();

        assert!(pauli.is_identity());
        assert_eq!(pauli.weight(), 0);
        assert_eq!(pauli.support_size(), 0);
    }

    #[test]
    fn pauli_support_is_deterministic() {
        let pauli = SparsePauli::from_supports(
            16,
            [7, 2, 12],
            [12, 4],
        )
        .unwrap();

        let support: Vec<_> = pauli.support().collect();

        assert_eq!(support, vec![2, 4, 7, 12]);
    }

    #[test]
    fn pauli_y_is_encoded_by_x_and_z() {
        let pauli =
            SparsePauli::from_supports(4, [2], [2]).unwrap();

        assert_eq!(pauli.component(2), 2);
        assert_eq!(pauli.weight(), 1);
    }

    #[test]
    fn pauli_symplectic_product_detects_anticommutation() {
        let x =
            SparsePauli::from_supports(2, [0], []).unwrap();

        let z =
            SparsePauli::from_supports(2, [], [0]).unwrap();

        assert_eq!(x.symplectic_product(&z).unwrap(), 1);
        assert!(!x.commutes_with(&z).unwrap());
    }

    #[test]
    fn pauli_multiplication_is_xor_based() {
        let x =
            SparsePauli::from_supports(2, [0], []).unwrap();

        let z =
            SparsePauli::from_supports(2, [], [0]).unwrap();

        let y = x.multiply(&z).unwrap();

        assert_eq!(y.component(0), 2);
    }

    #[test]
    fn stabilizer_matrix_is_dimension_safe() {
        let mut matrix =
            SparseStabilizerMatrix::new(5).unwrap();

        let row =
            SparsePauli::from_supports(5, [0], [1]).unwrap();

        matrix.insert(0, row).unwrap();

        assert_eq!(matrix.row_count(), 1);
        assert!(matrix.validate().is_ok());
    }

    #[test]
    fn graph_is_symmetric() {
        let mut graph = SparseGraph::new(4).unwrap();

        assert!(graph.add_edge(0, 2).unwrap());
        assert!(graph.contains_edge(0, 2));
        assert!(graph.contains_edge(2, 0));
        assert_eq!(graph.edge_count(), 1);

        graph.validate().unwrap();
    }

    #[test]
    fn graph_duplicate_edges_are_idempotent() {
        let mut graph = SparseGraph::new(4).unwrap();

        assert!(graph.add_edge(0, 1).unwrap());
        assert!(!graph.add_edge(1, 0).unwrap());

        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn graph_self_edges_are_rejected() {
        let mut graph = SparseGraph::new(4).unwrap();

        assert!(matches!(
            graph.add_edge(1, 1),
            Err(SparseError::SelfEdge { node: 1 })
        ));
    }

    #[test]
    fn syndrome_is_keyed_by_round_and_stabilizer() {
        let mut syndrome =
            SparseSyndrome::new(8, 16).unwrap();

        syndrome
            .insert(SparseSyndromeEvent::new(2, 7, true))
            .unwrap();

        assert_eq!(
            syndrome.get(2, 7).unwrap().value,
            true
        );
        assert_eq!(syndrome.event_count(), 1);
    }

    #[test]
    fn syndrome_duplicate_key_replaces_event() {
        let mut syndrome =
            SparseSyndrome::new(8, 16).unwrap();

        syndrome
            .insert(SparseSyndromeEvent::new(2, 7, true))
            .unwrap();

        syndrome
            .insert(SparseSyndromeEvent::new(2, 7, false))
            .unwrap();

        assert_eq!(syndrome.event_count(), 1);
        assert!(!syndrome.get(2, 7).unwrap().value);
    }

    #[test]
    fn correction_identity_is_not_stored() {
        let mut correction =
            SparseCorrection::new(8).unwrap();

        correction
            .insert(SparseCorrectionEntry::new(3, 1).unwrap())
            .unwrap();

        assert_eq!(correction.len(), 1);

        correction
            .insert(SparseCorrectionEntry::new(3, 0).unwrap())
            .unwrap();

        assert_eq!(correction.len(), 0);
    }

    #[test]
    fn correction_composition_is_deterministic() {
        let mut left =
            SparseCorrection::new(8).unwrap();

        let mut right =
            SparseCorrection::new(8).unwrap();

        left.insert(
            SparseCorrectionEntry::new(2, 1).unwrap(),
        )
        .unwrap();

        right.insert(
            SparseCorrectionEntry::new(2, 3).unwrap(),
        )
        .unwrap();

        left.compose(&right).unwrap();

        assert_eq!(left.get(2), Some(2));
    }

    #[test]
    fn aggregate_preflight_accepts_valid_structures() {
        let limits = QecLimits::new();

        let pauli =
            SparsePauli::from_supports(8, [1, 4], [4]).unwrap();

        let mut graph = SparseGraph::new(8).unwrap();
        graph.add_edge(0, 1).unwrap();
        graph.add_edge(1, 2).unwrap();

        let estimate = preflight(
            &limits,
            Some(&pauli),
            None,
            Some(&graph),
            None,
            None,
        )
        .unwrap();

        assert!(estimate.memory_bytes > 0);
        assert_eq!(estimate.graph_nodes, 8);
        assert_eq!(estimate.graph_edges, 2);
    }

    #[test]
    fn resource_limits_are_enforced() {
        let mut limits = QecLimits::new();
        limits.max_graph_nodes = 2;

        let graph = SparseGraph::new(3).unwrap();

        assert!(matches!(
            graph.preflight(&limits),
            Err(SparseError::ResourceLimitExceeded {
                resource: LimitKind::GraphNodes,
                ..
            })
        ));
    }

    #[test]
    fn invalid_confidence_is_rejected() {
        let event =
            SparseSyndromeEvent::new(0, 0, true)
                .with_confidence(2.0);

        assert!(matches!(
            event,
            Err(SparseError::InvalidValue { .. })
        ));
    }
}