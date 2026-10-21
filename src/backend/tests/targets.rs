//! Zamani backend target-dispatch and target-identity regression tests.
//!
//! Path: `src/backend/tests/targets.rs`
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: safe Rust only; no `unsafe`.
//!
//! # Purpose
//!
//! Verify that target selection preserves target identity, architecture,
//! execution model, ABI metadata, artifact-format declarations, and explicit
//! implementation-availability information.
//!
//! These tests cover two distinct boundaries:
//!
//! 1. The legacy `CodeGenerator` API in `src/backend.rs`.
//! 2. The target-description adapter in `src/backend/target.rs`.
//!
//! # Authority and integration
//!
//! - `src/compiler_types.rs` owns `CompilationTarget` and `CompilerConfig`.
//! - `src/backend.rs` owns the legacy `Backend` and `CodeGenerator` APIs.
//! - `src/backend/target.rs` owns `TargetDescriptor` and target metadata.
//! - `src/backend/selection.rs` owns capability-aware target selection.
//! - `src/backend/registry.rs` owns backend registration and discovery.
//! - `src/backend/validation.rs` owns backend precondition validation.
//! - `src/ir_gen.rs` owns the legacy `IrModule` representation.
//! - `grammar/Zamani.g4` owns the root grammar composition contract.
//! - `grammar/specification/semantic-model.md` owns source semantics.
//! - `grammar/validation/semantic-boundaries.md` defines the boundaries
//!   between semantic meaning, target realization, and physical execution.
//!
//! The canonical `backend` module must expose the legacy API and declare
//! `pub mod target;`. This file should be included from the backend test
//! module without defining another registry or another backend trait.
//!
//! # Important invariants
//!
//! - AArch64 must never be silently compiled by the x86-64 emitter.
//! - An unavailable target must fail explicitly rather than fall back.
//! - Target descriptors must not invent unspecified architecture properties.
//! - Architecture, operating system, target triple, ABI, and data layout
//!   remain independent pieces of metadata.
//! - A declared artifact format does not prove that an emitter produces a
//!   valid artifact.
//! - No universal hardware-capacity limits are introduced by these tests.
//!
//! # Completion rule
//!
//! These tests are intended to fail when the implementation violates a
//! contract. They must not be weakened merely to make an incorrect target
//! dispatcher pass.

#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

use crate::backend::target::{
    AbiDescriptor, ArtifactFormat, DataLayoutDescriptor, ExecutionModel,
    ImplementationAvailability, TargetDescriptor, TargetId, TargetTriple,
};
use crate::compiler_types::{CompilationTarget, CompilerConfig};
use crate::ir_gen::IrModule;

// ---------------------------------------------------------------------------
// Shared test helpers
// ---------------------------------------------------------------------------

/// Build a target descriptor from the repository's legacy target enum.
///
/// The descriptor adapter is responsible for translating the legacy target
/// without inventing unspecified target properties.
fn descriptor_for(
    target: CompilationTarget,
) -> TargetDescriptor {
    TargetDescriptor::from_compilation_target(&target)
        .expect("a supported legacy enum variant must produce valid metadata")
}

/// Assert that a descriptor's architecture matches the expected architecture.
fn assert_architecture(
    descriptor: &TargetDescriptor,
    expected: &str,
) {
    assert_eq!(
        descriptor.architecture(),
        Some(expected),
        "target `{}` must identify architecture `{expected}`",
        descriptor.id()
    );
}

/// Assert that the legacy dispatcher either selects the requested target
/// correctly or rejects it explicitly.
///
/// An error is accepted only when it clearly identifies an unsupported or
/// unavailable target. An error that conceals an unrelated failure must not
/// make this regression test pass.
fn assert_correct_dispatch_or_explicit_rejection(
    result: Result<crate::backend::CodeGenOutput, crate::backend::CodeGenError>,
    expected_target: &str,
) {
    match result {
        Ok(output) => {
            assert_eq!(
                output.target, expected_target,
                "dispatcher returned a different target; silent cross-architecture \
                 substitution is forbidden"
            );
        }
        Err(error) => {
            let message = error.message.to_ascii_lowercase();

            let explicitly_unsupported = [
                "unsupported",
                "unavailable",
                "not implemented",
                "not supported",
            ]
            .iter()
            .any(|marker| message.contains(marker));

            assert!(
                explicitly_unsupported,
                "target rejection must explicitly report unsupported or unavailable \
                 functionality; received target `{}` and message `{}`",
                error.target,
                error.message
            );

            assert!(
                !error.target.to_ascii_lowercase().contains("x86"),
                "an AArch64 request must not be reported as an x86 target"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Legacy dispatcher regression tests
// ---------------------------------------------------------------------------

#[test]
fn arm64_never_silently_dispatches_to_x86_64() {
    let config = CompilerConfig {
        target: CompilationTarget::Arm64,
        ..CompilerConfig::default()
    };

    let generator = crate::backend::CodeGenerator::new(config);
    let module = IrModule::new("arm64_dispatch_regression");

    let result = generator.generate(&module);

    // A valid implementation may emit an explicitly identified AArch64
    // artifact, or reject the target until an AArch64 implementation exists.
    //
    // Returning an x86-64 artifact is never an acceptable fallback.
    assert_correct_dispatch_or_explicit_rejection(result, "aarch64");
}

#[test]
fn x86_64_request_preserves_legacy_target_identity() {
    let config = CompilerConfig {
        target: CompilationTarget::X86_64Linux,
        ..CompilerConfig::default()
    };

    let generator = crate::backend::CodeGenerator::new(config);
    let module = IrModule::new("x86_64_dispatch_regression");

    let result = generator
        .generate(&module)
        .expect("the existing x86-64 emitter should handle an empty module");

    assert_eq!(result.target, "x86_64");
    assert_eq!(result.extension, ".s");
    assert_eq!(result.size_bytes, result.source.len());
}

// ---------------------------------------------------------------------------
// Legacy target-to-descriptor mapping
// ---------------------------------------------------------------------------

#[test]
fn x86_64_linux_descriptor_preserves_architecture_os_and_abi() {
    let descriptor = descriptor_for(CompilationTarget::X86_64Linux);

    assert_eq!(descriptor.id().as_str(), "x86-64-linux");
    assert_eq!(descriptor.execution_model(), &ExecutionModel::Native);
    assert_architecture(&descriptor, "x86_64");
    assert_eq!(descriptor.operating_system(), Some("linux"));

    let abi = descriptor
        .abi()
        .expect("the legacy descriptor explicitly declares its ABI");

    assert_eq!(abi.name(), Some("system-v"));

    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::LegacyEmitterPresent
    );

    assert!(
        descriptor.supports_artifact_format_name("assembly"),
        "the legacy x86-64 descriptor must declare its assembly format"
    );

    // A descriptor for a legacy emitter is not a production-readiness
    // certificate. Its emitted artifact still needs validation.
    assert!(descriptor.validate().is_ok());
}

#[test]
fn arm64_descriptor_identifies_aarch64_without_claiming_implementation() {
    let descriptor = descriptor_for(CompilationTarget::Arm64);

    assert_eq!(descriptor.id().as_str(), "aarch64");
    assert_eq!(descriptor.execution_model(), &ExecutionModel::Native);
    assert_architecture(&descriptor, "aarch64");

    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::Unavailable,
        "the descriptor must not advertise the misrouted legacy emitter \
         as a working AArch64 implementation"
    );

    // Do not invent a target triple, OS, ABI, or data layout when the
    // existing adapter has not established those properties.
    assert_eq!(descriptor.triple(), None);
    assert_eq!(descriptor.operating_system(), None);
    assert_eq!(descriptor.abi(), None);
    assert_eq!(descriptor.data_layout(), None);
}

#[test]
fn wasm32_descriptor_does_not_claim_a_native_cpu_abi() {
    let descriptor = descriptor_for(CompilationTarget::Wasm32);

    assert_eq!(descriptor.id().as_str(), "wasm32");
    assert_eq!(descriptor.execution_model(), &ExecutionModel::WebAssembly);
    assert_architecture(&descriptor, "wasm32");

    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::LegacyEmitterPresent
    );

    assert!(
        descriptor.supports_artifact_format_name("wat"),
        "the legacy WebAssembly descriptor must declare WAT output"
    );

    // A WebAssembly architecture label alone does not establish a host ABI.
    assert_eq!(descriptor.abi(), None);
    assert_eq!(descriptor.operating_system(), None);
}

#[test]
fn riscv_descriptor_preserves_its_own_architecture_identity() {
    let descriptor = descriptor_for(CompilationTarget::RiscV);

    assert_eq!(descriptor.id().as_str(), "riscv");
    assert_eq!(descriptor.execution_model(), &ExecutionModel::Native);
    assert_architecture(&descriptor, "riscv64");

    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::LegacyEmitterPresent
    );

    assert!(
        descriptor.supports_artifact_format_name("assembly"),
        "the RISC-V descriptor must declare its assembly format"
    );

    // The adapter does not currently provide a verified triple or ABI.
    // Keep those facts unknown instead of borrowing x86-64 metadata.
    assert_eq!(descriptor.triple(), None);
    assert_eq!(descriptor.abi(), None);
}

// ---------------------------------------------------------------------------
// Domain target identity
// ---------------------------------------------------------------------------

#[test]
fn qasm_descriptor_is_a_quantum_target_not_a_native_cpu_target() {
    let descriptor = descriptor_for(CompilationTarget::QASM);

    assert_eq!(descriptor.id().as_str(), "quantum-qasm");
    assert_eq!(descriptor.execution_model(), &ExecutionModel::Quantum);

    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::LegacyEmitterPresent
    );

    assert!(
        descriptor.supports_artifact_format_name("qasm"),
        "the QASM descriptor must declare its output format"
    );

    assert_eq!(descriptor.architecture(), None);
    assert_eq!(descriptor.abi(), None);
}

#[test]
fn bytecode_targets_keep_distinct_identities_and_formats() {
    let nano = descriptor_for(CompilationTarget::NanoControl);
    let mts = descriptor_for(CompilationTarget::MTSBytecode);

    assert_eq!(nano.id().as_str(), "nanocontrol");
    assert_eq!(nano.execution_model(), &ExecutionModel::Bytecode);
    assert!(nano.supports_artifact_format_name("nano-control"));

    assert_eq!(mts.id().as_str(), "mts-bytecode");
    assert_eq!(mts.execution_model(), &ExecutionModel::Bytecode);
    assert!(mts.supports_artifact_format_name("mts-bytecode"));

    assert_ne!(
        nano.id(),
        mts.id(),
        "different bytecode formats must not alias implicitly"
    );
}

#[test]
fn llvm_ir_descriptor_is_not_misclassified_as_a_native_executable() {
    let descriptor = descriptor_for(CompilationTarget::LLVMIR);

    assert_eq!(descriptor.id().as_str(), "llvm-ir");
    assert_eq!(
        descriptor.execution_model(),
        &ExecutionModel::IntermediateRepresentation
    );

    assert!(
        descriptor.supports_artifact_format_name("llvm-ir"),
        "the LLVM IR descriptor must declare its format"
    );

    // The descriptor's declaration is not proof that the legacy emitter
    // generates valid LLVM IR. Verification belongs to artifact validation.
    assert_eq!(descriptor.architecture(), None);
    assert_eq!(descriptor.abi(), None);
}

#[test]
fn universal_ir_export_is_not_advertised_as_an_executable_backend() {
    let descriptor = descriptor_for(
        CompilationTarget::UniversalIRExport("json".to_owned())
    );

    assert_eq!(descriptor.id().as_str(), "universal-ir:json");
    assert_eq!(
        descriptor.execution_model(),
        &ExecutionModel::IntermediateRepresentation
    );
    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::DescriptorOnly
    );
    assert!(
        descriptor.supports_artifact_format_name("universal-ir")
    );
}

// ---------------------------------------------------------------------------
// Metadata independence and validation
// ---------------------------------------------------------------------------

#[test]
fn target_metadata_fields_are_independent() {
    let descriptor = TargetDescriptor::from_name(
        "vendor.cpu-v2",
        "Vendor CPU v2",
        ExecutionModel::Native,
    )
    .expect("valid target identifier")
    .with_architecture("vendor-arch-v2")
    .expect("valid architecture metadata")
    .with_operating_system("example-os")
    .expect("valid OS metadata")
    .with_triple_name("vendor-arch-v2-example-os")
    .expect("valid target triple")
    .with_abi(
        AbiDescriptor::new("vendor-abi")
            .expect("valid ABI")
            .with_version("2")
            .with_calling_convention("vendor-call-v2")
            .with_object_format("vendor-object-v2"),
    )
    .expect("valid ABI metadata")
    .with_data_layout(
        DataLayoutDescriptor::new("vendor-layout-v2")
            .expect("valid data layout")
            .with_description("Target-defined address spaces and alignments"),
    )
    .expect("valid layout metadata")
    .with_artifact_format_name("vendor-object-v2")
    .expect("valid artifact format")
    .with_availability(ImplementationAvailability::DescriptorOnly);

    assert_eq!(descriptor.architecture(), Some("vendor-arch-v2"));
    assert_eq!(descriptor.operating_system(), Some("example-os"));

    assert_eq!(
        descriptor.triple().map(TargetTriple::as_str),
        Some("vendor-arch-v2-example-os")
    );

    let abi = descriptor.abi().expect("ABI explicitly supplied");
    assert_eq!(abi.name(), Some("vendor-abi"));
    assert_eq!(abi.version(), Some("2"));
    assert_eq!(abi.calling_convention(), Some("vendor-call-v2"));
    assert_eq!(abi.object_format(), Some("vendor-object-v2"));

    let layout = descriptor
        .data_layout()
        .expect("data layout explicitly supplied");

    assert_eq!(layout.format(), Some("vendor-layout-v2"));
    assert_eq!(
        layout.description(),
        Some("Target-defined address spaces and alignments")
    );

    assert!(
        descriptor.supports_artifact_format_name("vendor-object-v2")
    );
    assert_eq!(
        descriptor.availability(),
        ImplementationAvailability::DescriptorOnly
    );
    assert!(descriptor.validate().is_ok());
}

#[test]
fn target_identifiers_and_artifact_formats_are_open_world() {
    let target_id = TargetId::new("future.vendor:accelerator-v2")
        .expect("new target domains must not require a central enum change");

    let format = ArtifactFormat::new("vendor/custom:format")
        .expect("new artifact formats must not require a central enum change");

    assert_eq!(
        target_id.as_str(),
        "future.vendor:accelerator-v2"
    );
    assert_eq!(format.as_str(), "vendor/custom:format");
}

#[test]
fn malformed_target_metadata_is_rejected() {
    assert!(TargetId::new("").is_err());
    assert!(TargetId::new("-invalid-target").is_err());
    assert!(TargetTriple::new("").is_err());
    assert!(TargetTriple::new("target with spaces").is_err());
    assert!(AbiDescriptor::new(" ").is_err());
    assert!(DataLayoutDescriptor::new("").is_err());
    assert!(ArtifactFormat::new("").is_err());
}

#[test]
fn architecture_metadata_is_not_a_resource_capacity_claim() {
    let descriptor = TargetDescriptor::from_name(
        "future.compute-domain",
        "Future Compute Domain",
        ExecutionModel::Other("heterogeneous".to_owned()),
    )
    .expect("valid extension target")
    .with_architecture("future-architecture")
    .expect("valid architecture metadata");

    assert_eq!(descriptor.architecture(), Some("future-architecture"));
    assert_eq!(descriptor.operating_system(), None);
    assert_eq!(descriptor.triple(), None);
    assert_eq!(descriptor.abi(), None);
    assert_eq!(descriptor.data_layout(), None);

    // The descriptor has no universal CPU, qubit, memory, device, or
    // tensor-rank ceiling. Resource feasibility must be evaluated by the
    // appropriate target/resource-planning layers.
    assert!(descriptor.validate().is_ok());
}