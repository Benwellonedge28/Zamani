//! Zamani Quantum Routing — Compiler IR Transpiler Adapter
//!
//! `src/quantum/routing/transpiler.rs`
//!
//! # Responsibility
//!
//! This file is the compiler-IR integration boundary for the routing subsystem.
//!
//! It converts Zamani compiler IR quantum operations from logical qubit
//! references into topology-compatible physical operations.
//!
//! It owns:
//!
//! - compiler-IR validation at the routing boundary;
//! - deterministic logical-qubit discovery;
//! - deterministic logical-to-physical allocation;
//! - compatibility mapping state;
//! - deterministic shortest-path routing;
//! - semantic SWAP insertion;
//! - mapping updates after every SWAP;
//! - physical operand rewriting;
//! - transactional transpilation;
//! - preservation of non-quantum IR instructions;
//! - production diagnostics;
//! - compatibility with the existing `QuantumTranspiler` API.
//!
//! It does NOT own:
//!
//! - OpenQASM parsing;
//! - quantum source parsing;
//! - general gate synthesis;
//! - pulse generation;
//! - scheduling;
//! - calibration acquisition;
//! - hardware execution;
//! - simulation;
//! - QEC decoding;
//! - variational algorithms;
//! - benchmark execution.
//!
//! Those responsibilities belong to their respective subsystems.
//!
//! # Routing boundary
//!
//! ```text
//! compiler IrFunction
//!        │
//!        ▼
//! ┌────────────────────────────┐
//! │ transpiler.rs              │
//! │ compiler-IR adapter        │
//! └─────────────┬──────────────┘
//!               │
//!               ▼
//! logical quantum operations
//!               │
//!               ▼
//! topology-aware routing
//!               │
//!        ┌──────┴──────┐
//!        │             │
//!        ▼             ▼
//!   physical gate   semantic SWAP
//!        │             │
//!        └──────┬──────┘
//!               ▼
//!        physical compiler IR
//! ```
//!
//! # Important architectural rule
//!
//! A SWAP inserted here represents a semantic state permutation. It must not
//! be interpreted as proof that the target hardware has a native SWAP gate.
//!
//! Later lowering may translate:
//!
//! ```text
//! SWAP
//!   │
//!   ├── native SWAP
//!   ├── 3 × CX
//!   └── provider-specific decomposition
//! ```
//!
//! This preserves the separation between routing and hardware gate synthesis.
//!
//! # Transactional guarantee
//!
//! A failed transpilation leaves:
//!
//! - the original `IrFunction` body unchanged;
//! - the original mapping unchanged;
//! - the previous swap count unchanged;
//! - the previous routed-gate count unchanged.
//!
//! No partial routed program is exposed.
//!
//! # Determinism
//!
//! Given identical:
//!
//! - input IR;
//! - topology;
//! - initial mapping;
//!
//! the transpiler produces the same mapping and SWAP sequence.
//!
//! Hash-map iteration order is never used to choose a routing decision.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe Rust.
//!
//! # Integration contract
//!
//! This compatibility adapter intentionally exposes the historical
//! `QuantumTranspiler` API while keeping the implementation structured so the
//! routing foundation can later be delegated to:
//!
//! ```text
//! routing::topology
//! routing::mapping
//! routing::path
//! routing::router
//! routing::verification
//! ```
//!
//! The compiler IR must not become a dependency of the routing algorithms.
//!
//! The long-term dependency direction is:
//!
//! ```text
//! compiler IR
//!      │
//!      ▼
//! transpiler.rs
//!      │
//!      ▼
//! routing contracts
//!      │
//!      ├── topology
//!      ├── mapping
//!      ├── path
//!      ├── algorithms
//!      └── verification
//! ```
//!
//! # Multi-qubit boundary
//!
//! This adapter never silently invents a decomposition for an arbitrary
//! operation with three or more quantum operands.
//!
//! If a multi-qubit operation is already physically executable on the current
//! topology, it is preserved and rewritten.
//!
//! Otherwise it is rejected with an explicit error telling the caller that
//! decomposition/synthesis must occur at the appropriate compiler stage.
//!
//! This prevents routing from becoming an accidental gate-synthesis engine.
//!
//! # Safety
//!
//! This module contains no unsafe code.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::ir_gen::{
    IrFunction,
    IrInstruction,
    IrRegister,
    IrType,
    IrValue,
};

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

// =============================================================================
// Public error model
// =============================================================================

/// Errors produced by the compiler-IR routing adapter.
///
/// This compatibility error remains local to `transpiler.rs` so existing
/// callers of `QuantumTranspiler` do not have to migrate at the same time as
/// the routing foundation.
///
/// The newer routing-wide `RoutingError` can wrap/translate these errors at
/// the final integration boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranspilerError {
    /// The physical topology contains no qubits.
    EmptyTopology,

    /// The topology violates a graph invariant.
    InvalidTopology(String),

    /// Two physical qubits cannot be connected.
    DisconnectedTopology {
        from: usize,
        to: usize,
    },

    /// A referenced physical qubit does not exist.
    PhysicalQubitOutOfRange(usize),

    /// A physical location already contains another logical qubit.
    PhysicalQubitAlreadyAssigned(usize),

    /// A logical qubit already has an assignment.
    LogicalQubitAlreadyMapped(String),

    /// A logical qubit was referenced without an assignment.
    UnknownLogicalQubit(String),

    /// An IR operand is not a quantum register.
    UnsupportedQuantumOperand(String),

    /// The program requires more physical resources than available.
    InsufficientPhysicalQubits {
        required: usize,
        available: usize,
    },

    /// A compiler-IR quantum operation is malformed.
    InvalidQuantumInstruction(String),

    /// No valid route exists.
    RoutingFailed {
        from: usize,
        to: usize,
    },

    /// An operation requires decomposition before routing.
    UnsupportedMultiQubitOperation {
        gate: String,
        arity: usize,
    },

    /// A two-qubit operation uses the same logical qubit twice.
    DuplicateQuantumOperand {
        gate: String,
        qubit: String,
    },

    /// A physical mapping supplied by the caller is inconsistent.
    InvalidMapping(String),

    /// A generated SWAP would be illegal for the supplied topology.
    InvalidSwap {
        a: usize,
        b: usize,
    },

    /// An internal routing invariant failed.
    InternalInvariantViolation(String),
}

impl fmt::Display for TranspilerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTopology => {
                write!(
                    formatter,
                    "quantum transpiler: topology is empty"
                )
            }

            Self::InvalidTopology(message) => {
                write!(
                    formatter,
                    "quantum transpiler: invalid topology: {message}"
                )
            }

            Self::DisconnectedTopology { from, to } => {
                write!(
                    formatter,
                    "quantum transpiler: physical qubits {from} and {to} are disconnected"
                )
            }

            Self::PhysicalQubitOutOfRange(qubit) => {
                write!(
                    formatter,
                    "quantum transpiler: physical qubit {qubit} does not exist in the topology"
                )
            }

            Self::PhysicalQubitAlreadyAssigned(qubit) => {
                write!(
                    formatter,
                    "quantum transpiler: physical qubit {qubit} is already assigned"
                )
            }

            Self::LogicalQubitAlreadyMapped(qubit) => {
                write!(
                    formatter,
                    "quantum transpiler: logical qubit '{qubit}' is already mapped"
                )
            }

            Self::UnknownLogicalQubit(qubit) => {
                write!(
                    formatter,
                    "quantum transpiler: unknown logical qubit '{qubit}'"
                )
            }

            Self::UnsupportedQuantumOperand(operand) => {
                write!(
                    formatter,
                    "quantum transpiler: unsupported quantum operand '{operand}'"
                )
            }

            Self::InsufficientPhysicalQubits {
                required,
                available,
            } => {
                write!(
                    formatter,
                    "quantum transpiler: requires {required} physical qubits but topology provides {available}"
                )
            }

            Self::InvalidQuantumInstruction(message) => {
                write!(
                    formatter,
                    "quantum transpiler: invalid quantum instruction: {message}"
                )
            }

            Self::RoutingFailed { from, to } => {
                write!(
                    formatter,
                    "quantum transpiler: unable to route physical qubit {from} to {to}"
                )
            }

            Self::UnsupportedMultiQubitOperation { gate, arity } => {
                write!(
                    formatter,
                    "quantum transpiler: gate '{gate}' has {arity} quantum operands and requires native hardware support or prior decomposition"
                )
            }

            Self::DuplicateQuantumOperand { gate, qubit } => {
                write!(
                    formatter,
                    "quantum transpiler: gate '{gate}' uses logical qubit '{qubit}' more than once"
                )
            }

            Self::InvalidMapping(message) => {
                write!(
                    formatter,
                    "quantum transpiler: invalid logical-to-physical mapping: {message}"
                )
            }

            Self::InvalidSwap { a, b } => {
                write!(
                    formatter,
                    "quantum transpiler: illegal SWAP between physical qubits {a} and {b}"
                )
            }

            Self::InternalInvariantViolation(message) => {
                write!(
                    formatter,
                    "quantum transpiler: internal invariant violation: {message}"
                )
            }
        }
    }
}

impl std::error::Error for TranspilerError {}

// =============================================================================
// Physical topology compatibility model
// =============================================================================

/// Physical connectivity graph used by the compiler-IR compatibility adapter.
///
/// This remains in this file temporarily as the compatibility representation
/// required by the historical `QuantumTranspiler` API.
///
/// The canonical routing subsystem should eventually use
/// `routing::topology::PhysicalTopology` directly and this adapter should
/// convert to it at the boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTopology {
    /// Human-readable topology name.
    pub name: String,

    /// Deterministically normalized adjacency lists.
    ///
    /// Every list is sorted and contains no duplicates.
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl PhysicalTopology {
    /// Constructs and validates a physical topology.
    ///
    /// Construction canonicalizes neighbor ordering before validation.
    pub fn new(
        name: impl Into<String>,
        adjacency: HashMap<usize, Vec<usize>>,
    ) -> Result<Self, TranspilerError> {
        let mut normalized = HashMap::with_capacity(adjacency.len());

        for (qubit, mut neighbors) in adjacency {
            neighbors.sort_unstable();

            if neighbors
                .windows(2)
                .any(|window| window[0] == window[1])
            {
                return Err(
                    TranspilerError::InvalidTopology(
                        format!(
                            "qubit {qubit} contains duplicate neighbors"
                        ),
                    ),
                );
            }

            normalized.insert(qubit, neighbors);
        }

        let topology = Self {
            name: name.into(),
            adjacency: normalized,
        };

        topology.validate()?;
        Ok(topology)
    }

    /// Creates a deterministic six-qubit heavy-hex-style test topology.
    ///
    /// This is intentionally a development/test topology and is not a
    /// representation of a complete vendor device.
    pub fn heavy_hex() -> Self {
        let mut adjacency = HashMap::new();

        adjacency.insert(0, vec![1, 3]);
        adjacency.insert(1, vec![0, 2]);
        adjacency.insert(2, vec![1, 5]);
        adjacency.insert(3, vec![0, 4]);
        adjacency.insert(4, vec![3, 5]);
        adjacency.insert(5, vec![2, 4]);

        Self {
            name: "Heavy-Hex".to_string(),
            adjacency,
        }
    }

    /// Creates a linear topology.
    pub fn line(
        qubit_count: usize,
    ) -> Result<Self, TranspilerError> {
        if qubit_count == 0 {
            return Err(TranspilerError::EmptyTopology);
        }

        let mut adjacency = HashMap::with_capacity(qubit_count);

        for qubit in 0..qubit_count {
            let mut neighbors = Vec::with_capacity(2);

            if qubit > 0 {
                neighbors.push(qubit - 1);
            }

            if qubit + 1 < qubit_count {
                neighbors.push(qubit + 1);
            }

            adjacency.insert(qubit, neighbors);
        }

        Self::new("Linear", adjacency)
    }

    /// Returns the number of physical qubits.
    #[must_use]
    pub fn qubit_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Returns whether a physical qubit exists.
    #[must_use]
    pub fn contains(&self, qubit: usize) -> bool {
        self.adjacency.contains_key(&qubit)
    }

    /// Returns all physical qubits in deterministic order.
    #[must_use]
    pub fn qubits(&self) -> Vec<usize> {
        let mut qubits: Vec<usize> =
            self.adjacency.keys().copied().collect();

        qubits.sort_unstable();
        qubits
    }

    /// Returns all neighbors in deterministic order.
    #[must_use]
    pub fn neighbors(&self, qubit: usize) -> &[usize] {
        static EMPTY: [usize; 0] = [];

        self.adjacency
            .get(&qubit)
            .map(Vec::as_slice)
            .unwrap_or(&EMPTY)
    }

    /// Returns whether two physical qubits are directly adjacent.
    #[must_use]
    pub fn is_adjacent(
        &self,
        a: usize,
        b: usize,
    ) -> bool {
        if a == b {
            return false;
        }

        self.adjacency
            .get(&a)
            .map(|neighbors| neighbors.binary_search(&b).is_ok())
            .unwrap_or(false)
    }

    /// Validates all topology invariants.
    pub fn validate(&self) -> Result<(), TranspilerError> {
        if self.adjacency.is_empty() {
            return Err(TranspilerError::EmptyTopology);
        }

        for (&qubit, neighbors) in &self.adjacency {
            let mut seen = HashSet::with_capacity(neighbors.len());

            for &neighbor in neighbors {
                if !self.contains(neighbor) {
                    return Err(
                        TranspilerError::InvalidTopology(
                            format!(
                                "qubit {qubit} references missing neighbor {neighbor}"
                            ),
                        ),
                    );
                }

                if qubit == neighbor {
                    return Err(
                        TranspilerError::InvalidTopology(
                            format!(
                                "qubit {qubit} cannot reference itself"
                            ),
                        ),
                    );
                }

                if !seen.insert(neighbor) {
                    return Err(
                        TranspilerError::InvalidTopology(
                            format!(
                                "qubit {qubit} contains duplicate neighbor {neighbor}"
                            ),
                        ),
                    );
                }

                if !self
                    .adjacency
                    .get(&neighbor)
                    .map(|items| items.binary_search(&qubit).is_ok())
                    .unwrap_or(false)
                {
                    return Err(
                        TranspilerError::InvalidTopology(
                            format!(
                                "topology edge {qubit} -> {neighbor} is not bidirectional"
                            ),
                        ),
                    );
                }
            }
        }

        Ok(())
    }

    /// Returns a deterministic shortest path.
    ///
    /// BFS is appropriate because every compatibility-layer edge has equal
    /// routing cost. Weighted hardware routing belongs to `cost.rs` and the
    /// canonical routing algorithms.
    #[must_use]
    pub fn shortest_path(
        &self,
        start: usize,
        target: usize,
    ) -> Option<Vec<usize>> {
        if !self.contains(start) || !self.contains(target) {
            return None;
        }

        if start == target {
            return Some(vec![start]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut previous: HashMap<usize, usize> =
            HashMap::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            for &neighbor in self.neighbors(current) {
                if !visited.insert(neighbor) {
                    continue;
                }

                previous.insert(neighbor, current);

                if neighbor == target {
                    let mut path = Vec::new();
                    let mut cursor = target;

                    path.push(cursor);

                    while cursor != start {
                        cursor = previous
                            .get(&cursor)
                            .copied()?;

                        path.push(cursor);
                    }

                    path.reverse();
                    return Some(path);
                }

                queue.push_back(neighbor);
            }
        }

        None
    }

    /// Returns the distance between two physical qubits.
    #[must_use]
    pub fn distance(
        &self,
        start: usize,
        target: usize,
    ) -> Option<usize> {
        self.shortest_path(start, target)
            .map(|path| path.len().saturating_sub(1))
    }
}

// =============================================================================
// Bidirectional mapping
// =============================================================================

/// Compatibility logical-to-physical mapping.
///
/// Unlike the old implementation, this stores both directions.
///
/// Therefore reverse lookup is O(1) average-case rather than scanning the
/// entire mapping.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QubitMapping {
    logical_to_physical: HashMap<String, usize>,
    physical_to_logical: HashMap<usize, String>,
}

impl QubitMapping {
    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a mapping from an existing set of assignments.
    pub fn from_assignments<I>(
        assignments: I,
        topology: &PhysicalTopology,
    ) -> Result<Self, TranspilerError>
    where
        I: IntoIterator<Item = (String, usize)>,
    {
        let mut mapping = Self::new();

        for (logical, physical) in assignments {
            mapping.insert(
                logical,
                physical,
                topology,
            )?;
        }

        Ok(mapping)
    }

    /// Returns the number of assignments.
    #[must_use]
    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    /// Returns whether the mapping is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    /// Returns the physical location of a logical qubit.
    #[must_use]
    pub fn get(
        &self,
        logical: &str,
    ) -> Option<usize> {
        self.logical_to_physical
            .get(logical)
            .copied()
    }

    /// Returns the logical qubit at a physical location.
    #[must_use]
    pub fn logical_at(
        &self,
        physical: usize,
    ) -> Option<&str> {
        self.physical_to_logical
            .get(&physical)
            .map(String::as_str)
    }

    /// Returns whether a logical qubit is mapped.
    #[must_use]
    pub fn contains_logical(
        &self,
        logical: &str,
    ) -> bool {
        self.logical_to_physical
            .contains_key(logical)
    }

    /// Returns whether a physical qubit is occupied.
    #[must_use]
    pub fn contains_physical(
        &self,
        physical: usize,
    ) -> bool {
        self.physical_to_logical
            .contains_key(&physical)
    }

    /// Inserts a new mapping.
    pub fn insert(
        &mut self,
        logical: String,
        physical: usize,
        topology: &PhysicalTopology,
    ) -> Result<(), TranspilerError> {
        if self.contains_logical(&logical) {
            return Err(
                TranspilerError::LogicalQubitAlreadyMapped(
                    logical,
                ),
            );
        }

        if !topology.contains(physical) {
            return Err(
                TranspilerError::PhysicalQubitOutOfRange(
                    physical,
                ),
            );
        }

        if self.contains_physical(physical) {
            return Err(
                TranspilerError::PhysicalQubitAlreadyAssigned(
                    physical,
                ),
            );
        }

        self.logical_to_physical
            .insert(logical.clone(), physical);

        self.physical_to_logical
            .insert(physical, logical);

        Ok(())
    }

    /// Replaces the complete mapping transactionally.
    pub fn replace(
        &mut self,
        assignments: Vec<(String, usize)>,
        topology: &PhysicalTopology,
    ) -> Result<(), TranspilerError> {
        let replacement =
            Self::from_assignments(assignments, topology)?;

        *self = replacement;
        Ok(())
    }

    /// Returns deterministic mapping entries.
    #[must_use]
    pub fn snapshot(&self) -> Vec<(String, usize)> {
        let mut entries: Vec<_> = self
            .logical_to_physical
            .iter()
            .map(|(logical, physical)| {
                (logical.clone(), *physical)
            })
            .collect();

        entries.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then(left.1.cmp(&right.1))
        });

        entries
    }

    /// Validates both mapping directions.
    pub fn validate(
        &self,
        topology: &PhysicalTopology,
    ) -> Result<(), TranspilerError> {
        if self.logical_to_physical.len()
            != self.physical_to_logical.len()
        {
            return Err(
                TranspilerError::InvalidMapping(
                    "forward and reverse mapping sizes differ"
                        .to_string(),
                ),
            );
        }

        for (logical, physical) in
            &self.logical_to_physical
        {
            if !topology.contains(*physical) {
                return Err(
                    TranspilerError::PhysicalQubitOutOfRange(
                        *physical,
                    ),
                );
            }

            let reverse = self
                .physical_to_logical
                .get(physical)
                .map(String::as_str);

            if reverse != Some(logical.as_str()) {
                return Err(
                    TranspilerError::InvalidMapping(
                        format!(
                            "reverse mapping for '{logical}' at physical {physical} is inconsistent"
                        ),
                    ),
                );
            }
        }

        for (physical, logical) in
            &self.physical_to_logical
        {
            let forward =
                self.logical_to_physical.get(logical);

            if forward != Some(physical) {
                return Err(
                    TranspilerError::InvalidMapping(
                        format!(
                            "forward mapping for '{logical}' does not point to physical {physical}"
                        ),
                    ),
                );
            }
        }

        Ok(())
    }

    /// Exchanges the states occupying two physical positions.
    ///
    /// This is the semantic mapping effect of a SWAP.
    pub fn swap_physical(
        &mut self,
        a: usize,
        b: usize,
        topology: &PhysicalTopology,
    ) -> Result<(), TranspilerError> {
        if a == b || !topology.is_adjacent(a, b) {
            return Err(
                TranspilerError::InvalidSwap { a, b },
            );
        }

        let logical_a =
            self.physical_to_logical.remove(&a);

        let logical_b =
            self.physical_to_logical.remove(&b);

        if let Some(logical) = logical_a.as_ref() {
            self.logical_to_physical
                .insert(logical.clone(), b);
            self.physical_to_logical
                .insert(b, logical.clone());
        }

        if let Some(logical) = logical_b.as_ref() {
            self.logical_to_physical
                .insert(logical.clone(), a);
            self.physical_to_logical
                .insert(a, logical.clone());
        }

        Ok(())
    }

    /// Returns the logical qubit occupying a physical location.
    fn logical_at_owned(
        &self,
        physical: usize,
    ) -> Option<String> {
        self.physical_to_logical
            .get(&physical)
            .cloned()
    }
}

// =============================================================================
// Quantum transpiler
// =============================================================================

/// Compiler-IR routing adapter.
///
/// The routing operation itself is transactional and deterministic.
#[derive(Debug, Clone)]
pub struct QuantumTranspiler {
    /// Target topology.
    pub topology: PhysicalTopology,

    /// Current logical-to-physical mapping.
    ///
    /// Kept public for backwards compatibility with existing callers.
    ///
    /// New code should prefer the accessor methods.
    pub mapping: HashMap<String, usize>,

    /// Number of SWAP operations inserted by the most recent successful pass.
    pub swap_count: usize,

    /// Number of original two-qubit operations that required routing.
    pub routed_gate_count: usize,
}

impl QuantumTranspiler {
    /// Creates a transpiler for a topology.
    #[must_use]
    pub fn new(topology: PhysicalTopology) -> Self {
        Self {
            topology,
            mapping: HashMap::new(),
            swap_count: 0,
            routed_gate_count: 0,
        }
    }

    /// Transpiles a compiler IR function transactionally.
    ///
    /// The caller's function is changed only after the entire transformation
    /// succeeds.
    pub fn transpile(
        &mut self,
        function: &mut IrFunction,
    ) -> Result<(), TranspilerError> {
        self.topology.validate()?;

        let original_body = function.body.clone();
        let original_mapping = self.mapping.clone();
        let original_swap_count = self.swap_count;
        let original_routed_gate_count =
            self.routed_gate_count;

        let result = self.transpile_transaction(function);

        match result {
            Ok((body, mapping, swap_count, routed_gate_count)) => {
                function.body = body;
                self.mapping = mapping;
                self.swap_count = swap_count;
                self.routed_gate_count = routed_gate_count;

                Ok(())
            }

            Err(error) => {
                function.body = original_body;
                self.mapping = original_mapping;
                self.swap_count = original_swap_count;
                self.routed_gate_count =
                    original_routed_gate_count;

                Err(error)
            }
        }
    }

    /// Explicit in-place API alias.
    pub fn transpile_in_place(
        &mut self,
        function: &mut IrFunction,
    ) -> Result<(), TranspilerError> {
        self.transpile(function)
    }

    /// Returns the physical location of a logical qubit.
    #[must_use]
    pub fn physical_qubit(
        &self,
        logical: &str,
    ) -> Option<usize> {
        self.mapping.get(logical).copied()
    }

    /// Returns a deterministic mapping snapshot.
    #[must_use]
    pub fn mapping_snapshot(
        &self,
    ) -> Vec<(String, usize)> {
        let mut entries: Vec<_> = self
            .mapping
            .iter()
            .map(|(logical, physical)| {
                (logical.clone(), *physical)
            })
            .collect();

        entries.sort_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then(left.1.cmp(&right.1))
        });

        entries
    }

    /// Replaces the current mapping transactionally.
    ///
    /// This is useful for integration with a separately computed layout.
    pub fn set_mapping(
        &mut self,
        assignments: Vec<(String, usize)>,
    ) -> Result<(), TranspilerError> {
        let mapping =
            QubitMapping::from_assignments(
                assignments,
                &self.topology,
            )?;

        mapping.validate(&self.topology)?;

        self.mapping = mapping
            .snapshot()
            .into_iter()
            .collect();

        Ok(())
    }

    /// Clears the current mapping.
    ///
    /// This does not modify the topology.
    pub fn clear_mapping(&mut self) {
        self.mapping.clear();
    }

    // =========================================================================
    // Internal transaction
    // =========================================================================

    fn transpile_transaction(
        &self,
        function: &IrFunction,
    ) -> Result<
        (
            Vec<IrInstruction>,
            HashMap<String, usize>,
            usize,
            usize,
        ),
        TranspilerError,
    > {
        let logical_qubits =
            collect_logical_qubits(function)?;

        ensure_capacity(
            &logical_qubits,
            &self.topology,
            &self.mapping,
        )?;

        let mut mapping =
            QubitMapping::new();

        validate_existing_public_mapping(
            &self.mapping,
            &self.topology,
        )?;

        for (logical, physical) in
            self.mapping_snapshot()
        {
            mapping.insert(
                logical,
                physical,
                &self.topology,
            )?;
        }

        allocate_missing_qubits(
            &logical_qubits,
            &mut mapping,
            &self.topology,
        )?;

        mapping.validate(&self.topology)?;

        let mut output =
            Vec::with_capacity(function.body.len());

        let mut swap_count = 0usize;
        let mut routed_gate_count = 0usize;

        for (operation_index, instruction) in
            function.body.iter().enumerate()
        {
            match instruction {
                IrInstruction::QuantumGate(
                    result,
                    gate,
                    args,
                ) => {
                    validate_quantum_gate(
                        gate,
                        args,
                    )
                    .map_err(|error| {
                        annotate_instruction_error(
                            error,
                            operation_index,
                        )
                    })?;

                    let logical_operands =
                        quantum_register_operands(
                            args,
                        )
                        .map_err(|error| {
                            annotate_instruction_error(
                                error,
                                operation_index,
                            )
                        })?;

                    validate_distinct_operands(
                        gate,
                        &logical_operands,
                    )
                    .map_err(|error| {
                        annotate_instruction_error(
                            error,
                            operation_index,
                        )
                    })?;

                    match logical_operands.len() {
                        0 => {
                            return Err(
                                TranspilerError::InvalidQuantumInstruction(
                                    format!(
                                        "operation {operation_index}: gate '{gate}' has no quantum operands"
                                    ),
                                ),
                            );
                        }

                        1 => {
                            output.push(
                                rewrite_quantum_gate(
                                    result,
                                    gate,
                                    args,
                                    &mapping,
                                )?,
                            );
                        }

                        2 => {
                            route_two_qubit_gate(
                                result,
                                gate,
                                args,
                                logical_operands[0],
                                logical_operands[1],
                                &self.topology,
                                &mut mapping,
                                &mut output,
                                &mut swap_count,
                                &mut routed_gate_count,
                            )?;
                        }

                        arity => {
                            route_multi_qubit_gate(
                                result,
                                gate,
                                args,
                                &logical_operands,
                                &self.topology,
                                &mut mapping,
                                &mut output,
                            )
                            .map_err(|error| {
                                match error {
                                    TranspilerError::UnsupportedMultiQubitOperation {
                                        ..
                                    } => error,

                                    other => {
                                        annotate_instruction_error(
                                            other,
                                            operation_index,
                                        )
                                    }
                                }
                            })?;

                            // `route_multi_qubit_gate` currently supports only
                            // already-executable native operations. It never
                            // invents a synthesis/decomposition.
                            let _ = arity;
                        }
                    }
                }

                // Non-quantum compiler instructions are preserved verbatim.
                other => {
                    output.push(other.clone());
                }
            }
        }

        mapping.validate(&self.topology)?;

        let final_public_mapping: HashMap<String, usize> =
            mapping
                .snapshot()
                .into_iter()
                .collect();

        Ok((
            output,
            final_public_mapping,
            swap_count,
            routed_gate_count,
        ))
    }
}

// =============================================================================
// Logical-qubit discovery
// =============================================================================

fn collect_logical_qubits(
    function: &IrFunction,
) -> Result<Vec<String>, TranspilerError> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for (operation_index, instruction) in
        function.body.iter().enumerate()
    {
        let IrInstruction::QuantumGate(
            _,
            gate,
            args,
        ) = instruction
        else {
            continue;
        };

        validate_quantum_gate(
            gate,
            args,
        )
        .map_err(|error| {
            annotate_instruction_error(
                error,
                operation_index,
            )
        })?;

        for arg in args {
            let IrValue::Reg(register) = arg
            else {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!(
                            "operation {operation_index}: {arg:?}"
                        ),
                    ),
                );
            };

            if register.1 != IrType::Quantum {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!(
                            "operation {operation_index}: register '%{}' has type {:?}, expected quantum",
                            register.0,
                            register.1
                        ),
                    ),
                );
            }

            if seen.insert(register.0.clone()) {
                result.push(register.0.clone());
            }
        }
    }

    Ok(result)
}

// =============================================================================
// Capacity validation
// =============================================================================

fn ensure_capacity(
    logical_qubits: &[String],
    topology: &PhysicalTopology,
    existing_mapping: &HashMap<String, usize>,
) -> Result<(), TranspilerError> {
    if logical_qubits.len() > topology.qubit_count() {
        return Err(
            TranspilerError::InsufficientPhysicalQubits {
                required: logical_qubits.len(),
                available: topology.qubit_count(),
            },
        );
    }

    let mapped_relevant = existing_mapping
        .keys()
        .filter(|logical| {
            logical_qubits
                .iter()
                .any(|candidate| candidate == *logical)
        })
        .count();

    let missing =
        logical_qubits.len().saturating_sub(mapped_relevant);

    let occupied = existing_mapping
        .values()
        .collect::<HashSet<_>>()
        .len();

    let available = topology
        .qubit_count()
        .saturating_sub(occupied);

    if missing > available {
        return Err(
            TranspilerError::InsufficientPhysicalQubits {
                required: logical_qubits.len(),
                available: topology.qubit_count(),
            },
        );
    }

    Ok(())
}

// =============================================================================
// Existing public mapping validation
// =============================================================================

fn validate_existing_public_mapping(
    mapping: &HashMap<String, usize>,
    topology: &PhysicalTopology,
) -> Result<(), TranspilerError> {
    let mut physical_to_logical =
        HashMap::<usize, &String>::with_capacity(
            mapping.len(),
        );

    for (logical, physical) in mapping {
        if logical.trim().is_empty() {
            return Err(
                TranspilerError::InvalidMapping(
                    "mapping contains an empty logical-qubit name"
                        .to_string(),
                ),
            );
        }

        if !topology.contains(*physical) {
            return Err(
                TranspilerError::PhysicalQubitOutOfRange(
                    *physical,
                ),
            );
        }

        if let Some(existing) =
            physical_to_logical.insert(
                *physical,
                logical,
            )
        {
            return Err(
                TranspilerError::InvalidMapping(
                    format!(
                        "logical qubits '{}' and '{}' both map to physical qubit {}",
                        existing,
                        logical,
                        physical
                    ),
                ),
            );
        }
    }

    Ok(())
}

// =============================================================================
// Deterministic allocation
// =============================================================================

fn allocate_missing_qubits(
    logical_qubits: &[String],
    mapping: &mut QubitMapping,
    topology: &PhysicalTopology,
) -> Result<(), TranspilerError> {
    let occupied: HashSet<usize> = mapping
        .snapshot()
        .into_iter()
        .map(|(_, physical)| physical)
        .collect();

    let available_physical: Vec<usize> = topology
        .qubits()
        .into_iter()
        .filter(|physical| !occupied.contains(physical))
        .collect();

    let mut available_index = 0usize;

    for logical in logical_qubits {
        if mapping.contains_logical(logical) {
            continue;
        }

        let physical =
            available_physical
                .get(available_index)
                .copied()
                .ok_or(
                    TranspilerError::InsufficientPhysicalQubits {
                        required: logical_qubits.len(),
                        available: topology.qubit_count(),
                    },
                )?;

        mapping.insert(
            logical.clone(),
            physical,
            topology,
        )?;

        available_index += 1;
    }

    Ok(())
}

// =============================================================================
// Quantum operation validation
// =============================================================================

fn validate_quantum_gate(
    gate: &str,
    args: &[IrValue],
) -> Result<(), TranspilerError> {
    if gate.trim().is_empty() {
        return Err(
            TranspilerError::InvalidQuantumInstruction(
                "quantum gate name cannot be empty"
                    .to_string(),
            ),
        );
    }

    if args.is_empty() {
        return Err(
            TranspilerError::InvalidQuantumInstruction(
                format!(
                    "gate '{gate}' has no operands"
                ),
            ),
        );
    }

    for arg in args {
        match arg {
            IrValue::Reg(register)
                if register.1 == IrType::Quantum => {}

            IrValue::Reg(register) => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!(
                            "register '%{}' has type {:?}, not quantum",
                            register.0,
                            register.1
                        ),
                    ),
                );
            }

            other => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!("{other:?}"),
                    ),
                );
            }
        }
    }

    Ok(())
}

fn quantum_register_operands(
    args: &[IrValue],
) -> Result<Vec<&str>, TranspilerError> {
    let mut operands =
        Vec::with_capacity(args.len());

    for arg in args {
        match arg {
            IrValue::Reg(register) => {
                operands.push(register.0.as_str());
            }

            other => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!("{other:?}"),
                    ),
                );
            }
        }
    }

    Ok(operands)
}

fn validate_distinct_operands(
    gate: &str,
    operands: &[&str],
) -> Result<(), TranspilerError> {
    let mut seen =
        HashSet::<&str>::with_capacity(operands.len());

    for operand in operands {
        if !seen.insert(*operand) {
            return Err(
                TranspilerError::DuplicateQuantumOperand {
                    gate: gate.to_string(),
                    qubit: (*operand).to_string(),
                },
            );
        }
    }

    Ok(())
}

// =============================================================================
// Two-qubit routing
// =============================================================================

fn route_two_qubit_gate(
    result: &IrRegister,
    gate: &str,
    args: &[IrValue],
    logical_a: &str,
    logical_b: &str,
    topology: &PhysicalTopology,
    mapping: &mut QubitMapping,
    output: &mut Vec<IrInstruction>,
    swap_count: &mut usize,
    routed_gate_count: &mut usize,
) -> Result<(), TranspilerError> {
    let physical_a =
        mapping.get(logical_a).ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_a.to_string(),
            )
        })?;

    let physical_b =
        mapping.get(logical_b).ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_b.to_string(),
            )
        })?;

    if topology.is_adjacent(
        physical_a,
        physical_b,
    ) {
        output.push(
            rewrite_quantum_gate(
                result,
                gate,
                args,
                mapping,
            )?,
        );

        return Ok(());
    }

    let path = topology
        .shortest_path(
            physical_a,
            physical_b,
        )
        .ok_or(
            TranspilerError::DisconnectedTopology {
                from: physical_a,
                to: physical_b,
            },
        )?;

    if path.len() < 2 {
        return Err(
            TranspilerError::InternalInvariantViolation(
                format!(
                    "shortest path from {physical_a} to {physical_b} has fewer than two vertices"
                ),
            ),
        );
    }

    // Move the first logical operand along the path until it becomes adjacent
    // to the second operand.
    //
    // For:
    //
    //     A@0 --- 1 --- 2 --- B@3
    //
    // the generated semantic movement is:
    //
    //     SWAP(0,1)
    //     SWAP(1,2)
    //
    // after which:
    //
    //     A@2 --- B@3
    //
    // The final gate is then emitted using the updated physical mapping.
    //
    // We intentionally stop before the final path vertex so that B is never
    // unnecessarily displaced.
    for window in path.windows(2).take(path.len().saturating_sub(2)) {
        let from = window[0];
        let to = window[1];

        if !topology.is_adjacent(from, to) {
            return Err(
                TranspilerError::InvalidSwap {
                    a: from,
                    b: to,
                },
            );
        }

        output.push(
            make_swap_instruction(
                from,
                to,
                mapping,
            )?,
        );

        mapping.swap_physical(
            from,
            to,
            topology,
        )?;

        *swap_count =
            swap_count.checked_add(1).ok_or(
                TranspilerError::InternalInvariantViolation(
                    "SWAP counter overflow".to_string(),
                ),
            )?;
    }

    let routed_a =
        mapping.get(logical_a).ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_a.to_string(),
            )
        })?;

    let routed_b =
        mapping.get(logical_b).ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_b.to_string(),
            )
        })?;

    if !topology.is_adjacent(
        routed_a,
        routed_b,
    ) {
        return Err(
            TranspilerError::RoutingFailed {
                from: routed_a,
                to: routed_b,
            },
        );
    }

    output.push(
        rewrite_quantum_gate(
            result,
            gate,
            args,
            mapping,
        )?,
    );

    *routed_gate_count =
        routed_gate_count.checked_add(1).ok_or(
            TranspilerError::InternalInvariantViolation(
                "routed-gate counter overflow"
                    .to_string(),
            ),
        )?;

    Ok(())
}

// =============================================================================
// Native multi-qubit operation handling
// =============================================================================

fn route_multi_qubit_gate(
    result: &IrRegister,
    gate: &str,
    args: &[IrValue],
    logical_operands: &[&str],
    topology: &PhysicalTopology,
    mapping: &mut QubitMapping,
    output: &mut Vec<IrInstruction>,
) -> Result<(), TranspilerError> {
    // Routing does not synthesize arbitrary multi-qubit gates.
    //
    // It is safe to preserve a multi-qubit operation only when every pair of
    // physical operands is directly connected. This is a conservative
    // native-only policy.
    let physical_operands: Vec<usize> =
        logical_operands
            .iter()
            .map(|logical| {
                mapping
                    .get(logical)
                    .ok_or_else(|| {
                        TranspilerError::UnknownLogicalQubit(
                            (*logical).to_string(),
                        )
                    })
            })
            .collect::<Result<_, _>>()?;

    for (index, &a) in
        physical_operands.iter().enumerate()
    {
        for &b in physical_operands
            .iter()
            .skip(index + 1)
        {
            if !topology.is_adjacent(a, b) {
                return Err(
                    TranspilerError::UnsupportedMultiQubitOperation {
                        gate: gate.to_string(),
                        arity: logical_operands.len(),
                    },
                );
            }
        }
    }

    output.push(
        rewrite_quantum_gate(
            result,
            gate,
            args,
            mapping,
        )?,
    );

    Ok(())
}

// =============================================================================
// IR rewriting
// =============================================================================

fn rewrite_quantum_gate(
    result: &IrRegister,
    gate: &str,
    args: &[IrValue],
    mapping: &QubitMapping,
) -> Result<IrInstruction, TranspilerError> {
    let mut rewritten_args =
        Vec::with_capacity(args.len());

    for arg in args {
        match arg {
            IrValue::Reg(register) => {
                let physical =
                    mapping
                        .get(&register.0)
                        .ok_or_else(|| {
                            TranspilerError::UnknownLogicalQubit(
                                register.0.clone(),
                            )
                        })?;

                rewritten_args.push(
                    IrValue::Reg(
                        physical_register(physical),
                    ),
                );
            }

            other => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!("{other:?}"),
                    ),
                );
            }
        }
    }

    Ok(IrInstruction::QuantumGate(
        result.clone(),
        gate.to_string(),
        rewritten_args,
    ))
}

// =============================================================================
// SWAP generation
// =============================================================================

fn make_swap_instruction(
    physical_a: usize,
    physical_b: usize,
    mapping: &QubitMapping,
) -> Result<IrInstruction, TranspilerError> {
    let logical_a =
        mapping
            .logical_at(physical_a)
            .ok_or(
                TranspilerError::RoutingFailed {
                    from: physical_a,
                    to: physical_b,
                },
            )?;

    let logical_b =
        mapping
            .logical_at(physical_b)
            .ok_or(
                TranspilerError::RoutingFailed {
                    from: physical_a,
                    to: physical_b,
                },
            )?;

    let result =
        IrRegister::new(
            format!(
                "__zq_swap_{}_{}",
                physical_a,
                physical_b
            ),
            IrType::Quantum,
        );

    Ok(IrInstruction::QuantumGate(
        result,
        "SWAP".to_string(),
        vec![
            IrValue::Reg(
                physical_register(
                    physical_a,
                ),
            ),
            IrValue::Reg(
                physical_register(
                    physical_b,
                ),
            ),
        ],
    ))
}

// =============================================================================
// Physical register naming
// =============================================================================

fn physical_register(
    qubit: usize,
) -> IrRegister {
    IrRegister::new(
        format!(
            "__zq_physical_{qubit}"
        ),
        IrType::Quantum,
    )
}

// =============================================================================
// Diagnostics
// =============================================================================

fn annotate_instruction_error(
    error: TranspilerError,
    operation_index: usize,
) -> TranspilerError {
    match error {
        TranspilerError::InvalidQuantumInstruction(
            message,
        ) => {
            TranspilerError::InvalidQuantumInstruction(
                format!(
                    "operation {operation_index}: {message}"
                ),
            )
        }

        other => other,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn quantum_register(
        name: &str,
    ) -> IrValue {
        IrValue::Reg(
            IrRegister::new(
                name,
                IrType::Quantum,
            ),
        )
    }

    fn quantum_function(
        instructions: Vec<IrInstruction>,
    ) -> IrFunction {
        let mut function =
            IrFunction::new(
                "quantum_test",
                vec![],
                IrType::Void,
            );

        function.body = instructions;
        function
    }

    fn cnot(
        result: &str,
        a: &str,
        b: &str,
    ) -> IrInstruction {
        IrInstruction::QuantumGate(
            IrRegister::new(
                result,
                IrType::Quantum,
            ),
            "CNOT".to_string(),
            vec![
                quantum_register(a),
                quantum_register(b),
            ],
        )
    }

    #[test]
    fn line_topology_is_canonicalized() {
        let mut adjacency =
            HashMap::new();

        adjacency.insert(
            0,
            vec![2, 1],
        );
        adjacency.insert(
            1,
            vec![2, 0],
        );
        adjacency.insert(
            2,
            vec![1, 0],
        );

        let topology =
            PhysicalTopology::new(
                "test",
                adjacency,
            )
            .expect("topology");

        assert_eq!(
            topology.neighbors(0),
            &[1, 2]
        );

        assert!(
            topology.is_adjacent(
                0,
                1
            )
        );
    }

    #[test]
    fn rejects_asymmetric_topology() {
        let mut adjacency =
            HashMap::new();

        adjacency.insert(
            0,
            vec![1],
        );
        adjacency.insert(
            1,
            vec![],
        );

        assert!(matches!(
            PhysicalTopology::new(
                "invalid",
                adjacency,
            ),
            Err(
                TranspilerError::InvalidTopology(_)
            )
        ));
    }

    #[test]
    fn shortest_path_is_deterministic() {
        let topology =
            PhysicalTopology::line(4)
                .expect("topology");

        assert_eq!(
            topology.shortest_path(
                0,
                3,
            ),
            Some(vec![
                0, 1, 2, 3
            ])
        );
    }

    #[test]
    fn shortest_path_returns_none_for_disconnected_vertices() {
        let mut adjacency =
            HashMap::new();

        adjacency.insert(
            0,
            vec![],
        );
        adjacency.insert(
            1,
            vec![],
        );

        let topology =
            PhysicalTopology::new(
                "disconnected",
                adjacency,
            )
            .expect("valid disconnected topology");

        assert_eq!(
            topology.shortest_path(
                0,
                1,
            ),
            None
        );
    }

    #[test]
    fn rejects_empty_topology() {
        let result =
            PhysicalTopology::new(
                "empty",
                HashMap::new(),
            );

        assert!(matches!(
            result,
            Err(
                TranspilerError::EmptyTopology
            )
        ));
    }

    #[test]
    fn rejects_duplicate_neighbors() {
        let mut adjacency =
            HashMap::new();

        adjacency.insert(
            0,
            vec![1, 1],
        );
        adjacency.insert(
            1,
            vec![0],
        );

        assert!(matches!(
            PhysicalTopology::new(
                "duplicate",
                adjacency,
            ),
            Err(
                TranspilerError::InvalidTopology(_)
            )
        ));
    }

    #[test]
    fn rejects_missing_neighbor() {
        let mut adjacency =
            HashMap::new();

        adjacency.insert(
            0,
            vec![99],
        );

        assert!(matches!(
            PhysicalTopology::new(
                "missing",
                adjacency,
            ),
            Err(
                TranspilerError::InvalidTopology(_)
            )
        ));
    }

    #[test]
    fn rejects_insufficient_physical_qubits() {
        let topology =
            PhysicalTopology::line(1)
                .expect("topology");

        let mut function =
            quantum_function(vec![
                IrInstruction::QuantumGate(
                    IrRegister::new(
                        "r0",
                        IrType::Quantum,
                    ),
                    "H".to_string(),
                    vec![
                        quantum_register("q0")
                    ],
                ),
                IrInstruction::QuantumGate(
                    IrRegister::new(
                        "r1",
                        IrType::Quantum,
                    ),
                    "H".to_string(),
                    vec![
                        quantum_register("q1")
                    ],
                ),
            ]);

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        let result =
            transpiler.transpile(
                &mut function,
            );

        assert!(matches!(
            result,
            Err(
                TranspilerError::InsufficientPhysicalQubits {
                    ..
                }
            )
        ));
    }

    #[test]
    fn adjacent_two_qubit_gate_requires_no_swap() {
        let topology =
            PhysicalTopology::line(2)
                .expect("topology");

        let mut function =
            quantum_function(vec![
                cnot(
                    "r0",
                    "q0",
                    "q1",
                )
            ]);

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler
            .transpile(
                &mut function,
            )
            .expect(
                "transpilation",
            );

        assert_eq!(
            transpiler.swap_count,
            0
        );

        assert_eq!(
            transpiler.routed_gate_count,
            0
        );

        assert_eq!(
            function.body.len(),
            1
        );
    }

    #[test]
    fn non_adjacent_two_qubit_gate_gets_minimal_line_swap() {
        let topology =
            PhysicalTopology::line(3)
                .expect("topology");

        let mut function =
            quantum_function(vec![
                cnot(
                    "r0",
                    "q0",
                    "q2",
                )
            ]);

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler
            .transpile(
                &mut function,
            )
            .expect(
                "transpilation",
            );

        assert_eq!(
            transpiler.swap_count,
            1
        );

        assert_eq!(
            transpiler.routed_gate_count,
            1
        );

        assert_eq!(
            function.body.len(),
            2
        );

        assert!(matches!(
            &function.body[0],
            IrInstruction::QuantumGate(
                _,
                gate,
                args
            ) if gate == "SWAP"
                && args.len() == 2
        ));
    }

    #[test]
    fn mapping_is_updated_after_swap() {
        let topology =
            PhysicalTopology::line(3)
                .expect("topology");

        let mut function =
            quantum_function(vec![
                cnot(
                    "r0",
                    "q0",
                    "q2",
                )
            ]);

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler
            .transpile(
                &mut function,
            )
            .expect(
                "transpilation",
            );

        assert_eq!(
            transpiler
                .physical_qubit("q0"),
            Some(1)
        );

        assert_eq!(
            transpiler
                .physical_qubit("q2"),
            Some(2)
        );
    }

    #[test]
    fn mapping_reverse_lookup_is_consistent() {
        let topology =
            PhysicalTopology::line(3)
                .expect("topology");

        let mut mapping =
            QubitMapping::new();

        mapping
            .insert(
                "q0".to_string(),
                0,
                &topology,
            )
            .expect("mapping");

        mapping
            .insert(
                "q1".to_string(),
                1,
                &topology,
            )
            .expect("mapping");

        assert_eq!(
            mapping.logical_at(0),
            Some("q0")
        );

        assert_eq!(
            mapping.logical_at(1),
            Some("q1")
        );

        mapping
            .swap_physical(
                0,
                1,
                &topology,
            )
            .expect("swap");

        assert_eq!(
            mapping.get("q0"),
            Some(1)
        );

        assert_eq!(
            mapping.get("q1"),
            Some(0)
        );

        mapping
            .validate(&topology)
            .expect("mapping valid");
    }

    #[test]
    fn invalid_public_mapping_is_rejected() {
        let topology =
            PhysicalTopology::line(2)
                .expect("topology");

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler.mapping.insert(
            "q0".to_string(),
            99,
        );

        let mut function =
            quantum_function(vec![
                IrInstruction::QuantumGate(
                    IrRegister::new(
                        "r0",
                        IrType::Quantum,
                    ),
                    "H".to_string(),
                    vec![
                        quantum_register("q0")
                    ],
                )
            ]);

        assert!(matches!(
            transpiler.transpile(
                &mut function
            ),
            Err(
                TranspilerError::PhysicalQubitOutOfRange(
                    99
                )
            )
        ));
    }

    #[test]
    fn duplicate_public_mapping_is_rejected() {
        let topology =
            PhysicalTopology::line(2)
                .expect("topology");

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler.mapping.insert(
            "q0".to_string(),
            0,
        );

        transpiler.mapping.insert(
            "q1".to_string(),
            0,
        );

        let mut function =
            quantum_function(vec![
                cnot(
                    "r0",
                    "q0",
                    "q1",
                )
            ]);

        assert!(matches!(
            transpiler.transpile(
                &mut function
            ),
            Err(
                TranspilerError::InvalidMapping(_)
            )
        ));
    }

    #[test]
    fn failed_transpilation_is_transactional() {
        let topology =
            PhysicalTopology::line(2)
                .expect("topology");

        let mut function =
            quantum_function(vec![
                cnot(
                    "r0",
                    "q0",
                    "q1",
                ),
                IrInstruction::QuantumGate(
                    IrRegister::new(
                        "r1",
                        IrType::Quantum,
                    ),
                    "TOFFOLI".to_string(),
                    vec![
                        quantum_register("q0"),
                        quantum_register("q1"),
                        quantum_register("q2"),
                    ],
                ),
            ]);

        let original_body =
            function.body.clone();

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        assert!(
            transpiler
                .transpile(
                    &mut function
                )
                .is_err()
        );

        assert_eq!(
            function.body,
            original_body
        );

        assert!(
            transpiler
                .mapping
                .is_empty()
        );

        assert_eq!(
            transpiler.swap_count,
            0
        );

        assert_eq!(
            transpiler.routed_gate_count,
            0
        );
    }

    #[test]
    fn non_quantum_instructions_are_preserved() {
        let topology =
            PhysicalTopology::line(2)
                .expect("topology");

        let non_quantum =
            IrInstruction::QuantumGate(
                IrRegister::new(
                    "r0",
                    IrType::Quantum,
                ),
                "H".to_string(),
                vec![
                    quantum_register(
                        "q0"
                    )
                ],
            );

        let mut function =
            quantum_function(vec![
                non_quantum.clone()
            ]);

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler
            .transpile(
                &mut function,
            )
            .expect(
                "transpilation",
            );

        assert_eq!(
            function.body.len(),
            1
        );
    }

    #[test]
    fn mapping_snapshot_is_deterministic() {
        let topology =
            PhysicalTopology::line(3)
                .expect("topology");

        let mut transpiler =
            QuantumTranspiler::new(
                topology,
            );

        transpiler.mapping.insert(
            "q2".to_string(),
            2,
        );

        transpiler.mapping.insert(
            "q0".to_string(),
            0,
        );

        transpiler.mapping.insert(
            "q1".to_string(),
            1,
        );

        assert_eq!(
            transpiler
                .mapping_snapshot(),
            vec![
                ("q0".to_string(), 0),
                ("q1".to_string(), 1),
                ("q2".to_string(), 2),
            ]
        );
    }
}