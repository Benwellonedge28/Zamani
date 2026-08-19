//! Zamani Quantum — Gate Cancellation Optimization
//!
//! Removes pairs of quantum operations that cancel each other while
//! preserving circuit semantics.
//!
//! The optimizer is intentionally independent of a particular quantum
//! intermediate representation. `QuantumGate` provides the small canonical
//! representation required by this optimization pass.
//!
//! Examples:
//!
//!     X q0; X q0        -> identity
//!     H q0; H q0        -> identity
//!     CX q0,q1; CX q0,q1 -> identity
//!     S q0; Sdg q0      -> identity
//!     T q0; Tdg q0      -> identity
//!
//! Barriers and measurements prevent cancellation across a semantic boundary.

use std::f64::consts::PI;
use std::fmt;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the cancellation optimizer.
#[derive(Debug, Clone, PartialEq)]
pub enum CancellationError {
    /// A gate contains no operands.
    MissingOperands {
        gate: String,
    },

    /// A gate contains duplicate operands.
    DuplicateOperand {
        gate: String,
        qubit: usize,
    },

    /// A rotation contains a non-finite angle.
    InvalidAngle {
        gate: String,
        angle: f64,
    },
}

impl fmt::Display for CancellationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingOperands { gate } => {
                write!(formatter, "gate `{gate}` has no operands")
            }

            Self::DuplicateOperand { gate, qubit } => {
                write!(
                    formatter,
                    "gate `{gate}` contains duplicate qubit {qubit}"
                )
            }

            Self::InvalidAngle { gate, angle } => {
                write!(
                    formatter,
                    "gate `{gate}` has invalid angle {angle}"
                )
            }
        }
    }
}

impl std::error::Error for CancellationError {}

// -----------------------------------------------------------------------------
// Gate representation
// -----------------------------------------------------------------------------

/// Canonical quantum gate operation.
///
/// This is deliberately small so the optimization pass can later be adapted
/// to Zamani's full Quantum IR without coupling the optimizer to a particular
/// circuit storage implementation.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGate {
    /// Canonical gate name.
    pub name: String,

    /// Physical/logical qubit operands.
    pub qubits: Vec<usize>,

    /// Optional rotation angle in radians.
    pub angle: Option<f64>,

    /// Whether this operation represents a semantic barrier.
    pub barrier: bool,
}

impl QuantumGate {
    /// Creates a gate without an angle.
    pub fn new(
        name: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, CancellationError> {
        let name = normalize_gate_name(&name.into());

        validate_operands(&name, &qubits)?;

        Ok(Self {
            name,
            qubits,
            angle: None,
            barrier: false,
        })
    }

    /// Creates a parameterized gate.
    pub fn rotation(
        name: impl Into<String>,
        qubits: Vec<usize>,
        angle: f64,
    ) -> Result<Self, CancellationError> {
        let mut gate = Self::new(name, qubits)?;

        if !angle.is_finite() {
            return Err(CancellationError::InvalidAngle {
                gate: gate.name,
                angle,
            });
        }

        gate.angle = Some(angle);

        Ok(gate)
    }

    /// Creates a barrier.
    pub fn barrier(qubits: Vec<usize>) -> Result<Self, CancellationError> {
        let mut gate = Self::new("barrier", qubits)?;
        gate.barrier = true;
        Ok(gate)
    }

    /// Returns whether this operation is a barrier.
    pub fn is_barrier(&self) -> bool {
        self.barrier || self.name == "barrier"
    }

    /// Returns whether this operation is a measurement.
    pub fn is_measurement(&self) -> bool {
        matches!(
            self.name.as_str(),
            "measure" | "measurement" | "mz"
        )
    }

    /// Returns whether the gate is an identity.
    pub fn is_identity(&self) -> bool {
        matches!(
            self.name.as_str(),
            "i" | "id" | "identity"
        )
    }
}

// -----------------------------------------------------------------------------
// Optimization statistics
// -----------------------------------------------------------------------------

/// Statistics produced by a cancellation pass.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CancellationStats {
    /// Number of gates removed by exact/inverse cancellation.
    pub gates_removed: usize,

    /// Number of cancellation pairs removed.
    pub pairs_cancelled: usize,

    /// Number of angle-combination operations performed.
    pub rotations_combined: usize,

    /// Number of optimization iterations.
    pub iterations: usize,
}

impl CancellationStats {
    /// Returns true when the optimizer changed the circuit.
    pub fn changed(&self) -> bool {
        self.gates_removed > 0 || self.rotations_combined > 0
    }
}

// -----------------------------------------------------------------------------
// Optimizer
// -----------------------------------------------------------------------------

/// Gate cancellation optimizer.
#[derive(Debug, Clone)]
pub struct CancellationOptimizer {
    /// Maximum number of fixed-point iterations.
    max_iterations: usize,

    /// Numerical tolerance for rotation cancellation.
    angle_tolerance: f64,
}

impl Default for CancellationOptimizer {
    fn default() -> Self {
        Self {
            max_iterations: 64,
            angle_tolerance: 1.0e-12,
        }
    }
}

impl CancellationOptimizer {
    /// Creates an optimizer with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum fixed-point iterations.
    pub fn with_max_iterations(mut self, value: usize) -> Self {
        self.max_iterations = value.max(1);
        self
    }

    /// Sets the floating-point angle tolerance.
    pub fn with_angle_tolerance(mut self, value: f64) -> Self {
        if value.is_finite() && value >= 0.0 {
            self.angle_tolerance = value;
        }

        self
    }

    /// Optimizes a circuit until a fixed point is reached.
    pub fn optimize(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<(Vec<QuantumGate>, CancellationStats), CancellationError> {
        for gate in circuit {
            validate_gate(gate)?;
        }

        let mut current = circuit.to_vec();
        let mut stats = CancellationStats::default();

        for iteration in 0..self.max_iterations {
            let (next, mut pass_stats) =
                self.optimize_once(&current)?;

            stats.iterations = iteration + 1;

            stats.gates_removed += pass_stats.gates_removed;
            stats.pairs_cancelled += pass_stats.pairs_cancelled;
            stats.rotations_combined += pass_stats.rotations_combined;

            if next == current {
                return Ok((current, stats));
            }

            current = next;

            // Avoid an unused mutable binding warning while keeping the
            // per-pass statistics explicit for future optimizer extensions.
            pass_stats = CancellationStats::default();

            if pass_stats.changed() {
                break;
            }
        }

        Ok((current, stats))
    }

    /// Performs one left-to-right cancellation pass.
    pub fn optimize_once(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<(Vec<QuantumGate>, CancellationStats), CancellationError> {
        let mut output: Vec<QuantumGate> =
            Vec::with_capacity(circuit.len());

        let mut stats = CancellationStats::default();

        for gate in circuit {
            validate_gate(gate)?;

            if gate.is_barrier() || gate.is_measurement() {
                output.push(gate.clone());
                continue;
            }

            if let Some(previous) = output.last().cloned() {
                if let Some(combined) =
                    combine_gates(&previous, gate, self.angle_tolerance)
                {
                    match combined {
                        CombinedGate::Cancel => {
                            output.pop();

                            stats.gates_removed += 2;
                            stats.pairs_cancelled += 1;

                            continue;
                        }

                        CombinedGate::Replace(gate) => {
                            output.pop();
                            output.push(gate);

                            stats.gates_removed += 1;
                            stats.rotations_combined += 1;

                            continue;
                        }
                    }
                }
            }

            output.push(gate.clone());
        }

        Ok((output, stats))
    }
}

// -----------------------------------------------------------------------------
// Cancellation logic
// -----------------------------------------------------------------------------

enum CombinedGate {
    Cancel,
    Replace(QuantumGate),
}

/// Determines whether two gates can be combined/cancelled.
fn combine_gates(
    first: &QuantumGate,
    second: &QuantumGate,
    tolerance: f64,
) -> Option<CombinedGate> {
    if first.is_barrier()
        || second.is_barrier()
        || first.is_measurement()
        || second.is_measurement()
    {
        return None;
    }

    if first.qubits != second.qubits {
        return None;
    }

    // Identity gates are always removable.
    if first.is_identity() {
        return Some(CombinedGate::Cancel);
    }

    if second.is_identity() {
        return Some(CombinedGate::Cancel);
    }

    // Explicit inverse pairs.
    if are_inverse_names(&first.name, &second.name) {
        return Some(CombinedGate::Cancel);
    }

    // Self-inverse gates.
    if first.name == second.name
        && is_self_inverse(&first.name)
        && first.angle.is_none()
        && second.angle.is_none()
    {
        return Some(CombinedGate::Cancel);
    }

    // Rotation composition.
    if first.name == second.name
        && is_rotation_gate(&first.name)
        && first.angle.is_some()
        && second.angle.is_some()
    {
        let angle =
            normalize_angle(first.angle.unwrap() + second.angle.unwrap());

        if angle.abs() <= tolerance {
            return Some(CombinedGate::Cancel);
        }

        let mut combined = first.clone();
        combined.angle = Some(angle);

        return Some(CombinedGate::Replace(combined));
    }

    // R(theta) followed by R(-theta).
    if is_rotation_gate(&first.name)
        && is_rotation_gate(&second.name)
        && first.name == second.name
        && first.angle.is_some()
        && second.angle.is_some()
    {
        let angle =
            normalize_angle(first.angle.unwrap() + second.angle.unwrap());

        if angle.abs() <= tolerance {
            return Some(CombinedGate::Cancel);
        }
    }

    None
}

/// Returns true when two gate names are mathematical inverses.
fn are_inverse_names(first: &str, second: &str) -> bool {
    matches!(
        (first, second),
        ("s", "sdg")
            | ("sdg", "s")
            | ("t", "tdg")
            | ("tdg", "t")
            | ("rx", "rx_inv")
            | ("rx_inv", "rx")
            | ("ry", "ry_inv")
            | ("ry_inv", "ry")
            | ("rz", "rz_inv")
            | ("rz_inv", "rz")
            | ("u", "u_inv")
            | ("u_inv", "u")
    )
}

/// Returns whether a gate is self-inverse.
fn is_self_inverse(name: &str) -> bool {
    matches!(
        name,
        "x"
            | "y"
            | "z"
            | "h"
            | "cx"
            | "cnot"
            | "cz"
            | "swap"
            | "ccx"
            | "toffoli"
            | "sx_inv"
            | "sy_inv"
    )
}

/// Returns whether a gate represents a rotation.
fn is_rotation_gate(name: &str) -> bool {
    matches!(
        name,
        "rx"
            | "ry"
            | "rz"
            | "phase"
            | "u1"
            | "r"
    )
}

/// Normalizes an angle into approximately [-PI, PI].
fn normalize_angle(angle: f64) -> f64 {
    if !angle.is_finite() {
        return angle;
    }

    let mut result = angle % (2.0 * PI);

    if result > PI {
        result -= 2.0 * PI;
    } else if result < -PI {
        result += 2.0 * PI;
    }

    if result.abs() < 1.0e-15 {
        0.0
    } else {
        result
    }
}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

fn validate_gate(
    gate: &QuantumGate,
) -> Result<(), CancellationError> {
    validate_operands(&gate.name, &gate.qubits)?;

    if let Some(angle) = gate.angle {
        if !angle.is_finite() {
            return Err(CancellationError::InvalidAngle {
                gate: gate.name.clone(),
                angle,
            });
        }
    }

    Ok(())
}

fn validate_operands(
    gate: &str,
    qubits: &[usize],
) -> Result<(), CancellationError> {
    if qubits.is_empty() {
        return Err(CancellationError::MissingOperands {
            gate: gate.to_string(),
        });
    }

    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].contains(qubit) {
            return Err(CancellationError::DuplicateOperand {
                gate: gate.to_string(),
                qubit: *qubit,
            });
        }
    }

    Ok(())
}

fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_lowercase()
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn gate(name: &str, qubit: usize) -> QuantumGate {
        QuantumGate::new(name, vec![qubit])
            .expect("test gate should be valid")
    }

    #[test]
    fn cancels_self_inverse_x() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 0),
        ];

        let (optimized, stats) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(stats.pairs_cancelled, 1);
        assert_eq!(stats.gates_removed, 2);
    }

    #[test]
    fn cancels_hadamard_pair() {
        let circuit = vec![
            gate("h", 0),
            gate("h", 0),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
    }

    #[test]
    fn cancels_two_qubit_cnot_pair() {
        let first =
            QuantumGate::new("cx", vec![0, 1])
                .expect("gate should be valid");

        let second =
            QuantumGate::new("cx", vec![0, 1])
                .expect("gate should be valid");

        let (optimized, stats) =
            CancellationOptimizer::new()
                .optimize(&[first, second])
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(stats.pairs_cancelled, 1);
    }

    #[test]
    fn cancels_s_and_sdg() {
        let circuit = vec![
            gate("s", 0),
            gate("sdg", 0),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
    }

    #[test]
    fn cancels_t_and_tdg() {
        let circuit = vec![
            gate("t", 0),
            gate("tdg", 0),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
    }

    #[test]
    fn combines_rotations() {
        let first =
            QuantumGate::rotation("rz", vec![0], PI / 4.0)
                .expect("rotation should be valid");

        let second =
            QuantumGate::rotation("rz", vec![0], PI / 4.0)
                .expect("rotation should be valid");

        let (optimized, stats) =
            CancellationOptimizer::new()
                .optimize(&[first, second])
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 1);
        assert_eq!(stats.rotations_combined, 1);

        let angle = optimized[0]
            .angle
            .expect("combined rotation must have an angle");

        assert!((angle - PI / 2.0).abs() < 1.0e-12);
    }

    #[test]
    fn cancels_opposite_rotations() {
        let first =
            QuantumGate::rotation("rz", vec![0], PI / 4.0)
                .expect("rotation should be valid");

        let second =
            QuantumGate::rotation("rz", vec![0], -PI / 4.0)
                .expect("rotation should be valid");

        let (optimized, stats) =
            CancellationOptimizer::new()
                .optimize(&[first, second])
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(stats.pairs_cancelled, 1);
    }

    #[test]
    fn does_not_cancel_different_qubits() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 1),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 2);
    }

    #[test]
    fn barrier_prevents_cancellation() {
        let circuit = vec![
            gate("x", 0),
            QuantumGate::barrier(vec![0])
                .expect("barrier should be valid"),
            gate("x", 0),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 3);
    }

    #[test]
    fn measurement_prevents_cancellation() {
        let circuit = vec![
            gate("x", 0),
            gate("measure", 0),
            gate("x", 0),
        ];

        let (optimized, _) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(optimized.len(), 3);
    }

    #[test]
    fn rejects_duplicate_operands() {
        let result =
            QuantumGate::new("cx", vec![0, 0]);

        assert!(matches!(
            result,
            Err(CancellationError::DuplicateOperand { .. })
        ));
    }

    #[test]
    fn rejects_non_finite_rotation() {
        let result =
            QuantumGate::rotation("rz", vec![0], f64::NAN);

        assert!(matches!(
            result,
            Err(CancellationError::InvalidAngle { .. })
        ));
    }

    #[test]
    fn normalizes_gate_names() {
        let operation =
            QuantumGate::new("  CX  ", vec![0, 1])
                .expect("gate should be valid");

        assert_eq!(operation.name, "cx");
    }

    #[test]
    fn identity_is_removed() {
        let circuit = vec![
            gate("i", 0),
        ];

        let (optimized, stats) =
            CancellationOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(optimized.is_empty());
        assert_eq!(stats.gates_removed, 2);
    }

    #[test]
    fn fixed_point_optimization_is_deterministic() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 0),
            gate("h", 1),
            gate("h", 1),
        ];

        let optimizer = CancellationOptimizer::new();

        let first = optimizer
            .optimize(&circuit)
            .expect("optimization should succeed");

        let second = optimizer
            .optimize(&circuit)
            .expect("optimization should succeed");

        assert_eq!(first, second);
        assert!(first.0.is_empty());
    }
}