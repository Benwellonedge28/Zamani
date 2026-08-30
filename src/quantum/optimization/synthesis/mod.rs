//! Zamani Quantum Optimization — Synthesis
//!
//! Production synthesis subsystem for the Zamani quantum compiler.
//!
//! # Architectural position
//!
//! Synthesis is a middle-end capability of quantum optimization:
//
//! ```text
//!                         Zamani source
//!                              │
//!                              ▼
//!                     quantum::frontend
//!                              │
//!                              ▼
//!                       quantum::ir
//!                              │
//!                              ▼
//!                 quantum::optimization
//!                              │
//!                    ┌─────────┴─────────┐
//!                    │                   │
//!                    ▼                   ▼
//!                algebra             synthesis
//!                                        │
//!               ┌────────────────────────┼────────────────────────┐
//!               │                        │                        │
//!               ▼                        ▼                        ▼
//!        single_qubit              two_qubit                Clifford
//!               │                        │                        │
//!               ▼                        ▼                        ▼
//!             phase                 unitary                 isometry
//!               │                        │                        │
//!               └────────────────────────┼────────────────────────┘
//!                                        ▼
//!                              canonical quantum::ir
//!                                        │
//!                                        ▼
//!                                     routing
//!                                        │
//!                                        ▼
//!                                    scheduling
//!                                        │
//!                                        ▼
//!                                     hardware
//! ```
//!
//! The canonical semantic representation remains:
//!
//! `crate::quantum::ir`
//!
//! This module owns only synthesis namespace composition and synthesis-level
//! public contracts. Individual synthesis algorithms remain in their
//! specialized child modules.
//!
//! # Responsibilities
//!
//! This subsystem provides the namespace for:
//!
//! - single-qubit unitary synthesis;
//! - two-qubit synthesis;
//! - Clifford synthesis;
//! - phase-polynomial synthesis;
//! - general unitary synthesis;
//! - isometry/state-preparation synthesis;
//! - future synthesis algorithms;
//! - deterministic resource accounting;
//! - target-independent synthesis contracts;
//! - synthesis capability discovery.
//!
//! It does NOT own:
//!
//! - source-language parsing;
//! - OpenQASM parsing;
//! - the canonical Quantum IR;
//! - optimization pass scheduling;
//! - routing;
//! - physical qubit placement;
//! - hardware topology;
//! - pulse scheduling;
//! - backend execution;
//! - QPU communication;
//! - quantum error-correction codes;
//! - benchmark execution;
//! - algorithm construction.
//!
//! Those responsibilities remain in their respective quantum subsystems.
//!
//! # Canonical IR invariant
//!
//! No synthesis child module is permitted to define another:
//!
//! - QuantumCircuit;
//! - QuantumOperation;
//! - QuantumGate;
//! - Qubit;
//! - canonical parameter representation.
//!
//! Temporary mathematical representations are permitted when required by the
//! synthesis algorithm, but they must have explicit semantic ownership and
//! must lower back into `crate::quantum::ir` or an explicitly documented
//! backend-neutral synthesis plan.
//!
//! This rule prevents the optimizer from developing multiple incompatible
//! quantum IRs.
//!
//! # Synthesis versus decomposition
//!
//! Synthesis and decomposition are deliberately exposed through this same
//! subsystem but are not semantically identical.
//!
//! Decomposition normally means:
//!
//! ```text
//! known high-level operation
//!          │
//!          ▼
//! known equivalent lower-level implementation
//! ```
//!
//! Synthesis means:
//!
//! ```text
//! mathematical specification
//!          │
//!          ▼
//! construct an implementation
//! ```
//!
//! The child modules may therefore use very different algorithms while sharing
//! this namespace.
//!
//! # Scaling model
//!
//! "Scalable to infinity" is interpreted as:
//!
//! > The subsystem imposes no artificial circuit-size ceiling beyond explicit
//! > resource budgets, addressable memory, integer ranges, algorithmic
//! > complexity, and the resources actually available to the compiler.
//!
//! This is not the same as claiming that every synthesis problem has a
//! polynomial-time or finite-memory solution.
//!
//! Different synthesis domains have fundamentally different complexity:
//
//! - single-qubit synthesis is constant-size mathematically;
//! - Clifford synthesis is polynomial in tableau size;
//! - phase-polynomial synthesis scales with qubit/term/parity structure;
//! - arbitrary unitary synthesis can grow exponentially with qubit count;
//! - exact isometry synthesis can also grow exponentially;
//! - globally optimal synthesis may be computationally intractable.
//!
//! Therefore this module does not pretend that all synthesis can be solved
//! optimally at arbitrary scale.
//!
//! Instead, production scalability is achieved through:
//!
//! - explicit budgets;
//! - bounded algorithms;
//! - fallible allocation;
//! - checked arithmetic;
//! - deterministic behavior;
//! - target-specific strategies;
//! - streaming/chunkable plans where applicable;
//! - polynomial representations where available;
//! - no dense exponential representation unless the caller explicitly selects
//!   an algorithm requiring it;
//! - no recursion proportional to unbounded circuit size;
//! - no unsafe code.
//!
//! # Resource policy
//!
//! Individual synthesis modules own their domain-specific limits.
//!
//! The parent module intentionally does NOT introduce a second global resource
//! limit type. Global optimization limits belong to:
//!
//! `crate::quantum::optimization::limits`
//!
//! Synthesis-specific limits belong to the corresponding synthesis module.
//!
//! This avoids two competing sources of truth.
//!
//! # Determinism
//!
//! The namespace itself introduces no nondeterminism.
//!
//! Individual synthesis algorithms are expected to document whether they are:
//!
//! - deterministic;
//! - seeded-randomized;
//! - heuristic;
//! - approximate;
//! - exact.
//!
//! Any randomized synthesis must receive its random seed explicitly through
//! its public configuration rather than using process-global randomness.
//!
//! # Verification
//!
//! Synthesis implementations are responsible for verifying their mathematical
//! output when their configuration requires verification.
//!
//! The synthesis namespace does not perform verification itself because the
//! correct verification method depends on the synthesis domain.
//!
//! Examples:
//!
//! - Clifford synthesis → tableau equivalence;
//! - single-qubit synthesis → matrix equivalence up to the configured phase
//!   relation;
//! - phase synthesis → phase-polynomial equivalence;
//! - general unitary synthesis → exact or approximate unitary equivalence;
//! - isometry synthesis → isometry/state-preparation equivalence.
//!
//! Higher-level optimization verification remains owned by:
//!
//! `crate::quantum::optimization::verification`
//!
//! # Target independence
//!
//! Synthesis algorithms must not directly communicate with hardware backends.
//!
//! The permitted direction is:
//!
//! ```text
//! OptimizationTarget
//!        │
//!        ▼
//! synthesis configuration/capabilities
//!        │
//!        ▼
//! synthesis algorithm
//!        │
//!        ▼
//! canonical Quantum IR
//! ```
//!
//! Hardware topology and physical qubit placement remain owned by routing.
//!
//! # Gate-set independence
//!
//! The synthesis subsystem must not assume that H/S/CX is the universal target
//! for every operation.
//!
//! Some child modules have a natural canonical basis. For example, the current
//! Clifford synthesizer emits H/S/CX because those operations generate the
//! Clifford group.
//!
//! That is an algorithm-level implementation choice, not a global restriction
//! on Zamani synthesis.
//!
//! Future target-aware lowering can transform the synthesized result into a
//! target-native gate set without modifying this namespace.
//!
//! # Public API stability
//!
//! The child module paths are intentionally stable:
//!
//! ```text
//! quantum::optimization::synthesis::single_qubit
//! quantum::optimization::synthesis::two_qubit
//! quantum::optimization::synthesis::clifford
//! quantum::optimization::synthesis::phase
//! quantum::optimization::synthesis::unitary
//! quantum::optimization::synthesis::isometry
//! ```
//!
//! New synthesis algorithms should normally be added as new child modules
//! rather than expanding this file with implementation-specific algorithms.
//!
//! This means that adding, for example:
//!
//! ```text
//! synthesis::zx
//! synthesis::kak
//! synthesis::qsd
//! synthesis::cosine_sine
//! synthesis::pauli
//! synthesis::stabilizer
//! synthesis::variational
//! ```
//!
//! does not require changing the existing synthesis contracts.
//!
//! # Current synthesis domains
//!
//! ## `single_qubit`
//!
//! Single-qubit unitary mathematics and decomposition.
//!
//! The current implementation provides a dependency-free fixed-size complex
//! representation, unitary validation, global-phase-aware comparison, Euler
//! decompositions and canonical-gate decomposition facilities.
//!
//! It uses fixed-size storage and therefore does not allocate proportional to
//! circuit size.
//!
//! ## `two_qubit`
//!
//! Two-qubit unitary decomposition and synthesis.
//!
//! This is the correct home for canonical two-qubit decompositions such as
//! KAK/Cartan-style constructions and target-specific two-qubit synthesis.
//!
//! ## `clifford`
//!
//! Exact Clifford synthesis from the Zamani Clifford tableau representation.
//!
//! The existing implementation deliberately uses a polynomial tableau instead
//! of a dense `2^n × 2^n` matrix and emits H/S/CX operations.
//!
//! ## `phase`
//!
//! Phase-polynomial synthesis.
//!
//! This module intentionally returns a backend-neutral synthesis plan where
//! necessary because global phase currently has no dedicated canonical GateKind.
//!
//! It must not silently discard global phase.
//!
//! ## `unitary`
//!
//! General unitary synthesis.
//!
//! This domain is potentially exponential and therefore must remain explicitly
//! budgeted. It must never silently attempt an exponentially large operation
//! for an unbounded input.
//!
//! ## `isometry`
//!
//! Isometry/state-preparation synthesis.
//!
//! This domain has similar exponential worst-case behavior and must therefore
//! expose explicit resource controls.
//!
//! # Integration contract with optimization
//!
//! The optimizer may consume this subsystem through:
//!
//! ```text
//! crate::quantum::optimization::synthesis
//! ```
//!
//! The preferred direction is:
//!
//! ```text
//! optimization pass
//!        │
//!        ▼
//! synthesis child module
//!        │
//!        ▼
//! synthesized representation / plan
//!        │
//!        ▼
//! canonical quantum::ir
//! ```
//!
//! Synthesis must not call the optimizer recursively unless a future,
//! explicitly designed synthesis algorithm requires an optimization callback.
//! Such callbacks must be injected through a contract rather than importing
//! the complete optimization pipeline.
//!
//! # Integration contract with algebra
//!
//! Algebra provides mathematical representations.
//!
//! Synthesis consumes them.
//!
//! The intended relationship is:
//!
//! ```text
//! algebra::clifford
//!          │
//!          ▼
//! synthesis::clifford
//! ```
//!
//! and:
//!
//! ```text
//! algebra::phase_polynomial
//!          │
//!          ▼
//! synthesis::phase
//! ```
//!
//! Algebra does not depend on synthesis.
//!
//! This one-way dependency prevents cyclic architecture.
//!
//! # Integration contract with targets
//!
//! Target information may be used to choose a synthesis strategy, but target
//! definitions themselves remain outside this module.
//!
//! The preferred architecture is:
//!
//! ```text
//! optimization::targets
//!          │
//!          ▼
//! planner
//!          │
//!          ▼
//! synthesis configuration
//! ```
//!
//! A synthesis child must not import hardware topology simply because a target
//! happens to originate from a hardware backend.
//!
//! # Integration contract with routing
//!
//! The synthesis result is logical.
//!
//! Routing owns conversion from logical qubit relationships to physical
//! connectivity.
//!
//! Therefore:
//!
//! ```text
//! synthesis
//!     │
//!     ▼
//! logical canonical IR
//!     │
//!     ▼
//! routing
//!     │
//!     ▼
//! physical canonical IR
//! ```
//!
//! A future architecture may allow routing to provide cost information to the
//! optimizer, but routing remains the owner of physical placement.
//!
//! # Integration contract with scheduling
//!
//! Synthesis must not assign physical execution times.
//!
//! Scheduling consumes the synthesized/routed circuit later.
//!
//! ```text
//! synthesis → routing → scheduling
//! ```
//!
//! # Integration contract with verification
//!
//! Synthesis-specific verification belongs close to the algorithm because the
//! algorithm knows its mathematical representation.
//!
//! Global compiler-level verification belongs to:
//!
//! `crate::quantum::optimization::verification`
//!
//! This separation allows a Clifford synthesizer to verify a tableau without
//! constructing a dense matrix, while a single-qubit synthesizer can use its
//! fixed-size matrix representation.
//!
//! # Integration contract with benchmarking
//!
//! Benchmarking must consume synthesis statistics rather than become a
//! dependency of this module.
//!
//! Correct dependency direction:
//!
//! ```text
//! benchmarking → optimization/synthesis
//! ```
//!
//! Never:
//!
//! ```text
//! synthesis → benchmarking
//! ```
//!
//! # Integration contract with algorithms
//!
//! Quantum algorithms may construct canonical circuits that later enter the
//! optimization pipeline.
//!
//! Synthesis is not an algorithm-construction layer.
//!
//! Therefore:
//!
//! ```text
//! algorithms → IR → optimization → synthesis
//! ```
//!
//! not:
//!
//! ```text
//! synthesis → algorithms
//! ```
//!
//! # Integration contract with error correction
//!
//! Logical fault-tolerant synthesis is allowed to consume fault-tolerant
//! mathematical models, but this namespace does not own QEC encoding,
//! decoding, syndrome processing or code semantics.
//!
//! QEC remains:
//!
//! `crate::quantum::error_correction`
//!
//! # Compatibility
//!
//! Target:
//!
//! - Rust 1.97;
//! - Rust 1.97.1;
//! - Rust 2021;
//! - stable Rust;
//! - no nightly features;
//! - no unsafe code.
//!
//! This module deliberately uses only Rust's module system and the standard
//! library. It introduces no dependency solely for namespace composition.
//!
//! # Safety
//!
//! `unsafe` is forbidden for this entire synthesis namespace.
//!
//! The attribute below makes accidental introduction of unsafe code fail the
//! build.
//!
//! # Versioning
//!
//! `SYNTHESIS_API_VERSION` describes the public namespace contract, not the
//! implementation version of any individual synthesis algorithm.
//!
//! Individual algorithms may expose their own API versions when required.
//!
//! # References
//!
//! The architecture is informed by established quantum compiler approaches:
//!
//! - scalable quantum IR/optimization representations;
//! - cost-directed circuit transformation;
//! - tableau-based Clifford synthesis;
//! - phase-polynomial/parity-network synthesis;
//! - arbitrary-gate-set synthesis.
//!
//! The implementation remains Zamani-owned and uses Zamani's canonical IR.
//!
//! -----------------------------------------------------------------------------
//! Module declarations
//! -----------------------------------------------------------------------------
//
// These declarations are deliberately kept together and explicit. They are
// the only namespace-composition responsibility of this file.
//
// Each child module owns its own implementation, public types, algorithms,
// limits, validation and tests.
//
// Adding a future synthesis domain is therefore additive:
////
//!     pub mod new_domain;
//
// No existing child module needs to be edited merely because another synthesis
//! domain is introduced.

#![deny(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

/// Stable public synthesis namespace API version.
///
/// Increment this when the public synthesis-module contract itself changes in
/// a breaking way. Algorithm-specific versions remain owned by their child
/// modules.
pub const SYNTHESIS_API_VERSION: u32 = 1;

/// Single-qubit unitary synthesis and decomposition.
///
/// This module operates on fixed-size 2×2 complex matrices and can decompose
/// arbitrary validated single-qubit unitaries into canonical Zamani gate
/// operations.
pub mod single_qubit;

/// Two-qubit unitary synthesis and decomposition.
///
/// This module owns two-qubit-specific mathematical decomposition algorithms
/// and their resource controls.
pub mod two_qubit;

/// Exact Clifford transformation synthesis.
///
/// The implementation uses the Zamani Clifford/tableau representation rather
/// than exponentially sized dense matrices.
pub mod clifford;

/// Phase-polynomial and parity-network synthesis.
///
/// This module owns phase-gadget, parity-network and affine-parity synthesis
/// plans.
pub mod phase;

/// General unitary synthesis.
///
/// Potentially exponential algorithms in this module must remain explicitly
/// budgeted and must never silently bypass resource limits.
pub mod unitary;

/// Isometry and state-preparation synthesis.
///
/// Potentially exponential algorithms in this module must expose explicit
/// resource controls and failure modes.
pub mod isometry;

/// Stable synthesis-domain classification.
///
/// This is intentionally descriptive rather than an algorithm-selection API.
/// The optimization planner remains responsible for selecting an algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SynthesisDomain {
    /// Single-qubit unitary synthesis.
    SingleQubit,

    /// Two-qubit unitary synthesis.
    TwoQubit,

    /// Clifford synthesis.
    Clifford,

    /// Phase-polynomial synthesis.
    PhasePolynomial,

    /// General unitary synthesis.
    Unitary,

    /// Isometry/state-preparation synthesis.
    Isometry,
}

impl SynthesisDomain {
    /// Returns a stable machine-readable identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SingleQubit => "single-qubit",
            Self::TwoQubit => "two-qubit",
            Self::Clifford => "clifford",
            Self::PhasePolynomial => "phase-polynomial",
            Self::Unitary => "unitary",
            Self::Isometry => "isometry",
        }
    }

    /// Returns a human-readable description.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::SingleQubit => "single-qubit unitary synthesis",
            Self::TwoQubit => "two-qubit unitary synthesis",
            Self::Clifford => "exact Clifford synthesis",
            Self::PhasePolynomial => "phase-polynomial and parity-network synthesis",
            Self::Unitary => "general unitary synthesis",
            Self::Isometry => "isometry and state-preparation synthesis",
        }
    }

    /// Returns whether the domain has a potentially exponential worst-case
    /// synthesis representation.
    ///
    /// This is informational metadata for planners and diagnostics. It does
    /// not imply that every implementation of the domain is exponential.
    #[must_use]
    pub const fn may_be_exponential(self) -> bool {
        match self {
            Self::SingleQubit | Self::TwoQubit | Self::Clifford | Self::PhasePolynomial => false,
            Self::Unitary | Self::Isometry => true,
        }
    }
}

/// Stable description of synthesis capabilities exposed by this namespace.
///
/// This structure intentionally contains only compile-time/domain-level
/// capabilities. Concrete target capabilities belong to
/// `optimization::targets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynthesisCapabilities {
    /// Whether single-qubit synthesis is available.
    pub single_qubit: bool,

    /// Whether two-qubit synthesis is available.
    pub two_qubit: bool,

    /// Whether Clifford synthesis is available.
    pub clifford: bool,

    /// Whether phase-polynomial synthesis is available.
    pub phase_polynomial: bool,

    /// Whether general unitary synthesis is available.
    pub unitary: bool,

    /// Whether isometry synthesis is available.
    pub isometry: bool,
}

impl SynthesisCapabilities {
    /// Returns the capabilities of the current built-in synthesis subsystem.
    #[must_use]
    pub const fn built_in() -> Self {
        Self {
            single_qubit: true,
            two_qubit: true,
            clifford: true,
            phase_polynomial: true,
            unitary: true,
            isometry: true,
        }
    }

    /// Returns whether a domain is available.
    #[must_use]
    pub const fn supports(self, domain: SynthesisDomain) -> bool {
        match domain {
            SynthesisDomain::SingleQubit => self.single_qubit,
            SynthesisDomain::TwoQubit => self.two_qubit,
            SynthesisDomain::Clifford => self.clifford,
            SynthesisDomain::PhasePolynomial => self.phase_polynomial,
            SynthesisDomain::Unitary => self.unitary,
            SynthesisDomain::Isometry => self.isometry,
        }
    }
}

impl Default for SynthesisCapabilities {
    fn default() -> Self {
        Self::built_in()
    }
}

/// Lightweight synthesis namespace metadata.
///
/// This is useful for diagnostics, compiler introspection and future plugin
/// registries without exposing implementation internals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynthesisInfo {
    /// Public synthesis API version.
    pub api_version: u32,

    /// Built-in capabilities.
    pub capabilities: SynthesisCapabilities,
}

impl SynthesisInfo {
    /// Returns metadata for the built-in synthesis subsystem.
    #[must_use]
    pub const fn built_in() -> Self {
        Self {
            api_version: SYNTHESIS_API_VERSION,
            capabilities: SynthesisCapabilities::built_in(),
        }
    }
}

impl Default for SynthesisInfo {
    fn default() -> Self {
        Self::built_in()
    }
}

/// Returns metadata describing the built-in synthesis subsystem.
///
/// This function is intentionally allocation-free and side-effect-free.
#[must_use]
pub const fn info() -> SynthesisInfo {
    SynthesisInfo::built_in()
}

/// Returns the stable list of synthesis domains currently implemented by
/// Zamani.
///
/// The returned array is fixed-size and therefore allocation-free.
#[must_use]
pub const fn domains() -> [SynthesisDomain; 6] {
    [
        SynthesisDomain::SingleQubit,
        SynthesisDomain::TwoQubit,
        SynthesisDomain::Clifford,
        SynthesisDomain::PhasePolynomial,
        SynthesisDomain::Unitary,
        SynthesisDomain::Isometry,
    ]
}

/// Returns whether the requested synthesis domain is implemented by the
/// current built-in subsystem.
///
/// This helper is intentionally independent of target gate sets and optimizer
/// configuration. It answers only whether Zamani has an implementation for the
/// mathematical synthesis domain.
#[must_use]
pub const fn supports(domain: SynthesisDomain) -> bool {
    SynthesisCapabilities::built_in().supports(domain)
}

// =============================================================================
// Controlled public prelude
// =============================================================================

/// Small, intentionally curated synthesis prelude.
///
/// The prelude exposes namespace-level contracts rather than glob-exporting
/// every algorithm-specific symbol. This prevents unrelated synthesis modules
/// from creating public-name collisions as the subsystem grows.
pub mod prelude {
    pub use super::{
        domains,
        info,
        supports,
        SynthesisCapabilities,
        SynthesisDomain,
        SynthesisInfo,
        SYNTHESIS_API_VERSION,
    };

    pub use super::clifford;
    pub use super::isometry;
    pub use super::phase;
    pub use super::single_qubit;
    pub use super::two_qubit;
    pub use super::unitary;
}

// =============================================================================
// Compile-time architectural assertions
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_stable_and_nonzero() {
        assert!(SYNTHESIS_API_VERSION > 0);
        assert_eq!(info().api_version, SYNTHESIS_API_VERSION);
    }

    #[test]
    fn all_builtin_domains_are_advertised() {
        let domains = domains();

        assert_eq!(domains.len(), 6);

        for domain in domains {
            assert!(supports(domain));
        }
    }

    #[test]
    fn domain_identifiers_are_stable_and_nonempty() {
        for domain in domains() {
            assert!(!domain.id().is_empty());
            assert!(!domain.description().is_empty());
        }
    }

    #[test]
    fn exponential_domains_are_explicit() {
        assert!(!SynthesisDomain::SingleQubit.may_be_exponential());
        assert!(!SynthesisDomain::TwoQubit.may_be_exponential());
        assert!(!SynthesisDomain::Clifford.may_be_exponential());
        assert!(!SynthesisDomain::PhasePolynomial.may_be_exponential());

        assert!(SynthesisDomain::Unitary.may_be_exponential());
        assert!(SynthesisDomain::Isometry.may_be_exponential());
    }

    #[test]
    fn builtin_capabilities_match_domain_list() {
        let capabilities = SynthesisCapabilities::built_in();

        for domain in domains() {
            assert!(capabilities.supports(domain));
        }
    }

    #[test]
    fn namespace_metadata_is_allocation_free() {
        let metadata = SynthesisInfo::built_in();

        assert_eq!(metadata.api_version, SYNTHESIS_API_VERSION);
        assert!(metadata.capabilities.single_qubit);
        assert!(metadata.capabilities.two_qubit);
        assert!(metadata.capabilities.clifford);
        assert!(metadata.capabilities.phase_polynomial);
        assert!(metadata.capabilities.unitary);
        assert!(metadata.capabilities.isometry);
    }
}