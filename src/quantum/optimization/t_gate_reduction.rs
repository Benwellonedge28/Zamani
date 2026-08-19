//! Zamani Quantum — T-Gate Reduction
//!
//! Reduces T and T† gate cost using exact phase identities.
//!
//! Important:
//! - This pass is semantics-preserving.
//! - It does not perform hardware routing.
//! - It does not approximate arbitrary rotations.
//! - It works on exact multiples of π/4.
//!
//! Core identities:
//!
//!     T^8       = I
//!     T^4       = Z
//!     T^2       = S
//!     T^6       = S†
//!
//! and therefore:
//!
//!     T T       -> S
//!     T T T T   -> Z
//!     T† T†     -> S†
//!
//! Mixed T/T† sequences are accumulated modulo 8.

use std::f64::consts::PI;
use std::fmt;

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum TGateReductionError {
    MissingOperands {
        gate: String,
    },

    DuplicateOperand {
        gate: String,
        qubit: usize,
    },

    InvalidAngle {
        gate: String,
        angle: f64,
    },
}

impl fmt::Display for TGateReductionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingOperands { gate } => {
                write!(f, "gate `{gate}` has no operands")
            }

            Self::DuplicateOperand { gate, qubit } => {
                write!(
                    f,
                    "gate `{gate}` contains duplicate qubit {qubit}"
                )
            }

            Self::InvalidAngle { gate, angle } => {
                write!(
                    f,
                    "gate `{gate}` has invalid angle {angle}"
                )
            }
        }
    }
}

impl std::error::Error for TGateReductionError {}

// -----------------------------------------------------------------------------
// Quantum gate
// -----------------------------------------------------------------------------

/// Minimal gate representation used by the T-gate reduction pass.
///
/// This can later be replaced by Zamani's canonical Quantum IR once that
/// representation is finalized.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGate {
    pub name: String,
    pub qubits: Vec<usize>,
    pub angle: Option<f64>,
    pub barrier: bool,
}

impl QuantumGate {
    pub fn new(
        name: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, TGateReductionError> {
        let name = normalize_name(&name.into());

        validate_operands(&name, &qubits)?;

        Ok(Self {
            name,
            qubits,
            angle: None,
            barrier: false,
        })
    }

    pub fn rotation(
        name: impl Into<String>,
        qubits: Vec<usize>,
        angle: f64,
    ) -> Result<Self, TGateReductionError> {
        let mut gate = Self::new(name, qubits)?;

        if !angle.is_finite() {
            return Err(TGateReductionError::InvalidAngle {
                gate: gate.name,
                angle,
            });
        }

        gate.angle = Some(angle);

        Ok(gate)
    }

    pub fn barrier(
        qubits: Vec<usize>,
    ) -> Result<Self, TGateReductionError> {
        let mut gate = Self::new("barrier", qubits)?;
        gate.barrier = true;
        Ok(gate)
    }

    pub fn is_barrier(&self) -> bool {
        self.barrier || self.name == "barrier"
    }

    pub fn is_measurement(&self) -> bool {
        matches!(
            self.name.as_str(),
            "measure" | "measurement" | "mz"
        )
    }

    pub fn is_identity(&self) -> bool {
        matches!(
            self.name.as_str(),
            "i" | "id" | "identity"
        )
    }
}

// -----------------------------------------------------------------------------
// Statistics
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TGateReductionStats {
    /// Number of original T-family gates removed.
    pub t_gates_removed: usize,

    /// Number of T-family groups reduced.
    pub groups_reduced: usize,

    /// Number of T gates eliminated completely.
    pub t_gates_eliminated: usize,

    /// Number of replacement gates generated.
    pub replacement_gates: usize,

    /// Number of T gates remaining after optimization.
    pub t_gates_remaining: usize,

    /// Number of optimization iterations.
    pub iterations: usize,
}

impl TGateReductionStats {
    pub fn changed(&self) -> bool {
        self.t_gates_removed > 0
            || self.t_gates_eliminated > 0
            || self.replacement_gates > 0
    }
}

// -----------------------------------------------------------------------------
// Optimizer
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TGateReducer {
    max_iterations: usize,
}

impl Default for TGateReducer {
    fn default() -> Self {
        Self {
            max_iterations: 16,
        }
    }
}

impl TGateReducer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_iterations(
        mut self,
        iterations: usize,
    ) -> Self {
        self.max_iterations = iterations.max(1);
        self
    }

    /// Reduces T/T† sequences until a fixed point is reached.
    pub fn optimize(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<
        (Vec<QuantumGate>, TGateReductionStats),
        TGateReductionError,
    > {
        for gate in circuit {
            validate_gate(gate)?;
        }

        let mut current = circuit.to_vec();
        let mut stats = TGateReductionStats::default();

        for iteration in 0..self.max_iterations {
            let (next, pass_stats) =
                self.optimize_once(&current)?;

            stats.iterations = iteration + 1;

            stats.t_gates_removed +=
                pass_stats.t_gates_removed;

            stats.groups_reduced +=
                pass_stats.groups_reduced;

            stats.t_gates_eliminated +=
                pass_stats.t_gates_eliminated;

            stats.replacement_gates +=
                pass_stats.replacement_gates;

            if next == current {
                stats.t_gates_remaining =
                    count_t_gates(&current);

                return Ok((current, stats));
            }

            current = next;
        }

        stats.t_gates_remaining =
            count_t_gates(&current);

        Ok((current, stats))
    }

    /// Performs one T-gate reduction pass.
    pub fn optimize_once(
        &self,
        circuit: &[QuantumGate],
    ) -> Result<
        (Vec<QuantumGate>, TGateReductionStats),
        TGateReductionError,
    > {
        let mut output = Vec::with_capacity(circuit.len());
        let mut stats = TGateReductionStats::default();

        let mut index = 0;

        while index < circuit.len() {
            let gate = &circuit[index];

            validate_gate(gate)?;

            // Never cross semantic boundaries.
            if gate.is_barrier() || gate.is_measurement() {
                output.push(gate.clone());
                index += 1;
                continue;
            }

            // Only T/T† participate in this pass.
            if !is_t_gate(gate) {
                output.push(gate.clone());
                index += 1;
                continue;
            }

            let qubits = gate.qubits.clone();

            // Accumulate one contiguous T-family sequence on the same qubits.
            let mut exponent: i32 = 0;
            let mut consumed = 0;

            while index + consumed < circuit.len() {
                let candidate =
                    &circuit[index + consumed];

                if candidate.is_barrier()
                    || candidate.is_measurement()
                    || candidate.qubits != qubits
                    || !is_t_gate(candidate)
                {
                    break;
                }

                exponent += t_exponent(candidate);
                consumed += 1;
            }

            let reduced =
                reduce_exponent(exponent, &qubits)?;

            if consumed > 0 {
                stats.groups_reduced += 1;
                stats.t_gates_removed += consumed;

                stats.t_gates_eliminated += consumed;

                stats.replacement_gates += reduced.len();

                output.extend(reduced);

                index += consumed;
            } else {
                output.push(gate.clone());
                index += 1;
            }
        }

        stats.t_gates_remaining =
            count_t_gates(&output);

        Ok((output, stats))
    }
}

// -----------------------------------------------------------------------------
// T-family representation
// -----------------------------------------------------------------------------

/// T  = +1
/// T† = -1
fn t_exponent(gate: &QuantumGate) -> i32 {
    match gate.name.as_str() {
        "t" => 1,
        "tdg" | "t_dagger" | "t†" => -1,
        _ => 0,
    }
}

fn is_t_gate(gate: &QuantumGate) -> bool {
    matches!(
        gate.name.as_str(),
        "t" | "tdg" | "t_dagger" | "t†"
    )
}

/// Reduce an integer T exponent modulo 8.
///
/// T^8 = I.
fn reduce_exponent(
    exponent: i32,
    qubits: &[usize],
) -> Result<Vec<QuantumGate>, TGateReductionError> {
    let mut n = exponent % 8;

    if n < 0 {
        n += 8;
    }

    match n {
        0 => Ok(Vec::new()),

        // T
        1 => Ok(vec![
            QuantumGate::new("t", qubits.to_vec())?
        ]),

        // T² = S
        2 => Ok(vec![
            QuantumGate::new("s", qubits.to_vec())?
        ]),

        // T³ = S T
        3 => Ok(vec![
            QuantumGate::new("t", qubits.to_vec())?,
            QuantumGate::new("s", qubits.to_vec())?,
        ]),

        // T⁴ = Z
        4 => Ok(vec![
            QuantumGate::new("z", qubits.to_vec())?
        ]),

        // T⁵ = Z T
        5 => Ok(vec![
            QuantumGate::new("t", qubits.to_vec())?,
            QuantumGate::new("z", qubits.to_vec())?,
        ]),

        // T⁶ = S†
        6 => Ok(vec![
            QuantumGate::new("sdg", qubits.to_vec())?
        ]),

        // T⁷ = T†
        7 => Ok(vec![
            QuantumGate::new("tdg", qubits.to_vec())?
        ]),

        _ => unreachable!(),
    }
}

// -----------------------------------------------------------------------------
// Validation
// -----------------------------------------------------------------------------

fn validate_gate(
    gate: &QuantumGate,
) -> Result<(), TGateReductionError> {
    validate_operands(
        &gate.name,
        &gate.qubits,
    )?;

    if let Some(angle) = gate.angle {
        if !angle.is_finite() {
            return Err(
                TGateReductionError::InvalidAngle {
                    gate: gate.name.clone(),
                    angle,
                },
            );
        }
    }

    Ok(())
}

fn validate_operands(
    gate: &str,
    qubits: &[usize],
) -> Result<(), TGateReductionError> {
    if qubits.is_empty() {
        return Err(
            TGateReductionError::MissingOperands {
                gate: gate.to_string(),
            },
        );
    }

    for (index, qubit) in qubits.iter().enumerate() {
        if qubits[index + 1..].contains(qubit) {
            return Err(
                TGateReductionError::DuplicateOperand {
                    gate: gate.to_string(),
                    qubit: *qubit,
                },
            );
        }
    }

    Ok(())
}

fn normalize_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

fn count_t_gates(
    circuit: &[QuantumGate],
) -> usize {
    circuit
        .iter()
        .filter(|gate| is_t_gate(gate))
        .count()
}

// -----------------------------------------------------------------------------
// Optional angle helper
// -----------------------------------------------------------------------------

/// Returns the canonical angle associated with an exact T exponent.
///
/// This is useful when integrating the reducer with a rotation-based Quantum
/// IR.
pub fn t_exponent_to_angle(exponent: i32) -> f64 {
    let normalized = exponent.rem_euclid(8);
    normalized as f64 * PI / 4.0
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
            .expect("test gate should be valid")
    }

    #[test]
    fn eight_t_gates_become_identity() {
        let circuit = vec![
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
        ];

        let (result, stats) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(result.is_empty());
        assert_eq!(stats.t_gates_removed, 8);
        assert_eq!(stats.t_gates_remaining, 0);
    }

    #[test]
    fn two_t_gates_become_s() {
        let circuit = vec![
            gate("t", 0),
            gate("t", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "s");
    }

    #[test]
    fn four_t_gates_become_z() {
        let circuit = vec![
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
            gate("t", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "z");
    }

    #[test]
    fn two_t_daggers_become_s_dagger() {
        let circuit = vec![
            gate("tdg", 0),
            gate("tdg", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "sdg");
    }

    #[test]
    fn t_and_t_dagger_cancel() {
        let circuit = vec![
            gate("t", 0),
            gate("tdg", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert!(result.is_empty());
    }

    #[test]
    fn mixed_sequence_reduces_modulo_eight() {
        let circuit = vec![
            gate("t", 0),
            gate("t", 0),
            gate("tdg", 0),
            gate("t", 0),
        ];

        // +1 +1 -1 +1 = +2 => S
        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "s");
    }

    #[test]
    fn different_qubits_are_not_combined() {
        let circuit = vec![
            gate("t", 0),
            gate("t", 1),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn barrier_stops_accumulation() {
        let circuit = vec![
            gate("t", 0),
            QuantumGate::barrier(vec![0])
                .expect("barrier should be valid"),
            gate("tdg", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn measurement_stops_accumulation() {
        let circuit = vec![
            gate("t", 0),
            gate("measure", 0),
            gate("tdg", 0),
        ];

        let (result, _) =
            TGateReducer::new()
                .optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn t_exponent_angle_is_correct() {
        assert!(
            (t_exponent_to_angle(1) - PI / 4.0).abs()
                < 1.0e-12
        );

        assert!(
            (t_exponent_to_angle(4) - PI).abs()
                < 1.0e-12
        );

        assert!(
            (t_exponent_to_angle(-1)
                - 7.0 * PI / 4.0)
                .abs()
                < 1.0e-12
        );
    }

    #[test]
    fn names_are_normalized() {
        let gate =
            QuantumGate::new("  T  ", vec![0])
                .expect("gate should be valid");

        assert_eq!(gate.name, "t");
    }

    #[test]
    fn duplicate_operands_are_rejected() {
        let result =
            QuantumGate::new("t", vec![0, 0]);

        assert!(matches!(
            result,
            Err(
                TGateReductionError::DuplicateOperand { .. }
            )
        ));
    }

    #[test]
    fn optimizer_is_deterministic() {
        let circuit = vec![
            gate("t", 0),
            gate("tdg", 0),
            gate("t", 0),
            gate("t", 0),
        ];

        let reducer = TGateReducer::new();

        let first =
            reducer.optimize(&circuit)
                .expect("optimization should succeed");

        let second =
            reducer.optimize(&circuit)
                .expect("optimization should succeed");

        assert_eq!(first, second);
    }
}