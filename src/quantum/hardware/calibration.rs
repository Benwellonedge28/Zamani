//! Zamani Quantum — Hardware Calibration
//!
//! Hardware-independent calibration data and validation primitives for
//! quantum execution backends.
//!
//! This module deliberately does not perform device or network I/O.
//! Hardware providers should collect calibration measurements and construct
//! `CalibrationSnapshot` values which can then be consumed by scheduling,
//! routing, compilation, and backend validation.
//!
//! Design goals:
//! - deterministic calibration state;
//! - explicit units;
//! - validation at construction boundaries;
//! - immutable calibration snapshots;
//! - support for qubit, gate, and readout calibration;
//! - backend-independent representation;
//! - safe handling of missing calibration data;
//! - no hidden global state.

use std::collections::BTreeMap;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// -----------------------------------------------------------------------------
// Errors
// -----------------------------------------------------------------------------

/// Errors produced while constructing or validating calibration data.
#[derive(Debug, Clone, PartialEq)]
pub enum CalibrationError {
    InvalidQubit {
        qubit: usize,
        message: String,
    },

    InvalidGate {
        gate: String,
        message: String,
    },

    InvalidProbability {
        field: String,
        value: f64,
    },

    InvalidDuration {
        field: String,
        value_ns: u64,
    },

    InvalidCoherence {
        field: String,
        value_ns: f64,
    },

    EmptySnapshot,

    StaleCalibration {
        age_ns: u64,
        max_age_ns: u64,
    },
}

impl fmt::Display for CalibrationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQubit { qubit, message } => {
                write!(formatter, "invalid qubit {qubit}: {message}")
            }

            Self::InvalidGate { gate, message } => {
                write!(formatter, "invalid gate `{gate}`: {message}")
            }

            Self::InvalidProbability { field, value } => {
                write!(
                    formatter,
                    "invalid probability for `{field}`: {value}"
                )
            }

            Self::InvalidDuration {
                field,
                value_ns,
            } => {
                write!(
                    formatter,
                    "invalid duration for `{field}`: {value_ns} ns"
                )
            }

            Self::InvalidCoherence {
                field,
                value_ns,
            } => {
                write!(
                    formatter,
                    "invalid coherence value for `{field}`: {value_ns} ns"
                )
            }

            Self::EmptySnapshot => {
                write!(formatter, "calibration snapshot contains no qubit data")
            }

            Self::StaleCalibration {
                age_ns,
                max_age_ns,
            } => {
                write!(
                    formatter,
                    "calibration is stale: age={age_ns} ns, maximum={max_age_ns} ns"
                )
            }
        }
    }
}

impl std::error::Error for CalibrationError {}

// -----------------------------------------------------------------------------
// Calibration timestamp
// -----------------------------------------------------------------------------

/// Monotonic calibration timestamp represented as Unix nanoseconds.
///
/// The timestamp is stored explicitly so calibration snapshots can be
/// serialized and compared without depending on a live clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CalibrationTimestamp {
    unix_ns: u128,
}

impl CalibrationTimestamp {
    /// Creates a timestamp from Unix nanoseconds.
    pub const fn from_unix_nanos(unix_ns: u128) -> Self {
        Self { unix_ns }
    }

    /// Returns the timestamp as Unix nanoseconds.
    pub const fn as_unix_nanos(self) -> u128 {
        self.unix_ns
    }

    /// Captures the current system time.
    ///
    /// If the system clock is unavailable, this returns zero rather than
    /// introducing an unchecked panic into backend code.
    pub fn now() -> Self {
        let unix_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);

        Self { unix_ns }
    }

    /// Returns the elapsed time since this timestamp.
    pub fn age(self) -> Duration {
        let now = Self::now();

        if now.unix_ns <= self.unix_ns {
            Duration::ZERO
        } else {
            let elapsed = now.unix_ns - self.unix_ns;

            Duration::from_nanos(
                elapsed.min(u64::MAX as u128) as u64
            )
        }
    }
}

// -----------------------------------------------------------------------------
// Qubit calibration
// -----------------------------------------------------------------------------

/// Calibration information for a single physical qubit.
#[derive(Debug, Clone, PartialEq)]
pub struct QubitCalibration {
    /// Physical qubit identifier.
    pub qubit: usize,

    /// Relaxation time (T1), in nanoseconds.
    pub t1_ns: f64,

    /// Dephasing/coherence time (T2), in nanoseconds.
    pub t2_ns: f64,

    /// Probability of thermal/reset failure.
    pub reset_error: f64,

    /// Frequency of the physical qubit, in Hz.
    pub frequency_hz: f64,

    /// Readout calibration.
    pub readout: ReadoutCalibration,
}

impl QubitCalibration {
    /// Creates a qubit calibration record.
    pub fn new(qubit: usize) -> Result<Self, CalibrationError> {
        if qubit == usize::MAX {
            return Err(CalibrationError::InvalidQubit {
                qubit,
                message: "qubit identifier is reserved".to_string(),
            });
        }

        Ok(Self {
            qubit,
            t1_ns: 0.0,
            t2_ns: 0.0,
            reset_error: 0.0,
            frequency_hz: 0.0,
            readout: ReadoutCalibration::default(),
        })
    }

    /// Sets T1.
    pub fn with_t1_ns(mut self, value: f64) -> Result<Self, CalibrationError> {
        validate_coherence("T1", value)?;
        self.t1_ns = value;
        Ok(self)
    }

    /// Sets T2.
    pub fn with_t2_ns(mut self, value: f64) -> Result<Self, CalibrationError> {
        validate_coherence("T2", value)?;
        self.t2_ns = value;
        Ok(self)
    }

    /// Sets reset error probability.
    pub fn with_reset_error(
        mut self,
        value: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("reset_error", value)?;
        self.reset_error = value;
        Ok(self)
    }

    /// Sets qubit frequency.
    pub fn with_frequency_hz(mut self, value: f64) -> Result<Self, CalibrationError> {
        if !value.is_finite() || value < 0.0 {
            return Err(CalibrationError::InvalidQubit {
                qubit: self.qubit,
                message: "frequency must be finite and non-negative".to_string(),
            });
        }

        self.frequency_hz = value;
        Ok(self)
    }

    /// Returns whether the qubit has usable coherence data.
    pub fn has_coherence_data(&self) -> bool {
        self.t1_ns > 0.0 && self.t2_ns > 0.0
    }

    /// Returns a conservative coherence estimate.
    pub fn effective_coherence_ns(&self) -> f64 {
        match (self.t1_ns, self.t2_ns) {
            (t1, t2) if t1 > 0.0 && t2 > 0.0 => t1.min(t2),
            (t1, _) if t1 > 0.0 => t1,
            (_, t2) if t2 > 0.0 => t2,
            _ => 0.0,
        }
    }
}

// -----------------------------------------------------------------------------
// Readout calibration
// -----------------------------------------------------------------------------

/// Measurement/readout calibration for one qubit.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadoutCalibration {
    /// Probability of measuring `1` when the physical state is `0`.
    pub p01: f64,

    /// Probability of measuring `0` when the physical state is `1`.
    pub p10: f64,

    /// Number of calibration shots used to estimate the parameters.
    pub shots: u64,
}

impl Default for ReadoutCalibration {
    fn default() -> Self {
        Self {
            p01: 0.0,
            p10: 0.0,
            shots: 0,
        }
    }
}

impl ReadoutCalibration {
    /// Creates readout calibration data.
    pub fn new(
        p01: f64,
        p10: f64,
        shots: u64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("readout.p01", p01)?;
        validate_probability("readout.p10", p10)?;

        Ok(Self {
            p01,
            p10,
            shots,
        })
    }

    /// Average readout error.
    pub fn average_error(&self) -> f64 {
        (self.p01 + self.p10) / 2.0
    }

    /// Returns true if readout calibration has empirical measurements.
    pub fn is_measured(&self) -> bool {
        self.shots > 0
    }
}

// -----------------------------------------------------------------------------
// Gate calibration
// -----------------------------------------------------------------------------

/// Calibration information for a physical gate.
#[derive(Debug, Clone, PartialEq)]
pub struct GateCalibration {
    /// Canonical gate name.
    pub gate: String,

    /// Physical qubits used by the gate.
    pub qubits: Vec<usize>,

    /// Gate duration in nanoseconds.
    pub duration_ns: u64,

    /// Estimated gate error probability.
    pub error_rate: f64,

    /// Number of calibration experiments/shots.
    pub shots: u64,

    /// Whether this gate is currently considered operational.
    pub operational: bool,
}

impl GateCalibration {
    /// Creates a gate calibration record.
    pub fn new(
        gate: impl Into<String>,
        qubits: Vec<usize>,
    ) -> Result<Self, CalibrationError> {
        let gate = normalize_gate_name(&gate.into());

        if gate.is_empty() {
            return Err(CalibrationError::InvalidGate {
                gate,
                message: "gate name cannot be empty".to_string(),
            });
        }

        if qubits.is_empty() {
            return Err(CalibrationError::InvalidGate {
                gate,
                message: "gate must reference at least one qubit".to_string(),
            });
        }

        Ok(Self {
            gate,
            qubits,
            duration_ns: 0,
            error_rate: 0.0,
            shots: 0,
            operational: true,
        })
    }

    /// Sets gate duration.
    pub fn with_duration_ns(
        mut self,
        duration_ns: u64,
    ) -> Result<Self, CalibrationError> {
        if duration_ns == 0 {
            return Err(CalibrationError::InvalidDuration {
                field: "gate.duration_ns".to_string(),
                value_ns: duration_ns,
            });
        }

        self.duration_ns = duration_ns;
        Ok(self)
    }

    /// Sets gate error rate.
    pub fn with_error_rate(
        mut self,
        error_rate: f64,
    ) -> Result<Self, CalibrationError> {
        validate_probability("gate.error_rate", error_rate)?;
        self.error_rate = error_rate;
        Ok(self)
    }

    /// Sets the number of calibration shots.
    pub fn with_shots(mut self, shots: u64) -> Self {
        self.shots = shots;
        self
    }

    /// Marks the gate operational or unavailable.
    pub fn with_operational(mut self, operational: bool) -> Self {
        self.operational = operational;
        self
    }

    /// Returns whether the calibration is usable.
    pub fn is_usable(&self) -> bool {
        self.operational && self.duration_ns > 0
    }
}

// -----------------------------------------------------------------------------
// Calibration snapshot
// -----------------------------------------------------------------------------

/// Immutable collection of calibration data describing a backend at a point
/// in time.
#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationSnapshot {
    /// Backend identifier.
    pub backend_id: String,

    /// Calibration timestamp.
    pub timestamp: CalibrationTimestamp,

    /// Qubit calibration indexed by physical qubit ID.
    pub qubits: BTreeMap<usize, QubitCalibration>,

    /// Gate calibration indexed by canonical gate signature.
    pub gates: BTreeMap<String, GateCalibration>,

    /// Optional backend-specific metadata.
    pub metadata: BTreeMap<String, String>,
}

impl CalibrationSnapshot {
    /// Creates an empty snapshot.
    pub fn new(backend_id: impl Into<String>) -> Self {
        Self {
            backend_id: backend_id.into(),
            timestamp: CalibrationTimestamp::now(),
            qubits: BTreeMap::new(),
            gates: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Creates a snapshot with an explicit timestamp.
    pub fn with_timestamp(
        backend_id: impl Into<String>,
        timestamp: CalibrationTimestamp,
    ) -> Self {
        Self {
            backend_id: backend_id.into(),
            timestamp,
            qubits: BTreeMap::new(),
            gates: BTreeMap::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Inserts or replaces qubit calibration data.
    pub fn insert_qubit(&mut self, calibration: QubitCalibration) {
        self.qubits.insert(calibration.qubit, calibration);
    }

    /// Inserts or replaces gate calibration data.
    pub fn insert_gate(&mut self, calibration: GateCalibration) {
        let key = gate_key(
            &calibration.gate,
            &calibration.qubits,
        );

        self.gates.insert(key, calibration);
    }

    /// Adds metadata.
    pub fn insert_metadata(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Retrieves qubit calibration.
    pub fn qubit(&self, qubit: usize) -> Option<&QubitCalibration> {
        self.qubits.get(&qubit)
    }

    /// Retrieves gate calibration.
    pub fn gate(
        &self,
        gate: &str,
        qubits: &[usize],
    ) -> Option<&GateCalibration> {
        self.gates.get(&gate_key(gate, qubits))
    }

    /// Validates the snapshot.
    pub fn validate(&self) -> Result<(), CalibrationError> {
        if self.qubits.is_empty() {
            return Err(CalibrationError::EmptySnapshot);
        }

        for calibration in self.qubits.values() {
            validate_coherence("T1", calibration.t1_ns)?;
            validate_coherence("T2", calibration.t2_ns)?;
            validate_probability(
                "reset_error",
                calibration.reset_error,
            )?;
            validate_probability(
                "readout.p01",
                calibration.readout.p01,
            )?;
            validate_probability(
                "readout.p10",
                calibration.readout.p10,
            )?;
        }

        for calibration in self.gates.values() {
            if calibration.duration_ns == 0 {
                return Err(CalibrationError::InvalidDuration {
                    field: format!(
                        "gate `{}` duration",
                        calibration.gate
                    ),
                    value_ns: 0,
                });
            }

            validate_probability(
                "gate.error_rate",
                calibration.error_rate,
            )?;
        }

        Ok(())
    }

    /// Returns true when the snapshot is older than `max_age`.
    pub fn is_stale(&self, max_age: Duration) -> bool {
        self.timestamp.age() > max_age
    }

    /// Validates that the snapshot has not become stale.
    pub fn require_fresh(
        &self,
        max_age: Duration,
    ) -> Result<(), CalibrationError> {
        if self.is_stale(max_age) {
            let age_ns = self.timestamp.age().as_nanos();
            let max_age_ns = max_age.as_nanos();

            return Err(CalibrationError::StaleCalibration {
                age_ns: age_ns.min(u64::MAX as u128) as u64,
                max_age_ns: max_age_ns.min(u64::MAX as u128) as u64,
            });
        }

        Ok(())
    }

    /// Returns the number of calibrated qubits.
    pub fn qubit_count(&self) -> usize {
        self.qubits.len()
    }

    /// Returns the number of calibrated gates.
    pub fn gate_count(&self) -> usize {
        self.gates.len()
    }

    /// Returns the average calibrated gate error.
    pub fn average_gate_error(&self) -> f64 {
        if self.gates.is_empty() {
            return 0.0;
        }

        self.gates
            .values()
            .map(|gate| gate.error_rate)
            .sum::<f64>()
            / self.gates.len() as f64
    }

    /// Returns the worst calibrated gate error.
    pub fn worst_gate_error(&self) -> f64 {
        self.gates
            .values()
            .map(|gate| gate.error_rate)
            .fold(0.0, f64::max)
    }

    /// Returns a deterministic fingerprint of the calibration state.
    ///
    /// This is deliberately a stable, dependency-free hash rather than a
    /// cryptographic hash. It is intended for cache invalidation and
    /// scheduling decisions, not authentication.
    pub fn fingerprint(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;

        fn feed(hash: &mut u64, bytes: &[u8]) {
            for byte in bytes {
                *hash ^= u64::from(*byte);
                *hash = hash.wrapping_mul(0x100000001b3);
            }
        }

        feed(&mut hash, self.backend_id.as_bytes());
        feed(
            &mut hash,
            &self.timestamp.as_unix_nanos().to_le_bytes(),
        );

        for (qubit, calibration) in &self.qubits {
            feed(&mut hash, &qubit.to_le_bytes());
            feed(
                &mut hash,
                &calibration.t1_ns.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.t2_ns.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.reset_error.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.frequency_hz.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.readout.p01.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.readout.p10.to_bits().to_le_bytes(),
            );
        }

        for (key, calibration) in &self.gates {
            feed(&mut hash, key.as_bytes());
            feed(
                &mut hash,
                &calibration.duration_ns.to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.error_rate.to_bits().to_le_bytes(),
            );
            feed(
                &mut hash,
                &calibration.shots.to_le_bytes(),
            );
            feed(
                &mut hash,
                &[u8::from(calibration.operational)],
            );
        }

        hash
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn validate_probability(
    field: &str,
    value: f64,
) -> Result<(), CalibrationError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(CalibrationError::InvalidProbability {
            field: field.to_string(),
            value,
        });
    }

    Ok(())
}

fn validate_coherence(
    field: &str,
    value_ns: f64,
) -> Result<(), CalibrationError> {
    if !value_ns.is_finite() || value_ns < 0.0 {
        return Err(CalibrationError::InvalidCoherence {
            field: field.to_string(),
            value_ns,
        });
    }

    Ok(())
}

fn normalize_gate_name(gate: &str) -> String {
    gate.trim().to_ascii_lowercase()
}

fn gate_key(gate: &str, qubits: &[usize]) -> String {
    let gate = normalize_gate_name(gate);

    let qubits = qubits
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",");

    format!("{gate}:{qubits}")
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qubit_calibration_accepts_valid_values() {
        let calibration = QubitCalibration::new(0)
            .expect("qubit should be valid")
            .with_t1_ns(100_000.0)
            .expect("T1 should be valid")
            .with_t2_ns(80_000.0)
            .expect("T2 should be valid")
            .with_reset_error(0.001)
            .expect("reset error should be valid")
            .with_frequency_hz(5.0e9)
            .expect("frequency should be valid");

        assert_eq!(calibration.qubit, 0);
        assert_eq!(calibration.effective_coherence_ns(), 80_000.0);
    }

    #[test]
    fn invalid_probability_is_rejected() {
        let result = ReadoutCalibration::new(1.2, 0.1, 100);

        assert!(matches!(
            result,
            Err(CalibrationError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn readout_error_is_calculated() {
        let calibration =
            ReadoutCalibration::new(0.02, 0.04, 10_000)
                .expect("readout calibration should be valid");

        assert!((calibration.average_error() - 0.03).abs() < f64::EPSILON);
        assert!(calibration.is_measured());
    }

    #[test]
    fn gate_calibration_is_stored_and_retrieved() {
        let gate = GateCalibration::new("CX", vec![0, 1])
            .expect("gate should be valid")
            .with_duration_ns(300)
            .expect("duration should be valid")
            .with_error_rate(0.01)
            .expect("error rate should be valid")
            .with_shots(10_000);

        let mut snapshot =
            CalibrationSnapshot::new("test-qpu");

        snapshot.insert_gate(gate);

        let stored = snapshot
            .gate("cx", &[0, 1])
            .expect("gate should be present");

        assert_eq!(stored.duration_ns, 300);
        assert!((stored.error_rate - 0.01).abs() < f64::EPSILON);
    }

    #[test]
    fn snapshot_validation_requires_qubits() {
        let snapshot =
            CalibrationSnapshot::new("test-qpu");

        assert!(matches!(
            snapshot.validate(),
            Err(CalibrationError::EmptySnapshot)
        ));
    }

    #[test]
    fn snapshot_validation_accepts_valid_data() {
        let mut snapshot =
            CalibrationSnapshot::new("test-qpu");

        let qubit = QubitCalibration::new(0)
            .expect("qubit should be valid")
            .with_t1_ns(100_000.0)
            .expect("T1 should be valid")
            .with_t2_ns(80_000.0)
            .expect("T2 should be valid");

        snapshot.insert_qubit(qubit);

        assert!(snapshot.validate().is_ok());
    }

    #[test]
    fn gate_name_normalization_is_deterministic() {
        let first = gate_key(" CX ", &[0, 1]);
        let second = gate_key("cx", &[0, 1]);

        assert_eq!(first, second);
    }

    #[test]
    fn fingerprint_is_deterministic() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(1_000);

        let mut first =
            CalibrationSnapshot::with_timestamp("qpu", timestamp);

        let mut second =
            CalibrationSnapshot::with_timestamp("qpu", timestamp);

        let qubit = QubitCalibration::new(0)
            .expect("qubit should be valid")
            .with_t1_ns(100_000.0)
            .expect("T1 should be valid")
            .with_t2_ns(80_000.0)
            .expect("T2 should be valid");

        first.insert_qubit(qubit.clone());
        second.insert_qubit(qubit);

        assert_eq!(first.fingerprint(), second.fingerprint());
    }

    #[test]
    fn stale_calibration_is_detected() {
        let timestamp =
            CalibrationTimestamp::from_unix_nanos(0);

        let snapshot =
            CalibrationSnapshot::with_timestamp("qpu", timestamp);

        assert!(snapshot.is_stale(Duration::from_secs(1)));
    }
}