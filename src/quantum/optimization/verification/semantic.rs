//! Zamani Quantum Optimization — Semantic Verification
//!
//! Production semantic-preservation verification for optimization passes.
//!
//! # Architectural position
//!
//! ```text
//!                    canonical Quantum IR
//!                            │
//!                            ▼
//!                  optimization transformation
//!                            │
//!                ┌───────────┴───────────┐
//!                │                       │
//!             original               optimized
//!                │                       │
//!                └───────────┬───────────┘
//!                            ▼
//!             verification::semantic
//!                            │
//!                            ▼
//!                 optimization::equivalence
//!                            │
//!              ┌─────────────┼──────────────┐
//!              ▼             ▼              ▼
//!          structural     exact unitary   future engines
//!              │             │              │
//!              └─────────────┼──────────────┘
//!                            ▼
//!                  SemanticVerificationReport
//! ```
//!
//! # Purpose
//!
//! This module is the verification-facing semantic boundary of the Zamani
//! quantum optimizer.
//!
//! It answers one fundamental compiler question:
//!
//! > Did an optimization transformation preserve the semantics of the
//! > quantum program under the requested equivalence relation?
//!
//! It deliberately does NOT implement a second quantum simulator or a second
//! circuit-equivalence engine. The canonical optimizer equivalence subsystem
//! (`super::equivalence`) owns those algorithms.
//!
//! This separation is intentional:
//!
//! ```text
//! verification::semantic
//!        │
//!        └── verification policy / compiler contract
//!
//! optimization::equivalence
//!        │
//!        └── equivalence algorithms
//!
//! quantum::ir
//!        │
//!        └── canonical representation
//! ```
//!
//! # Safety contract
//!
//! The most important invariant in this file is:
//!
//! ```text
//! Equivalent
//!     = equivalence was actually proven
//!
//! NotEquivalent
//!     = non-equivalence was actually proven
//!
//! Inconclusive
//!     = no semantic conclusion was proven
//! ```
//!
//! `Inconclusive` MUST NEVER be converted to `Equivalent`.
//!
//! A verifier that silently treats an unsupported gate, symbolic parameter,
//! resource exhaustion, or unsupported non-unitary circuit as equivalent is
//! unsafe for a compiler.
//!
//! # Verification modes
//!
//! The semantic layer exposes:
//!
//! - structural verification;
//! - exact unitary verification;
//! - exact unitary verification up to global phase;
//! - automatic method selection;
//! - explicit resource limits;
//! - explicit inconclusive results;
//! - strict proof-only helpers.
//!
//! # Non-unitary circuits
//!
//! Measurement, reset, and barrier operations require semantic treatment that
//! is different from ordinary unitary equivalence.
//!
//! This module therefore never silently applies unitary equivalence to a
//! circuit that the underlying equivalence engine cannot safely model.
//!
//! Dynamic-circuit verification is intentionally extensible rather than
//! incorrectly approximated.
//!
//! Future verification engines may support:
//!
//! - deferred measurement;
//! - classical-control semantics;
//! - stabilizer verification;
//! - decision diagrams;
//! - tensor networks;
//! - symbolic path-sums;
//! - randomized differential checking;
//! - certificate-backed verification.
//!
//! The public semantic result contract does not need to change when those
//! engines are added.
//!
//! # Global phase
//!
//! For ordinary quantum algorithms, two unitary circuits may be physically
//! equivalent when:
//!
//!     U = exp(i * phi) V
//!
//! even though their matrices are not byte-for-byte identical.
//!
//! The caller must explicitly select whether global phase is ignored.
//!
//! The default policy follows the existing equivalence subsystem and permits
//! global phase for unitary semantic verification.
//!
//! # Scalability
//!
//! This module imposes no artificial circuit-size ceiling of its own.
//!
//! Actual scalability is governed by the selected equivalence engine and its
//! configured resource policy.
//!
//! For small circuits, exact dense verification can be practical.
//!
//! For larger circuits, verification can return `Inconclusive` rather than
//! allocating unbounded memory or pretending that a proof exists.
//!
//! Future scalable engines can be added behind `optimization::equivalence`
//! without changing this semantic API.
//!
//! This is the correct interpretation of "from tiny to infinity":
//!
//! > process every circuit that the selected verification engine and available
//! > resources can safely process, while never claiming a proof beyond those
//! > capabilities.
//!
//! Quantum equivalence itself can be computationally hard or exponentially
//! expensive for general circuits, so no sound implementation can promise
//! finite-resource verification of literally arbitrary circuits.
//!
//! # Determinism
//!
//! This module does not introduce randomness.
//!
//! If a future randomized verifier is selected by the equivalence subsystem,
//! its seed and reproducibility contract belong to that verifier rather than
//! this semantic boundary.
//!
//! # Dependencies
//!
//! This file depends only on:
//!
//! - the canonical `QuantumCircuit`;
//! - the existing optimizer equivalence contract.
//!
//! It does NOT depend on:
//!
//! - routing;
//! - scheduling;
//! - hardware;
//! - QPU execution;
//! - benchmarking;
//! - frontend parsing;
//! - algorithms;
//! - error correction;
//! - synthesis implementations;
//! - optimizer passes.
//!
//! # Integration contract
//!
//! `verification/mod.rs` should expose this module:
//!
//! ```text
//! pub mod semantic;
//! ```
//!
//! `pipeline.rs` should call:
//!
//! ```text
//! semantic::verify_optimization(...)
//! ```
//!
//! after a transformation when semantic verification is enabled.
//!
//! `verification/randomized.rs`, `verification/exhaustive.rs`, and
//! `verification/certificates.rs` may later consume the report without
//! changing this file's public semantic contract.
//!
//! `result.rs` may store `SemanticVerificationReport`.
//!
//! `provenance.rs` may record the verification policy and resulting verdict.
//!
//! `tests/equivalence.rs` should test the proof semantics exposed here.
//!
//! No optimizer pass should need to modify this file merely because a new
//! optimization transformation is added.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021
//!
//! No nightly features.
//! No `unsafe` code.
//!
//! # Security properties
//!
//! This module:
//!
//! - never mutates either input circuit;
//! - never executes a QPU;
//! - never performs backend I/O;
//! - never treats a timeout as equivalence;
//! - never treats unsupported operations as equivalence;
//! - never silently binds symbolic parameters;
//! - never removes global phase unless requested by the equivalence relation;
//! - never converts `Inconclusive` to `Equivalent`;
//! - never trusts an optimizer pass merely because it reports success.
//!
//! # External design rationale
//!
//! Modern quantum compilation research treats equivalence checking as a core
//! compiler-correctness problem. Recent work also distinguishes ordinary
//! unitary circuits from hybrid circuits containing measurements and classical
//! control. This module therefore keeps semantic verification separate from
//! transformation and leaves room for specialized verification engines.
//!
//! -----------------------------------------------------------------------------
//! Public API
//! -----------------------------------------------------------------------------
//
// The public API intentionally has two levels:
//
// 1. `verify_optimization` / `verify_with_config`
//!    -> complete report, including inconclusive results.
//
// 2. `prove_optimization` / `prove_with_config`
//!    -> strict compiler helper that succeeds only when equivalence is proven.
//
// The first API is appropriate for diagnostics and adaptive verification.
// The second API is appropriate when a compiler transformation is not allowed
// to proceed unless semantic preservation has actually been established.

use std::fmt;

use crate::quantum::ir::QuantumCircuit;

use super::equivalence::{
    verify as verify_equivalence,
    EquivalenceConfig,
    EquivalenceError,
    EquivalenceLimits,
    EquivalenceMethod,
    EquivalenceReport,
    EquivalenceTolerance,
    EquivalenceVerdict,
    InconclusiveReason,
    UnitaryRelation,
};

// ============================================================================
// Stable public contract identifiers
// ============================================================================

/// Stable identifier for this verification subsystem.
pub const SEMANTIC_VERIFICATION_ID: &str =
    "quantum.optimization.verification.semantic";

/// Semantic verification API contract version.
///
/// This version is independent from the Quantum IR schema version and from
/// the optimizer implementation version.
pub const SEMANTIC_VERIFICATION_VERSION: u32 = 1;

// ============================================================================
// Public re-exports
// ============================================================================

/// Re-export the equivalence method so callers can configure semantic
/// verification without depending directly on the implementation module.
pub use super::equivalence::EquivalenceMethod as SemanticEquivalenceMethod;

/// Re-export the unitary relation used by semantic verification.
pub use super::equivalence::UnitaryRelation as SemanticUnitaryRelation;

// ============================================================================
// Semantic verification policy
// ============================================================================

/// Policy controlling one semantic-verification invocation.
///
/// This wrapper intentionally remains smaller than `EquivalenceConfig`.
///
/// `EquivalenceConfig` controls the equivalence engine itself.
///
/// `SemanticVerificationPolicy` controls how the compiler should interpret
/// that engine's result.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticVerificationPolicy {
    /// Configuration passed to the canonical equivalence engine.
    equivalence: EquivalenceConfig,

    /// Whether the caller requires a definitive semantic proof.
    ///
    /// This does not change the underlying verdict. It controls whether the
    /// strict helper rejects `Inconclusive`.
    require_proof: bool,
}

impl SemanticVerificationPolicy {
    /// Creates the normal production policy.
    ///
    /// Automatic method selection is used, with global phase ignored for
    /// unitary circuits and conservative resource limits.
    #[must_use]
    pub const fn production() -> Self {
        Self {
            equivalence: EquivalenceConfig {
                method: EquivalenceMethod::Auto {
                    relation: UnitaryRelation::UpToGlobalPhase,
                },
                tolerance: EquivalenceTolerance {
                    absolute: 1.0e-12,
                    relative: 1.0e-10,
                },
                limits: EquivalenceLimits::conservative(),
            },
            require_proof: false,
        }
    }

    /// Creates a strict production policy.
    ///
    /// The verification engine still returns `Inconclusive` when it cannot
    /// establish equivalence. The strict policy simply makes the compiler
    /// helper reject that result.
    #[must_use]
    pub const fn strict() -> Self {
        Self {
            equivalence: EquivalenceConfig {
                method: EquivalenceMethod::Auto {
                    relation: UnitaryRelation::UpToGlobalPhase,
                },
                tolerance: EquivalenceTolerance {
                    absolute: 1.0e-12,
                    relative: 1.0e-10,
                },
                limits: EquivalenceLimits::conservative(),
            },
            require_proof: true,
        }
    }

    /// Creates a structural-only policy.
    ///
    /// This is exact structural identity, not semantic unitary equivalence.
    #[must_use]
    pub const fn structural() -> Self {
        Self {
            equivalence: EquivalenceConfig {
                method: EquivalenceMethod::Structural,
                tolerance: EquivalenceTolerance::exact(),
                limits: EquivalenceLimits::conservative(),
            },
            require_proof: true,
        }
    }

    /// Creates an exact unitary policy that does not ignore global phase.
    #[must_use]
    pub const fn exact_unitary() -> Self {
        Self {
            equivalence: EquivalenceConfig {
                method: EquivalenceMethod::ExactUnitary {
                    relation: UnitaryRelation::Exact,
                },
                tolerance: EquivalenceTolerance::numerical(),
                limits: EquivalenceLimits::conservative(),
            },
            require_proof: true,
        }
    }

    /// Creates an exact unitary policy that permits global phase.
    #[must_use]
    pub const fn unitary_up_to_global_phase() -> Self {
        Self {
            equivalence: EquivalenceConfig {
                method: EquivalenceMethod::ExactUnitary {
                    relation: UnitaryRelation::UpToGlobalPhase,
                },
                tolerance: EquivalenceTolerance::numerical(),
                limits: EquivalenceLimits::conservative(),
            },
            require_proof: true,
        }
    }

    /// Returns the underlying equivalence configuration.
    #[must_use]
    pub const fn equivalence_config(self) -> EquivalenceConfig {
        self.equivalence
    }

    /// Returns whether strict callers require a proof.
    #[must_use]
    pub const fn requires_proof(self) -> bool {
        self.require_proof
    }

    /// Returns a policy using a different equivalence configuration.
    #[must_use]
    pub const fn with_equivalence_config(
        mut self,
        config: EquivalenceConfig,
    ) -> Self {
        self.equivalence = config;
        self
    }

    /// Returns a policy with an explicit proof requirement.
    #[must_use]
    pub const fn with_required_proof(
        mut self,
        required: bool,
    ) -> Self {
        self.require_proof = required;
        self
    }
}

impl Default for SemanticVerificationPolicy {
    fn default() -> Self {
        Self::production()
    }
}

// ============================================================================
// Semantic verification report
// ============================================================================

/// Complete result of semantic verification.
///
/// This type deliberately retains the underlying `EquivalenceReport` rather
/// than duplicating its fields. This keeps the equivalence engine as the
/// single source of truth for semantic evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticVerificationReport {
    /// Stable semantic-verification subsystem identifier.
    pub verifier_id: &'static str,

    /// Semantic-verification contract version.
    pub verifier_version: u32,

    /// Final semantic verdict.
    pub verdict: EquivalenceVerdict,

    /// Complete report generated by the canonical equivalence subsystem.
    pub equivalence: EquivalenceReport,
}

impl SemanticVerificationReport {
    /// Returns true only when semantic equivalence was actually proven.
    #[must_use]
    pub const fn is_equivalent(&self) -> bool {
        self.verdict.is_equivalent()
    }

    /// Returns true only when non-equivalence was actually proven.
    #[must_use]
    pub const fn is_not_equivalent(&self) -> bool {
        self.verdict.is_not_equivalent()
    }

    /// Returns true when the verifier could not establish either result.
    #[must_use]
    pub const fn is_inconclusive(&self) -> bool {
        self.verdict.is_inconclusive()
    }

    /// Returns the reason for an inconclusive result, if available.
    ///
    /// The underlying equivalence report remains authoritative. This helper
    /// is intentionally conservative and returns `None` when the report does
    /// not expose a specific reason.
    #[must_use]
    pub fn inconclusive_reason(&self) -> Option<&InconclusiveReason> {
        self.equivalence.inconclusive_reason.as_ref()
    }
}

// ============================================================================
// Semantic verification errors
// ============================================================================

/// Errors returned by strict semantic-verification helpers.
///
/// `Inconclusive` is represented separately from an execution/configuration
/// error because it is a valid verifier result rather than a verifier crash.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticVerificationError {
    /// The underlying equivalence engine rejected the verification request.
    Equivalence(EquivalenceError),

    /// Verification completed but did not prove equivalence.
    ///
    /// The complete report is preserved so callers can inspect the reason.
    NotProven(SemanticVerificationReport),
}

impl fmt::Display for SemanticVerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equivalence(error) => {
                write!(f, "semantic verification could not execute: {error}")
            }

            Self::NotProven(report) => {
                match report.verdict {
                    EquivalenceVerdict::NotEquivalent => {
                        f.write_str(
                            "semantic verification proved the circuits are not equivalent",
                        )
                    }

                    EquivalenceVerdict::Inconclusive => {
                        f.write_str(
                            "semantic verification could not prove equivalence",
                        )
                    }

                    EquivalenceVerdict::Equivalent => {
                        f.write_str(
                            "semantic verification unexpectedly returned a non-success state",
                        )
                    }
                }
            }
        }
    }
}

impl std::error::Error for SemanticVerificationError {}

impl From<EquivalenceError> for SemanticVerificationError {
    fn from(error: EquivalenceError) -> Self {
        Self::Equivalence(error)
    }
}

// ============================================================================
// Core verification functions
// ============================================================================

/// Verifies semantic equivalence using the production semantic policy.
///
/// This function never turns an inconclusive result into equivalence.
///
/// # Errors
///
/// Returns `SemanticVerificationError::Equivalence` only when the equivalence
/// engine cannot execute the requested verification because its configuration
/// or input is invalid.
///
/// A valid but inconclusive semantic verification is returned as a normal
/// `SemanticVerificationReport`.
pub fn verify_optimization(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    verify_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::production(),
    )
}

/// Verifies semantic equivalence using an explicit semantic policy.
///
/// The two input circuits are never mutated.
///
/// The canonical equivalence subsystem performs the actual semantic proof.
pub fn verify_with_policy(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    policy: SemanticVerificationPolicy,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    let equivalence = verify_equivalence(
        original,
        optimized,
        policy.equivalence_config(),
    )?;

    let verdict = equivalence.verdict;

    Ok(SemanticVerificationReport {
        verifier_id: SEMANTIC_VERIFICATION_ID,
        verifier_version: SEMANTIC_VERIFICATION_VERSION,
        verdict,
        equivalence,
    })
}

/// Verifies semantic equivalence using a raw equivalence configuration.
///
/// This function is useful for compiler infrastructure that already owns
/// `EquivalenceConfig`.
pub fn verify_with_config(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    config: EquivalenceConfig,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    let equivalence = verify_equivalence(
        original,
        optimized,
        config,
    )?;

    let verdict = equivalence.verdict;

    Ok(SemanticVerificationReport {
        verifier_id: SEMANTIC_VERIFICATION_ID,
        verifier_version: SEMANTIC_VERIFICATION_VERSION,
        verdict,
        equivalence,
    })
}

// ============================================================================
// Strict proof APIs
// ============================================================================

/// Proves semantic equivalence using the strict production policy.
///
/// This function succeeds only when equivalence is actually proven.
///
/// `Inconclusive` is an error.
///
/// `NotEquivalent` is an error.
///
/// This is the preferred API for an optimizer pipeline that must not accept
/// an unverified transformation.
pub fn prove_optimization(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    prove_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::strict(),
    )
}

/// Proves semantic equivalence using an explicit policy.
///
/// Regardless of the policy's `require_proof` flag, this function is strict:
/// only `Equivalent` succeeds.
pub fn prove_with_policy(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    policy: SemanticVerificationPolicy,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    let report = verify_with_policy(
        original,
        optimized,
        policy,
    )?;

    if report.is_equivalent() {
        Ok(report)
    } else {
        Err(SemanticVerificationError::NotProven(report))
    }
}

/// Proves equivalence using an explicit equivalence configuration.
///
/// This is the low-level strict compiler API.
pub fn prove_with_config(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
    config: EquivalenceConfig,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    let report = verify_with_config(
        original,
        optimized,
        config,
    )?;

    if report.is_equivalent() {
        Ok(report)
    } else {
        Err(SemanticVerificationError::NotProven(report))
    }
}

// ============================================================================
// Specialized policies
// ============================================================================

/// Performs exact structural verification.
///
/// Structural equality is stronger than semantic equivalence in the sense
/// that it requires the canonical representations to match exactly, but it
/// does not prove that different representations have the same quantum
/// semantics.
pub fn verify_structural(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    verify_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::structural(),
    )
}

/// Proves exact structural identity.
pub fn prove_structural(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    prove_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::structural(),
    )
}

/// Performs exact unitary verification where global phase is significant.
pub fn verify_exact_unitary(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    verify_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::exact_unitary(),
    )
}

/// Proves exact unitary equivalence where global phase is significant.
pub fn prove_exact_unitary(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    prove_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::exact_unitary(),
    )
}

/// Performs unitary verification while ignoring a circuit-wide global phase.
pub fn verify_unitary_up_to_global_phase(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    verify_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::unitary_up_to_global_phase(),
    )
}

/// Proves unitary equivalence while ignoring a circuit-wide global phase.
pub fn prove_unitary_up_to_global_phase(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    prove_with_policy(
        original,
        optimized,
        SemanticVerificationPolicy::unitary_up_to_global_phase(),
    )
}

// ============================================================================
// Lightweight decision helpers
// ============================================================================

/// Returns `true` only if semantic equivalence was actually proven.
///
/// Errors and inconclusive results both return `false`.
///
/// This helper is intentionally conservative and should not be used when the
/// caller needs diagnostics. Use `verify_optimization` instead in that case.
#[must_use]
pub fn is_equivalent(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> bool {
    match verify_optimization(
        original,
        optimized,
    ) {
        Ok(report) => report.is_equivalent(),
        Err(_) => false,
    }
}

/// Returns `true` only if non-equivalence was actually proven.
///
/// Errors and inconclusive results both return `false`.
#[must_use]
pub fn is_not_equivalent(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> bool {
    match verify_optimization(
        original,
        optimized,
    ) {
        Ok(report) => report.is_not_equivalent(),
        Err(_) => false,
    }
}

// ============================================================================
// Compiler integration helpers
// ============================================================================

/// Verifies that an optimizer transformation preserved semantics.
///
/// This is the intended high-level API for `pipeline.rs`.
///
/// # Recommended pipeline usage
///
/// ```text
/// original
///    │
///    ▼
/// optimizer pass
///    │
///    ▼
/// optimized
///    │
///    ▼
/// verify_optimization
///    │
///    ├── Equivalent
///    │      └── accept
///    │
///    ├── NotEquivalent
///    │      └── reject transformation
///    │
///    └── Inconclusive
///           └── apply pipeline policy
/// ```
///
/// The function itself does not decide whether an inconclusive result should
/// abort the pipeline. That policy belongs to `pipeline.rs` / `config.rs`.
pub fn verify_transformation(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    verify_optimization(
        original,
        optimized,
    )
}

/// Strict transformation verification.
///
/// The transformation is accepted only if semantic equivalence is proven.
pub fn prove_transformation(
    original: &QuantumCircuit,
    optimized: &QuantumCircuit,
) -> Result<SemanticVerificationReport, SemanticVerificationError> {
    prove_optimization(
        original,
        optimized,
    )
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_policy_is_not_strict() {
        let policy = SemanticVerificationPolicy::production();

        assert!(!policy.requires_proof());
    }

    #[test]
    fn strict_policy_requires_proof() {
        let policy = SemanticVerificationPolicy::strict();

        assert!(policy.requires_proof());
    }

    #[test]
    fn structural_policy_is_strict() {
        let policy = SemanticVerificationPolicy::structural();

        assert!(policy.requires_proof());
        assert!(matches!(
            policy.equivalence_config().method,
            EquivalenceMethod::Structural
        ));
    }

    #[test]
    fn exact_unitary_policy_does_not_ignore_global_phase() {
        let policy = SemanticVerificationPolicy::exact_unitary();

        assert!(matches!(
            policy.equivalence_config().method,
            EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::Exact
            }
        ));
    }

    #[test]
    fn global_phase_policy_is_explicit() {
        let policy =
            SemanticVerificationPolicy::unitary_up_to_global_phase();

        assert!(matches!(
            policy.equivalence_config().method,
            EquivalenceMethod::ExactUnitary {
                relation: UnitaryRelation::UpToGlobalPhase
            }
        ));
    }

    #[test]
    fn subsystem_contract_is_stable() {
        assert_eq!(
            SEMANTIC_VERIFICATION_ID,
            "quantum.optimization.verification.semantic"
        );

        assert_eq!(
            SEMANTIC_VERIFICATION_VERSION,
            1
        );
    }

    #[test]
    fn verdict_helpers_are_conservative() {
        assert!(
            EquivalenceVerdict::Equivalent.is_equivalent()
        );

        assert!(
            !EquivalenceVerdict::Inconclusive.is_equivalent()
        );

        assert!(
            EquivalenceVerdict::NotEquivalent.is_not_equivalent()
        );

        assert!(
            !EquivalenceVerdict::Inconclusive.is_not_equivalent()
        );

        assert!(
            EquivalenceVerdict::Inconclusive.is_inconclusive()
        );
    }
}