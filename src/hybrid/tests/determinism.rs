
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

//! Determinism conformance tests for Zamani hybrid computation.
//!
//! Path: `src/hybrid/tests/determinism.rs`
//! Edition: Rust 2021
//! Minimum supported Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Verify reproducibility guarantees at the currently available hybrid
//! compiler-pipeline boundary:
//!
//! - Identical profiles and module inputs produce identical artifacts.
//! - Repeated compilation produces identical bytes.
//! - Compilation results do not depend on the order in which independent
//!   compilation requests are made.
//! - Declared synchronization semantics survive repeated compilation and
//!   changes to target metadata.
//! - Validation failures and compatibility-API failure representations
//!   are stable for identical inputs.
//! - Escaping of manifest metadata is deterministic.
//!
//! # Canonical ownership
//!
//! - Normative determinism semantics:
//!   `grammar/spec/determinism.md`.
//! - Hybrid-computation contract:
//!   `grammar/spec/hybrid.md`.
//! - Hybrid syntax:
//!   `grammar/hybrid/hybrid.g4`.
//! - Existing compilation-profile and artifact implementation:
//!   `compiler::hybrid_pipeline`.
//! - Canonical quantum semantics:
//!   `quantum::ir`.
//! - Physical target lowering and execution:
//!   existing compiler, backend, and runtime subsystems.
//!
//! These tests deliberately do not define a second hybrid IR, graph,
//! semantic model, serializer, backend registry, or runtime.
//!
//! # Scope and limitations
//!
//! `HybridPipelineOrchestrator` currently produces a hybrid artifact
//! manifest. Reproducible manifests do not prove that native code or quantum
//! instructions were generated, that target capability requirements are
//! satisfied, or that execution on different physical targets is equivalent.
//!
//! Interchange-document determinism, canonical hybrid-graph fingerprints,
//! semantic-preservation proofs, and end-to-end execution determinism must
//! be tested against their respective canonical implementations when those
//! implementations are integrated and available through the crate.
//!
//! # Scalability
//!
//! The test suite imposes no universal machine-size or computation-size
//! limits. Its finite fixtures are test workloads, not language restrictions.
//! No test depends on elapsed time, thread scheduling, external hardware,
//! network services, or process-global randomness.

use crate::compiler::hybrid_pipeline::{
    ClassicalTarget,
    HybridArtifact,
    HybridCompilationProfile,
    HybridPipelineError,
    HybridPipelineOrchestrator,
    QuantumTarget,
    SynchronizationMode,
};

/// Construct an orchestrator through the existing public compiler API.
///
/// This helper intentionally reuses the pipeline's canonical profile
/// validation rather than defining an independent profile representation.
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
    .expect("the test fixture must satisfy existing profile validation");

    HybridPipelineOrchestrator::from_profile(profile)
        .expect("the validated test profile must be accepted")
}

/// Compile an identical request twice and compare both the complete
/// artifact and its externally observable byte representation.
fn assert_repeated_compilation_is_identical(
    compiler: &HybridPipelineOrchestrator,
    module_name: &str,
) -> HybridArtifact {
    let first = compiler
        .synthesize(module_name)
        .expect("the first synthesis should succeed");

    let second = compiler
        .synthesize(module_name)
        .expect("the repeated synthesis should succeed");

    assert_eq!(
        first, second,
        "identical profile and module inputs must produce identical artifacts"
    );

    assert_eq!(
        first.as_bytes(),
        second.as_bytes(),
        "identical inputs must produce identical artifact bytes"
    );

    let compiled_bytes = compiler
        .compile(module_name)
        .expect("compilation of a valid module should succeed");

    assert_eq!(
        compiled_bytes,
        first.as_bytes(),
        "compile() and synthesize() must expose the same artifact bytes"
    );

    first
}

#[test]
fn identical_inputs_produce_identical_artifacts_and_bytes() {
    let compiler = orchestrator(
        "portable",
        "zamani-portable",
        "qir",
        "portable hybrid computation",
        SynchronizationMode::Barrier,
    );

    let artifact = assert_repeated_compilation_is_identical(&compiler, "main");

    assert_eq!(artifact.module_name, "main");
    assert_eq!(artifact.profile_name, "portable");
    assert_eq!(
        artifact.synchronization,
        SynchronizationMode::Barrier
    );
    assert_eq!(artifact.format(), "zamani-hybrid-v1");
}

#[test]
fn independent_request_order_does_not_change_each_artifact() {
    let requests = [
        (
            "portable-a",
            "zamani-portable",
            "qir",
            SynchronizationMode::Synchronous,
            "module_a",
        ),
        (
            "portable-b",
            "aarch64",
            "openqasm3",
            SynchronizationMode::Asynchronous,
            "module_b",
        ),
        (
            "portable-c",
            "riscv64",
            "neutral-atom",
            SynchronizationMode::Barrier,
            "module_c",
        ),
    ];

    // Record artifacts in the original request order.
    let original_results: Vec<HybridArtifact> = requests
        .iter()
        .map(
            |(profile, classical, quantum, synchronization, module)| {
                let compiler = orchestrator(
                    profile,
                    classical,
                    quantum,
                    "independent deterministic request",
                    *synchronization,
                );

                compiler
                    .synthesize(module)
                    .expect("a valid request should synthesize")
            },
        )
        .collect();

    // Execute the same independent requests in reverse order. The resulting
    // artifacts are indexed by their original request, not by execution order.
    let mut reversed_results: Vec<(usize, HybridArtifact)> = requests
        .iter()
        .enumerate()
        .rev()
        .map(
            |(index, (profile, classical, quantum, synchronization, module))| {
                let compiler = orchestrator(
                    profile,
                    classical,
                    quantum,
                    "independent deterministic request",
                    *synchronization,
                );

                let artifact = compiler
                    .synthesize(module)
                    .expect("the same valid request should synthesize");

                (index, artifact)
            },
        )
        .collect();

    reversed_results.sort_by_key(|(index, _)| *index);

    for (index, artifact) in reversed_results {
        assert_eq!(
            original_results[index],
            artifact,
            "request order must not change the artifact for request {index}"
        );

        assert_eq!(
            original_results[index].as_bytes(),
            artifact.as_bytes(),
            "request order must not change serialized bytes for request {index}"
        );
    }
}

#[test]
fn repeated_compilation_preserves_each_declared_synchronization_mode() {
    let cases = [
        (
            "sync-profile",
            "zamani-portable",
            "qir",
            SynchronizationMode::Synchronous,
        ),
        (
            "async-profile",
            "aarch64",
            "openqasm3",
            SynchronizationMode::Asynchronous,
        ),
        (
            "barrier-profile",
            "riscv64",
            "neutral-atom",
            SynchronizationMode::Barrier,
        ),
    ];

    for (profile, classical, quantum, synchronization) in cases {
        let compiler = orchestrator(
            profile,
            classical,
            quantum,
            "synchronization contract fixture",
            synchronization,
        );

        let artifact =
            assert_repeated_compilation_is_identical(&compiler, "main");

        assert_eq!(
            artifact.synchronization,
            synchronization,
            "the declared synchronization mode must be preserved"
        );

        let expected = match synchronization {
            SynchronizationMode::Synchronous => {
                "synchronization=synchronous"
            }
            SynchronizationMode::Asynchronous => {
                "synchronization=asynchronous"
            }
            SynchronizationMode::Barrier => "synchronization=barrier",
        };

        assert!(
            artifact.manifest.contains(expected),
            "the artifact must retain its declared synchronization metadata"
        );
    }
}

#[test]
fn target_metadata_changes_do_not_change_the_declared_synchronization() {
    let synchronization = SynchronizationMode::Barrier;

    let first = orchestrator(
        "same-portable-program",
        "x86_64",
        "qir",
        "same source-level intent",
        synchronization,
    )
    .synthesize("main")
    .expect("the first target profile should synthesize");

    let second = orchestrator(
        "same-portable-program",
        "aarch64",
        "openqasm3",
        "same source-level intent",
        synchronization,
    )
    .synthesize("main")
    .expect("the second target profile should synthesize");

    // Target metadata is expected to differ. The synchronization contract
    // must not silently change with that metadata.
    assert_ne!(first.classical_target, second.classical_target);
    assert_ne!(first.quantum_target, second.quantum_target);

    assert_eq!(first.synchronization, synchronization);
    assert_eq!(second.synchronization, synchronization);

    assert!(
        first.manifest.contains("synchronization=barrier"),
        "the first manifest must preserve the barrier contract"
    );
    assert!(
        second.manifest.contains("synchronization=barrier"),
        "the second manifest must preserve the barrier contract"
    );

    // Repeat each target independently to distinguish intentional target
    // metadata changes from nondeterministic artifact generation.
    let first_repeat = orchestrator(
        "same-portable-program",
        "x86_64",
        "qir",
        "same source-level intent",
        synchronization,
    )
    .synthesize("main")
    .expect("repeated synthesis for the first target should succeed");

    let second_repeat = orchestrator(
        "same-portable-program",
        "aarch64",
        "openqasm3",
        "same source-level intent",
        synchronization,
    )
    .synthesize("main")
    .expect("repeated synthesis for the second target should succeed");

    assert_eq!(first, first_repeat);
    assert_eq!(second, second_repeat);
}

#[test]
fn validation_failures_are_stable_for_identical_inputs() {
    let first = HybridPipelineOrchestrator::try_new(
        "",
        "zamani-portable",
        "qir",
        "invalid profile fixture",
    )
    .expect_err("an empty profile name must be rejected");

    let second = HybridPipelineOrchestrator::try_new(
        "",
        "zamani-portable",
        "qir",
        "invalid profile fixture",
    )
    .expect_err("the identical invalid profile must also be rejected");

    assert_eq!(
        first, second,
        "identical invalid profile inputs must produce the same structured error"
    );

    assert_eq!(
        first.to_string(),
        second.to_string(),
        "identical validation failures must produce the same display text"
    );
}

#[test]
fn module_validation_failures_are_stable() {
    let compiler = orchestrator(
        "stable-errors",
        "zamani-portable",
        "qir",
        "stable error fixture",
        SynchronizationMode::Synchronous,
    );

    let first = compiler
        .synthesize("")
        .expect_err("an empty module name must be rejected");

    let second = compiler
        .synthesize("")
        .expect_err("the identical invalid module name must also be rejected");

    assert_eq!(
        first, second,
        "identical invalid requests must produce the same error value"
    );

    assert_eq!(
        first.to_string(),
        second.to_string(),
        "identical invalid requests must produce the same diagnostic text"
    );

    // The legacy compatibility API cannot return a Result. Its explicit
    // failure representation must nevertheless be reproducible.
    let legacy_first = compiler.synthesize_hybrid_binary("");
    let legacy_second = compiler.synthesize_hybrid_binary("");

    assert_eq!(
        legacy_first, legacy_second,
        "the compatibility API must represent identical failures identically"
    );

    assert!(
        legacy_first.starts_with("HYBRID_COMPILATION_ERROR:"),
        "the compatibility API must not disguise a failed compilation as success"
    );
}

#[test]
fn manifest_metadata_escaping_is_deterministic() {
    let description = "first line\nsecond line\rthird line\\fourth line";

    let compiler = orchestrator(
        "escaped-metadata",
        "zamani-portable",
        "qir",
        description,
        SynchronizationMode::Synchronous,
    );

    let artifact =
        assert_repeated_compilation_is_identical(&compiler, "escaped_module");

    assert!(
        artifact
            .manifest
            .contains("description=first line\\nsecond line\\rthird line\\\\fourth line"),
        "manifest metadata must use the compiler's deterministic escaping contract"
    );
}

#[test]
fn synthesis_does_not_mutate_the_configured_profile() {
    let compiler = orchestrator(
        "immutable-profile",
        "zamani-portable",
        "qir",
        "profile immutability fixture",
        SynchronizationMode::Asynchronous,
    );

    let original_profile = compiler.profile.clone();

    let _first = compiler
        .synthesize("first_module")
        .expect("the first module should synthesize");

    let _second = compiler
        .synthesize("second_module")
        .expect("the second module should synthesize");

    assert_eq!(
        compiler.profile, original_profile,
        "synthesizing one module must not mutate shared profile state"
    );
}
