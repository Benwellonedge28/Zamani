//! Zamani Quantum Optimization — Verification Subsystem
//!
//! Production verification boundary for the quantum optimization framework.
//!
//! # Architectural position
//!
//! ```text
//!                         canonical Quantum IR
//!                                  │
//!                                  ▼
//!                         optimization pipeline
//!                                  │
//!                    ┌─────────────┴─────────────┐
//!                    │                           │
//!              original circuit           optimized circuit
//!                    │                           │
//!                    └─────────────┬─────────────┘
//!                                  │
//!                                  ▼
//!                    optimization::verification
//!                                  │
//!          ┌───────────────────────┼────────────────────────┐
//!          │                       │                        │
//!          ▼                       ▼                        ▼
//!      structural              semantic                randomized
//!      validation              proof                    evidence
//!          │                       │                        │
//!          │                       │                        │
//!          └───────────────┬───────┴────────────────────────┘
//!                          ▼
//!                    exhaustive checking
//!                          │
//!                          ▼
//!                    certificates
//! ```
//!
//! This module is the public module boundary for all verification facilities
//! belonging to `quantum::optimization`.
//!
//! It does not implement the verification algorithms itself. Algorithmic
//! ownership remains in:
//!
//! - [`structural`] — canonical-IR structural validation;
//! - [`semantic`] — semantic-preservation policy over the canonical
//!   equivalence engine;
//! - [`randomized`] — randomized/differential verification;
//! - [`exhaustive`] — deterministic exhaustive verification;
//! - [`certificates`] — deterministic verification evidence and certificates.
//!
//! The canonical semantic equivalence engine remains owned by:
//!
//! `crate::quantum::optimization::equivalence`
//!
//! This separation is deliberate. The verification subsystem is a policy and
//! orchestration boundary; it must not become a second implementation of the
//! Quantum IR or a second circuit-equivalence engine.
//!
//! # Canonical representation
//!
//! Every verification module operates on the canonical Quantum IR:
//!
//! `crate::quantum::ir::QuantumCircuit`
//!
//! and, where a qubit identity is required:
//!
//! `crate::quantum::ir::qubit::QubitId`
//!
//! The verification subsystem MUST NOT define:
//!
//! - `QuantumGate`;
//! - `VerificationCircuit`;
//! - `Qubit`;
//! - `QubitId`;
//! - an optimizer-specific circuit representation;
//! - a backend-specific circuit representation.
//!
//! The canonical IR remains the single source of truth.
//!
//! # Verification hierarchy
//!
//! Verification methods have different strengths and meanings.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Strongest / deterministic                                   │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Structural identity                                         │
//! │ Exact semantic equivalence                                  │
//! │ Exhaustive semantic verification                            │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Evidence, not arbitrary-circuit proof                       │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Randomized / differential verification                      │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Audit artifact                                              │
//! ├─────────────────────────────────────────────────────────────┤
//! │ Verification certificate                                    │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! A randomized result MUST NOT be promoted to an exact semantic proof merely
//! because all requested randomized trials passed.
//!
//! Likewise, an inconclusive exact verification MUST NOT be treated as
//! equivalent.
//!
//! # Core soundness rule
//!
//! The subsystem preserves the following invariant:
//!
//! ```text
//! Equivalent
//!     = equivalence was actually proven
//!
//! NotEquivalent
//!     = non-equivalence was actually proven
//!
//! Inconclusive
//!     = the verifier did not establish either conclusion
//! ```
//!
//! `Inconclusive` is a legitimate result, not a failure of the verifier.
//!
//! A production compiler MUST distinguish:
//!
//! - proof;
//! - counterexample;
//! - evidence;
//! - inability to decide.
//!
//! # No false scalability promise
//!
//! Verification is designed to scale from tiny circuits to circuits limited
//! only by the available computational resources.
//!
//! That does NOT mean that arbitrary quantum-equivalence problems become
//! polynomial or that an infinite circuit can literally be materialized.
//!
//! Instead:
//!
//! - no artificial verification-size constant is imposed by this module;
//! - each engine owns explicit resource limits;
//! - allocation and arithmetic must remain bounded and checked;
//! - expensive verification may return `Inconclusive`;
//! - future scalable engines can be added without changing this module's
//!   public boundary;
//! - callers can select stronger or more scalable engines as they become
//!   available.
//!
//! This makes the architecture resource-scalable without making unsound
//! mathematical claims.
//!
//! # Verification layers
//!
//! ## Structural verification
//!
//! Structural verification answers:
//!
//! > Is the candidate circuit a valid and internally consistent canonical
//! > Quantum IR program?
//!
//! It delegates canonical semantic/IR invariants to the Quantum IR validation
//! subsystem and adds optimizer-boundary checks.
//!
//! It does NOT establish arbitrary circuit semantic equivalence.
//!
//! ## Semantic verification
//!
//! Semantic verification answers:
//!
//! > Did the optimization transformation preserve the requested semantics?
//!
//! It delegates equivalence algorithms to:
//!
//! `crate::quantum::optimization::equivalence`
//!
//! and adds compiler-facing proof policy.
//!
//! ## Exhaustive verification
//!
//! Exhaustive verification deterministically checks all computational-basis
//! inputs when the selected method and resource limits permit it.
//!
//! It is stronger than checking only one input state.
//!
//! ## Randomized verification
//!
//! Randomized verification searches for counterexamples using deterministic
//! seeds and/or randomized probes.
//!
//! Passing randomized trials is evidence, not an unconditional mathematical
//! proof of arbitrary-circuit equivalence.
//!
//! ## Certificates
//!
//! Certificates record verification evidence in a deterministic,
//! tamper-evident, machine-auditable representation.
//!
//! Certificates do not replace verification.
//!
//! A certificate records what a verifier established; it does not make an
//! unproven result proven.
//!
//! # Dependency direction
//!
//! The intended dependency graph is:
//!
//! ```text
//!                         quantum::ir
//!                              │
//!                              ▼
//!                 optimization::equivalence
//!                              │
//!                              ▼
//!             optimization::verification::semantic
//!                              │
//!                              │
//!          ┌───────────────────┼───────────────────┐
//!          │                   │                   │
//!          ▼                   ▼                   ▼
//!     structural          exhaustive          randomized
//!          │                   │                   │
//!          └───────────────────┼───────────────────┘
//!                              ▼
//!                         certificates
//! ```
//!
//! The verification subsystem MUST NOT introduce dependencies on:
//!
//! - routing;
//! - scheduling;
//! - hardware backends;
//! - QPU communication;
//! - frontend parsers;
//! - algorithm implementations;
//! - benchmarking orchestration;
//! - error-correction implementations;
//! - backend authentication;
//! - network services.
//!
//! This keeps verification deterministic, offline-capable, testable, and safe
//! to invoke from compiler pipelines.
//!
//! # Optimization pipeline integration
//!
//! The expected production flow is:
//!
//! ```text
//! original QuantumCircuit
//!          │
//!          ├── structural verification
//!          │
//!          ▼
//!     optimization pass
//!          │
//!          ▼
//! optimized QuantumCircuit
//!          │
//!          ├── structural verification
//!          │
//!          ├── semantic verification
//!          │
//!          ├── optional exhaustive verification
//!          │
//!          ├── optional randomized verification
//!          │
//!          └── optional certificate generation
//!          │
//!          ▼
//!     verified OptimizationResult
//! ```
//!
//! `pipeline.rs` remains responsible for deciding when verification is
//! requested. This module only exposes the verification facilities.
//!
//! The optimizer itself must never assume that a pass is correct merely
//! because the pass returned `Ok`.
//!
//! # Result interpretation
//!
//! The public verification APIs must preserve the distinction between:
//!
//! ```text
//! proof of equivalence
//!         ≠
//! no detected counterexample
//!         ≠
//! structurally valid
//!         ≠
//! successful execution
//! ```
//!
//! This distinction is essential for compiler correctness.
//!
//! # Resource ownership
//!
//! Resource limits are intentionally layered:
//!
//! ```text
//! Quantum IR limits
//!        │
//!        ▼
//! Optimization limits
//!        │
//!        ▼
//! Verification-engine limits
//! ```
//!
//! A verification engine MUST respect its own limits and MUST NOT silently
//! bypass broader compiler resource policies.
//!
//! The verification module itself does not duplicate the limits structures.
//!
//! # Determinism
//!
//! Structural and exhaustive verification are deterministic.
//!
//! Semantic verification is deterministic for deterministic equivalence
//! engines.
//!
//! Randomized verification must expose its seed/configuration through its own
//! public contract and must be reproducible when deterministic execution is
//! requested.
//!
//! Certificate generation must remain deterministic for identical:
//!
//! - circuits;
//! - verifier configuration;
//! - verifier version;
//! - evidence;
//! - metadata.
//!
//! # Mutation policy
//!
//! Verification is observational.
//!
//! Verification functions MUST NOT mutate the original or optimized circuit
//! merely to perform verification.
//!
//! If a verification engine needs a temporary representation, that
//! representation must remain internal to the engine and must never become a
//! second public Quantum IR.
//!
//! # Error policy
//!
//! Verification distinguishes:
//!
//! 1. an actual non-equivalence result;
//! 2. an inconclusive result;
//! 3. an operational/configuration error.
//!
//! These must not be collapsed into one generic success/failure boolean.
//!
//! In particular:
//!
//! ```text
//! timeout       → NOT equivalent
//!                 NO
//!
//! timeout       → Inconclusive
//!                 YES
//!
//! unsupported   → Equivalent
//!                 NO
//!
//! unsupported   → Inconclusive
//!                 YES
//! ```
//!
//! # Global phase
//!
//! Global phase policy belongs to the equivalence engine and the semantic
//! verification configuration.
//!
//! This module MUST NOT independently decide that two unitary circuits are
//! equivalent up to global phase.
//!
//! It simply exposes the semantic verifier that delegates that decision to the
//! canonical equivalence subsystem.
//!
//! # Non-unitary circuits
//!
//! Circuits containing measurement, reset, dynamic classical control, or other
//! non-unitary operations must not accidentally be sent through a unitary-only
//! equivalence proof.
//!
//! The underlying semantic/equivalence implementation owns that semantic
//! distinction.
//!
//! Unsupported non-unitary verification must remain explicit and may return
//! `Inconclusive` until a verifier supporting the relevant dynamic semantics is
//! available.
//!
//! # Future verification engines
//!
//! This module is intentionally extensible.
//!
//! Future implementations may add:
//!
//! - stabilizer verification;
//! - Clifford-specific verification;
//! - tensor-network verification;
//! - decision-diagram verification;
//! - symbolic verification;
//! - ZX-calculus verification;
//! - SMT-backed verification;
//! - SAT-backed verification;
//! - randomized polynomial identity testing;
//! - state-preparation equivalence;
//! - observable-equivalence checking;
//! - measurement-distribution equivalence;
//! - logical-qubit equivalence;
//! - approximate equivalence;
//! - certificate-backed proof checking;
//! - hardware-aware semantic verification.
//!
//! Such engines should be added as new modules rather than changing the
//! meaning of existing verdicts.
//!
//! # Public module contract
//!
//! The verification directory currently contains:
//!
//! - [`structural`]
//! - [`semantic`]
//! - [`exhaustive`]
//! - [`randomized`]
//! - [`certificates`]
//!
//! Each implementation is deliberately kept separate so that a caller can
//! select the verification strength appropriate to its use case.
//!
//! # Integration with `quantum::optimization`
//!
//! The parent optimization module should expose this subsystem with:
//!
//! ```text
//! pub mod verification;
//! ```
//!
//! The parent optimizer should NOT copy these module declarations into its own
//! namespace.
//!
//! The expected public paths are therefore:
//!
//! ```text
//! crate::quantum::optimization::verification::structural
//! crate::quantum::optimization::verification::semantic
//! crate::quantum::optimization::verification::exhaustive
//! crate::quantum::optimization::verification::randomized
//! crate::quantum::optimization::verification::certificates
//! ```
//!
//! # Integration with `pipeline.rs`
//!
//! `pipeline.rs` should depend on the public contracts of the individual
//! verification modules.
//!
//! It should not depend on implementation-private details.
//!
//! Conceptually:
//!
//! ```text
//! pipeline
//!    │
//!    ├── structural::...
//!    ├── semantic::...
//!    ├── exhaustive::...
//!    ├── randomized::...
//!    └── certificates::...
//! ```
//!
//! The pipeline remains the owner of orchestration order.
//!
//! This module remains the owner of verification namespace and boundaries.
//!
//! # Integration with `result.rs`
//!
//! `result.rs` may store verification reports as optional evidence.
//!
//! The result layer should preserve:
//!
//! - exact proof;
//! - non-equivalence;
//! - inconclusive;
//! - randomized evidence;
//! - certificate metadata.
//!
//! It must not convert an inconclusive report into success.
//!
//! # Integration with `provenance.rs`
//!
//! Provenance may record:
//!
//! - verifier identifier;
//! - verifier version;
//! - method;
//! - configuration;
//! - resource limits;
//! - verdict;
//! - seed for randomized verification;
//! - certificate digest;
//! - verification timestamp if timestamps are explicitly part of the
//!   provenance policy.
//!
//! The core mathematical result should remain reproducible independently of
//! wall-clock metadata.
//!
//! # Integration with routing and hardware
//!
//! Verification happens on logical Quantum IR unless a future verifier
//! explicitly supports a different semantic layer.
//!
//! This subsystem does not:
//!
//! - route;
//! - schedule;
//! - execute;
//! - communicate with hardware.
//!
//! If routing changes a circuit, routing is responsible for requesting
//! appropriate verification through the compiler's verification policy.
//!
//! # Integration with benchmarking
//!
//! Benchmarking may consume verification reports and certificate metadata.
//!
//! Verification MUST NOT depend on benchmarking.
//!
//! This prevents benchmark infrastructure from becoming a correctness
//! dependency of the compiler.
//!
//! # Integration with tests
//!
//! Tests should exercise the verification contracts at several levels:
//!
//! ```text
//! unit tests
//!      │
//!      ▼
//! property tests
//!      │
//!      ▼
//! equivalence tests
//!      │
//!      ▼
//! regression tests
//!      │
//!      ▼
//! integration tests
//!      │
//!      ▼
//! large-circuit/corpus tests
//! ```
//!
//! Tests for one implementation belong primarily in that implementation's
//! module. Cross-engine contracts belong in the optimization verification test
//! suite.
//!
//! # Rust compatibility
//!
//! Target:
//!
//! - Rust 1.97
//! - Rust 1.97.1
//! - Rust 2021 edition
//!
//! Requirements:
//!
//! - no nightly features;
//! - no `unsafe`;
//! - no unsafe FFI;
//! - no compiler-version-specific unstable APIs.
//!
//! # Security and correctness invariants
//!
//! This module enforces the architectural namespace needed to preserve the
//! following properties:
//!
//! - canonical Quantum IR only;
//! - canonical `quantum::ir::qubit::QubitId` only;
//! - no backend I/O;
//! - no QPU execution;
//! - no implicit parameter binding;
//! - no silent approximation;
//! - no silent global-phase relaxation;
//! - no conversion of evidence into proof;
//! - no conversion of inconclusive results into equivalence;
//! - bounded verification;
//! - deterministic verification where promised;
//! - explicit extension points for stronger engines.
//!
//! # Why this file intentionally contains little executable logic
//!
//! `mod.rs` is an architectural boundary, not an algorithm container.
//!
//! Keeping algorithmic code out of this file gives Zamani:
//!
//! - stable module ownership;
//! - independently testable engines;
//! - reduced coupling;
//! - easier parallel development;
//! - easier future replacement of a verifier;
//! - clearer audit boundaries;
//! - smaller compiler dependency surfaces.
//!
//! A verification implementation should therefore be completed in its own
//! file. This file should only need modification when the verification
//! subsystem itself gains a new top-level verification capability.
//!
//! # Current verification implementation status
//!
//! ```text
//! structural.rs       → structural verifier
//! semantic.rs         → semantic-preservation boundary
//! exhaustive.rs       → exhaustive verifier
//! randomized.rs       → randomized/differential verifier
//! certificates.rs     → verification certificates
//! ```
//!
//! The canonical equivalence algorithms remain in:
//!
//! `crate::quantum::optimization::equivalence`
//!
//! This module does not duplicate those algorithms.

/// Structural validation of optimized canonical Quantum IR.
///
/// This verifier is responsible for structural correctness and optimizer
/// boundary invariants, not arbitrary semantic equivalence.
pub mod structural;

/// Semantic-preservation verification.
///
/// This module provides the compiler-facing semantic verification policy and
/// delegates equivalence algorithms to `optimization::equivalence`.
pub mod semantic;

/// Deterministic exhaustive semantic verification.
///
/// This verifier is appropriate when complete computational-basis checking is
/// feasible under the configured resource limits.
pub mod exhaustive;

/// Randomized/differential verification.
///
/// This verifier can discover counterexamples and provide probabilistic
/// evidence but MUST NOT be interpreted as an unconditional exact proof.
pub mod randomized;

/// Deterministic verification certificates.
///
/// Certificates record verification evidence for auditing, reproducibility,
/// and downstream compiler tooling.
pub mod certificates;

// =============================================================================
// Stable subsystem identifiers
// =============================================================================

/// Stable identifier for the complete optimization verification subsystem.
pub const VERIFICATION_SUBSYSTEM_ID: &str =
    "quantum.optimization.verification";

/// Public verification subsystem contract version.
///
/// This version describes the module-level verification API boundary. It is
/// intentionally independent from the Quantum IR version and optimizer
/// implementation version.
pub const VERIFICATION_SUBSYSTEM_VERSION: u32 = 1;

/// Stable identifier for structural verification.
pub const STRUCTURAL_VERIFIER_ID: &str =
    "quantum.optimization.verification.structural";

/// Stable identifier for semantic verification.
pub const SEMANTIC_VERIFIER_ID: &str =
    "quantum.optimization.verification.semantic";

/// Stable identifier for exhaustive verification.
pub const EXHAUSTIVE_VERIFIER_ID: &str =
    "quantum.optimization.verification.exhaustive";

/// Stable identifier for randomized verification.
pub const RANDOMIZED_VERIFIER_ID: &str =
    "quantum.optimization.verification.randomized";

/// Stable identifier for certificate generation.
pub const CERTIFICATE_VERIFIER_ID: &str =
    "quantum.optimization.verification.certificates";

// =============================================================================
// Public verification taxonomy
// =============================================================================

/// Top-level verification capability.
///
/// This is intentionally a capability classification rather than an algorithm
/// selector. Individual modules own their detailed configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationKind {
    /// Canonical IR structural validation.
    Structural,

    /// Deterministic semantic verification.
    Semantic,

    /// Exhaustive deterministic semantic verification.
    Exhaustive,

    /// Randomized/differential verification.
    Randomized,

    /// Verification evidence/certificate generation.
    Certificate,
}

impl VerificationKind {
    /// Returns the stable subsystem identifier associated with this kind.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Structural => STRUCTURAL_VERIFIER_ID,
            Self::Semantic => SEMANTIC_VERIFIER_ID,
            Self::Exhaustive => EXHAUSTIVE_VERIFIER_ID,
            Self::Randomized => RANDOMIZED_VERIFIER_ID,
            Self::Certificate => CERTIFICATE_VERIFIER_ID,
        }
    }

    /// Returns whether this capability can itself establish deterministic
    /// semantic equivalence.
    ///
    /// Randomized verification deliberately returns `false`: it provides
    /// evidence and counterexample discovery rather than an unconditional
    /// mathematical proof for arbitrary circuits.
    #[must_use]
    pub const fn can_prove_exact_equivalence(self) -> bool {
        match self {
            Self::Structural => false,
            Self::Semantic => true,
            Self::Exhaustive => true,
            Self::Randomized => false,
            Self::Certificate => false,
        }
    }

    /// Returns whether this capability is fundamentally observational.
    #[must_use]
    pub const fn is_observational(self) -> bool {
        true
    }
}

// =============================================================================
// Verification strength
// =============================================================================

/// Semantic strength of verification evidence.
///
/// This type deliberately avoids a boolean `verified` flag because such a
/// flag would be too weak to represent the distinction between proof, evidence,
/// counterexample, and inability to decide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationStrength {
    /// Exact deterministic proof.
    Proof,

    /// Deterministic proof that the candidates are not equivalent.
    Counterexample,

    /// Probabilistic/differential evidence without arbitrary-circuit proof.
    Evidence,

    /// The verifier could not establish a definitive conclusion.
    Inconclusive,

    /// Structural validity only; no semantic equivalence claim is made.
    StructuralOnly,
}

impl VerificationStrength {
    /// Returns true only when the result represents a proof of equivalence.
    #[must_use]
    pub const fn is_proof(self) -> bool {
        matches!(self, Self::Proof)
    }

    /// Returns true when the result identifies a semantic counterexample.
    #[must_use]
    pub const fn is_counterexample(self) -> bool {
        matches!(self, Self::Counterexample)
    }

    /// Returns true when the result is probabilistic evidence rather than a
    /// deterministic proof.
    #[must_use]
    pub const fn is_evidence(self) -> bool {
        matches!(self, Self::Evidence)
    }

    /// Returns true when no definitive semantic conclusion was established.
    #[must_use]
    pub const fn is_inconclusive(self) -> bool {
        matches!(self, Self::Inconclusive)
    }

    /// Returns true when only structural validity was established.
    #[must_use]
    pub const fn is_structural_only(self) -> bool {
        matches!(self, Self::StructuralOnly)
    }
}

// =============================================================================
// Public verification policy
// =============================================================================

/// High-level policy for selecting verification strength.
///
/// Detailed algorithm-specific settings remain in the individual verifier
/// modules. This type exists only so compiler orchestration can express intent
/// without depending on implementation details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationPolicy {
    /// Do not perform verification.
    ///
    /// This should normally be restricted to explicitly configured
    /// non-production compiler modes.
    Disabled,

    /// Validate only canonical structure.
    Structural,

    /// Request semantic verification using the semantic verifier's automatic
    /// safe method selection.
    Semantic,

    /// Require deterministic exhaustive verification where supported.
    Exhaustive,

    /// Perform semantic verification and allow randomized evidence as an
    /// additional diagnostic/evidence layer.
    SemanticWithRandomizedEvidence,

    /// Require a proof or fail the compiler operation if proof cannot be
    /// established.
    StrictProof,
}

impl Default for VerificationPolicy {
    fn default() -> Self {
        Self::Semantic
    }
}

impl VerificationPolicy {
    /// Returns true if verification has been explicitly disabled.
    #[must_use]
    pub const fn is_disabled(self) -> bool {
        matches!(self, Self::Disabled)
    }

    /// Returns true if structural verification is required.
    #[must_use]
    pub const fn requires_structural(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns true if exact semantic verification is requested.
    #[must_use]
    pub const fn requires_semantic(self) -> bool {
        matches!(
            self,
            Self::Semantic
                | Self::SemanticWithRandomizedEvidence
                | Self::StrictProof
        )
    }

    /// Returns true if exhaustive verification is explicitly requested.
    #[must_use]
    pub const fn requires_exhaustive(self) -> bool {
        matches!(self, Self::Exhaustive)
    }

    /// Returns true if randomized evidence is requested.
    #[must_use]
    pub const fn allows_randomized_evidence(self) -> bool {
        matches!(self, Self::SemanticWithRandomizedEvidence)
    }

    /// Returns true if the caller requires an actual proof rather than
    /// evidence or an inconclusive result.
    #[must_use]
    pub const fn requires_proof(self) -> bool {
        matches!(self, Self::StrictProof)
    }
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================

/// Compile-time marker proving that this module's canonical qubit contract
/// resolves through `quantum::ir::qubit`.
///
/// The type alias is intentionally private. It exists to make an accidental
/// migration back to `quantum::ir::qubits` fail during compilation rather than
/// silently becoming an architectural regression.
type _CanonicalQubitId = crate::quantum::ir::qubit::QubitId;

/// Compile-time marker proving that verification consumes the canonical
/// QuantumCircuit type.
type _CanonicalQuantumCircuit = crate::quantum::ir::QuantumCircuit;

// =============================================================================
// Tests for module-level contracts
// =============================================================================

#[cfg(test)]
mod tests {
    use super::{
        VerificationKind,
        VerificationPolicy,
        VerificationStrength,
        CERTIFICATE_VERIFIER_ID,
        EXHAUSTIVE_VERIFIER_ID,
        RANDOMIZED_VERIFIER_ID,
        SEMANTIC_VERIFIER_ID,
        STRUCTURAL_VERIFIER_ID,
        VERIFICATION_SUBSYSTEM_ID,
        VERIFICATION_SUBSYSTEM_VERSION,
    };

    #[test]
    fn subsystem_identity_is_stable() {
        assert_eq!(
            VERIFICATION_SUBSYSTEM_ID,
            "quantum.optimization.verification"
        );
        assert_eq!(VERIFICATION_SUBSYSTEM_VERSION, 1);
    }

    #[test]
    fn verifier_ids_are_stable() {
        assert_eq!(
            VerificationKind::Structural.id(),
            STRUCTURAL_VERIFIER_ID
        );
        assert_eq!(
            VerificationKind::Semantic.id(),
            SEMANTIC_VERIFIER_ID
        );
        assert_eq!(
            VerificationKind::Exhaustive.id(),
            EXHAUSTIVE_VERIFIER_ID
        );
        assert_eq!(
            VerificationKind::Randomized.id(),
            RANDOMIZED_VERIFIER_ID
        );
        assert_eq!(
            VerificationKind::Certificate.id(),
            CERTIFICATE_VERIFIER_ID
        );
    }

    #[test]
    fn only_exact_verifiers_claim_proof_capability() {
        assert!(!VerificationKind::Structural.can_prove_exact_equivalence());
        assert!(VerificationKind::Semantic.can_prove_exact_equivalence());
        assert!(VerificationKind::Exhaustive.can_prove_exact_equivalence());
        assert!(!VerificationKind::Randomized.can_prove_exact_equivalence());
        assert!(!VerificationKind::Certificate.can_prove_exact_equivalence());
    }

    #[test]
    fn verification_strength_never_confuses_evidence_with_proof() {
        assert!(VerificationStrength::Proof.is_proof());
        assert!(!VerificationStrength::Evidence.is_proof());
        assert!(!VerificationStrength::Inconclusive.is_proof());
        assert!(VerificationStrength::Evidence.is_evidence());
        assert!(VerificationStrength::Inconclusive.is_inconclusive());
        assert!(VerificationStrength::Counterexample.is_counterexample());
        assert!(VerificationStrength::StructuralOnly.is_structural_only());
    }

    #[test]
    fn default_policy_requests_semantic_verification() {
        assert_eq!(VerificationPolicy::default(), VerificationPolicy::Semantic);
        assert!(VerificationPolicy::Semantic.requires_structural());
        assert!(VerificationPolicy::Semantic.requires_semantic());
        assert!(!VerificationPolicy::Semantic.requires_proof());
    }

    #[test]
    fn strict_policy_requires_proof() {
        assert!(VerificationPolicy::StrictProof.requires_structural());
        assert!(VerificationPolicy::StrictProof.requires_semantic());
        assert!(VerificationPolicy::StrictProof.requires_proof());
        assert!(!VerificationPolicy::StrictProof.allows_randomized_evidence());
    }

    #[test]
    fn randomized_policy_is_explicitly_evidence_oriented() {
        let policy = VerificationPolicy::SemanticWithRandomizedEvidence;

        assert!(policy.requires_structural());
        assert!(policy.requires_semantic());
        assert!(policy.allows_randomized_evidence());
        assert!(!policy.requires_proof());
    }

    #[test]
    fn disabled_policy_does_not_require_verification() {
        let policy = VerificationPolicy::Disabled;

        assert!(policy.is_disabled());
        assert!(!policy.requires_structural());
        assert!(!policy.requires_semantic());
        assert!(!policy.requires_exhaustive());
        assert!(!policy.allows_randomized_evidence());
        assert!(!policy.requires_proof());
    }
}