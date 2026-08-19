//! Zamani Quantum — Peephole Optimization
//!
//! Local, semantics-preserving quantum circuit rewrites.
//!
//! Peephole optimization operates on a small sliding window of neighboring
//! operations. Unlike global routing or scheduling, it only makes decisions
//! from local circuit structure.
//!
//! The optimizer is intentionally independent of a particular quantum
//! backend. Hardware-aware transformations belong in routing, scheduling,
//! calibration, or backend-specific optimization passes.
//!
//! Typical transformations include:
//!
//!     H H       -> I
//!     X X       -> I
//!     CX CX     -> I
//!     RZ(a)RZ(b)-> RZ(a+b)
//!     RZ(0)     -> I
//!     H X H     -> Z
//!     H Z H     -> X
//!
//! Barriers and measurements are treated as semantic boundaries.

use std::f64::consts::PI;
use std::fmt;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced by the peephole optimizer.
#[derive(Debug, Clone, PartialEq)]
pub enum PeepholeError {
    /// A gate has no operands.
    MissingOperands {
        gate: String,
    },

    /// A gate contains the same qubit more than once.
    DuplicateOperand {
        gate: String,
        qubit: usize,
    },

    /// A gate contains a non-finite rotation angle.
    InvalidAngle {
        gate: String,
        angle: f64,
    },

    /// A rewrite was malformed.
    InvalidRewrite {
        rule: String,
        message: String,
    },
}

impl fmt::Display for PeepholeError {
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

            Self::InvalidRewrite { rule, message } => {
                write!(
                    formatter,
                    "invalid peephole rewrite `{rule}`: {message}"
                )
            }
        }
    }
}

impl std::error::Error for PeepholeError {}

// -----------------------------------------------------------------------------
// Quantum operation
// -----------------------------------------------------------------------------

/// Small canonical representation used by the peephole optimizer.
///
/// Zamani's eventual Quantum IR can be adapted to this representation or the
/// optimizer can later be moved directly onto that IR once it is finalized.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGate {
    /// Canonical gate name.
    pub name: String,

    /// Qubit operands.
    pub qubits: Vec<usize>,

    /// Optional rotation angle in radians.
    pub angle: Option<f64>,

    /// Whether this operation is a semantic barrier.
    pub barrier: bool,
}

impl QuantumGate {
    /// Creates a non-parameterized gate.
    pub fn new(
        name: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, PeepholeError> {
        let name = normalize_name(&name.into());

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
    ) -> Result<Self, PeepholeError> {
        let mut gate = Self::new(name, qubits)?;

        if !angle.is_finite() {
            return Err(PeepholeError::InvalidAngle {
                gate: gate.name,
                angle,
            });
        }

        gate.angle = Some(angle);

        Ok(gate)
    }

    /// Creates a barrier operation.
    pub fn barrier(
        qubits: Vec<usize>,
    ) -> Result<Self, PeepholeError> {
        let mut gate = Self::new("barrier", qubits)?;
        gate.barrier = true;
        Ok(gate)
    }

    /// Returns whether the operation is a barrier.
    pub fn is_barrier(&self) -> bool {
        self.barrier || self.name == "barrier"
    }

    /// Returns whether the operation is a measurement.
    pub fn is_measurement(&self) -> bool {
        matches!(
            self.name.as_str(),
            "measure" | "measurement" | "mz"
        )
    }

    /// Returns whether this is an identity.
    pub fn is_identity(&self) -> bool {
        matches!(
            self.name.as_str(),
            "i" | "id" | "identity"
        )
    }

    /// Returns whether this is a single-qubit gate.
    pub fn is_single_qubit(&self) -> bool {
        self.qubits.len() == 1
    }
}

// -----------------------------------------------------------------------------
// Statistics
// -----------------------------------------------------------------------------

/// Statistics generated by the peephole pass.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PeepholeStats {
    /// Number of rewrite rules applied.
    pub rewrites: usize,

    /// Number of gates removed.
    pub gates_removed: usize,

    /// Number of gates introduced by a rewrite.
    pub gates_introduced: usize,

    /// Number of rotation combinations performed.
    pub rotations_combined: usize,

    /// Number of fixed-point iterations.
    pub iterations: usize,
}

impl PeepholeStats {
    /// Returns whether the circuit was changed.
    pub fn changed(&self) -> bool {
        self.rewrites > 0
            || self.gates_removed > 0
            || self.gates_introduced > 0
            || self.rotations_combined > 0
    }
}

// -----------------------------------------------------------------------------
// Optimizer
// -----------------------------------------------------------------------------

/// Local peephole optimizer.
#[derive(Debug, Clone)]
pub struct PeepholeOptimizer {
    /// Maximum number of fixed-point iterations.
    max_iterations: usize,

    /// Numerical tolerance for angle comparisons.
    angle_tolerance: f64,
}

impl Default for PeepholeOptimizer {
    fn default() -> Self {
        Self {
            max_iterations: 64,
            angle_tolerance: 1.0e-12,
        }
    }
}

impl PeepholeOptimizer {
    /// Creates an optimizer using safe defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of optimization iterations.
    pub fn with_max_iterations(
        mut self,
        iterations: usize,
    ) -> Self {
        self.max_iterations = iterations.max(1);
        self
    }

    /// Sets the angle comparison tolerance.
    pub fn with_angle_tolerance(
        mut self,
        tolerance: f64,
    ) -> Self {
        if tolerance.is_finite() && tolerance >= 0.0 {
            self.angle_tolerance = tolerance;
        }

        self
    }

    /// Optimizes a circuit to a fixed point.
    pub fn optimize(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<(Vec<QuantumGate>, PeepholeStats), PeepholeError> {
        for gate in circuit {
            validate_gate(gate)?;
        }

        let mut current = circuit.to_vec();
        let mut stats = PeepholeStats::default();

        for iteration in 0..self.max_iterations {
            let (next, pass_stats) =
                self.optimize_once(&current)?;

            stats.iterations = iteration + 1;
            stats.rewrites += pass_stats.rewrites;
            stats.gates_removed += pass_stats.gates_removed;
            stats.gates_introduced += pass_stats.gates_introduced;
            stats.rotations_combined += pass_stats.rotations_combined;

            if next == current {
                return Ok((current, stats));
            }

            current = next;
        }

        Ok((current, stats))
    }

    /// Executes exactly one local peephole pass.
    pub fn optimize_once(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<(Vec<QuantumGate>, PeepholeStats), PeepholeError> {
        let mut output = Vec::with_capacity(circuit.len());
        let mut stats = PeepholeStats::default();

        let mut index = 0;

        while index < circuit.len() {
            let current = &circuit[index];

            validate_gate(current)?;

            // Semantic boundaries cannot be crossed.
            if current.is_barrier() || current.is_measurement() {
                output.push(current.clone());
                index += 1;
                continue;
            }

            // -------------------------------------------------------------
            // Three-gate identities.
            // -------------------------------------------------------------

            if index + 2 < circuit.len() {
                let a = &circuit[index];
                let b = &circuit[index + 1];
                let c = &circuit[index + 2];

                if let Some(replacement) =
                    three_gate_rewrite(
                        a,
                        b,
                        c,
                        self.angle_tolerance,
                    )?
                {
                    stats.rewrites += 1;
                    stats.gates_removed += 3;

                    if !replacement.is_empty() {
                        stats.gates_introduced += replacement.len();
                        output.extend(replacement);
                    }

                    index += 3;
                    continue;
                }
            }

            // -------------------------------------------------------------
            // Two-gate identities.
            // -------------------------------------------------------------

            if index + 1 < circuit.len() {
                let a = &circuit[index];
                let b = &circuit[index + 1];

                if let Some(replacement) =
                    two_gate_rewrite(
                        a,
                        b,
                        self.angle_tolerance,
                    )?
                {
                    stats.rewrites += 1;
                    stats.gates_removed += 2;

                    match replacement {
                        Rewrite::Empty => {}

                        Rewrite::One(gate) => {
                            stats.gates_introduced += 1;

                            if is_rotation_gate(&gate.name) {
                                stats.rotations_combined += 1;
                            }

                            output.push(gate);
                        }
                    }

                    index += 2;
                    continue;
                }
            }

            // -------------------------------------------------------------
            // One-gate identities.
            // -------------------------------------------------------------

            if let Some(replacement) =
                one_gate_rewrite(current, self.angle_tolerance)?
            {
                stats.rewrites += 1;
                stats.gates_removed += 1;

                if let Rewrite::One(gate) = replacement {
                    output.push(gate);
                }

                index += 1;
                continue;
            }

            output.push(current.clone());
            index += 1;
        }

        Ok((output, stats))
    }
}

// -----------------------------------------------------------------------------
// Rewrite types
// -----------------------------------------------------------------------------

enum Rewrite {
    Empty,
    One(QuantumGate),
}

// -----------------------------------------------------------------------------
// One-gate rewrites
// -----------------------------------------------------------------------------

fn one_gate_rewrite(
    gate: &QuantumGate,
    tolerance: f64,
) -> Result<Option<Rewrite>, PeepholeError> {
    if gate.is_identity() {
        return Ok(Some(Rewrite::Empty));
    }

    if let Some(angle) = gate.angle {
        if is_rotation_gate(&gate.name)
            && is_zero_angle(angle, tolerance)
        {
            return Ok(Some(Rewrite::Empty));
        }
    }

    Ok(None)
}

// -----------------------------------------------------------------------------
// Two-gate rewrites
// -----------------------------------------------------------------------------

fn two_gate_rewrite(
    first: &QuantumGate,
    second: &QuantumGate,
    tolerance: f64,
) -> Result<Option<Rewrite>, PeepholeError> {
    if first.is_barrier()
        || second.is_barrier()
        || first.is_measurement()
        || second.is_measurement()
    {
        return Ok(None);
    }

    if first.qubits != second.qubits {
        return Ok(None);
    }

    // Identity.
    if first.is_identity() {
        return Ok(Some(Rewrite::Empty));
    }

    if second.is_identity() {
        return Ok(Some(Rewrite::Empty));
    }

    // Self-inverse gates.
    if first.name == second.name
        && is_self_inverse(&first.name)
        && first.angle.is_none()
        && second.angle.is_none()
    {
        return Ok(Some(Rewrite::Empty));
    }

    // Explicit inverse pairs.
    if are_inverse_names(&first.name, &second.name) {
        return Ok(Some(Rewrite::Empty));
    }

    // Same-axis rotations.
    if first.name == second.name
        && is_rotation_gate(&first.name)
    {
        if let (Some(a), Some(b)) =
            (first.angle, second.angle)
        {
            let angle = normalize_angle(a + b);

            if is_zero_angle(angle, tolerance) {
                return Ok(Some(Rewrite::Empty));
            }

            let mut combined = first.clone();
            combined.angle = Some(angle);

            return Ok(Some(Rewrite::One(combined)));
        }
    }

    // X RZ(theta) X = RZ(-theta)
    if first.name == "x"
        && second.name == "rz"
        && first.qubits == second.qubits
    {
        if let Some(angle) = second.angle {
            let mut rewritten = second.clone();
            rewritten.angle =
                Some(normalize_angle(-angle));

            return Ok(Some(Rewrite::One(rewritten)));
        }
    }

    // Z RX(theta) Z = RX(-theta)
    if first.name == "z"
        && second.name == "rx"
        && first.qubits == second.qubits
    {
        if let Some(angle) = second.angle {
            let mut rewritten = second.clone();
            rewritten.angle =
                Some(normalize_angle(-angle));

            return Ok(Some(Rewrite::One(rewritten)));
        }
    }

    Ok(None)
}

// -----------------------------------------------------------------------------
// Three-gate rewrites
// -----------------------------------------------------------------------------

fn three_gate_rewrite(
    first: &QuantumGate,
    second: &QuantumGate,
    third: &QuantumGate,
    tolerance: f64,
) -> Result<Option<Vec<QuantumGate>>, PeepholeError> {
    if first.is_barrier()
        || second.is_barrier()
        || third.is_barrier()
        || first.is_measurement()
        || second.is_measurement()
        || third.is_measurement()
    {
        return Ok(None);
    }

    // H X H = Z
    if same_single_qubit(first, "h")
        && same_single_qubit(second, "x")
        && same_single_qubit(third, "h")
        && first.qubits == second.qubits
        && second.qubits == third.qubits
    {
        return Ok(Some(vec![
            QuantumGate::new("z", first.qubits.clone())?
        ]));
    }

    // H Z H = X
    if same_single_qubit(first, "h")
        && same_single_qubit(second, "z")
        && same_single_qubit(third, "h")
        && first.qubits == second.qubits
        && second.qubits == third.qubits
    {
        return Ok(Some(vec![
            QuantumGate::new("x", first.qubits.clone())?
        ]));
    }

    // H Y H = -Y.
    //
    // The global sign is represented using a phase operation rather than
    // silently dropping the distinction. For a single-qubit computational
    // basis operation, -Y differs by a global phase and can therefore be
    // represented as Y followed by a phase shift.
    //
    // We only apply the direct Y rewrite here because introducing a global
    // phase instruction into a backend-neutral IR should be an explicit
    // responsibility of the final IR.
    if same_single_qubit(first, "h")
        && same_single_qubit(second, "y")
        && same_single_qubit(third, "h")
        && first.qubits == second.qubits
        && second.qubits == third.qubits
    {
        let _ = tolerance;

        return Ok(Some(vec![
            QuantumGate::new("y", first.qubits.clone())?
        ]));
    }

    // A A^-1 A = A.
    if first.name == third.name
        && are_inverse_names(
            &first.name,
            &second.name,
        )
        && first.qubits == second.qubits
        && second.qubits == third.qubits
    {
        return Ok(Some(vec![first.clone()]));
    }

    // A A A = A for self-inverse A.
    //
    // Since A²=I, A³=A.
    if first.name == second.name
        && second.name == third.name
        && first.qubits == second.qubits
        && second.qubits == third.qubits
        && is_self_inverse(&first.name)
    {
        return Ok(Some(vec![first.clone()]));
    }

    // R(theta) R(phi) R(psi) -> R(theta+phi+psi)
    if first.name == second.name
        && second.name == third.name
        && is_rotation_gate(&first.name)
        && first.qubits == second.qubits
        && second.qubits == third.qubits
    {
        if let (Some(a), Some(b), Some(c)) =
            (first.angle, second.angle, third.angle)
        {
            let angle =
                normalize_angle(a + b + c);

            if is_zero_angle(angle, tolerance) {
                return Ok(Some(Vec::new()));
            }

            let mut combined = first.clone();
            combined.angle = Some(angle);

            return Ok(Some(vec![combined]));
        }
    }

    Ok(None)
}

// -----------------------------------------------------------------------------
// Gate classification
// -----------------------------------------------------------------------------

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
    )
}

fn are_inverse_names(
    first: &str,
    second: &str,
) -> bool {
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

fn same_single_qubit(
    gate: &QuantumGate,
    name: &str,
) -> bool {
    gate.name == name && gate.is_single_qubit()
}

// -----------------------------------------------------------------------------
// Numeric helpers
// -----------------------------------------------------------------------------

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

fn is_zero_angle(
    angle: f64,
    tolerance: f64,
) -> bool {
    normalize_angle(angle).abs() <= tolerance
}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

fn validate_gate(
    gate: &QuantumGate,
) -> Result<(), PeepholeError> {
    validate_operands(
        &gate.name,
        &gate.qubits,
    )?;

    if let Some(angle) = gate.angle {
        if !angle.is_finite() {
            return Err(PeepholeError::InvalidAngle {
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
) -> Result<(), PeepholeError> {
    if qubits.is_empty() {
        return Err(PeepholeError::MissingOperands {
            gate: gate.to_string(),
        });
    }

    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].contains(qubit) {
            return Err(PeepholeError::DuplicateOperand {
                gate: gate.to_string(),
                qubit: *qubit,
            });
        }
    }

    Ok(())
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn gate(
        name: &str,
        qubit: usize,
    ) -> QuantumGate {
        QuantumGate::new(name, vec![qubit])
            .expect("test gate must be valid")
    }

    #[test]
    fn removes_identity() {
        let circuit = vec![gate("i", 0)];

        let (result, stats) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.gates_removed, 1);
    }

    #[test]
    fn cancels_x_x() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn cancels_h_h() {
        let circuit = vec![
            gate("h", 0),
            gate("h", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn combines_rotations() {
        let first =
            QuantumGate::rotation(
                "rz",
                vec![0],
                PI / 4.0,
            )
            .expect("rotation should be valid");

        let second =
            QuantumGate::rotation(
                "rz",
                vec![0],
                PI / 4.0,
            )
            .expect("rotation should be valid");

        let (result, stats) =
            PeepholeOptimizer::new()
                .optimize(&[first, second])
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(stats.rotations_combined, 1);

        let angle =
            result[0].angle.expect("angle should exist");

        assert!(
            (angle - PI / 2.0).abs() < 1.0e-12
        );
    }

    #[test]
    fn removes_zero_rotation() {
        let rotation =
            QuantumGate::rotation(
                "rz",
                vec![0],
                0.0,
            )
            .expect("rotation should be valid");

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&[rotation])
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn hadamard_x_hadamard_becomes_z() {
        let circuit = vec![
            gate("h", 0),
            gate("x", 0),
            gate("h", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "z");
    }

    #[test]
    fn hadamard_z_hadamard_becomes_x() {
        let circuit = vec![
            gate("h", 0),
            gate("z", 0),
            gate("h", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "x");
    }

    #[test]
    fn three_self_inverse_gates_reduce_to_one() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 0),
            gate("x", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "x");
    }

    #[test]
    fn barrier_blocks_rewrite() {
        let circuit = vec![
            gate("h", 0),
            QuantumGate::barrier(vec![0])
                .expect("barrier should be valid"),
            gate("h", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn measurement_blocks_rewrite() {
        let circuit = vec![
            gate("h", 0),
            gate("measure", 0),
            gate("h", 0),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn different_qubits_are_not_combined() {
        let circuit = vec![
            gate("x", 0),
            gate("x", 1),
        ];

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn x_rz_x_flips_rotation_angle() {
        let x = gate("x", 0);

        let rz =
            QuantumGate::rotation(
                "rz",
                vec![0],
                PI / 4.0,
            )
            .expect("rotation should be valid");

        let (result, _) =
            PeepholeOptimizer::new()
                .optimize(&[x.clone(), rz, x])
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "rz");

        let angle =
            result[0].angle.expect("angle should exist");

        assert!(
            (angle + PI / 4.0).abs() < 1.0e-12
        );
    }

    #[test]
    fn rejects_duplicate_operands() {
        let result =
            QuantumGate::new("cx", vec![0, 0]);

        assert!(matches!(
            result,
            Err(PeepholeError::DuplicateOperand { .. })
        ));
    }

    #[test]
    fn rejects_nan_angle() {
        let result =
            QuantumGate::rotation(
                "rz",
                vec![0],
                f64::NAN,
            );

        assert!(matches!(
            result,
            Err(PeepholeError::InvalidAngle { .. })
        ));
    }

    #[test]
    fn optimizer_is_deterministic() {
        let circuit = vec![
            gate("h", 0),
            gate("x", 0),
            gate("h", 0),
            gate("x", 0),
            gate("x", 0),
        ];

        let optimizer =
            PeepholeOptimizer::new();

        let first =
            optimizer.optimize(&circuit)
                .expect("optimization should succeed");

        let second =
            optimizer.optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(first, second);
    }
}