
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![deny(missing_docs)]

//! # Zamani Compiler — Reproducibility
//!
//! Path:
//!     src/compiler/reproducibility/mod.rs
//!
//! Edition:
//!     Rust 2021
//!
//! Minimum supported Rust version:
//!     Rust 1.97
//!
//! Safety:
//!     Safe Rust only.
//!
//! # 1. Mission
//!
//! Provide the canonical compiler-facing composition boundary for
//! reproducible compilation, deterministic build assessment,
//! compilation-input identity, environment identity and
//! provenance fingerprinting.
//!
//! The architectural objective is:
//!
//!     Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever
//!
//!     POCO-REAF
//!
//! Reproducibility must preserve the intended computation independently
//! of incidental physical machine characteristics.
//!
//! This module coordinates reproducibility contracts across the
//! compiler without introducing competing semantic, IR, fingerprint,
//! provenance, artifact or security authorities.
//!
//! # 2. Ownership
//!
//! This module owns:
//!
//! - Reproducibility subsystem composition.
//! - Public reproducibility module organization.
//! - Reproducibility API exposure.
//! - Cross-module architectural boundaries.
//! - Reproducibility subsystem integration contracts.
//! - Reproducibility subsystem API versioning.
//!
//! This module does not own:
//!
//! - Source grammar.
//! - Lexical analysis.
//! - Parsing.
//! - AST construction.
//! - Source-level semantic interpretation.
//! - Semantic reproducibility policy.
//! - Numerical arithmetic.
//! - Canonical IR definitions.
//! - Domain-specific IR definitions.
//! - Fingerprint algorithms.
//! - Dependency resolution.
//! - Provenance ledger storage.
//! - Artifact serialization.
//! - Cryptographic signing.
//! - Supply-chain authentication.
//! - Backend implementation.
//! - Physical hardware discovery.
//! - Physical resource allocation.
//! - Runtime scheduling.
//! - Physical execution.
//!
//! # 3. Canonical authorities
//!
//! ## Source syntax
//!
//!     grammar/compile/reproducibility.g4
//!     grammar/compile/deterministic-builds.g4
//!     grammar/compile/provenance.g4
//!
//! ## Normative provenance semantics
//!
//!     grammar/spec/provenance.md
//!
//! ## Frontend semantic authority
//!
//!     crate::frontend::semantic
//!
//! ## Canonical universal IR
//!
//!     crate::zuir
//!
//! ## Canonical fingerprint implementation
//!
//!     crate::zuir::fingerprint::stable_hash
//!
//! ## Compiler fingerprint contracts
//!
//!     crate::compiler::incremental::fingerprints
//!
//! ## Authoritative provenance ledger
//!
//!     crate::compiler::context::provenance_context
//!
//! ## Provenance analysis
//!
//!     crate::compiler::analysis::provenance
//!
//! ## Artifact management
//!
//!     crate::compiler::artifacts
//!
//! ## Security and trust verification
//!
//!     crate::compiler::security
//!
//! ## Backend reproducibility
//!
//!     crate::backend::reproducibility
//!
//! ## Classical numerical reproducibility
//!
//!     crate::classical::numeric::reproducibility
//!
//! These authorities remain independent.
//!
//! This composition module must not replace or duplicate them.
//!
//! # 4. Subsystem architecture
//!
//! The reproducibility subsystem consists of four independently
//! owned implementation modules.
//!
//! ```text
//! compiler/reproducibility/
//! |
//! +-- mod.rs
//! |
//! +-- environment.rs
//! |   |
//! |   +-- CompilationEnvironment
//! |   +-- EnvironmentScope
//! |   +-- EnvironmentFact
//! |   +-- ToolchainIdentity
//! |   +-- Environment fingerprint
//! |
//! +-- inputs.rs
//! |   |
//! |   +-- CompilationInputs
//! |   +-- CompilationInputsBuilder
//! |   +-- SourceInput
//! |   +-- InputCategory
//! |   +-- Complete input fingerprint
//! |
//! +-- determinism.rs
//! |   |
//! |   +-- ComponentId
//! |   +-- DeterminismObservation
//! |   +-- DeterminismEvidence
//! |   +-- DeterminismAssessment
//! |   +-- Assessment coverage
//! |
//! +-- provenance.rs
//!     |
//!     +-- ProvenanceFingerprint
//!     +-- ProvenanceFingerprintError
//!     +-- fingerprint_snapshot
//!     +-- fingerprint_context
//! ```
//!
//! The diagram describes actual module ownership.
//! It does not introduce replacement data models.
//!
//! # 5. Module responsibilities
//!
//! ## 5.1 Environment
//!
//! `environment.rs` owns immutable compilation-environment identity.
//!
//! It records explicitly supplied, output-affecting information,
//! including:
//!
//! - Compiler implementation fingerprint.
//! - Grammar fingerprint.
//! - Semantic-model fingerprint.
//! - Optional target fingerprint.
//! - Toolchain fingerprints.
//! - Explicit environment facts.
//! - Environment fingerprint schema.
//!
//! It does not discover the host environment or hardware.
//!
//! Environment information must be supplied by authoritative
//! compiler, toolchain and target providers.
//!
//! Portable environment identity must not accidentally depend
//! on incidental physical machine characteristics.
//!
//! ## 5.2 Compilation inputs
//!
//! `inputs.rs` owns complete compilation-input identity.
//!
//! It integrates:
//!
//! - Source-unit identities.
//! - Named input identities.
//! - Dependency fingerprints.
//! - Configuration fingerprints.
//! - Semantic-context fingerprints.
//! - Canonical IR fingerprints.
//! - Domain-specific IR fingerprints.
//! - Generated input fingerprints.
//! - Compiler component fingerprints.
//! - Explicit environment identity.
//!
//! Dependency discovery remains owned by dependency resolution
//! and the canonical compilation dependency graph.
//!
//! The inputs module records the identities supplied by those
//! authoritative systems.
//!
//! ## 5.3 Determinism
//!
//! `determinism.rs` owns compiler-wide determinism assessment.
//!
//! It evaluates the determinism declarations and evidence supplied
//! by participating compiler components.
//!
//! It does not claim that an implementation is deterministic merely
//! because that implementation declares itself deterministic.
//!
//! Actual verification remains the responsibility of the relevant
//! implementation, testing, security and assurance systems.
//!
//! The component inventory must be supplied by the actual
//! compilation architecture.
//!
//! No closed computing-domain registry is permitted.
//!
//! ## 5.4 Provenance
//!
//! `provenance.rs` owns reproducibility-specific fingerprinting
//! of authoritative provenance snapshots.
//!
//! The provenance ledger itself remains owned by:
//!
//!     crate::compiler::context::provenance_context
//!
//! The provenance fingerprint must preserve the canonical event
//! representation and causal relationships.
//!
//! It must not create a second provenance ledger.
//!
//! It must not treat insertion order or ledger revision as
//! semantic content identity when the authoritative provenance
//! contract defines equivalent causal graphs.
//!
//! # 6. Canonical integration flow
//!
//! The intended integration flow is:
//!
//! ```text
//! Zamani source
//!      |
//!      v
//! Canonical frontend
//!      |
//!      v
//! Semantic validation
//!      |
//!      v
//! Canonical semantic model
//!      |
//!      v
//! Canonical IR construction
//!      |
//!      v
//! Verified IR
//!      |
//!      v
//! Compilation dependency graph
//!      |
//!      v
//! Explicit compilation inputs
//!      |
//!      +-------------------------+
//!      |                         |
//!      v                         v
//! Compilation environment   Compiler components
//!      |                         |
//!      +-------------+-----------+
//!                    |
//!                    v
//!           Determinism assessment
//!                    |
//!                    v
//!          Compiler transformations
//!                    |
//!                    v
//!           Optimization / lowering
//!                    |
//!                    v
//!            Backend realization
//!                    |
//!                    v
//!              Artifacts
//!                    |
//!                    v
//!          Artifact verification
//!                    |
//!                    v
//!            Provenance ledger
//!                    |
//!                    v
//!         Provenance fingerprint
//!                    |
//!                    v
//!         Reproducibility evidence
//! ```
//!
//! This is a coordination contract, not an assertion that every
//! pipeline stage is already implemented.
//!
//! The pipeline remains the owner of stage execution and ordering.
//!
//! # 7. Repository integration contracts
//!
//! ## 7.1 Compiler composition root
//!
//!     src/compiler/mod.rs
//!
//! The parent compiler module must declare:
//!
//!     pub mod reproducibility;
//!
//! This module must not modify or redeclare the parent compiler.
//!
//! ## 7.2 Incremental compilation
//!
//!     src/compiler/incremental/
//!
//! Incremental compilation owns:
//!
//! - Cache keys.
//! - Cache storage.
//! - Cache persistence.
//! - Dependency invalidation.
//! - Incremental build planning.
//! - Reuse decisions.
//!
//! Reproducibility supplies stable identities and determinism
//! evidence.
//!
//! A cache hit must never bypass mandatory semantic validation,
//! IR verification, security checks or artifact validation.
//!
//! ## 7.3 Compiler context
//!
//!     src/compiler/context/
//!
//! Compilation contexts supply shared compilation information.
//!
//! The provenance context owns the authoritative append-only
//! provenance ledger.
//!
//! Reproducibility may consume immutable provenance snapshots.
//!
//! It must not create another ledger or lifecycle authority.
//!
//! ## 7.4 Analysis
//!
//!     src/compiler/analysis/
//!
//! Analysis owns compiler-facing interpretation of dependencies,
//! requirements, capabilities, effects, feasibility and provenance.
//!
//! Reproducibility consumes the relevant results and identities.
//!
//! It does not repeat source-level semantic analysis.
//!
//! ## 7.5 Canonical IR
//!
//!     src/zuir/
//!     src/classical/
//!     src/quantum/ir/
//!     src/hdl/hdlir/
//!     src/ai/
//!     src/hybrid/
//!     src/distributed/
//!
//! Each subsystem owns its authoritative computation representation.
//!
//! Reproducibility accepts their canonical identities and fingerprints.
//!
//! It must not define a competing universal IR or domain IR.
//!
//! ## 7.6 Artifacts
//!
//!     src/compiler/artifacts/
//!
//! Artifacts own:
//!
//! - Artifact identities.
//! - Artifact payloads.
//! - Artifact manifests.
//! - Artifact serialization.
//! - Artifact portability metadata.
//! - Artifact validation.
//!
//! Reproducibility supplies the identities and evidence needed
//! by artifact metadata and validation.
//!
//! It does not serialize or persist artifacts.
//!
//! ## 7.7 Security
//!
//!     src/compiler/security/
//!
//! Security owns:
//!
//! - Trust verification.
//! - Cryptographic authentication.
//! - Supply-chain verification.
//! - Provider trust.
//! - Artifact authenticity.
//! - Security policy enforcement.
//!
//! Fingerprints are not digital signatures.
//!
//! Fingerprint equality does not establish authenticity,
//! authorization, correctness or safety.
//!
//! ## 7.8 Backend
//!
//!     src/backend/reproducibility.rs
//!
//! Backend reproducibility owns backend-specific input and output
//! fingerprint contracts.
//!
//! Compiler reproducibility coordinates those identities.
//!
//! Neither subsystem should independently redefine the other's
//! canonical fingerprint encoding.
//!
//! ## 7.9 Classical numerical reproducibility
//!
//!     src/classical/numeric/reproducibility.rs
//!
//! Classical numerical reproducibility owns numerical reduction
//! policies and arithmetic reproducibility contracts.
//!
//! Compiler-wide determinism assessment must preserve those
//! numerical guarantees.
//!
//! Numerical determinism must not be confused with universal
//! bitwise reproducibility across all hardware.
//!
//! # 8. Grammar integration
//!
//! Source-level intent is established by the canonical grammar:
//!
//!     grammar/compile/reproducibility.g4
//!     grammar/compile/deterministic-builds.g4
//!     grammar/compile/provenance.g4
//!
//! Their parsed constructs must be preserved by the canonical AST
//! and interpreted by the authoritative semantic implementation.
//!
//! This module receives validated semantic and compiler contracts.
//!
//! It must not:
//!
//! - Import ANTLR parser internals.
//! - Parse grammar productions.
//! - Create a second AST.
//! - Define a second semantic model.
//! - Interpret source declarations independently.
//! - Turn syntax into implicit hardware commitments.
//!
//! # 9. Determinism requirements
//!
//! Reproducibility contracts must distinguish:
//!
//! - Semantic reproducibility.
//! - Numerical reproducibility.
//! - Bitwise reproducibility.
//! - Deterministic compilation.
//! - Deterministic artifact serialization.
//! - Deterministic runtime execution.
//! - Reproducible provenance.
//!
//! These guarantees are related but not interchangeable.
//!
//! A deterministic compiler does not automatically guarantee
//! deterministic execution.
//!
//! Deterministic serialization does not prove semantic correctness.
//!
//! A stable fingerprint does not prove that all relevant inputs
//! were included.
//!
//! Every fingerprint schema must have an explicit stable domain
//! and versioned canonical encoding.
//!
//! # 10. Scalability and POCO-REAF
//!
//! This subsystem must not introduce arbitrary universal ceilings.
//!
//! No fixed maximum is established for:
//!
//! - Source units.
//! - Source bytes.
//! - Compilation inputs.
//! - Dependencies.
//! - Compiler components.
//! - Computing domains.
//! - Toolchains.
//! - Targets.
//! - Artifacts.
//! - Provenance events.
//! - Determinism observations.
//! - Evidence references.
//! - Hardware resources.
//! - Distributed participants.
//! - Quantum resources.
//! - Tensor dimensions or rank.
//! - Compiler stages.
//! - Optimization passes.
//!
//! Collections must grow according to supplied inputs and available
//! resources.
//!
//! Operational budgets belong to compiler configuration and
//! compilation-context authorities.
//!
//! Physical resource limitations must be reported explicitly.
//!
//! Representation overflow must be reported explicitly.
//!
//! No collection may be silently truncated.
//!
//! No missing capability may be interpreted as unlimited capability.
//!
//! Literal infinite physical resources are not promised.
//!
//! # 11. Concurrency
//!
//! This composition module owns no mutable global state.
//!
//! It creates no threads.
//!
//! It creates no workers.
//!
//! It creates no locks.
//!
//! Child modules retain their documented ownership and concurrency
//! contracts.
//!
//! Parallel execution must not alter canonical fingerprint ordering
//! or the semantic meaning of compilation.
//!
//! # 12. Error handling
//!
//! Errors remain owned by their respective modules.
//!
//! EnvironmentError:
//!     Environment identity construction and validation.
//!
//! CompilationInputsError:
//!     Compilation-input identity construction and validation.
//!
//! DeterminismAssessmentError:
//!     Determinism assessment and coverage validation.
//!
//! ProvenanceFingerprintError:
//!     Provenance snapshot fingerprinting.
//!
//! FingerprintError:
//!     Canonical fingerprint construction.
//!
//! This facade must not flatten these errors into strings or replace
//! them with an unrelated generic error type.
//!
//! # 13. API versioning
//!
//! This composition module has an API version independent of:
//!
//! - Zamani language version.
//! - Grammar version.
//! - Semantic-model version.
//! - Compiler release.
//! - ZUIR schema version.
//! - Domain IR versions.
//! - Artifact format version.
//! - Backend ABI version.
//! - Runtime version.
//!
//! Increment the composition API version when this facade changes
//! incompatibly.
//!
//! Child modules retain their own independently versioned contracts.
//!
//! # 14. Completion invariants
//!
//! This module is correct only when:
//!
//! - Every declared child module exists.
//! - Every public module path resolves.
//! - Child modules retain their ownership boundaries.
//! - No duplicate fingerprint implementation is introduced.
//! - No duplicate provenance ledger is introduced.
//! - No duplicate semantic model is introduced.
//! - No duplicate canonical IR is introduced.
//! - No arbitrary hardware ceilings are introduced.
//! - No unsafe Rust is permitted.
//! - No cache decision bypasses mandatory verification.
//! - Existing child-module imports remain valid.
//! - Public composition tests pass.
//!
//! The facade must be updated deliberately whenever a child module
//! is added, removed or renamed.
//!
//! ============================================================================
// MODULE COMPOSITION
//! ============================================================================

/// Explicit compilation environment identity and fingerprinting.
pub mod environment;

/// Complete compilation-input identity and fingerprinting.
pub mod inputs;

/// Compiler-wide determinism declarations and assessment.
pub mod determinism;

/// Fingerprinting of authoritative compilation provenance.
pub mod provenance;

// ============================================================================
// COMPOSITION API VERSION
// ============================================================================

/// Public API version of the compiler reproducibility composition.
///
/// This version is independent of the API versions of individual
/// reproducibility child modules.
pub const REPRODUCIBILITY_API_VERSION: u16 = 1;

/// Returns the reproducibility composition API version.
#[must_use]
pub const fn api_version() -> u16 {
    REPRODUCIBILITY_API_VERSION
}

// ============================================================================
// FACADE TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_api_version_is_stable() {
        assert_eq!(
            api_version(),
            REPRODUCIBILITY_API_VERSION,
        );
    }

    #[test]
    fn all_existing_reproducibility_modules_are_exposed() {
        // These type references verify that the existing child
        // implementations are accessible through the composition
        // module without introducing duplicate authorities.

        fn environment_type(
            _: &environment::CompilationEnvironment,
        ) {}

        fn inputs_type(
            _: &inputs::CompilationInputs,
        ) {}

        fn determinism_type(
            _: &determinism::DeterminismAssessment,
        ) {}

        fn provenance_type(
            _: &provenance::ProvenanceFingerprint,
        ) {}

        let _ = (
            environment_type,
            inputs_type,
            determinism_type,
            provenance_type,
        );
    }
}
