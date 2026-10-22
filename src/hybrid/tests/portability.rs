
#![forbid(unsafe_code)]

//! Hybrid portability contract tests for Zamani.
//!
//! Path: `src/hybrid/tests/portability.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! # Purpose
//!
//! Verify the portability properties that can currently be exercised through
//! the repository's existing hybrid compilation API.
//!
//! These tests deliberately reuse `crate::compiler::hybrid_pipeline`.
//! They do not create a second hybrid graph, semantic model, resource model,
//! capability registry, compiler pipeline, quantum IR, or runtime.
//!
//! # Current integration boundary
//!
//! `HybridPipelineOrchestrator` currently produces a deterministic hybrid
//! artifact manifest. It does not itself prove that a particular target can
//! execute a program or that target-specific lowering preserves its meaning.
//!
//! Consequently, these tests verify manifest-level portability contracts,
//! not end-to-end execution equivalence or target feasibility.
//!
//! Full capability-negotiation and feasibility tests must be added when those
//! contracts exist in their canonical owning subsystem.
//!
//! # Scalability
//!
//! These tests do not define universal hardware-size limits. Test inputs are
//! finite workloads, not architectural limits on program size, domain count,
//! hardware capacity, or resource requirements.

use crate::compiler::hybrid_pipeline::{
    ClassicalTarget,
    HybridCompilationProfile,
    HybridPipelineError,
    HybridPipelineOrchestrator,
    QuantumTarget,
    SynchronizationMode,
};

/// Create an orchestrator using the existing public compiler API.
///
/// This helper intentionally does not invent target capability semantics.
/// The target strings identify compilation-profile metadata only.
fn orchestrator(
    profile_name: &str,
    classical_target: &str,
    quantum_target: &str,
    description: &str,
    synchronization: SynchronizationMode,
) -> HybridPipelineOrchestrator {
    let profile = HybridCompilationProfile::with_synchronization(
        profile_name,
        classical_target,
        quantum_target,
        description,
        synchronization,
    )
    .expect("test profile should be valid");

    HybridPipelineOrchestrator::from_profile(profile)
        .expect("test profile should be accepted by the orchestrator")
}

/// Remove only target-specific manifest fields.
///
/// This is used to compare the target-independent portion of two manifests.
/// It does not claim that the remaining manifest is a complete semantic IR.
fn without_target_metadata(manifest: &str) -> String {
    manifest
        .lines()
        .filter(|line| {
            !line.starts_with("classical_target=")
                && !line.starts_with("quantum_target=")
                && !line.starts_with("target=")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn identical_inputs_produce_identical_artifacts() {
    let compiler = orchestrator(
        "portable",
        "zamani-portable",
        "qir",
        "portable hybrid program",
        SynchronizationMode::Barrier,
    );

    let first = compiler
        .synthesize("main")
        .expect("first synthesis should succeed");

    let second = compiler
        .synthesize("main")
        .expect("second synthesis should succeed");

    assert_eq!(
        first, second,
        "identical profile and module inputs must produce identical artifacts"
    );

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "artifact bytes must be deterministic for identical inputs"
    );
}

#[test]
fn changing_targets_does_not_change_the_declared_synchronization_semantics() {
    let profiles = [
        ("x86_64", "qir"),
        ("aarch64", "openqasm3"),
        ("riscv64", "ion-trap"),
        ("wasm32", "neutral-atom"),
    ];

    for (classical_target, quantum_target) in profiles {
        let compiler = orchestrator(
            "same-program",
            classical_target,
            quantum_target,
            "same portable program",
            SynchronizationMode::Barrier,
        );

        let artifact = compiler
            .synthesize("main")
            .expect("target profile should synthesize");

        assert_eq!(
            artifact.synchronization,
            SynchronizationMode::Barrier,
            "changing target metadata must not silently change the requested synchronization mode"
        );

        assert!(
            artifact
                .manifest
                .contains("synchronization=barrier"),
            "the artifact must retain the declared synchronization contract"
        );

        assert!(
            artifact
                .manifest
                .contains("native_instructions_embedded=false"),
            "the hybrid manifest must not pretend to contain native instructions"
        );

        assert!(
            artifact
                .manifest
                .contains("quantum_instructions_embedded=false"),
            "the hybrid manifest must not pretend to contain quantum instructions"
        );
    }
}

#[test]
fn target_changes_are_explicit_in_artifact_metadata() {
    let first = orchestrator(
        "portable",
        "x86_64",
        "qir",
        "same program",
        SynchronizationMode::Synchronous,
    )
    .synthesize("main")
    .expect("first target should synthesize");

    let second = orchestrator(
        "portable",
        "aarch64",
        "openqasm3",
        "same program",
        SynchronizationMode::Synchronous,
    )
    .synthesize("main")
    .expect("second target should synthesize");

    assert_ne!(
        first.classical_target,
        second.classical_target,
        "the artifacts must identify their different classical targets"
    );

    assert_ne!(
        first.quantum_target,
        second.quantum_target,
        "the artifacts must identify their different quantum targets"
    );

    assert_ne!(
        first.manifest,
        second.manifest,
        "target-specific artifact metadata must not be hidden"
    );

    assert_eq!(
        without_target_metadata(&first.manifest),
        without_target_metadata(&second.manifest),
        "for otherwise identical profiles, removing target metadata should leave the same target-independent manifest structure"
    );
}

#[test]
fn synchronous_asynchronous_and_barrier_modes_are_preserved() {
    let cases = [
        (SynchronizationMode::Synchronous, "synchronous"),
        (SynchronizationMode::Asynchronous, "asynchronous"),
        (SynchronizationMode::Barrier, "barrier"),
    ];

    for (mode, expected) in cases {
        let compiler = orchestrator(
            "sync-contract",
            "zamani-portable",
            "custom-quantum-interface",
            "synchronization regression test",
            mode,
        );

        let artifact = compiler
            .synthesize("main")
            .expect("valid synchronization mode should synthesize");

        assert_eq!(artifact.synchronization, mode);

        assert!(
            artifact
                .manifest
                .contains(&format!("synchronization={expected}")),
            "the manifest must preserve the requested synchronization mode"
        );
    }
}

#[test]
fn custom_target_identifiers_use_the_existing_extensible_target_api() {
    let custom_classical = "future-classical-target";
    let custom_quantum = "future-quantum-interface";

    let compiler = orchestrator(
        "future-targets",
        custom_classical,
        custom_quantum,
        "future target compatibility",
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("main")
        .expect("non-empty custom target identifiers should be accepted");

    assert_eq!(artifact.classical_target, custom_classical);
    assert_eq!(artifact.quantum_target, custom_quantum);

    assert!(
        artifact
            .manifest
            .contains(&format!("classical_target={custom_classical}"))
    );

    assert!(
        artifact
            .manifest
            .contains(&format!("quantum_target={custom_quantum}"))
    );
}

#[test]
fn target_family_identifiers_have_stable_public_names() {
    assert_eq!(ClassicalTarget::X86_64.as_str(), "x86_64");
    assert_eq!(ClassicalTarget::Aarch64.as_str(), "aarch64");
    assert_eq!(ClassicalTarget::RiscV64.as_str(), "riscv64");
    assert_eq!(ClassicalTarget::Wasm32.as_str(), "wasm32");
    assert_eq!(ClassicalTarget::Wasm64.as_str(), "wasm64");
    assert_eq!(
        ClassicalTarget::ZamaniPortable.as_str(),
        "zamani-portable"
    );

    assert_eq!(QuantumTarget::OpenQasm3.as_str(), "openqasm3");
    assert_eq!(QuantumTarget::Qir.as_str(), "qir");
    assert_eq!(QuantumTarget::IonTrap.as_str(), "ion-trap");
    assert_eq!(
        QuantumTarget::Superconducting.as_str(),
        "superconducting"
    );
    assert_eq!(QuantumTarget::NeutralAtom.as_str(), "neutral-atom");
    assert_eq!(QuantumTarget::Photonic.as_str(), "photonic");
}

#[test]
fn invalid_profile_is_reported_instead_of_becoming_a_valid_artifact() {
    let result = HybridCompilationProfile::new(
        "",
        "x86_64",
        "qir",
        "invalid profile",
    );

    assert!(
        result.is_err(),
        "an invalid profile must be rejected by the canonical compiler API"
    );
}

#[test]
fn invalid_module_name_is_reported_as_a_structured_error() {
    let compiler = orchestrator(
        "valid-profile",
        "x86_64",
        "qir",
        "valid profile",
        SynchronizationMode::Synchronous,
    );

    let result = compiler.synthesize("");

    assert_eq!(
        result,
        Err(HybridPipelineError::EmptyModuleName),
        "invalid module names must produce the established structured error"
    );
}

#[test]
fn compilation_bytes_match_the_canonical_manifest() {
    let compiler = orchestrator(
        "byte-stability",
        "zamani-portable",
        "qir",
        "manifest byte regression",
        SynchronizationMode::Asynchronous,
    );

    let artifact = compiler
        .synthesize("main")
        .expect("synthesis should succeed");

    let bytes = compiler
        .compile("main")
        .expect("compilation should produce artifact bytes");

    assert_eq!(
        bytes,
        artifact.as_bytes(),
        "compile() must return the bytes of the canonical hybrid artifact"
    );

    assert_eq!(
        std::str::from_utf8(&bytes)
            .expect("hybrid artifact must be UTF-8"),
        artifact.manifest
    );

    assert_eq!(artifact.format(), "zamani-hybrid-v1");
}

#[test]
fn hybrid_grammar_and_normative_specification_are_present() {
    // Paths are relative to src/hybrid/tests/portability.rs.
    const SPEC: &str = include_str!("../../../grammar/spec/hybrid.md");
    const GRAMMAR_README: &str =
        include_str!("../../../grammar/hybrid/README.md");
    const HYBRID_GRAMMAR: &str =
        include_str!("../../../grammar/hybrid/hybrid.g4");
    const CLASSICAL_QUANTUM_GRAMMAR: &str =
        include_str!("../../../grammar/hybrid/classical-quantum.g4");
    const QUANTUM_CLASSICAL_CONTROL_GRAMMAR: &str =
        include_str!("../../../grammar/hybrid/quantum-classical-control.g4");
    const ACCELERATOR_GRAMMAR: &str =
        include_str!("../../../grammar/hybrid/accelerator-interoperability.g4");
    const RESOURCE_GRAMMAR: &str =
        include_str!("../../../grammar/hybrid/hybrid-resources.g4");

    assert!(
        SPEC.contains("POCO-REAF")
            || SPEC.contains("Program_Once_Compile_Once_Run_Everywhere_Anywhere_Forever"),
        "the normative hybrid specification must state the portability objective"
    );

    assert!(
        GRAMMAR_README.contains("not a second quantum IR")
            || GRAMMAR_README.contains("not a second quantum language"),
        "hybrid grammar documentation must preserve canonical domain ownership"
    );

    assert!(
        HYBRID_GRAMMAR.contains("tokenVocab = ZamaniLexer")
            || HYBRID_GRAMMAR.contains("tokenVocab=ZamaniLexer"),
        "the hybrid grammar must reuse the canonical lexer vocabulary"
    );

    assert!(
        !CLASSICAL_QUANTUM_GRAMMAR.is_empty(),
        "the classical/quantum composition grammar must be available"
    );

    assert!(
        !QUANTUM_CLASSICAL_CONTROL_GRAMMAR.is_empty(),
        "the quantum/classical control grammar must be available"
    );

    assert!(
        !ACCELERATOR_GRAMMAR.is_empty(),
        "the accelerator interoperability grammar must be available"
    );

    assert!(
        !RESOURCE_GRAMMAR.is_empty(),
        "the hybrid resource adapter grammar must be available"
    );
}

#[test]
fn normative_specification_distinguishes_portability_from_universal_feasibility() {
    const SPEC: &str = include_str!("../../../grammar/spec/hybrid.md");
    const GRAMMAR_README: &str =
        include_str!("../../../grammar/hybrid/README.md");

    assert!(
        SPEC.contains("available resources")
            && SPEC.contains("required capabilities"),
        "the normative specification must condition portability on actual resources and capabilities"
    );

    assert!(
        GRAMMAR_README.contains("does not promise that every target can execute every program")
            || GRAMMAR_README.contains("does not mean that physical machines have infinite resources"),
        "POCO-REAF must not be interpreted as a promise of impossible execution"
    );
}
