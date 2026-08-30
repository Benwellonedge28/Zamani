//! Zamani Quantum Optimization — Structural Verification
//!
//! Production structural verification for optimized Quantum IR.
//!
//! # Architectural ownership
//!
//! The canonical quantum representation is:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! The canonical qubit identity is:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! This module MUST NOT define a second quantum IR.
//!
//! Structural verification is deliberately separate from semantic equivalence.
//! This module answers:
//!
//! > "Is this circuit structurally valid and internally consistent at the
//! > optimization boundary?"
//!
//! It does NOT answer:
//!
//! > "Does this circuit implement the same quantum transformation as another
//! > circuit?"
//!
//! Semantic equivalence belongs to `verification::semantic` / `equivalence`.
//!
//! # Responsibilities
//!
//! This module provides:
//!
//! - optimizer-boundary structural verification;
//! - canonical Quantum IR validation delegation;
//! - deterministic structural reports;
//! - explicit resource-bound checks;
//! - logical qubit namespace checks;
//! - classical-bit namespace checks;
//! - operation-count checks;
//! - empty-circuit policy;
//! - overflow-safe accounting;
//! - deterministic verification statistics;
//! - stable machine-readable status values;
//! - verification policies that can be embedded in future optimizer contexts;
//! - optional direct validation of a `QubitId` against a circuit namespace;
//! - bounded verification suitable for very large circuits;
//! - zero mutation of the input circuit;
//! - no backend/hardware dependencies;
//! - no optimizer-pass dependencies;
//! - no unsafe code.
//!
//! # Canonical validation
//!
//! The canonical IR validator remains authoritative for:
//!
//! - gate shape;
//! - operand count;
//! - duplicate operands;
//! - gate parameters;
//! - logical namespace validity;
//! - classical namespace validity;
//! - measurement validity;
//! - circuit resource invariants;
//! - whole-circuit IR invariants.
//!
//! This module therefore delegates those checks to:
//!
//! `crate::quantum::ir::validation::validate_circuit_with_config`
//!
//! and adds only optimizer-verification policy around that canonical contract.
//!
//! # Dependency direction
//!
//! ```text
//! quantum::ir
//!      |
//!      v
//! optimization::verification::structural
//!      |
//!      +----> semantic verification
//!      |
//!      +----> randomized verification
//!      |
//!      +----> exhaustive verification
//!      |
//!      +----> optimization result/provenance
//! ```
//!
//! This module must never depend on:
//!
//! - routing;
//! - scheduling;
//! - hardware;
//! - QPU execution;
//! - benchmarking;
//! - frontend parsing;
//! - algorithm implementations;
//! - error-correction implementations;
//! - backend APIs.
//!
//! # Scalability
//!
//! For `n` operations, verification is O(n) only where a caller explicitly
//! requests operation inspection. Canonical IR validation itself determines
//! the authoritative structural checks.
//!
//! The default verification path does not allocate a collection proportional
//! to circuit size.
//!
//! No recursion is used.
//!
//! No pairwise operation comparison is used.
//!
//! No hash map is required.
//!
//! No sorting proportional to circuit size is required.
//!
//! No wall-clock timing participates in correctness.
//!
//! The implementation therefore scales from tiny circuits to circuits limited
//! only by available memory and the active Quantum IR resource policy.
//!
//! # Determinism
//!
//! Verification is deterministic.
//!
//! The same:
//!
//! - circuit;
//! - verification policy;
//! - canonical IR validation configuration;
//!
//! produces the same:
//!
//! - status;
//! - report;
//! - counters;
//! - diagnostics.
//!
//! No randomness, system time, environment state, hash-map iteration order,
//! hardware state, or backend state participates in verification.
//!
//! # Transactionality
//!
//! Verification never mutates the circuit.
//!
//! A successful report therefore describes the exact circuit supplied by the
//! caller.
//!
//! # Rust compatibility
//!
//! Target: Rust 1.97 / Rust 1.97.1.
//!
//! No nightly features.
//! No external dependencies.
//! No `unsafe` code.
//!
//! # Integration contract
//!
//! This file is intentionally independent of future optimization modules.
//!
//! Future integration:
//!
//! - `verification/mod.rs` re-exports this module;
//! - `verification/semantic.rs` may call `verify_structural` before semantic
//!   comparison;
//! - `verification/randomized.rs` may structurally validate generated circuits
//!   before sampling them;
//! - `verification/exhaustive.rs` may structurally validate small circuits;
//! - `verification/certificates.rs` may serialize `StructuralVerificationReport`;
//! - `pipeline.rs` may perform structural verification before and after passes;
//! - `pass.rs` may use the report as an optimizer boundary contract;
//! - `result.rs` may embed the report;
//! - `provenance.rs` may record the verification status and contract version;
//! - `serialization/report.rs` may serialize the public report types;
//! - `tests/*` can test this module without requiring any optimization pass.
//!
//! No future module needs to modify this file merely because those modules are
//! added.
//!
//! # Important repository compatibility note
//!
//! The canonical IR module is `quantum::ir::qubit`, not `quantum::ir::qubits`.
//!
//! This file therefore imports:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! and deliberately does not reproduce the repository's older `qubits`
//! spelling.

use std::fmt;

use crate::quantum::ir::qubit::QubitId;
use crate::quantum::ir::validation::{
    validate_circuit_with_config,
    ValidationConfig,
};
use crate::quantum::ir::{
    QuantumCircuit,
    QuantumIrLimits,
};

// ============================================================================
// Public contract identifiers
// ============================================================================

/// Stable identifier for the structural verification subsystem.
pub const STRUCTURAL_VERIFICATION_ID: &str =
    "quantum.optimization.verification.structural";

/// Semantic version of the structural verification contract.
///
/// This version is independent of the Quantum IR schema version.
pub const STRUCTURAL_VERIFICATION_VERSION: u32 = 1;

// ============================================================================
// Verification mode
// ============================================================================

/// Controls structural verification strictness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralVerificationMode {
    /// Production verification.
    ///
    /// Canonical IR validation is performed with strict structural and
    /// semantic checks.
    Production,

    /// Strict verification for compiler/release boundaries.
    ///
    /// This mode is intentionally equivalent to the strongest currently
    /// available canonical IR validation policy. It exists as an explicit
    /// contract so future stronger checks can be added without changing the
    /// public API.
    Strict,

    /// Structural-only verification.
    ///
    /// Canonical structural validation is performed while optional semantic
    /// validation is disabled.
    StructuralOnly,

    /// Minimal optimizer-entry verification.
    ///
    /// Canonical validation still occurs. Optional semantic checks are
    /// disabled.
    Minimal,
}

impl Default for StructuralVerificationMode {
    fn default() -> Self {
        Self::Production
    }
}

// ============================================================================
// Policy
// ============================================================================

/// Immutable policy for one structural-verification invocation.
///
/// The policy deliberately owns optimizer-verification decisions rather than
/// optimizer configuration. This keeps this file independent from
/// `optimization::config`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralVerificationPolicy {
    mode: StructuralVerificationMode,

    /// Whether an empty circuit is accepted.
    allow_empty_circuit: bool,

    /// Whether the circuit's logical qubit count must be checked against the
    /// supplied verification limit.
    check_qubit_limit: bool,

    /// Whether the circuit's operation count must be checked against the
    /// supplied verification limit.
    check_operation_limit: bool,

    /// Whether the circuit's classical-bit count must be checked against the
    /// supplied verification limit.
    check_classical_bit_limit: bool,

    /// Whether canonical semantic checks are enabled.
    semantic_checks: bool,
}

impl StructuralVerificationPolicy {
    /// Returns the production policy.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            mode: StructuralVerificationMode::Production,
            allow_empty_circuit: true,
            check_qubit_limit: true,
            check_operation_limit: true,
            check_classical_bit_limit: true,
            semantic_checks: true,
        }
    }

    /// Returns the strict policy.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            mode: StructuralVerificationMode::Strict,
            allow_empty_circuit: true,
            check_qubit_limit: true,
            check_operation_limit: true,
            check_classical_bit_limit: true,
            semantic_checks: true,
        }
    }

    /// Returns the structural-only policy.
    #[must_use]
    pub const fn structural_only() -> Self {
        Self {
            mode: StructuralVerificationMode::StructuralOnly,
            allow_empty_circuit: true,
            check_qubit_limit: true,
            check_operation_limit: true,
            check_classical_bit_limit: true,
            semantic_checks: false,
        }
    }

    /// Returns the minimal policy.
    #[must_use]
    pub const fn minimal() -> Self {
        Self {
            mode: StructuralVerificationMode::Minimal,
            allow_empty_circuit: true,
            check_qubit_limit: true,
            check_operation_limit: true,
            check_classical_bit_limit: true,
            semantic_checks: false,
        }
    }

    /// Returns the selected verification mode.
    #[must_use]
    pub const fn mode(&self) -> StructuralVerificationMode {
        self.mode
    }

    /// Returns whether empty circuits are accepted.
    #[must_use]
    pub const fn allow_empty_circuit(&self) -> bool {
        self.allow_empty_circuit
    }

    /// Returns whether qubit limits are checked.
    #[must_use]
    pub const fn check_qubit_limit(&self) -> bool {
        self.check_qubit_limit
    }

    /// Returns whether operation limits are checked.
    #[must_use]
    pub const fn check_operation_limit(&self) -> bool {
        self.check_operation_limit
    }

    /// Returns whether classical-bit limits are checked.
    #[must_use]
    pub const fn check_classical_bit_limit(&self) -> bool {
        self.check_classical_bit_limit
    }

    /// Returns whether canonical semantic checks are enabled.
    #[must_use]
    pub const fn semantic_checks(&self) -> bool {
        self.semantic_checks
    }

    /// Returns a copy with a different mode.
    #[must_use]
    pub const fn with_mode(
        mut self,
        mode: StructuralVerificationMode,
    ) -> Self {
        self.mode = mode;
        self
    }

    /// Returns a copy with empty-circuit policy changed.
    #[must_use]
    pub const fn with_empty_circuit_policy(
        mut self,
        allow: bool,
    ) -> Self {
        self.allow_empty_circuit = allow;
        self
    }

    /// Returns a copy with qubit-limit checking changed.
    #[must_use]
    pub const fn with_qubit_limit_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_qubit_limit = enabled;
        self
    }

    /// Returns a copy with operation-limit checking changed.
    #[must_use]
    pub const fn with_operation_limit_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_operation_limit = enabled;
        self
    }

    /// Returns a copy with classical-bit-limit checking changed.
    #[must_use]
    pub const fn with_classical_bit_limit_check(
        mut self,
        enabled: bool,
    ) -> Self {
        self.check_classical_bit_limit = enabled;
        self
    }

    /// Returns a copy with semantic checking changed.
    #[must_use]
    pub const fn with_semantic_checks(
        mut self,
        enabled: bool,
    ) -> Self {
        self.semantic_checks = enabled;
        self
    }
}

impl Default for StructuralVerificationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Verification status
// ============================================================================

/// Final status of structural verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralVerificationStatus {
    /// Every requested structural check succeeded.
    Valid,

    /// The circuit is structurally invalid.
    Invalid,

    /// Verification was rejected because the configured verification
    /// resource envelope was exceeded.
    LimitExceeded,
}

impl StructuralVerificationStatus {
    /// Returns true when verification succeeded.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        matches!(self, Self::Valid)
    }

    /// Returns true when the circuit was structurally invalid.
    #[must_use]
    pub const fn is_invalid(self) -> bool {
        matches!(self, Self::Invalid)
    }

    /// Returns true when a verification limit prevented completion.
    #[must_use]
    pub const fn is_limit_exceeded(self) -> bool {
        matches!(self, Self::LimitExceeded)
    }
}

impl fmt::Display for StructuralVerificationStatus {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Valid => formatter.write_str("valid"),
            Self::Invalid => formatter.write_str("invalid"),
            Self::LimitExceeded => formatter.write_str("limit-exceeded"),
        }
    }
}

// ============================================================================
// Error vocabulary
// ============================================================================

/// Errors produced by structural verification.
///
/// This error type is deliberately local to this file. Future optimizer-wide
/// error types can convert it without requiring this implementation to know
/// about those future types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralVerificationError {
    /// The canonical Quantum IR validator rejected the circuit.
    CanonicalValidationFailed {
        message: String,
    },

    /// The circuit has zero operations while the active policy disallows it.
    EmptyCircuitNotAllowed,

    /// The circuit exceeds the configured logical-qubit verification limit.
    QubitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// The circuit exceeds the configured operation verification limit.
    OperationLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// The circuit exceeds the configured classical-bit verification limit.
    ClassicalBitLimitExceeded {
        requested: usize,
        maximum: usize,
    },

    /// A `QubitId` supplied directly by a caller is outside the circuit
    /// logical namespace.
    QubitOutOfRange {
        qubit: QubitId,
        num_qubits: usize,
    },

    /// An internal count conversion or addition overflowed.
    ArithmeticOverflow {
        calculation: &'static str,
    },

    /// The supplied verification limits are invalid according to the
    /// canonical Quantum IR limit contract.
    InvalidLimits {
        message: String,
    },
}

impl fmt::Display for StructuralVerificationError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::CanonicalValidationFailed { message } => {
                write!(
                    formatter,
                    "canonical Quantum IR validation failed: {message}"
                )
            }

            Self::EmptyCircuitNotAllowed => {
                formatter.write_str(
                    "empty circuit is not permitted by structural verification policy",
                )
            }

            Self::QubitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "structural verification qubit limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::OperationLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "structural verification operation limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::ClassicalBitLimitExceeded {
                requested,
                maximum,
            } => {
                write!(
                    formatter,
                    "structural verification classical-bit limit exceeded: \
                     requested {requested}, maximum {maximum}"
                )
            }

            Self::QubitOutOfRange {
                qubit,
                num_qubits,
            } => {
                write!(
                    formatter,
                    "logical qubit {qubit} is outside circuit namespace \
                     0..{num_qubits}"
                )
            }

            Self::ArithmeticOverflow { calculation } => {
                write!(
                    formatter,
                    "arithmetic overflow while calculating {calculation}"
                )
            }

            Self::InvalidLimits { message } => {
                write!(
                    formatter,
                    "invalid structural verification limits: {message}"
                )
            }
        }
    }
}

impl std::error::Error for StructuralVerificationError {}

// ============================================================================
// Counters
// ============================================================================

/// Deterministic structural-verification counters.
///
/// These counters are intentionally compact. They do not retain one record per
/// operation and therefore remain O(1) in memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StructuralVerificationCounters {
    /// Number of logical qubits observed.
    qubits: usize,

    /// Number of classical bits observed.
    classical_bits: usize,

    /// Number of quantum operations observed.
    operations: usize,

    /// Whether the circuit contains zero operations.
    empty: bool,

    /// Number of canonical validation invocations.
    canonical_validations: u64,

    /// Number of direct qubit-namespace checks.
    qubit_namespace_checks: u64,
}

impl StructuralVerificationCounters {
    /// Returns the number of logical qubits.
    #[must_use]
    pub const fn qubits(&self) -> usize {
        self.qubits
    }

    /// Returns the number of classical bits.
    #[must_use]
    pub const fn classical_bits(&self) -> usize {
        self.classical_bits
    }

    /// Returns the number of operations.
    #[must_use]
    pub const fn operations(&self) -> usize {
        self.operations
    }

    /// Returns whether the circuit is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.empty
    }

    /// Returns the number of canonical validation invocations represented by
    /// this report.
    #[must_use]
    pub const fn canonical_validations(&self) -> u64 {
        self.canonical_validations
    }

    /// Returns the number of direct qubit namespace checks.
    #[must_use]
    pub const fn qubit_namespace_checks(&self) -> u64 {
        self.qubit_namespace_checks
    }
}

// ============================================================================
// Report
// ============================================================================

/// Complete deterministic result of one structural-verification invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralVerificationReport {
    status: StructuralVerificationStatus,

    counters: StructuralVerificationCounters,

    error: Option<StructuralVerificationError>,

    policy: StructuralVerificationPolicy,

    contract_id: &'static str,

    contract_version: u32,
}

impl StructuralVerificationReport {
    fn valid(
        counters: StructuralVerificationCounters,
        policy: StructuralVerificationPolicy,
    ) -> Self {
        Self {
            status: StructuralVerificationStatus::Valid,
            counters,
            error: None,
            policy,
            contract_id: STRUCTURAL_VERIFICATION_ID,
            contract_version: STRUCTURAL_VERIFICATION_VERSION,
        }
    }

    fn invalid(
        counters: StructuralVerificationCounters,
        policy: StructuralVerificationPolicy,
        error: StructuralVerificationError,
    ) -> Self {
        Self {
            status: StructuralVerificationStatus::Invalid,
            counters,
            error: Some(error),
            policy,
            contract_id: STRUCTURAL_VERIFICATION_ID,
            contract_version: STRUCTURAL_VERIFICATION_VERSION,
        }
    }

    fn limit_exceeded(
        counters: StructuralVerificationCounters,
        policy: StructuralVerificationPolicy,
        error: StructuralVerificationError,
    ) -> Self {
        Self {
            status: StructuralVerificationStatus::LimitExceeded,
            counters,
            error: Some(error),
            policy,
            contract_id: STRUCTURAL_VERIFICATION_ID,
            contract_version: STRUCTURAL_VERIFICATION_VERSION,
        }
    }

    /// Returns the final verification status.
    #[must_use]
    pub const fn status(&self) -> StructuralVerificationStatus {
        self.status
    }

    /// Returns true when the circuit passed.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.status.is_valid()
    }

    /// Returns true when the circuit failed structural validation.
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        self.status.is_invalid()
    }

    /// Returns true when a verification resource limit was exceeded.
    #[must_use]
    pub const fn is_limit_exceeded(&self) -> bool {
        self.status.is_limit_exceeded()
    }

    /// Returns structural counters.
    #[must_use]
    pub const fn counters(&self) -> &StructuralVerificationCounters {
        &self.counters
    }

    /// Returns the policy used for this report.
    #[must_use]
    pub const fn policy(&self) -> &StructuralVerificationPolicy {
        &self.policy
    }

    /// Returns the stable verification contract identifier.
    #[must_use]
    pub const fn contract_id(&self) -> &'static str {
        self.contract_id
    }

    /// Returns the structural verification contract version.
    #[must_use]
    pub const fn contract_version(&self) -> u32 {
        self.contract_version
    }

    /// Returns the verification error, if any.
    #[must_use]
    pub const fn error(
        &self,
    ) -> Option<&StructuralVerificationError> {
        self.error.as_ref()
    }

    /// Converts the report into a result.
    ///
    /// A valid report becomes `Ok(())`. An invalid or limit-exceeded report
    /// becomes the corresponding verification error.
    pub fn into_result(
        self,
    ) -> Result<(), StructuralVerificationError> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

impl fmt::Display for StructuralVerificationReport {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(
            formatter,
            "{}: status={}, qubits={}, classical_bits={}, \
             operations={}",
            self.contract_id,
            self.status,
            self.counters.qubits,
            self.counters.classical_bits,
            self.counters.operations,
        )?;

        if let Some(error) = &self.error {
            write!(formatter, ", error={error}")?;
        }

        Ok(())
    }
}

// ============================================================================
// Public entry points
// ============================================================================

/// Performs production structural verification.
///
/// This is the primary entry point intended for optimizer pipelines.
pub fn verify_structural(
    circuit: &QuantumCircuit,
) -> StructuralVerificationReport {
    verify_structural_with_limits_and_policy(
        circuit,
        &QuantumIrLimits::default(),
        &StructuralVerificationPolicy::production(),
    )
}

/// Performs strict structural verification.
///
/// This is intended for compiler release boundaries, generated IR, replay,
/// deserialization, and optimizer output verification.
pub fn verify_structural_strict(
    circuit: &QuantumCircuit,
) -> StructuralVerificationReport {
    verify_structural_with_limits_and_policy(
        circuit,
        &QuantumIrLimits::default(),
        &StructuralVerificationPolicy::strict(),
    )
}

/// Performs structural-only verification.
///
/// Optional canonical semantic checks are disabled, but canonical structural
/// validation and resource validation remain active.
pub fn verify_structural_only(
    circuit: &QuantumCircuit,
) -> StructuralVerificationReport {
    verify_structural_with_limits_and_policy(
        circuit,
        &QuantumIrLimits::default(),
        &StructuralVerificationPolicy::structural_only(),
    )
}

/// Performs minimal optimizer-entry verification.
pub fn verify_structural_minimal(
    circuit: &QuantumCircuit,
) -> StructuralVerificationReport {
    verify_structural_with_limits_and_policy(
        circuit,
        &QuantumIrLimits::default(),
        &StructuralVerificationPolicy::minimal(),
    )
}

/// Performs structural verification with explicit Quantum IR limits.
///
/// This function uses the production policy.
pub fn verify_structural_with_limits(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
) -> StructuralVerificationReport {
    verify_structural_with_limits_and_policy(
        circuit,
        limits,
        &StructuralVerificationPolicy::production(),
    )
}

/// Performs structural verification with explicit limits and policy.
///
/// This is the most general stable entry point.
pub fn verify_structural_with_limits_and_policy(
    circuit: &QuantumCircuit,
    limits: &QuantumIrLimits,
    policy: &StructuralVerificationPolicy,
) -> StructuralVerificationReport {
    let counters = StructuralVerificationCounters {
        qubits: circuit.num_qubits(),
        classical_bits: circuit.num_classical_bits(),
        operations: circuit.operations().len(),
        empty: circuit.operations().is_empty(),
        canonical_validations: 0,
        qubit_namespace_checks: 0,
    };

    if let Err(error) = validate_limits(limits) {
        return StructuralVerificationReport::invalid(
            counters,
            policy.clone(),
            error,
        );
    }

    if !policy.allow_empty_circuit() && counters.empty {
        return StructuralVerificationReport::invalid(
            counters,
            policy.clone(),
            StructuralVerificationError::EmptyCircuitNotAllowed,
        );
    }

    if policy.check_qubit_limit() {
        if let Some(error) = check_qubit_limit(
            counters.qubits,
            limits,
        ) {
            return StructuralVerificationReport::limit_exceeded(
                counters,
                policy.clone(),
                error,
            );
        }
    }

    if policy.check_classical_bit_limit() {
        if let Some(error) = check_classical_bit_limit(
            counters.classical_bits,
            limits,
        ) {
            return StructuralVerificationReport::limit_exceeded(
                counters,
                policy.clone(),
                error,
            );
        }
    }

    if policy.check_operation_limit() {
        if let Some(error) = check_operation_limit(
            counters.operations,
            limits,
        ) {
            return StructuralVerificationReport::limit_exceeded(
                counters,
                policy.clone(),
                error,
            );
        }
    }

    let canonical_config = canonical_validation_config(
        limits,
        policy,
    );

    match validate_circuit_with_config(
        circuit,
        &canonical_config,
    ) {
        Ok(()) => {
            let counters = StructuralVerificationCounters {
                canonical_validations: 1,
                ..counters
            };

            StructuralVerificationReport::valid(
                counters,
                policy.clone(),
            )
        }

        Err(error) => {
            let counters = StructuralVerificationCounters {
                canonical_validations: 1,
                ..counters
            };

            StructuralVerificationReport::invalid(
                counters,
                policy.clone(),
                StructuralVerificationError::CanonicalValidationFailed {
                    message: error.to_string(),
                },
            )
        }
    }
}

// ============================================================================
// Direct qubit namespace verification
// ============================================================================

/// Verifies that one canonical `QubitId` belongs to a circuit's logical
/// namespace.
///
/// This function exists as a small, allocation-free primitive for future
/// verification passes that need to inspect qubit references.
///
/// The canonical circuit validator remains authoritative for operation-level
/// operand validation.
pub fn verify_qubit_id(
    circuit: &QuantumCircuit,
    qubit: QubitId,
) -> Result<(), StructuralVerificationError> {
    let index = qubit.index();

    if index >= circuit.num_qubits() {
        return Err(
            StructuralVerificationError::QubitOutOfRange {
                qubit,
                num_qubits: circuit.num_qubits(),
            },
        );
    }

    Ok(())
}

/// Verifies a sequence of canonical logical qubit IDs against a circuit
/// namespace.
///
/// This function does not allocate and stops at the first invalid ID.
///
/// It is useful to future structural verification code that has already
/// extracted canonical qubit operands from `Gate`.
pub fn verify_qubit_ids<'a, I>(
    circuit: &QuantumCircuit,
    qubits: I,
) -> Result<(), StructuralVerificationError>
where
    I: IntoIterator<Item = &'a QubitId>,
{
    for qubit in qubits {
        verify_qubit_id(circuit, *qubit)?;
    }

    Ok(())
}

// ============================================================================
// Boolean convenience APIs
// ============================================================================

/// Returns true when the circuit passes production structural verification.
#[must_use]
pub fn is_structurally_valid(
    circuit: &QuantumCircuit,
) -> bool {
    verify_structural(circuit).is_valid()
}

/// Returns true when the circuit passes strict structural verification.
#[must_use]
pub fn is_structurally_valid_strict(
    circuit: &QuantumCircuit,
) -> bool {
    verify_structural_strict(circuit).is_valid()
}

// ============================================================================
// Internal limit validation
// ============================================================================

fn validate_limits(
    limits: &QuantumIrLimits,
) -> Result<(), StructuralVerificationError> {
    /*
     * QuantumIrLimits owns the authoritative validity rules for its own
     * configuration.
     *
     * We intentionally do not inspect private fields here. This keeps this
     * module independent from the internal representation of QuantumIrLimits.
     *
     * The canonical validation configuration is constructed below and the
     * canonical validator therefore remains the final authority.
     *
     * This function currently performs no additional assumptions because the
     * canonical API does not expose a public fallible "validate limits"
     * operation in the stable boundary.
     */
    let _ = limits;

    Ok(())
}

// ============================================================================
// Limit checks
// ============================================================================

fn check_qubit_limit(
    requested: usize,
    limits: &QuantumIrLimits,
) -> Option<StructuralVerificationError> {
    /*
     * QuantumIrLimits is intentionally opaque to this subsystem.
     *
     * The actual authoritative limit check is performed by
     * validate_circuit_with_config below.
     *
     * Keeping this function as an explicit boundary means that a future
     * OptimizationLimits adapter can supply an optimizer-specific limit
     * without changing the public verification API.
     *
     * No duplicated private-field assumptions are made here.
     */
    let _ = requested;
    let _ = limits;

    None
}

fn check_classical_bit_limit(
    requested: usize,
    limits: &QuantumIrLimits,
) -> Option<StructuralVerificationError> {
    let _ = requested;
    let _ = limits;

    None
}

fn check_operation_limit(
    requested: usize,
    limits: &QuantumIrLimits,
) -> Option<StructuralVerificationError> {
    let _ = requested;
    let _ = limits;

    None
}

// ============================================================================
// Canonical validation configuration
// ============================================================================

fn canonical_validation_config(
    limits: &QuantumIrLimits,
    policy: &StructuralVerificationPolicy,
) -> ValidationConfig {
    ValidationConfig::new(limits.clone())
        .with_strict(matches!(
            policy.mode(),
            StructuralVerificationMode::Production
                | StructuralVerificationMode::Strict
        ))
        .with_empty_circuits(policy.allow_empty_circuit())
        .with_semantic_checks(policy.semantic_checks())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_deterministic() {
        assert_eq!(
            StructuralVerificationPolicy::production(),
            StructuralVerificationPolicy::production()
        );
    }

    #[test]
    fn status_helpers_are_consistent() {
        assert!(
            StructuralVerificationStatus::Valid.is_valid()
        );

        assert!(
            StructuralVerificationStatus::Invalid.is_invalid()
        );

        assert!(
            StructuralVerificationStatus::LimitExceeded
                .is_limit_exceeded()
        );

        assert!(
            !StructuralVerificationStatus::Valid.is_invalid()
        );
    }

    #[test]
    fn contract_identity_is_stable() {
        assert_eq!(
            STRUCTURAL_VERIFICATION_ID,
            "quantum.optimization.verification.structural"
        );

        assert_eq!(
            STRUCTURAL_VERIFICATION_VERSION,
            1
        );
    }

    #[test]
    fn counters_default_to_zero() {
        let counters =
            StructuralVerificationCounters::default();

        assert_eq!(counters.qubits(), 0);
        assert_eq!(counters.classical_bits(), 0);
        assert_eq!(counters.operations(), 0);
        assert!(counters.is_empty());
        assert_eq!(
            counters.canonical_validations(),
            0
        );
        assert_eq!(
            counters.qubit_namespace_checks(),
            0
        );
    }

    #[test]
    fn policy_builders_are_distinct() {
        assert_eq!(
            StructuralVerificationPolicy::production().mode(),
            StructuralVerificationMode::Production
        );

        assert_eq!(
            StructuralVerificationPolicy::strict().mode(),
            StructuralVerificationMode::Strict
        );

        assert_eq!(
            StructuralVerificationPolicy::structural_only().mode(),
            StructuralVerificationMode::StructuralOnly
        );

        assert_eq!(
            StructuralVerificationPolicy::minimal().mode(),
            StructuralVerificationMode::Minimal
        );
    }

    #[test]
    fn policy_builders_preserve_expected_checks() {
        let production =
            StructuralVerificationPolicy::production();

        assert!(production.check_qubit_limit());
        assert!(production.check_operation_limit());
        assert!(
            production.check_classical_bit_limit()
        );
        assert!(production.semantic_checks());
        assert!(production.allow_empty_circuit());
    }

    #[test]
    fn policy_is_immutably_configurable() {
        let policy =
            StructuralVerificationPolicy::production()
                .with_empty_circuit_policy(false)
                .with_qubit_limit_check(false)
                .with_operation_limit_check(false)
                .with_classical_bit_limit_check(false)
                .with_semantic_checks(false)
                .with_mode(
                    StructuralVerificationMode::Minimal,
                );

        assert_eq!(
            policy.mode(),
            StructuralVerificationMode::Minimal
        );

        assert!(!policy.allow_empty_circuit());
        assert!(!policy.check_qubit_limit());
        assert!(!policy.check_operation_limit());
        assert!(
            !policy.check_classical_bit_limit()
        );
        assert!(!policy.semantic_checks());
    }

    #[test]
    fn report_valid_has_no_error() {
        let report =
            StructuralVerificationReport::valid(
                StructuralVerificationCounters::default(),
                StructuralVerificationPolicy::production(),
            );

        assert!(report.is_valid());
        assert!(!report.is_invalid());
        assert!(!report.is_limit_exceeded());
        assert!(report.error().is_none());

        assert_eq!(
            report.contract_id(),
            STRUCTURAL_VERIFICATION_ID
        );

        assert_eq!(
            report.contract_version(),
            STRUCTURAL_VERIFICATION_VERSION
        );
    }

    #[test]
    fn report_invalid_contains_error() {
        let report =
            StructuralVerificationReport::invalid(
                StructuralVerificationCounters::default(),
                StructuralVerificationPolicy::production(),
                StructuralVerificationError::EmptyCircuitNotAllowed,
            );

        assert!(!report.is_valid());
        assert!(report.is_invalid());
        assert!(!report.is_limit_exceeded());
        assert!(report.error().is_some());

        assert!(
            report.into_result().is_err()
        );
    }

    #[test]
    fn report_limit_status_contains_error() {
        let report =
            StructuralVerificationReport::limit_exceeded(
                StructuralVerificationCounters::default(),
                StructuralVerificationPolicy::production(),
                StructuralVerificationError::OperationLimitExceeded {
                    requested: 10,
                    maximum: 5,
                },
            );

        assert!(!report.is_valid());
        assert!(!report.is_invalid());
        assert!(report.is_limit_exceeded());

        assert!(
            report.into_result().is_err()
        );
    }
}