
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Compatibility conformance tests for Zamani hybrid computing.
//!
//! Path: src/hybrid/tests/compatibility.rs
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//!
//! # Purpose
//!
//! Protect existing public hybrid compilation interfaces while the hybrid
//! subsystem is integrated with the canonical frontend, semantic model,
//! quantum IR, classical facilities, compiler, backend and runtime.
//!
//! These tests exercise the existing public compiler boundary. They do not
//! implement a second hybrid graph, semantic model, serializer, backend,
//! scheduler or runtime.
//!
//! # Canonical ownership
//!
//! - Hybrid-computing contract: grammar/spec/hybrid.md.
//! - Hybrid grammar: grammar/hybrid/hybrid.g4.
//! - Grammar composition: grammar/hybrid/README.md.
//! - Existing compiler compatibility boundary: compiler::hybrid_pipeline.
//! - Existing compilation profiling: compiler::hybrid_profiles.
//! - Canonical quantum semantics: quantum::ir.
//! - Target-specific code generation: existing compiler/backend modules.
//! - Execution lifecycle: existing runtime facilities.
//!
//! # Scope
//!
//! These tests verify:
//!
//! - Existing public constructors and accessors remain usable.
//! - Legacy synchronous defaults remain stable.
//! - Explicit synchronization modes survive compilation.
//! - Valid custom target identifiers remain supported.
//! - Artifact metadata remains internally consistent.
//! - Artifacts do not falsely claim to contain native instructions.
//! - Repeated compilation and compatibility APIs agree.
//! - Invalid inputs produce explicit, stable failures.
//! - Metadata escaping prevents line breaks from changing manifest structure.
//! - The tests do not depend on a particular machine's hardware inventory.
//!
//! # Scalability
//!
//! The finite inputs below are test workloads, not universal machine-size
//! limits. These tests introduce no maximum domain count, graph size,
//! device count, qubit count, register width, tensor rank or memory capacity.
//!
//! They do not assert that physical resources are infinite. Concrete
//! feasibility remains the responsibility of capability-aware planning,
//! compilation and execution layers.
//!
//! # Important limitation
//!
//! Passing these tests proves compatibility at the existing hybrid artifact
//! boundary only. It does not prove end-to-end execution, backend correctness,
//! semantic equivalence across physical targets, or removal of the existing
//! compiler pipeline's metadata-length restrictions.

use crate::compiler::hybrid_pipeline::{
    ClassicalTarget,
    HybridArtifact,
    HybridCompilationProfile,
    HybridPipelineError,
    HybridPipelineOrchestrator,
    QuantumTarget,
    SynchronizationMode,
};

/// Construct an orchestrator using the established public profile API.
fn make_orchestrator(
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
    .expect("the compatibility fixture must be a valid profile");

    HybridPipelineOrchestrator::from_profile(profile)
        .expect("the validated profile must remain accepted")
}

/// Assert the stable artifact properties shared by the existing public APIs.
fn assert_artifact_contract(
    artifact: &HybridArtifact,
    module_name: &str,
    profile_name: &str,
    classical_target: &str,
    quantum_target: &str,
    synchronization: SynchronizationMode,
) {
    assert_eq!(artifact.module_name, module_name);
    assert_eq!(artifact.profile_name, profile_name);
    assert_eq!(artifact.classical_target, classical_target);
    assert_eq!(artifact.quantum_target, quantum_target);
    assert_eq!(artifact.synchronization, synchronization);

    assert_eq!(
        artifact.format(),
        "zamani-hybrid-v1",
        "the existing artifact format identifier is compatibility-sensitive"
    );

    assert!(
        artifact.manifest.starts_with("ZAMANI-HYBRID-ARTIFACT\n"),
        "the manifest must retain its established header"
    );

    assert!(
        artifact.manifest.contains("format=zamani-hybrid-v1\n"),
        "the manifest format must agree with the public artifact format"
    );

    assert!(
        artifact.manifest.contains("native_instructions_embedded=false\n"),
        "a hybrid manifest must not claim to contain native instructions"
    );

    assert!(
        artifact.manifest.contains("quantum_instructions_embedded=false\n"),
        "a hybrid manifest must not claim to contain emitted quantum instructions"
    );

    assert_eq!(
        artifact.as_bytes(),
        artifact.manifest.as_bytes(),
        "the byte representation must remain the manifest representation"
    );
}

#[test]
fn legacy_constructor_preserves_synchronous_default() {
    let compiler = HybridPipelineOrchestrator::new(
        "compatibility-profile",
        "zamani-portable",
        "qir",
        "legacy constructor compatibility",
    );

    assert_eq!(
        compiler.profile.synchronization,
        SynchronizationMode::Synchronous,
        "the legacy constructor must retain its synchronous default"
    );

    let artifact = compiler
        .synthesize("main")
        .expect("a valid legacy profile must synthesize");

    assert_artifact_contract(
        &artifact,
        "main",
        "compatibility-profile",
        "zamani-portable",
        "qir",
        SynchronizationMode::Synchronous,
    );
}

#[test]
fn fallible_constructor_remains_available() {
    let compiler = HybridPipelineOrchestrator::try_new(
        "portable-profile",
        "zamani-portable",
        "qir",
        "fallible constructor compatibility",
    )
    .expect("a valid profile must be accepted");

    assert!(compiler.is_valid());

    let artifact = compiler
        .synthesize("application")
        .expect("a valid profile must produce an artifact");

    assert_eq!(artifact.module_name, "application");
}

#[test]
fn profile_constructor_and_orchestrator_preserve_profile_fields() {
    let profile = HybridCompilationProfile::new(
        "profile-a",
        "custom-classical-target",
        "custom-quantum-target",
        "profile compatibility",
    )
    .expect("a valid profile must be accepted");

    assert_eq!(profile.profile_name, "profile-a");
    assert_eq!(profile.classical_target, "custom-classical-target");
    assert_eq!(profile.quantum_target, "custom-quantum-target");
    assert_eq!(profile.description, "profile compatibility");
    assert_eq!(profile.synchronization, SynchronizationMode::Synchronous);

    let compiler = HybridPipelineOrchestrator::from_profile(profile.clone())
        .expect("the existing profile must remain usable");

    assert_eq!(compiler.profile, profile);
    assert!(compiler.validate().is_ok());
}

#[test]
fn target_family_identifiers_remain_stable() {
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
fn custom_target_identifiers_remain_extensible() {
    let classical = ClassicalTarget::Custom(
        "future-classical-architecture".to_owned(),
    );

    let quantum = QuantumTarget::Custom(
        "future-quantum-architecture".to_owned(),
    );

    assert_eq!(
        classical.as_str(),
        "future-classical-architecture"
    );

    assert_eq!(
        quantum.as_str(),
        "future-quantum-architecture"
    );

    let compiler = make_orchestrator(
        "future-target-profile",
        classical.as_str(),
        quantum.as_str(),
        "custom target compatibility",
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("future_module")
        .expect("valid custom target identifiers must be accepted");

    assert_eq!(
        artifact.classical_target,
        "future-classical-architecture"
    );

    assert_eq!(
        artifact.quantum_target,
        "future-quantum-architecture"
    );
}

#[test]
fn every_existing_synchronization_mode_survives_synthesis() {
    let cases = [
        (
            SynchronizationMode::Synchronous,
            "synchronous",
        ),
        (
            SynchronizationMode::Asynchronous,
            "asynchronous",
        ),
        (
            SynchronizationMode::Barrier,
            "barrier",
        ),
    ];

    for (mode, expected_text) in cases {
        let compiler = make_orchestrator(
            "sync-profile",
            "zamani-portable",
            "qir",
            "synchronization compatibility",
            mode,
        );

        let artifact = compiler
            .synthesize("synchronization_module")
            .expect("a valid synchronization mode must synthesize");

        assert_eq!(artifact.synchronization, mode);

        assert!(
            artifact
                .manifest
                .contains(&format!("synchronization={expected_text}\n")),
            "the manifest must preserve synchronization mode {expected_text}"
        );

        assert!(
            artifact
                .manifest
                .contains(&format!(
                    "[QUANTUM_CLASSICAL_BOUNDARY]\nsynchronization={expected_text}\n"
                )),
            "the boundary section must preserve synchronization mode {expected_text}"
        );
    }
}

#[test]
fn synthesize_and_compile_return_the_same_artifact_bytes() {
    let compiler = make_orchestrator(
        "artifact-profile",
        "zamani-portable",
        "openqasm3",
        "artifact byte compatibility",
        SynchronizationMode::Barrier,
    );

    let artifact = compiler
        .synthesize("hybrid_application")
        .expect("synthesis must succeed");

    let compiled = compiler
        .compile("hybrid_application")
        .expect("compilation must succeed");

    assert_eq!(compiled.as_slice(), artifact.as_bytes());
}

#[test]
fn legacy_string_api_returns_the_same_manifest_for_valid_input() {
    let compiler = make_orchestrator(
        "legacy-profile",
        "zamani-portable",
        "qir",
        "legacy string compatibility",
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("legacy_module")
        .expect("synthesis must succeed");

    let legacy_result = compiler.synthesize_hybrid_binary("legacy_module");

    assert_eq!(
        legacy_result, artifact.manifest,
        "the compatibility API must return the canonical manifest on success"
    );

    assert!(
        !legacy_result.starts_with("HYBRID_COMPILATION_ERROR:"),
        "valid input must not be reported as a compatibility failure"
    );
}

#[test]
fn legacy_string_api_reports_invalid_input_explicitly() {
    let compiler = HybridPipelineOrchestrator::new(
        "legacy-profile",
        "zamani-portable",
        "qir",
        "legacy error compatibility",
    );

    let result = compiler.synthesize_hybrid_binary("");

    assert!(
        result.starts_with("HYBRID_COMPILATION_ERROR:"),
        "the compatibility API must not silently emit a valid-looking artifact for invalid input"
    );
}

#[test]
fn empty_module_name_has_a_stable_structured_error() {
    let compiler = HybridPipelineOrchestrator::try_new(
        "valid-profile",
        "zamani-portable",
        "qir",
        "empty module compatibility",
    )
    .expect("the profile must be valid");

    let first = compiler
        .synthesize("")
        .expect_err("an empty module name must be rejected");

    let second = compiler
        .synthesize("")
        .expect_err("the same invalid input must be rejected");

    assert_eq!(first, HybridPipelineError::EmptyModuleName);
    assert_eq!(first, second);
    assert_eq!(first.to_string(), second.to_string());
}

#[test]
fn invalid_profiles_are_rejected_without_panics() {
    let cases = [
        ("", "zamani-portable", "qir", "empty profile"),
        ("profile", "", "qir", "empty classical target"),
        ("profile", "zamani-portable", "", "empty quantum target"),
    ];

    for (name, classical, quantum, description) in cases {
        let result = HybridPipelineOrchestrator::try_new(
            name,
            classical,
            quantum,
            description,
        );

        assert!(
            result.is_err(),
            "invalid profile case {description:?} must be rejected"
        );
    }
}

#[test]
fn invalid_legacy_constructor_state_is_detected_by_validation() {
    // The historical constructor is intentionally infallible. Preserve that
    // signature, but ensure invalid state is rejected at the validation and
    // synthesis boundary rather than becoming a successful artifact.
    let compiler = HybridPipelineOrchestrator::new(
        "",
        "zamani-portable",
        "qir",
        "invalid legacy profile",
    );

    assert!(!compiler.is_valid());

    assert!(
        compiler.validate().is_err(),
        "invalid legacy state must fail explicit validation"
    );

    assert!(
        compiler.synthesize("main").is_err(),
        "invalid legacy state must not synthesize successfully"
    );
}

#[test]
fn manifest_metadata_escaping_preserves_record_boundaries() {
    let description = "first line\nsecond line\rthird line\\fourth line";

    let compiler = make_orchestrator(
        "escaping-profile",
        "zamani-portable",
        "qir",
        description,
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("escaping_module")
        .expect("valid description metadata must synthesize");

    assert!(
        artifact
            .manifest
            .contains("description=first line\\nsecond line\\rthird line\\\\fourth line\n"),
        "metadata line breaks and backslashes must be escaped"
    );

    assert!(
        !artifact
            .manifest
            .contains("description=first line\nsecond line"),
        "raw metadata line breaks must not create additional manifest records"
    );
}

#[test]
fn manifest_metadata_preserves_unicode() {
    let description = "hybrid computation: 量子・classical・Δ・🚀";

    let compiler = make_orchestrator(
        "unicode-profile",
        "zamani-portable",
        "qir",
        description,
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("unicode_module")
        .expect("valid Unicode metadata must synthesize");

    assert!(
        artifact.manifest.contains(description),
        "valid Unicode metadata must be preserved"
    );

    assert_eq!(
        artifact.as_bytes(),
        artifact.manifest.as_bytes()
    );
}

#[test]
fn repeated_synthesis_preserves_the_complete_artifact() {
    let compiler = make_orchestrator(
        "repeat-profile",
        "zamani-portable",
        "openqasm3",
        "repeatability compatibility",
        SynchronizationMode::Asynchronous,
    );

    let first = compiler
        .synthesize("repeat_module")
        .expect("first synthesis must succeed");

    let second = compiler
        .synthesize("repeat_module")
        .expect("second synthesis must succeed");

    assert_eq!(first, second);
    assert_eq!(first.as_bytes(), second.as_bytes());
}

#[test]
fn artifact_metadata_is_consistent_across_sections() {
    let compiler = make_orchestrator(
        "consistent-profile",
        "custom-cpu-family",
        "custom-qpu-family",
        "manifest consistency",
        SynchronizationMode::Barrier,
    );

    let artifact = compiler
        .synthesize("consistency_module")
        .expect("synthesis must succeed");

    assert_artifact_contract(
        &artifact,
        "consistency_module",
        "consistent-profile",
        "custom-cpu-family",
        "custom-qpu-family",
        SynchronizationMode::Barrier,
    );

    assert!(
        artifact
            .manifest
            .contains("classical_target=custom-cpu-family\n")
    );

    assert!(
        artifact
            .manifest
            .contains("quantum_target=custom-qpu-family\n")
    );

    assert!(
        artifact
            .manifest
            .contains("[CLASSICAL_CONTROL]\nrole=control\ntarget=custom-cpu-family\n")
    );

    assert!(
        artifact
            .manifest
            .contains("[QUANTUM_KERNEL]\nrole=accelerator\ntarget=custom-qpu-family\n")
    );
}

#[test]
fn compatibility_does_not_depend_on_prior_request_order() {
    let compiler = make_orchestrator(
        "order-profile",
        "zamani-portable",
        "qir",
        "request order compatibility",
        SynchronizationMode::Synchronous,
    );

    let first_a = compiler
        .synthesize("module_a")
        .expect("module A must synthesize");

    let first_b = compiler
        .synthesize("module_b")
        .expect("module B must synthesize");

    let second_b = compiler
        .synthesize("module_b")
        .expect("module B must synthesize after the repeated request");

    let second_a = compiler
        .synthesize("module_a")
        .expect("module A must synthesize after the repeated request");

    assert_eq!(first_a, second_a);
    assert_eq!(first_b, second_b);
}

#[test]
fn portable_profile_does_not_claim_physical_target_realization() {
    let compiler = make_orchestrator(
        "portable-profile",
        "zamani-portable",
        "qir",
        "portable intent only",
        SynchronizationMode::Synchronous,
    );

    let artifact = compiler
        .synthesize("portable_module")
        .expect("portable artifact synthesis must succeed");

    assert!(
        artifact
            .manifest
            .contains("native_instructions_embedded=false\n")
    );

    assert!(
        artifact
            .manifest
            .contains("quantum_instructions_embedded=false\n")
    );

    // Manifest generation alone must not be mistaken for proof that a
    // backend exists, target capabilities were discovered, or execution
    // succeeded.
    assert_eq!(artifact.format(), "zamani-hybrid-v1");
}

#[test]
fn configurable_test_workloads_do_not_encode_machine_capacity_limits() {
    // Exercise a range of finite input sizes without declaring any of those
    // sizes to be a universal machine or language limit.
    //
    // This is a regression test for deterministic handling of varying module
    // names, not a claim that the existing pipeline accepts arbitrarily long
    // metadata. The pipeline's current identifier and description ceilings
    // remain a separate implementation issue.
    let compiler = make_orchestrator(
        "scaling-profile",
        "zamani-portable",
        "qir",
        "configurable workload",
        SynchronizationMode::Synchronous,
    );

    for workload_size in [0_usize, 1, 8, 32, 128, 512] {
        let module_name = format!("module_{workload_size}");

        let artifact = compiler
            .synthesize(&module_name)
            .expect("the generated finite test module name must be valid");

        assert_eq!(artifact.module_name, module_name);

        assert!(
            artifact
                .manifest
                .contains(&format!("module={module_name}\n"))
        );
    }
}
