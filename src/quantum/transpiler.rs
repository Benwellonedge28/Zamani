//! Zamani Quantum Transpiler
//!
//! Production-grade logical-to-physical quantum transpilation.
//!
//! Responsibilities:
//! - Validate physical topology.
//! - Allocate logical qubits deterministically.
//! - Reject malformed/unknown quantum registers.
//! - Route non-adjacent multi-qubit operations.
//! - Insert real SWAP operations rather than merely reporting them.
//! - Update the logical -> physical mapping after every SWAP.
//! - Prevent physical-qubit collisions.
//! - Provide deterministic shortest-path routing.
//! - Preserve non-quantum IR instructions.
//!
//! This module performs topology-aware routing only. It does not claim to
//! perform hardware-specific gate decomposition, pulse scheduling, calibration,
//! or noise modelling. Those concerns belong to later backend stages.

use crate::ir_gen::{IrFunction, IrInstruction, IrRegister, IrType, IrValue};

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranspilerError {
    EmptyTopology,
    InvalidTopology(String),
    DisconnectedTopology {
        from: usize,
        to: usize,
    },
    PhysicalQubitOutOfRange(usize),
    PhysicalQubitAlreadyAssigned(usize),
    LogicalQubitAlreadyMapped(String),
    UnknownLogicalQubit(String),
    UnsupportedQuantumOperand(String),
    InsufficientPhysicalQubits {
        required: usize,
        available: usize,
    },
    InvalidQuantumInstruction(String),
    RoutingFailed {
        from: usize,
        to: usize,
    },
}

impl fmt::Display for TranspilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTopology => {
                write!(f, "quantum transpiler: topology is empty")
            }

            Self::InvalidTopology(message) => {
                write!(f, "quantum transpiler: invalid topology: {message}")
            }

            Self::DisconnectedTopology { from, to } => {
                write!(
                    f,
                    "quantum transpiler: physical qubits {from} and {to} are disconnected"
                )
            }

            Self::PhysicalQubitOutOfRange(qubit) => {
                write!(
                    f,
                    "quantum transpiler: physical qubit {qubit} is outside topology"
                )
            }

            Self::PhysicalQubitAlreadyAssigned(qubit) => {
                write!(
                    f,
                    "quantum transpiler: physical qubit {qubit} is already assigned"
                )
            }

            Self::LogicalQubitAlreadyMapped(qubit) => {
                write!(
                    f,
                    "quantum transpiler: logical qubit '{qubit}' is already mapped"
                )
            }

            Self::UnknownLogicalQubit(qubit) => {
                write!(
                    f,
                    "quantum transpiler: unknown logical qubit '{qubit}'"
                )
            }

            Self::UnsupportedQuantumOperand(operand) => {
                write!(
                    f,
                    "quantum transpiler: unsupported quantum operand '{operand}'"
                )
            }

            Self::InsufficientPhysicalQubits {
                required,
                available,
            } => {
                write!(
                    f,
                    "quantum transpiler: requires {required} physical qubits but topology provides {available}"
                )
            }

            Self::InvalidQuantumInstruction(message) => {
                write!(
                    f,
                    "quantum transpiler: invalid quantum instruction: {message}"
                )
            }

            Self::RoutingFailed { from, to } => {
                write!(
                    f,
                    "quantum transpiler: unable to route physical qubit {from} to {to}"
                )
            }
        }
    }
}

impl std::error::Error for TranspilerError {}

// -----------------------------------------------------------------------------
// Physical topology
// -----------------------------------------------------------------------------

/// Undirected physical quantum-computer connectivity graph.
///
/// `adjacency[a]` contains every physical qubit directly connected to `a`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTopology {
    pub name: String,
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl PhysicalTopology {
    /// Creates a topology after validating its graph structure.
    pub fn new(
        name: impl Into<String>,
        adjacency: HashMap<usize, Vec<usize>>,
    ) -> Result<Self, TranspilerError> {
        let topology = Self {
            name: name.into(),
            adjacency,
        };

        topology.validate()?;
        Ok(topology)
    }

    /// Production-safe heavy-hex-style example topology.
    ///
    /// This is intentionally a small deterministic topology useful for tests
    /// and development. It is not presented as a complete vendor topology.
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

    /// Creates a linear topology with `qubit_count` physical qubits.
    pub fn line(qubit_count: usize) -> Result<Self, TranspilerError> {
        if qubit_count == 0 {
            return Err(TranspilerError::EmptyTopology);
        }

        let mut adjacency = HashMap::new();

        for qubit in 0..qubit_count {
            let mut neighbours = Vec::new();

            if qubit > 0 {
                neighbours.push(qubit - 1);
            }

            if qubit + 1 < qubit_count {
                neighbours.push(qubit + 1);
            }

            adjacency.insert(qubit, neighbours);
        }

        Self::new("Linear", adjacency)
    }

    /// Number of physical qubits.
    pub fn qubit_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Returns whether a physical qubit exists.
    pub fn contains(&self, qubit: usize) -> bool {
        self.adjacency.contains_key(&qubit)
    }

    /// Returns whether two physical qubits are directly connected.
    pub fn is_adjacent(&self, a: usize, b: usize) -> bool {
        if a == b {
            return false;
        }

        self.adjacency
            .get(&a)
            .map(|neighbours| neighbours.binary_search(&b).is_ok())
            .unwrap_or(false)
    }

    /// Validates the topology.
    pub fn validate(&self) -> Result<(), TranspilerError> {
        if self.adjacency.is_empty() {
            return Err(TranspilerError::EmptyTopology);
        }

        for (&qubit, neighbours) in &self.adjacency {
            let mut seen = HashSet::new();

            for &neighbour in neighbours {
                if !self.contains(neighbour) {
                    return Err(TranspilerError::InvalidTopology(format!(
                        "qubit {qubit} references missing neighbour {neighbour}"
                    )));
                }

                if qubit == neighbour {
                    return Err(TranspilerError::InvalidTopology(format!(
                        "qubit {qubit} cannot be connected to itself"
                    )));
                }

                if !seen.insert(neighbour) {
                    return Err(TranspilerError::InvalidTopology(format!(
                        "qubit {qubit} contains duplicate neighbour {neighbour}"
                    )));
                }

                let reverse = self
                    .adjacency
                    .get(&neighbour)
                    .map(|items| items.contains(&qubit))
                    .unwrap_or(false);

                if !reverse {
                    return Err(TranspilerError::InvalidTopology(format!(
                        "topology edge {qubit} -> {neighbour} is not bidirectional"
                    )));
                }
            }
        }

        Ok(())
    }

    /// Returns the deterministic shortest path between two physical qubits.
    ///
    /// Neighbours are sorted before traversal so identical input graphs always
    /// produce identical routing decisions.
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
        let mut previous: HashMap<usize, usize> = HashMap::new();
        let mut visited = HashSet::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            let mut neighbours = self
                .adjacency
                .get(&current)
                .cloned()
                .unwrap_or_default();

            neighbours.sort_unstable();

            for neighbour in neighbours {
                if !visited.insert(neighbour) {
                    continue;
                }

                previous.insert(neighbour, current);

                if neighbour == target {
                    let mut path = vec![target];
                    let mut cursor = target;

                    while cursor != start {
                        cursor = previous[&cursor];
                        path.push(cursor);
                    }

                    path.reverse();
                    return Some(path);
                }

                queue.push_back(neighbour);
            }
        }

        None
    }
}

// -----------------------------------------------------------------------------
// Mapping
// -----------------------------------------------------------------------------

/// Logical-to-physical qubit allocation state.
#[derive(Debug, Clone, Default)]
pub struct QubitMapping {
    logical_to_physical: HashMap<String, usize>,
}

impl QubitMapping {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.logical_to_physical.len()
    }

    pub fn is_empty(&self) -> bool {
        self.logical_to_physical.is_empty()
    }

    pub fn get(&self, logical: &str) -> Option<usize> {
        self.logical_to_physical.get(logical).copied()
    }

    pub fn contains_logical(&self, logical: &str) -> bool {
        self.logical_to_physical.contains_key(logical)
    }

    pub fn contains_physical(&self, physical: usize) -> bool {
        self.logical_to_physical
            .values()
            .any(|mapped| *mapped == physical)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &usize)> {
        self.logical_to_physical.iter()
    }

    pub fn insert(
        &mut self,
        logical: impl Into<String>,
        physical: usize,
        topology: &PhysicalTopology,
    ) -> Result<(), TranspilerError> {
        let logical = logical.into();

        if self.contains_logical(&logical) {
            return Err(TranspilerError::LogicalQubitAlreadyMapped(logical));
        }

        if !topology.contains(physical) {
            return Err(TranspilerError::PhysicalQubitOutOfRange(physical));
        }

        if self.contains_physical(physical) {
            return Err(TranspilerError::PhysicalQubitAlreadyAssigned(physical));
        }

        self.logical_to_physical.insert(logical, physical);

        Ok(())
    }

    fn swap_physical_locations(
        &mut self,
        logical_a: &str,
        logical_b: &str,
    ) -> Result<(), TranspilerError> {
        let physical_a = self
            .get(logical_a)
            .ok_or_else(|| TranspilerError::UnknownLogicalQubit(
                logical_a.to_string(),
            ))?;

        let physical_b = self
            .get(logical_b)
            .ok_or_else(|| TranspilerError::UnknownLogicalQubit(
                logical_b.to_string(),
            ))?;

        self.logical_to_physical
            .insert(logical_a.to_string(), physical_b);

        self.logical_to_physical
            .insert(logical_b.to_string(), physical_a);

        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Transpiler
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct QuantumTranspiler {
    pub topology: PhysicalTopology,

    /// Public for compatibility with the existing Zamani API.
    ///
    /// The map is kept synchronized with the internal mapping.
    pub mapping: HashMap<String, usize>,

    /// Total number of SWAP operations inserted by the most recent pass.
    pub swap_count: usize,

    /// Total number of routed multi-qubit gates.
    pub routed_gate_count: usize,
}

impl QuantumTranspiler {
    pub fn new(topology: PhysicalTopology) -> Self {
        Self {
            topology,
            mapping: HashMap::new(),
            swap_count: 0,
            routed_gate_count: 0,
        }
    }

    /// Transpile a quantum IR function in place.
    ///
    /// The operation is transactional: the function body and mapping are only
    /// committed after the complete transpilation succeeds.
    pub fn transpile(
        &mut self,
        func: &mut IrFunction,
    ) -> Result<(), TranspilerError> {
        self.topology.validate()?;

        let original_body = func.body.clone();
        let original_mapping = self.mapping.clone();

        let mut working_mapping = QubitMapping {
            logical_to_physical: self.mapping.clone(),
        };

        let result = self.transpile_body(func, &mut working_mapping);

        match result {
            Ok((new_body, swap_count, routed_count)) => {
                func.body = new_body;
                self.mapping = working_mapping.logical_to_physical;
                self.swap_count = swap_count;
                self.routed_gate_count = routed_count;

                Ok(())
            }

            Err(error) => {
                func.body = original_body;
                self.mapping = original_mapping;

                Err(error)
            }
        }
    }

    /// Explicit alias for callers that prefer an in-place API name.
    pub fn transpile_in_place(
        &mut self,
        func: &mut IrFunction,
    ) -> Result<(), TranspilerError> {
        self.transpile(func)
    }

    fn transpile_body(
        &self,
        func: &IrFunction,
        mapping: &mut QubitMapping,
    ) -> Result<(Vec<IrInstruction>, usize, usize), TranspilerError> {
        let logical_qubits = collect_logical_qubits(func)?;

        ensure_capacity(&logical_qubits, &self.topology)?;

        allocate_missing_qubits(
            &logical_qubits,
            mapping,
            &self.topology,
        )?;

        let mut output = Vec::with_capacity(func.body.len());

        let mut swap_count = 0usize;
        let mut routed_gate_count = 0usize;

        for instruction in &func.body {
            match instruction {
                IrInstruction::QuantumGate(result, gate, args) => {
                    validate_quantum_gate(gate, args)?;

                    let logical_operands =
                        quantum_register_operands(args)?;

                    if logical_operands.len() <= 1 {
                        let rewritten = rewrite_quantum_gate(
                            result,
                            gate,
                            args,
                            mapping,
                        )?;

                        output.push(rewritten);
                        continue;
                    }

                    if logical_operands.len() == 2 {
                        let logical_a = logical_operands[0];
                        let logical_b = logical_operands[1];

                        let physical_a = mapping
                            .get(logical_a)
                            .ok_or_else(|| {
                                TranspilerError::UnknownLogicalQubit(
                                    logical_a.to_string(),
                                )
                            })?;

                        let physical_b = mapping
                            .get(logical_b)
                            .ok_or_else(|| {
                                TranspilerError::UnknownLogicalQubit(
                                    logical_b.to_string(),
                                )
                            })?;

                        if !self.topology.is_adjacent(
                            physical_a,
                            physical_b,
                        ) {
                            let path = self
                                .topology
                                .shortest_path(
                                    physical_a,
                                    physical_b,
                                )
                                .ok_or(
                                    TranspilerError::RoutingFailed {
                                        from: physical_a,
                                        to: physical_b,
                                    },
                                )?;

                            /*
                             * Move the first logical qubit along the path
                             * until it is adjacent to the second.
                             *
                             * Example:
                             *
                             * logical A: 0
                             * logical B: 3
                             *
                             * path = [0, 1, 2, 3]
                             *
                             * SWAP A/B-neighbour states along:
                             * 0 <-> 1
                             * 1 <-> 2
                             *
                             * The mapping is updated after each SWAP.
                             */
                            for window in path.windows(2) {
                                let from = window[0];
                                let to = window[1];

                                let logical_from =
                                    mapping.iter().find_map(
                                        |(logical, physical)| {
                                            if *physical == from {
                                                Some(logical.clone())
                                            } else {
                                                None
                                            }
                                        },
                                    );

                                let logical_to =
                                    mapping.iter().find_map(
                                        |(logical, physical)| {
                                            if *physical == to {
                                                Some(logical.clone())
                                            } else {
                                                None
                                            }
                                        },
                                    );

                                let logical_from =
                                    logical_from.ok_or_else(|| {
                                        TranspilerError::RoutingFailed {
                                            from,
                                            to,
                                        }
                                    })?;

                                let logical_to =
                                    logical_to.ok_or_else(|| {
                                        TranspilerError::RoutingFailed {
                                            from,
                                            to,
                                        }
                                    })?;

                                output.push(make_swap_instruction(
                                    &logical_from,
                                    &logical_to,
                                    mapping,
                                )?);

                                mapping.swap_physical_locations(
                                    &logical_from,
                                    &logical_to,
                                )?;

                                swap_count += 1;
                            }

                            routed_gate_count += 1;
                        }

                        output.push(rewrite_quantum_gate(
                            result,
                            gate,
                            args,
                            mapping,
                        )?);
                    } else {
                        /*
                         * Multi-qubit gates with arity > 2 require a gate
                         * decomposition/routing policy that is hardware
                         * specific. Silently guessing would be unsafe.
                         */
                        return Err(
                            TranspilerError::InvalidQuantumInstruction(
                                format!(
                                    "gate '{}' has {} operands; only one- and two-qubit gates are currently supported by topology routing",
                                    gate,
                                    logical_operands.len()
                                ),
                            ),
                        );
                    }
                }

                other => output.push(other.clone()),
            }
        }

        Ok((output, swap_count, routed_gate_count))
    }

    /// Returns the current physical location of a logical qubit.
    pub fn physical_qubit(
        &self,
        logical: &str,
    ) -> Option<usize> {
        self.mapping.get(logical).copied()
    }

    /// Returns a deterministic snapshot of the current mapping.
    pub fn mapping_snapshot(&self) -> Vec<(String, usize)> {
        let mut entries: Vec<_> = self
            .mapping
            .iter()
            .map(|(logical, physical)| {
                (logical.clone(), *physical)
            })
            .collect();

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }
}

// -----------------------------------------------------------------------------
// Validation and allocation
// -----------------------------------------------------------------------------

fn collect_logical_qubits(
    func: &IrFunction,
) -> Result<Vec<String>, TranspilerError> {
    let mut qubits = Vec::new();
    let mut seen = HashSet::new();

    for instruction in &func.body {
        if let IrInstruction::QuantumGate(_, gate, args) = instruction {
            validate_quantum_gate(gate, args)?;

            for arg in args {
                if let IrValue::Reg(register) = arg {
                    if register.1 != IrType::Quantum {
                        return Err(
                            TranspilerError::UnsupportedQuantumOperand(
                                format!(
                                    "%{} is {:?}, not a quantum register",
                                    register.0, register.1
                                ),
                            ),
                        );
                    }

                    if seen.insert(register.0.clone()) {
                        qubits.push(register.0.clone());
                    }
                }
            }
        }
    }

    Ok(qubits)
}

fn ensure_capacity(
    logical_qubits: &[String],
    topology: &PhysicalTopology,
) -> Result<(), TranspilerError> {
    if logical_qubits.len() > topology.qubit_count() {
        return Err(TranspilerError::InsufficientPhysicalQubits {
            required: logical_qubits.len(),
            available: topology.qubit_count(),
        });
    }

    Ok(())
}

fn allocate_missing_qubits(
    logical_qubits: &[String],
    mapping: &mut QubitMapping,
    topology: &PhysicalTopology,
) -> Result<(), TranspilerError> {
    let mut occupied = mapping
        .iter()
        .map(|(_, physical)| *physical)
        .collect::<HashSet<_>>();

    for logical in logical_qubits {
        if mapping.contains_logical(logical) {
            continue;
        }

        let physical = (0..topology.qubit_count())
            .find(|candidate| !occupied.contains(candidate))
            .ok_or(TranspilerError::InsufficientPhysicalQubits {
                required: mapping.len() + 1,
                available: topology.qubit_count(),
            })?;

        mapping.insert(logical.clone(), physical, topology)?;
        occupied.insert(physical);
    }

    Ok(())
}

// -----------------------------------------------------------------------------
// Quantum instruction handling
// -----------------------------------------------------------------------------

fn validate_quantum_gate(
    gate: &str,
    args: &[IrValue],
) -> Result<(), TranspilerError> {
    if gate.trim().is_empty() {
        return Err(TranspilerError::InvalidQuantumInstruction(
            "quantum gate name cannot be empty".to_string(),
        ));
    }

    if args.is_empty() {
        return Err(TranspilerError::InvalidQuantumInstruction(
            format!("gate '{gate}' has no operands"),
        ));
    }

    for arg in args {
        match arg {
            IrValue::Reg(register)
                if register.1 == IrType::Quantum => {}

            IrValue::Reg(register) => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!(
                            "%{} has type {:?}",
                            register.0, register.1
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
    let mut result = Vec::with_capacity(args.len());

    for arg in args {
        match arg {
            IrValue::Reg(register) => result.push(register.0.as_str()),

            other => {
                return Err(
                    TranspilerError::UnsupportedQuantumOperand(
                        format!("{other:?}"),
                    ),
                );
            }
        }
    }

    Ok(result)
}

fn rewrite_quantum_gate(
    result: &IrRegister,
    gate: &str,
    args: &[IrValue],
    mapping: &QubitMapping,
) -> Result<IrInstruction, TranspilerError> {
    let mut rewritten_args = Vec::with_capacity(args.len());

    for arg in args {
        match arg {
            IrValue::Reg(register) => {
                let physical = mapping
                    .get(&register.0)
                    .ok_or_else(|| {
                        TranspilerError::UnknownLogicalQubit(
                            register.0.clone(),
                        )
                    })?;

                rewritten_args.push(IrValue::Reg(physical_register(
                    physical,
                )));
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

fn make_swap_instruction(
    logical_a: &str,
    logical_b: &str,
    mapping: &QubitMapping,
) -> Result<IrInstruction, TranspilerError> {
    let physical_a = mapping
        .get(logical_a)
        .ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_a.to_string(),
            )
        })?;

    let physical_b = mapping
        .get(logical_b)
        .ok_or_else(|| {
            TranspilerError::UnknownLogicalQubit(
                logical_b.to_string(),
            )
        })?;

    let result = IrRegister::new(
        format!(
            "__zq_swap_{}_{}",
            physical_a, physical_b
        ),
        IrType::Quantum,
    );

    Ok(IrInstruction::QuantumGate(
        result,
        "SWAP".to_string(),
        vec![
            IrValue::Reg(physical_register(physical_a)),
            IrValue::Reg(physical_register(physical_b)),
        ],
    ))
}

fn physical_register(qubit: usize) -> IrRegister {
    IrRegister::new(
        format!("__zq_physical_{qubit}"),
        IrType::Quantum,
    )
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn quantum_register(name: &str) -> IrValue {
        IrValue::Reg(IrRegister::new(
            name,
            IrType::Quantum,
        ))
    }

    fn quantum_function(
        instructions: Vec<IrInstruction>,
    ) -> IrFunction {
        let mut function =
            IrFunction::new("quantum_test", vec![], IrType::Void);

        function.body = instructions;
        function
    }

    #[test]
    fn heavy_hex_topology_is_valid() {
        let topology = PhysicalTopology::heavy_hex();

        assert!(topology.validate().is_ok());
        assert_eq!(topology.qubit_count(), 6);
    }

    #[test]
    fn line_topology_builds() {
        let topology =
            PhysicalTopology::line(4).expect("line topology");

        assert!(topology.is_adjacent(0, 1));
        assert!(topology.is_adjacent(1, 2));
        assert!(topology.is_adjacent(2, 3));
        assert!(!topology.is_adjacent(0, 3));
    }

    #[test]
    fn shortest_path_is_deterministic() {
        let topology =
            PhysicalTopology::line(4).expect("line topology");

        assert_eq!(
            topology.shortest_path(0, 3),
            Some(vec![0, 1, 2, 3])
        );
    }

    #[test]
    fn rejects_non_bidirectional_topology() {
        let mut adjacency = HashMap::new();
        adjacency.insert(0, vec![1]);
        adjacency.insert(1, vec![]);

        let result =
            PhysicalTopology::new("invalid", adjacency);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_insufficient_physical_qubits() {
        let topology =
            PhysicalTopology::line(1).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "H".to_string(),
                vec![quantum_register("q0")],
            ),
            IrInstruction::QuantumGate(
                IrRegister::new("r1", IrType::Quantum),
                "H".to_string(),
                vec![quantum_register("q1")],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        let result = transpiler.transpile(&mut function);

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
    fn adjacent_gate_requires_no_swap() {
        let topology =
            PhysicalTopology::line(2).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "CNOT".to_string(),
                vec![
                    quantum_register("q0"),
                    quantum_register("q1"),
                ],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        transpiler
            .transpile(&mut function)
            .expect("transpilation");

        assert_eq!(transpiler.swap_count, 0);
        assert_eq!(transpiler.routed_gate_count, 0);
        assert_eq!(function.body.len(), 1);
    }

    #[test]
    fn non_adjacent_gate_gets_real_swaps() {
        let topology =
            PhysicalTopology::line(3).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "CNOT".to_string(),
                vec![
                    quantum_register("q0"),
                    quantum_register("q2"),
                ],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        transpiler
            .transpile(&mut function)
            .expect("transpilation");

        assert_eq!(transpiler.swap_count, 1);
        assert_eq!(transpiler.routed_gate_count, 1);
        assert_eq!(function.body.len(), 2);

        assert!(matches!(
            &function.body[0],
            IrInstruction::QuantumGate(_, gate, args)
                if gate == "SWAP" && args.len() == 2
        ));
    }

    #[test]
    fn mapping_is_updated_after_swap() {
        let topology =
            PhysicalTopology::line(3).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "CNOT".to_string(),
                vec![
                    quantum_register("q0"),
                    quantum_register("q2"),
                ],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        transpiler
            .transpile(&mut function)
            .expect("transpilation");

        assert_eq!(
            transpiler.physical_qubit("q0"),
            Some(1)
        );

        assert_eq!(
            transpiler.physical_qubit("q2"),
            Some(2)
        );
    }

    #[test]
    fn unknown_preexisting_mapping_does_not_default_to_zero() {
        let topology =
            PhysicalTopology::line(2).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "H".to_string(),
                vec![quantum_register("q0")],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        transpiler.mapping.insert(
            "unknown".to_string(),
            0,
        );

        transpiler
            .transpile(&mut function)
            .expect("transpilation");

        assert_eq!(
            transpiler.physical_qubit("q0"),
            Some(1)
        );
    }

    #[test]
    fn invalid_quantum_operand_is_rejected() {
        let topology =
            PhysicalTopology::line(2).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "H".to_string(),
                vec![IrValue::ConstInt(
                    1,
                    IrType::I64,
                )],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        assert!(transpiler.transpile(&mut function).is_err());
    }

    #[test]
    fn empty_gate_name_is_rejected() {
        let topology =
            PhysicalTopology::line(2).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                String::new(),
                vec![quantum_register("q0")],
            ),
        ]);

        let mut transpiler =
            QuantumTranspiler::new(topology);

        assert!(matches!(
            transpiler.transpile(&mut function),
            Err(
                TranspilerError::InvalidQuantumInstruction(_)
            )
        ));
    }

    #[test]
    fn failed_transpilation_is_transactional() {
        let topology =
            PhysicalTopology::line(2).expect("topology");

        let mut function = quantum_function(vec![
            IrInstruction::QuantumGate(
                IrRegister::new("r0", IrType::Quantum),
                "CNOT".to_string(),
                vec![
                    quantum_register("q0"),
                    quantum_register("q1"),
                ],
            ),
            IrInstruction::QuantumGate(
                IrRegister::new("r1", IrType::Quantum),
                "TOFFOLI".to_string(),
                vec![
                    quantum_register("q0"),
                    quantum_register("q1"),
                    quantum_register("q2"),
                ],
            ),
        ]);

        let original_body = function.body.clone();

        let mut transpiler =
            QuantumTranspiler::new(topology);

        assert!(transpiler.transpile(&mut function).is_err());

        assert_eq!(function.body, original_body);
        assert!(transpiler.mapping.is_empty());
    }
}