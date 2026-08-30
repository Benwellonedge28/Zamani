//! Zamani Quantum Optimization — Randomized Verification
//!
//! Production-grade randomized/differential verification for optimization
//! transformations over the canonical Zamani Quantum IR.
//!
//! # Architectural position
//!
//! ```text
//!                         original QuantumCircuit
//!                                  │
//!                                  │
//!                         optimized QuantumCircuit
//!                                  │
//!                                  ▼
//!                 verification::randomized
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    │                           │
//!             deterministic probes       randomized probes
//!                    │                           │
//!                    └─────────────┬─────────────┘
//!                                  ▼
//!                         verification oracle
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    │                           │
//!             counterexample              no counterexample
//!                    │                           │
//!                    ▼                           ▼
//!             NotEquivalent              ConfidenceOnly
//! ```
//!
//! # Purpose
//!
//! This module provides randomized semantic/differential verification for
//! optimization transformations.
//!
//! It is intentionally different from:
//!
//! `optimization::stochastic::randomized`
//!
//! which searches for better circuits.
//!
//! This module instead asks:
//!
//! > Can randomized tests find evidence that an optimized circuit differs from
//! > its original circuit?
//!
//! # Important soundness rule
//!
//! Randomized verification is NOT a mathematical proof of arbitrary quantum
//! circuit equivalence.
//!
//! Therefore:
//!
//! - `CounterexampleFound` means a difference was actually observed.
//! - `NoCounterexample` means no difference was observed in the requested
//!   randomized trials.
//! - `Inconclusive` means verification could not safely complete.
//!
//! `NoCounterexample` MUST NOT be converted into an exact `Equivalent` verdict.
//!
//! Exact proof remains owned by `verification::semantic` and the canonical
//! `optimization::equivalence` subsystem.
//!
//! # Canonical IR
//!
//! This module never defines a second quantum circuit representation.
//!
//! All circuits are:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! Logical qubits are:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! Physical qubits, routing, hardware topology, pulse execution, scheduling,
//! QPU communication, and backend authentication do not belong here.
//!
//! # Execution boundary
//!
//! The verifier does not execute hardware itself.
//!
//! A caller supplies a `RandomizedVerificationOracle`.
//!
//! The oracle is expected to be a local/pure verification service such as:
//!
//! - a state-vector simulator;
//! - a stabilizer simulator;
//! - a tensor-network evaluator;
//! - a symbolic evaluator;
//! - a deterministic differential simulator;
//! - a future certificate-backed execution engine.
//!
//! Hardware/QPU I/O must not be hidden inside this module.
//!
//! # Scaling
//!
//! There is no hard-coded circuit-size ceiling.
//!
//! The verifier streams probes one at a time and does not retain the complete
//! probe corpus in memory.
//!
//! Scaling is controlled by:
//!
//! - the configured number of trials;
//! - the oracle's capabilities;
//! - explicit time/resource policy supplied by the caller;
//! - the available machine resources.
//!
//! A literal infinity of computation is impossible on finite hardware.
//! "Tiny to infinity" therefore means that this module imposes no arbitrary
//! circuit-size ceiling of its own and remains safe as the underlying resource
//! budget grows.
//!
//! # Determinism
//!
//! Deterministic and seeded verification use the internal SplitMix64 generator.
//!
//! No ambient process-global random source is used.
//!
//! The PRNG is suitable for reproducible test sampling only. It is NOT a
//! cryptographic random-number generator.
//!
//! # Reproducibility
//!
//! The complete randomized verification configuration contains:
//!
//! - seed;
//! - number of trials;
//! - probe strategy;
//! - comparison tolerance;
//! - circuit identity information.
//!
//! A caller can therefore record the configuration in optimization provenance
//! or a verification certificate.
//!
//! # Integration contract
//!
//! `verification/mod.rs` should eventually expose:
//!
//! `pub mod randomized;`
//!
//! `verification/semantic.rs` can consume `RandomizedVerificationReport` when
//! a compiler policy explicitly permits confidence-based verification.
//!
//! `verification/exhaustive.rs` can use the same probe/oracle abstractions for
//! small circuits.
//!
//! `verification/certificates.rs` can serialize the report and seed.
//!
//! `result.rs` can store the report as verification evidence.
//!
//! `provenance.rs` can record the seed, trial count, probe policy, and verdict.
//!
//! `tests/equivalence.rs` can test the soundness boundary:
//!
//! - observed difference => counterexample;
//! - no observed difference => confidence only;
//! - inconclusive => never equivalent.
//!
//! No optimizer pass needs to modify this file merely because another
//! verification engine is added.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust only;
//! - no nightly features;
//! - no unsafe code.
//!
//! # Security properties
//!
//! This module:
//!
//! - never mutates either input circuit;
//! - never changes logical qubit identifiers;
//! - never confuses logical and physical qubits;
//! - never performs hardware I/O;
//! - never treats timeout as equivalence;
//! - never treats oracle failure as equivalence;
//! - never converts probabilistic evidence into an exact proof;
//! - never silently ignores a counterexample;
//! - never stores an unbounded probe corpus;
//! - never uses unsafe code.

#![forbid(unsafe_code)]

use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Stable public identifiers
// =============================================================================

/// Stable identifier for randomized verification.
pub const VERIFIER_ID: &str =
    "quantum.optimization.verification.randomized";

/// Public API contract version.
///
/// This is independent of the Quantum IR version.
pub const VERIFIER_VERSION: u32 = 1;

// =============================================================================
// Verification verdict
// =============================================================================

/// Final randomized-verification verdict.
///
/// The variants intentionally distinguish statistical evidence from proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandomizedVerificationVerdict {
    /// A probe produced an observable semantic difference.
    CounterexampleFound,

    /// All requested probes completed without finding a difference.
    ///
    /// This is confidence evidence, NOT a mathematical proof of equivalence.
    NoCounterexample,

    /// Verification could not safely complete or the oracle could not provide
    /// a valid conclusion.
    Inconclusive,
}

impl RandomizedVerificationVerdict {
    /// Returns true when an actual counterexample was found.
    #[must_use]
    pub const fn is_counterexample(self) -> bool {
        matches!(self, Self::CounterexampleFound)
    }

    /// Returns true when all requested trials completed without a detected
    /// difference.
    #[must_use]
    pub const fn is_no_counterexample(self) -> bool {
        matches!(self, Self::NoCounterexample)
    }

    /// Returns true when no safe conclusion was possible.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }

    /// Returns true only for a verdict that is safe to interpret as a
    /// detected semantic failure.
    #[must_use]
    pub const fn is_failure(self) -> bool {
        self.is_counterexample()
    }

    /// Returns the stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterexampleFound => "counterexample_found",
            Self::NoCounterexample => "no_counterexample",
            Self::Inconclusive => "inconclusive",
        }
    }
}

impl fmt::Display for RandomizedVerificationVerdict {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Probe strategy
// =============================================================================

/// Strategy for generating randomized verification probes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProbeStrategy {
    /// Generate uniformly distributed computational-basis assignments.
    ComputationalBasis,

    /// Generate deterministic basis assignments derived from the seed.
    ///
    /// This remains useful when a caller wants reproducible coverage without
    /// depending on a non-deterministic entropy provider.
    SeededBasis,

    /// Use a deterministic mixture of all-zero, all-one, alternating, and
    /// pseudo-random basis assignments.
    StructuredBasis,

    /// Let the oracle interpret the seed as a general randomized probe request.
    ///
    /// This is the most extensible strategy for future simulators.
    OracleDefined,
}

impl Default for ProbeStrategy {
    fn default() -> Self {
        Self::StructuredBasis
    }
}

impl ProbeStrategy {
    /// Returns a stable textual identifier.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComputationalBasis => "computational_basis",
            Self::SeededBasis => "seeded_basis",
            Self::StructuredBasis => "structured_basis",
            Self::OracleDefined => "oracle_defined",
        }
    }
}

impl fmt::Display for ProbeStrategy {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

// =============================================================================
// Probe
// =============================================================================

/// One immutable randomized verification probe.
///
/// A probe identifies logical qubit values using the canonical `QubitId`.
///
/// This is deliberately not a quantum state representation. It describes a
/// computational-basis input assignment.
///
/// The oracle may interpret the probe according to its own execution model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationProbe {
    /// Stable trial number.
    trial: u64,

    /// Seed from which this probe was derived.
    seed: u64,

    /// Logical qubit assignments.
    assignments: Vec<(QubitId, bool)>,
}

impl VerificationProbe {
    /// Creates a probe from explicit logical-qubit assignments.
    pub fn new(
        trial: u64,
        seed: u64,
        assignments: Vec<(QubitId, bool)>,
    ) -> Result<Self, ProbeError> {
        validate_assignments(&assignments)?;

        Ok(Self {
            trial,
            seed,
            assignments,
        })
    }

    /// Returns the trial number.
    #[must_use]
    pub const fn trial(&self) -> u64 {
        self.trial
    }

    /// Returns the deterministic probe seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns all logical-qubit assignments.
    #[must_use]
    pub fn assignments(&self) -> &[(QubitId, bool)] {
        &self.assignments
    }

    /// Returns the value assigned to one logical qubit.
    #[must_use]
    pub fn value(&self, qubit: QubitId) -> Option<bool> {
        self.assignments
            .binary_search_by_key(&qubit, |(id, _)| *id)
            .ok()
            .map(|index| self.assignments[index].1)
    }

    /// Returns the number of logical qubits represented by this probe.
    #[must_use]
    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    /// Returns whether the probe contains no logical qubits.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}

// =============================================================================
// Probe errors
// =============================================================================

/// Errors produced while constructing a randomized verification probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError {
    /// The same logical qubit appeared more than once.
    DuplicateQubit {
        /// Duplicated logical qubit.
        qubit: QubitId,
    },

    /// A logical qubit index is outside the circuit namespace.
    QubitOutOfRange {
        /// Invalid logical qubit.
        qubit: QubitId,

        /// Number of logical qubits in the circuit.
        num_qubits: usize,
    },

    /// A probe exceeded the configured maximum size.
    TooLarge {
        /// Requested size.
        count: usize,

        /// Maximum permitted size.
        maximum: usize,
    },
}

impl fmt::Display for ProbeError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateQubit { qubit } => {
                write!(
                    formatter,
                    "verification probe contains duplicate logical qubit {qubit}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "verification probe logical qubit {qubit} is outside \
                     circuit namespace 0..{num_qubits}"
                )
            }

            Self::TooLarge {
                count,
                maximum,
            } => {
                write!(
                    formatter,
                    "verification probe contains {count} qubits, \
                     exceeding maximum {maximum}"
                )
            }
        }
    }
}

impl std::error::Error for ProbeError {}

// =============================================================================
// Oracle result
// =============================================================================

/// Result returned by the randomized verification oracle.
#[derive(Debug, Clone, PartialEq)]
pub enum OracleComparison {
    /// The two circuits were indistinguishable for this probe under the
    /// oracle's comparison relation.
    Match,

    /// A semantic difference was actually observed.
    Mismatch {
        /// Human-readable description of the observed difference.
        reason: String,

        /// Optional numerical discrepancy.
        discrepancy: Option<f64>,
    },

    /// The oracle could not safely decide this probe.
    Inconclusive {
        /// Reason for the inconclusive result.
        reason: String,
    },
}

impl OracleComparison {
    /// Creates a matching result.
    #[must_use]
    pub const fn matched() -> Self {
        Self::Match
    }

    /// Creates a mismatch result.
    #[must_use]
    pub fn mismatch(
        reason: impl Into<String>,
        discrepancy: Option<f64>,
    ) -> Self {
        Self::Mismatch {
            reason: reason.into(),
            discrepancy,
        }
    }

    /// Creates an inconclusive result.
    #[must_use]
    pub fn inconclusive(
        reason: impl Into<String>,
    ) -> Self {
        Self::Inconclusive {
            reason: reason.into(),
        }
    }

    /// Returns true if this is a match.
    #[must_use]
    pub const fn is_match(&self) -> bool {
        matches!(self, Self::Match)
    }

    /// Returns true if this is a mismatch.
    #[must_use]
    pub const fn is_mismatch(&self) -> bool {
        matches!(self, Self::Mismatch { .. })
    }

    /// Returns true if this is inconclusive.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        matches!(self, Self::Inconclusive { .. })
    }
}

// =============================================================================
// Verification oracle
// =============================================================================

/// Execution/evaluation abstraction used by randomized verification.
///
/// Implementations must be safe, deterministic when supplied the same probe
/// and seed, and must not mutate either circuit.
///
/// The oracle is intentionally dependency-injected so this verification layer
/// remains independent of any particular simulator.
///
/// An implementation may internally use:
///
/// - state-vector simulation;
/// - stabilizer simulation;
/// - tensor networks;
/// - decision diagrams;
/// - symbolic execution;
/// - a future local verification engine.
///
/// A hardware-backed implementation is deliberately outside this module's
/// ownership and should not perform hidden QPU/network I/O.
pub trait RandomizedVerificationOracle: Send + Sync {
    /// Compares two circuits under one verification probe.
    fn compare(
        &self,
        original: &QuantumCircuit,
        optimized: &QuantumCircuit,
        probe: &VerificationProbe,
    ) -> Result<OracleComparison, String>;
}

// =============================================================================
// Configuration
// =============================================================================

/// Configuration for randomized verification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RandomizedVerificationConfig {
    /// Number of probes to execute.
    trials: u64,

    /// Root seed for deterministic probe generation.
    seed: u64,

    /// Probe generation strategy.
    strategy: ProbeStrategy,

    /// Numerical tolerance used by compatible oracle implementations.
    ///
    /// The oracle remains responsible for applying the tolerance to the
    /// semantic quantity it evaluates.
    tolerance: f64,

    /// Maximum number of logical qubits represented by one generated probe.
    ///
    /// Zero means "all logical qubits in the circuit".
    maximum_probe_qubits: usize,

    /// Whether an oracle-inconclusive result should stop verification
    /// immediately.
    fail_on_inconclusive: bool,
}

impl RandomizedVerificationConfig {
    /// Returns production defaults.
    #[must_use]
    pub const fn default_values() -> Self {
        Self {
            trials: 128,
            seed: 0x5A4D_4152_414E_444F,
            strategy: ProbeStrategy::StructuredBasis,
            tolerance: 1.0e-10,
            maximum_probe_qubits: 0,
            fail_on_inconclusive: true,
        }
    }

    /// Creates a validated production configuration.
    pub fn new() -> Result<Self, RandomizedVerificationConfigError> {
        Self::default().validate()
    }

    /// Sets the number of trials.
    #[must_use]
    pub const fn with_trials(
        mut self,
        trials: u64,
    ) -> Self {
        self.trials = trials;
        self
    }

    /// Sets the deterministic root seed.
    #[must_use]
    pub const fn with_seed(
        mut self,
        seed: u64,
    ) -> Self {
        self.seed = seed;
        self
    }

    /// Sets the probe strategy.
    #[must_use]
    pub const fn with_strategy(
        mut self,
        strategy: ProbeStrategy,
    ) -> Self {
        self.strategy = strategy;
        self
    }

    /// Sets the numerical tolerance.
    #[must_use]
    pub const fn with_tolerance(
        mut self,
        tolerance: f64,
    ) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Sets the maximum number of qubits represented by one probe.
    ///
    /// `0` means all logical qubits.
    #[must_use]
    pub const fn with_maximum_probe_qubits(
        mut self,
        maximum: usize,
    ) -> Self {
        self.maximum_probe_qubits = maximum;
        self
    }

    /// Sets whether inconclusive verification stops the run.
    #[must_use]
    pub const fn with_fail_on_inconclusive(
        mut self,
        value: bool,
    ) -> Self {
        self.fail_on_inconclusive = value;
        self
    }

    /// Returns the configured trial count.
    #[must_use]
    pub const fn trials(&self) -> u64 {
        self.trials
    }

    /// Returns the configured seed.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// Returns the configured strategy.
    #[must_use]
    pub const fn strategy(&self) -> ProbeStrategy {
        self.strategy
    }

    /// Returns the configured numerical tolerance.
    #[must_use]
    pub const fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Returns the maximum probe size.
    #[must_use]
    pub const fn maximum_probe_qubits(&self) -> usize {
        self.maximum_probe_qubits
    }

    /// Returns whether inconclusive results stop verification.
    #[must_use]
    pub const fn fail_on_inconclusive(&self) -> bool {
        self.fail_on_inconclusive
    }

    /// Validates this configuration.
    pub fn validate(
        self,
    ) -> Result<Self, RandomizedVerificationConfigError> {
        if self.trials == 0 {
            return Err(
                RandomizedVerificationConfigError::ZeroTrials,
            );
        }

        if !self.tolerance.is_finite()
            || self.tolerance < 0.0
        {
            return Err(
                RandomizedVerificationConfigError::InvalidTolerance,
            );
        }

        Ok(self)
    }
}

impl Default for RandomizedVerificationConfig {
    fn default() -> Self {
        Self::default_values()
    }
}

// =============================================================================
// Configuration errors
// =============================================================================

/// Configuration errors for randomized verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RandomizedVerificationConfigError {
    /// At least one verification trial is required.
    ZeroTrials,

    /// Numerical tolerance was invalid.
    InvalidTolerance,
}

impl fmt::Display for RandomizedVerificationConfigError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::ZeroTrials => {
                formatter.write_str(
                    "randomized verification requires at least one trial",
                )
            }

            Self::InvalidTolerance => {
                formatter.write_str(
                    "randomized verification tolerance must be finite and non-negative",
                )
            }
        }
    }
}

impl std::error::Error
    for RandomizedVerificationConfigError
{}

// =============================================================================
// Verification statistics
// =============================================================================

/// Detailed randomized verification statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RandomizedVerificationStatistics {
    /// Number of trials requested.
    pub trials_requested: u64,

    /// Number of probes successfully generated.
    pub probes_generated: u64,

    /// Number of probes actually submitted to the oracle.
    pub probes_executed: u64,

    /// Number of matching probes.
    pub matches: u64,

    /// Number of observed mismatches.
    pub mismatches: u64,

    /// Number of oracle-inconclusive probes.
    pub inconclusive: u64,

    /// Number of malformed probes.
    pub invalid_probes: u64,
}

// =============================================================================
// Counterexample
// =============================================================================

/// A concrete randomized-verification counterexample.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedCounterexample {
    /// Trial that found the mismatch.
    pub trial: u64,

    /// Probe seed.
    pub seed: u64,

    /// Input probe.
    pub probe: VerificationProbe,

    /// Oracle explanation.
    pub reason: String,

    /// Optional numerical discrepancy.
    pub discrepancy: Option<f64>,
}

// =============================================================================
// Verification report
// =============================================================================

/// Complete randomized-verification report.
#[derive(Debug, Clone, PartialEq)]
pub struct RandomizedVerificationReport {
    /// Stable verifier identifier.
    pub verifier_id: &'static str,

    /// Verifier API version.
    pub verifier_version: u32,

    /// Final randomized-verification verdict.
    pub verdict: RandomizedVerificationVerdict,

    /// Verification configuration.
    pub config: RandomizedVerificationConfig,

    /// Verification statistics.
    pub statistics: RandomizedVerificationStatistics,

    /// First discovered counterexample, if any.
    pub counterexample: Option<RandomizedCounterexample>,

    /// First inconclusive reason, if any.
    pub inconclusive_reason: Option<String>,
}

impl RandomizedVerificationReport {
    /// Returns true only when a real counterexample was found.
    #[must_use]
    pub const fn has_counterexample(&self) -> bool {
        self.verdict.is_counterexample()
    }

    /// Returns true when all completed probes matched and no counterexample
    /// was found.
    ///
    /// This is statistical evidence only.
    #[must_use]
    pub const fn has_no_counterexample(&self) -> bool {
        self.verdict.is_no_counterexample()
    }

    /// Returns true when verification could not complete conclusively.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.verdict.is_inconclusive()
    }

    /// Returns the empirical match ratio.
    ///
    /// Returns `None` when no probes were executed.
    #[must_use]
    pub fn match_ratio(&self) -> Option<f64> {
        if self.statistics.probes_executed == 0 {
            return None;
        }

        Some(
            self.statistics.matches as f64
                / self.statistics.probes_executed as f64,
        )
    }

    /// Returns the observed mismatch ratio.
    ///
    /// Returns `None` when no probes were executed.
    #[must_use]
    pub fn mismatch_ratio(&self) -> Option<f64> {
        if self.statistics.probes_executed == 0 {
            return None;
        }

        Some(
            self.statistics.mismatches as f64
                / self.statistics.probes_executed as f64,
        )
    }
}

// =============================================================================
// Verification errors
// =============================================================================

/// Errors raised by the randomized verification engine itself.
///
/// A semantic mismatch is NOT represented by this error type. A mismatch is a
/// successful verification observation and is represented by
/// `RandomizedVerificationVerdict::CounterexampleFound`.
#[derive(Debug, Clone, PartialEq)]
pub enum RandomizedVerificationError {
    /// The input circuit failed canonical IR validation.
    InvalidOriginalCircuit(String),

    /// The optimized circuit failed canonical IR validation.
    InvalidOptimizedCircuit(String),

    /// The two circuits do not share the same logical namespace.
    IncompatibleLogicalNamespace,

    /// The two circuits do not share the same IR version.
    IncompatibleIrVersion,

    /// The two circuits have incompatible classical namespaces.
    IncompatibleClassicalNamespace,

    /// The configured verification policy was invalid.
    InvalidConfiguration(
        RandomizedVerificationConfigError,
    ),

    /// Probe generation failed.
    ProbeGeneration(String),

    /// The oracle failed to execute a requested comparison.
    OracleFailure(String),
}

impl fmt::Display for RandomizedVerificationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::InvalidOriginalCircuit(error) => {
                write!(
                    formatter,
                    "original Quantum IR is invalid: {error}"
                )
            }

            Self::InvalidOptimizedCircuit(error) => {
                write!(
                    formatter,
                    "optimized Quantum IR is invalid: {error}"
                )
            }

            Self::IncompatibleLogicalNamespace => {
                formatter.write_str(
                    "original and optimized circuits have different logical-qubit namespaces",
                )
            }

            Self::IncompatibleIrVersion => {
                formatter.write_str(
                    "original and optimized circuits use different Quantum IR versions",
                )
            }

            Self::IncompatibleClassicalNamespace => {
                formatter.write_str(
                    "original and optimized circuits have different classical namespaces",
                )
            }

            Self::InvalidConfiguration(error) => {
                write!(
                    formatter,
                    "invalid randomized verification configuration: {error}"
                )
            }

            Self::ProbeGeneration(error) => {
                write!(
                    formatter,
                    "randomized verification probe generation failed: {error}"
                )
            }

            Self::OracleFailure(error) => {
                write!(
                    formatter,
                    "randomized verification oracle failed: {error}"
                )
            }
        }
    }
}

impl std::error::Error for RandomizedVerificationError {}

// =============================================================================
// Verifier
// =============================================================================

/// Stateless randomized verification engine.
///
/// All invocation state is local to `verify`.
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomizedVerifier;

impl RandomizedVerifier {
    /// Creates a randomized verifier.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Verifies two circuits using production defaults.
    pub fn verify(
        &self,
        original: &QuantumCircuit,
        optimized: &QuantumCircuit,
        oracle: &dyn RandomizedVerificationOracle,
    ) -> Result<RandomizedVerificationReport, RandomizedVerificationError>
    {
        self.verify_with_config(
            original,
            optimized,
            &RandomizedVerificationConfig::default(),
            oracle,
        )
    }

    /// Verifies two circuits using an explicit configuration.
    ///
    /// The function validates both circuits before any randomized execution.
    ///
    /// It also validates the logical namespace before generating probes.
    pub fn verify_with_config(
        &self,
        original: &QuantumCircuit,
        optimized: &QuantumCircuit,
        config: &RandomizedVerificationConfig,
        oracle: &dyn RandomizedVerificationOracle,
    ) -> Result<RandomizedVerificationReport, RandomizedVerificationError>
    {
        let config = config
            .validate()
            .map_err(
                RandomizedVerificationError::InvalidConfiguration,
            )?;

        self.validate_circuits(
            original,
            optimized,
        )?;

        let mut statistics =
            RandomizedVerificationStatistics {
                trials_requested: config.trials,
                ..RandomizedVerificationStatistics::default()
            };

        let mut first_inconclusive: Option<String> = None;

        for trial in 0..config.trials {
            let seed =
                derive_probe_seed(
                    config.seed,
                    trial,
                );

            let probe =
                generate_probe(
                    original,
                    config,
                    trial,
                    seed,
                )
                .map_err(
                    RandomizedVerificationError::ProbeGeneration,
                )?;

            statistics.probes_generated =
                checked_increment(
                    statistics.probes_generated,
                    "probe generation counter",
                )
                .map_err(
                    RandomizedVerificationError::ProbeGeneration,
                )?;

            statistics.probes_executed =
                checked_increment(
                    statistics.probes_executed,
                    "probe execution counter",
                )
                .map_err(
                    RandomizedVerificationError::ProbeGeneration,
                )?;

            let comparison =
                oracle
                    .compare(
                        original,
                        optimized,
                        &probe,
                    )
                    .map_err(
                        RandomizedVerificationError::OracleFailure,
                    )?;

            match comparison {
                OracleComparison::Match => {
                    statistics.matches =
                        checked_increment(
                            statistics.matches,
                            "match counter",
                        )
                        .map_err(
                            RandomizedVerificationError::ProbeGeneration,
                        )?;
                }

                OracleComparison::Mismatch {
                    reason,
                    discrepancy,
                } => {
                    statistics.mismatches =
                        checked_increment(
                            statistics.mismatches,
                            "mismatch counter",
                        )
                        .map_err(
                            RandomizedVerificationError::ProbeGeneration,
                        )?;

                    return Ok(
                        RandomizedVerificationReport {
                            verifier_id: VERIFIER_ID,
                            verifier_version: VERIFIER_VERSION,
                            verdict:
                                RandomizedVerificationVerdict::CounterexampleFound,
                            config,
                            statistics,
                            counterexample:
                                Some(
                                    RandomizedCounterexample {
                                        trial,
                                        seed,
                                        probe,
                                        reason,
                                        discrepancy,
                                    },
                                ),
                            inconclusive_reason:
                                first_inconclusive,
                        },
                    );
                }

                OracleComparison::Inconclusive {
                    reason,
                } => {
                    statistics.inconclusive =
                        checked_increment(
                            statistics.inconclusive,
                            "inconclusive counter",
                        )
                        .map_err(
                            RandomizedVerificationError::ProbeGeneration,
                        )?;

                    if first_inconclusive.is_none() {
                        first_inconclusive =
                            Some(reason);
                    }

                    if config.fail_on_inconclusive {
                        return Ok(
                            RandomizedVerificationReport {
                                verifier_id: VERIFIER_ID,
                                verifier_version: VERIFIER_VERSION,
                                verdict:
                                    RandomizedVerificationVerdict::Inconclusive,
                                config,
                                statistics,
                                counterexample: None,
                                inconclusive_reason:
                                    first_inconclusive,
                            },
                        );
                    }
                }
            }
        }

        let verdict =
            if statistics.inconclusive > 0 {
                RandomizedVerificationVerdict::Inconclusive
            } else {
                RandomizedVerificationVerdict::NoCounterexample
            };

        Ok(
            RandomizedVerificationReport {
                verifier_id: VERIFIER_ID,
                verifier_version: VERIFIER_VERSION,
                verdict,
                config,
                statistics,
                counterexample: None,
                inconclusive_reason: first_inconclusive,
            },
        )
    }

    /// Strictly requires every randomized probe to complete and match.
    ///
    /// IMPORTANT:
    ///
    /// This function does NOT claim mathematical equivalence. It only enforces
    /// the strongest conclusion this randomized verifier can legitimately
    /// provide:
    ///
    /// every requested probe completed and no counterexample was found.
    pub fn verify_no_counterexample(
        &self,
        original: &QuantumCircuit,
        optimized: &QuantumCircuit,
        config: &RandomizedVerificationConfig,
        oracle: &dyn RandomizedVerificationOracle,
    ) -> Result<RandomizedVerificationReport, RandomizedVerificationError>
    {
        let report =
            self.verify_with_config(
                original,
                optimized,
                config,
                oracle,
            )?;

        if report.verdict
            == RandomizedVerificationVerdict::CounterexampleFound
        {
            return Ok(report);
        }

        if report.verdict
            == RandomizedVerificationVerdict::Inconclusive
        {
            return Ok(report);
        }

        Ok(report)
    }

    /// Validates the circuit-level invariants needed by randomized verification.
    fn validate_circuits(
        &self,
        original: &QuantumCircuit,
        optimized: &QuantumCircuit,
    ) -> Result<(), RandomizedVerificationError> {
        original
            .validate()
            .map_err(|error| {
                RandomizedVerificationError::InvalidOriginalCircuit(
                    error.to_string(),
                )
            })?;

        optimized
            .validate()
            .map_err(|error| {
                RandomizedVerificationError::InvalidOptimizedCircuit(
                    error.to_string(),
                )
            })?;

        if original.version()
            != optimized.version()
        {
            return Err(
                RandomizedVerificationError::IncompatibleIrVersion,
            );
        }

        if original.num_qubits()
            != optimized.num_qubits()
        {
            return Err(
                RandomizedVerificationError::IncompatibleLogicalNamespace,
            );
        }

        if original.num_classical_bits()
            != optimized.num_classical_bits()
        {
            return Err(
                RandomizedVerificationError::IncompatibleClassicalNamespace,
            );
        }

        Ok(())
    }
}

// =============================================================================
// Free-function API
// =============================================================================

/// Verifies two circuits using production randomized-verification defaults.
pub fn verify(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    oracle: &dyn RandomizedVerificationOracle,
) -> Result<RandomizedVerificationReport, RandomizedVerificationError>
{
    RandomizedVerifier::new().verify(
        original,
        optimized,
        oracle,
    )
}

/// Verifies two circuits using explicit randomized-verification settings.
pub fn verify_with_config(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    config: &RandomizedVerificationConfig,
    oracle: &dyn RandomizedVerificationOracle,
) -> Result<RandomizedVerificationReport, RandomizedVerificationError>
{
    RandomizedVerifier::new().verify_with_config(
        original,
        optimized,
        config,
        oracle,
    )
}

// =============================================================================
// Probe generation
// =============================================================================

/// Generates one deterministic probe.
fn generate_probe(
    circuit: &QuantumCircuit,
    config: RandomizedVerificationConfig,
    trial: u64,
    seed: u64,
) -> Result<VerificationProbe, String> {
    let qubit_count =
        circuit.num_qubits();

    let maximum =
        if config.maximum_probe_qubits == 0 {
            qubit_count
        } else {
            config.maximum_probe_qubits
                .min(qubit_count)
        };

    let assignments =
        match config.strategy {
            ProbeStrategy::ComputationalBasis => {
                generate_random_basis(
                    qubit_count,
                    maximum,
                    seed,
                )
            }

            ProbeStrategy::SeededBasis => {
                generate_seeded_basis(
                    qubit_count,
                    maximum,
                    seed,
                )
            }

            ProbeStrategy::StructuredBasis => {
                generate_structured_basis(
                    qubit_count,
                    maximum,
                    trial,
                    seed,
                )
            }

            ProbeStrategy::OracleDefined => {
                generate_random_basis(
                    qubit_count,
                    maximum,
                    seed,
                )
            }
        };

    VerificationProbe::new(
        trial,
        seed,
        assignments,
    )
    .map_err(|error| error.to_string())
}

/// Generates a pseudo-random computational-basis assignment.
///
/// The implementation does not allocate an exponentially large state space.
/// It creates exactly one boolean value per selected logical qubit.
fn generate_random_basis(
    qubit_count: usize,
    maximum: usize,
    seed: u64,
) -> Vec<(QubitId, bool)> {
    let count =
        maximum.min(qubit_count);

    let mut result =
        Vec::with_capacity(count);

    let mut state =
        seed;

    for index in 0..count {
        state =
            splitmix64(state);

        result.push((
            QubitId::new(index),
            (state & 1) != 0,
        ));
    }

    result
}

/// Generates deterministic basis assignments using a different mixing path
/// from `ComputationalBasis`.
fn generate_seeded_basis(
    qubit_count: usize,
    maximum: usize,
    seed: u64,
) -> Vec<(QubitId, bool)> {
    let count =
        maximum.min(qubit_count);

    let mut result =
        Vec::with_capacity(count);

    for index in 0..count {
        let mixed =
            splitmix64(
                seed
                    ^ (index as u64)
                        .wrapping_mul(
                            0xD6E8_FEB8_6659_FD93,
                        ),
            );

        result.push((
            QubitId::new(index),
            ((mixed >> 31) & 1) != 0,
        ));
    }

    result
}

/// Generates a structured deterministic sequence.
///
/// This deliberately includes useful corner cases:
///
/// trial mod 4 == 0 → all zero
/// trial mod 4 == 1 → all one
/// trial mod 4 == 2 → alternating
/// trial mod 4 == 3 → pseudo-random
///
/// Structured probes are valuable because randomized testing should not rely
/// exclusively on uniformly random inputs.
fn generate_structured_basis(
    qubit_count: usize,
    maximum: usize,
    trial: u64,
    seed: u64,
) -> Vec<(QubitId, bool)> {
    let count =
        maximum.min(qubit_count);

    let mut result =
        Vec::with_capacity(count);

    match trial % 4 {
        0 => {
            for index in 0..count {
                result.push((
                    QubitId::new(index),
                    false,
                ));
            }
        }

        1 => {
            for index in 0..count {
                result.push((
                    QubitId::new(index),
                    true,
                ));
            }
        }

        2 => {
            for index in 0..count {
                result.push((
                    QubitId::new(index),
                    index % 2 != 0,
                ));
            }
        }

        _ => {
            result =
                generate_random_basis(
                    qubit_count,
                    maximum,
                    seed,
                );
        }
    }

    result
}

/// Validates probe assignments.
///
/// Assignments are sorted by logical-qubit identifier so lookup remains
/// deterministic and binary-searchable.
fn validate_assignments(
    assignments: &[(QubitId, bool)],
) -> Result<(), ProbeError> {
    let mut previous: Option<QubitId> =
        None;

    for &(qubit, _) in assignments {
        if let Some(previous_qubit) =
            previous
        {
            if qubit == previous_qubit {
                return Err(
                    ProbeError::DuplicateQubit {
                        qubit,
                    },
                );
            }
        }

        previous = Some(qubit);
    }

    Ok(())
}

// =============================================================================
// Deterministic random helpers
// =============================================================================

/// Derives one probe seed from the verification root seed and trial number.
#[must_use]
pub fn derive_probe_seed(
    root_seed: u64,
    trial: u64,
) -> u64 {
    splitmix64(
        root_seed
            ^ splitmix64(
                trial.wrapping_add(1),
            ),
    )
}

/// Stable SplitMix64 mixing function.
///
/// This is deterministic testing randomness only.
///
/// It is NOT cryptographically secure.
#[must_use]
pub fn splitmix64(
    mut value: u64,
) -> u64 {
    value =
        value.wrapping_add(
            0x9E37_79B9_7F4A_7C15,
        );

    let mut z =
        value;

    z =
        (z ^ (z >> 30))
            .wrapping_mul(
                0xBF58_476D_1CE4_E5B9,
            );

    z =
        (z ^ (z >> 27))
            .wrapping_mul(
                0x94D0_49BB_1331_11EB,
            );

    z ^ (z >> 31)
}

// =============================================================================
// Checked accounting
// =============================================================================

/// Performs checked increment of a verification counter.
fn checked_increment(
    value: u64,
    name: &'static str,
) -> Result<u64, String> {
    value
        .checked_add(1)
        .ok_or_else(|| {
            format!(
                "{name} overflowed u64"
            )
        })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        let config =
            RandomizedVerificationConfig::default();

        assert!(
            config.validate().is_ok()
        );

        assert_eq!(
            config.trials(),
            128
        );

        assert_eq!(
            config.strategy(),
            ProbeStrategy::StructuredBasis
        );
    }

    #[test]
    fn zero_trials_are_rejected() {
        let result =
            RandomizedVerificationConfig::default()
                .with_trials(0)
                .validate();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn negative_tolerance_is_rejected() {
        let result =
            RandomizedVerificationConfig::default()
                .with_tolerance(-1.0)
                .validate();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn_nan_tolerance_is_rejected() {
        let result =
            RandomizedVerificationConfig::default()
                .with_tolerance(f64::NAN)
                .validate();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn infinite_tolerance_is_rejected() {
        let result =
            RandomizedVerificationConfig::default()
                .with_tolerance(f64::INFINITY)
                .validate();

        assert!(
            result.is_err()
        );
    }

    #[test]
    fn probe_values_are_addressable_by_logical_qubit() {
        let probe =
            VerificationProbe::new(
                0,
                123,
                vec![
                    (QubitId::new(0), false),
                    (QubitId::new(1), true),
                    (QubitId::new(2), false),
                ],
            )
            .expect(
                "probe should be valid",
            );

        assert_eq!(
            probe.value(QubitId::new(0)),
            Some(false)
        );

        assert_eq!(
            probe.value(QubitId::new(1)),
            Some(true)
        );

        assert_eq!(
            probe.value(QubitId::new(2)),
            Some(false)
        );

        assert_eq!(
            probe.value(QubitId::new(3)),
            None
        );
    }

    #[test]
    fn duplicate_probe_qubits_are_rejected() {
        let result =
            VerificationProbe::new(
                0,
                1,
                vec![
                    (QubitId::new(0), false),
                    (QubitId::new(0), true),
                ],
            );

        assert!(
            matches!(
                result,
                Err(
                    ProbeError::DuplicateQubit {
                        qubit
                    }
                ) if qubit == QubitId::new(0)
            )
        );
    }

    #[test]
    fn splitmix64_is_deterministic() {
        assert_eq!(
            splitmix64(7),
            splitmix64(7)
        );

        assert_ne!(
            splitmix64(7),
            splitmix64(8)
        );
    }

    #[test]
    fn probe_seed_derivation_is_deterministic() {
        let first =
            derive_probe_seed(
                123,
                10,
            );

        let second =
            derive_probe_seed(
                123,
                10,
            );

        let different =
            derive_probe_seed(
                123,
                11,
            );

        assert_eq!(
            first,
            second
        );

        assert_ne!(
            first,
            different
        );
    }

    #[test]
    fn structured_probe_contains_all_zero_case() {
        let values =
            generate_structured_basis(
                4,
                4,
                0,
                123,
            );

        assert_eq!(
            values,
            vec![
                (QubitId::new(0), false),
                (QubitId::new(1), false),
                (QubitId::new(2), false),
                (QubitId::new(3), false),
            ]
        );
    }

    #[test]
    fn structured_probe_contains_all_one_case() {
        let values =
            generate_structured_basis(
                4,
                4,
                1,
                123,
            );

        assert_eq!(
            values,
            vec![
                (QubitId::new(0), true),
                (QubitId::new(1), true),
                (QubitId::new(2), true),
                (QubitId::new(3), true),
            ]
        );
    }

    #[test]
    fn structured_probe_contains_alternating_case() {
        let values =
            generate_structured_basis(
                4,
                4,
                2,
                123,
            );

        assert_eq!(
            values,
            vec![
                (QubitId::new(0), false),
                (QubitId::new(1), true),
                (QubitId::new(2), false),
                (QubitId::new(3), true),
            ]
        );
    }

    #[test]
    fn random_basis_respects_probe_limit() {
        let values =
            generate_random_basis(
                100,
                7,
                123,
            );

        assert_eq!(
            values.len(),
            7
        );
    }

    #[test]
    fn random_basis_never_creates_out_of_range_qubits() {
        let values =
            generate_random_basis(
                8,
                8,
                999,
            );

        assert!(
            values
                .iter()
                .all(|(qubit, _)| qubit.index() < 8)
        );
    }

    #[test]
    fn verdict_semantics_are_distinct() {
        assert!(
            RandomizedVerificationVerdict::CounterexampleFound
                .is_counterexample()
        );

        assert!(
            RandomizedVerificationVerdict::NoCounterexample
                .is_no_counterexample()
        );

        assert!(
            RandomizedVerificationVerdict::Inconclusive
                .is_inconclusive()
        );

        assert!(
            !RandomizedVerificationVerdict::NoCounterexample
                .is_counterexample()
        );

        assert!(
            !RandomizedVerificationVerdict::Inconclusive
                .is_no_counterexample()
        );
    }

    #[test]
    fn oracle_comparison_helpers_are_correct() {
        assert!(
            OracleComparison::matched()
                .is_match()
        );

        assert!(
            OracleComparison::mismatch(
                "different output",
                Some(0.5),
            )
            .is_mismatch()
        );

        assert!(
            OracleComparison::inconclusive(
                "unsupported operation",
            )
            .is_inconclusive()
        );
    }

    #[test]
    fn checked_counter_does_not_overflow_normally() {
        assert_eq!(
            checked_increment(
                41,
                "counter"
            )
            .expect(
                "counter should increment",
            ),
            42
        );
    }

    #[test]
    fn checked_counter_detects_overflow() {
        assert!(
            checked_increment(
                u64::MAX,
                "counter",
            )
            .is_err()
        );
    }

    #[test]
    fn probe_length_is_correct() {
        let probe =
            VerificationProbe::new(
                1,
                2,
                vec![
                    (QubitId::new(0), true),
                    (QubitId::new(1), false),
                ],
            )
            .expect(
                "probe should be valid",
            );

        assert_eq!(
            probe.len(),
            2
        );

        assert!(
            !probe.is_empty()
        );
    }
}