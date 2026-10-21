//! Backend registry and target-dispatch acceptance tests.
//!
//! Path: src/backend/tests/registry.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Current repository status
//!
//! The existing src/backend.rs contains CodeGenerator and Backend, but
//! does not yet provide a production BackendRegistry API. The dispatcher
//! currently uses a hard-coded match over CompilationTarget.
//!
//! These tests intentionally use the actual existing API. They do not
//! invent a registry type or assume that registration, lookup, or
//! capability-negotiation methods have already been implemented.
//!
//! # Required integration
//!
//! Until src/backend/registry.rs and its public API are implemented,
//! add this test module to src/backend.rs:
//
//! ```ignore
//! #[cfg(test)]
//! #[path = "backend/tests/registry.rs"]
//! mod registry_tests;
//! ```
//!
//! Do not declare the same tests under a competing backend module root.
//!
//! # Migration contract
//!
//! The eventual authoritative registry must:
//!
//! - have one canonical registration per backend identity;
//! - reject duplicate identities without replacing existing entries;
//! - expose deterministic enumeration and lookup;
//! - reject incompatible API registrations explicitly;
//! - never select a different target silently;
//! - permit new target implementations without editing a central
//!   target-dispatch match;
//! - leave source-language semantics and domain IR ownership upstream;
//! - avoid arbitrary universal hardware-capacity ceilings.
//!
//! The current dispatcher tests below are regression gates, not proof
//! that a production registry already exists.

#![forbid(unsafe_code)]

use crate::backend::{
    Backend,
    CodeGenerator,
    MtsBackend,
    NanoBackend,
    NewWasmBackendAdapter,
    QasmBackend,
    RiscVBackend,
    WasmBackend,
    X86Backend,
};
use crate::compiler_types::{
    CompilationTarget,
    CompilerConfig,
};
use crate::ir_gen::{
    IrFunction,
    IrInstruction,
    IrModule,
    IrType,
    IrValue,
};

/// Minimal input independent of parsing, grammar, and semantic analysis.
fn minimal_module() -> IrModule {
    let mut module = IrModule::new("registry_contract");

    let mut main = IrFunction::new("main", Vec::new(), IrType::I64);
    main.push(IrInstruction::Ret(Some(IrValue::ConstInt(
        0,
        IrType::I64,
    ))));

    module.add_function(main);
    module
}

fn config_for(target: CompilationTarget) -> CompilerConfig {
    CompilerConfig {
        target,
        ..CompilerConfig::default()
    }
}

fn generate_target(target: CompilationTarget) -> crate::backend::CodeGenOutput {
    CodeGenerator::new(config_for(target))
        .generate(&minimal_module())
        .expect("target generation must succeed or expose an explicit error")
}

fn assert_backend_identity(backend: &dyn Backend) {
    assert!(
        !backend.target_name().trim().is_empty(),
        "registered backend identity must not be empty"
    );

    assert!(
        !backend.file_extension().trim().is_empty(),
        "backend artifact extension must not be empty"
    );

    assert!(
        backend.file_extension().starts_with('.'),
        "artifact extension must begin with a dot: {:?}",
        backend.file_extension()
    );
}

#[test]
fn existing_backend_descriptors_have_nonempty_identities() {
    assert_backend_identity(&X86Backend);
    assert_backend_identity(&WasmBackend);
    assert_backend_identity(&NewWasmBackendAdapter);
    assert_backend_identity(&QasmBackend);
    assert_backend_identity(&RiscVBackend);
    assert_backend_identity(&NanoBackend);
    assert_backend_identity(&MtsBackend);
}

#[test]
fn canonical_backend_inventory_has_no_duplicate_target_identities() {
    // The legacy WasmBackend and NewWasmBackendAdapter currently advertise
    // the same identity. Until migration is complete, choose exactly one
    // as the canonical WebAssembly implementation.
    //
    // The legacy implementation is deliberately excluded here; it must
    // not be registered alongside the adapter under the same identity.
    let canonical: Vec<Box<dyn Backend>> = vec![
        Box::new(X86Backend),
        Box::new(NewWasmBackendAdapter),
        Box::new(QasmBackend),
        Box::new(RiscVBackend),
        Box::new(NanoBackend),
        Box::new(MtsBackend),
    ];

    let mut identities: Vec<&str> = canonical
        .iter()
        .map(|backend| backend.target_name())
        .collect();

    identities.sort_unstable();

    for pair in identities.windows(2) {
        assert_ne!(
            pair[0],
            pair[1],
            "canonical backend registrations must not have duplicate identities"
        );
    }
}

#[test]
fn legacy_and_adapter_wasm_backends_expose_the_same_identity() {
    // This documents a real migration hazard. A future registry must
    // reject registering both implementations under one target identity,
    // or the migration must retire/rename the legacy implementation.
    assert_eq!(WasmBackend.target_name(), "wasm32");
    assert_eq!(NewWasmBackendAdapter.target_name(), "wasm32");

    assert_eq!(
        WasmBackend.file_extension(),
        NewWasmBackendAdapter.file_extension()
    );
}

#[test]
fn x86_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::X86_64Linux);

    assert_eq!(output.target, "x86_64");
    assert_eq!(output.extension, ".s");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn wasm_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::Wasm32);

    assert_eq!(output.target, "wasm32");
    assert_eq!(output.extension, ".wat");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn qasm_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::QASM);

    assert_eq!(output.target, "qasm");
    assert_eq!(output.extension, ".qasm");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn riscv_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::RiscV);

    assert_eq!(output.target, "riscv64");
    assert_eq!(output.extension, ".rv64.s");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn nano_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::NanoControl);

    assert_eq!(output.target, "nano");
    assert_eq!(output.extension, ".nano");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn mts_target_dispatch_preserves_target_identity() {
    let output = generate_target(CompilationTarget::MTSBytecode);

    assert_eq!(output.target, "mts");
    assert_eq!(output.extension, ".mts");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn arm64_must_never_dispatch_to_x86_64() {
    // Acceptance test for a known defect in src/backend.rs.
    //
    // The current dispatcher maps CompilationTarget::Arm64 to
    // X86Backend. This assertion is expected to fail until that defect
    // is fixed. The correct implementation may either generate valid
    // ARM64 output or return a typed unsupported-target error.
    let result = CodeGenerator::new(config_for(CompilationTarget::Arm64))
        .generate(&minimal_module());

    match result {
        Ok(output) => {
            assert_ne!(
                output.target, "x86_64",
                "ARM64 must not be reported as x86-64"
            );
        }
        Err(error) => {
            assert!(
                !error.message.trim().is_empty(),
                "unsupported-target errors must be actionable"
            );
        }
    }
}

#[test]
fn repeated_target_dispatch_is_deterministic() {
    let targets = [
        CompilationTarget::X86_64Linux,
        CompilationTarget::Wasm32,
        CompilationTarget::QASM,
        CompilationTarget::NanoControl,
        CompilationTarget::RiscV,
        CompilationTarget::MTSBytecode,
    ];

    for target in targets {
        let first = CodeGenerator::new(config_for(target.clone()))
            .generate(&minimal_module())
            .expect("first generation must succeed");

        let second = CodeGenerator::new(config_for(target))
            .generate(&minimal_module())
            .expect("repeated generation must succeed");

        assert_eq!(first.target, second.target);
        assert_eq!(first.extension, second.extension);
        assert_eq!(first.source, second.source);
        assert_eq!(first.size_bytes, second.size_bytes);
    }
}

#[test]
fn registry_identity_must_not_depend_on_registration_order() {
    // This verifies the deterministic identity properties required by
    // the future registry using the current backend implementations.
    // It does not substitute for a real BackendRegistry enumeration test.
    let forward: Vec<Box<dyn Backend>> = vec![
        Box::new(X86Backend),
        Box::new(NewWasmBackendAdapter),
        Box::new(QasmBackend),
        Box::new(RiscVBackend),
        Box::new(NanoBackend),
        Box::new(MtsBackend),
    ];

    let reverse: Vec<Box<dyn Backend>> = vec![
        Box::new(MtsBackend),
        Box::new(NanoBackend),
        Box::new(RiscVBackend),
        Box::new(QasmBackend),
        Box::new(NewWasmBackendAdapter),
        Box::new(X86Backend),
    ];

    let mut forward_names: Vec<String> = forward
        .iter()
        .map(|backend| backend.target_name().to_owned())
        .collect();

    let mut reverse_names: Vec<String> = reverse
        .iter()
        .map(|backend| backend.target_name().to_owned())
        .collect();

    forward_names.sort();
    reverse_names.sort();

    assert_eq!(
        forward_names, reverse_names,
        "backend identity inventory must be independent of declaration order"
    );
}

#[test]
fn backend_generation_does_not_impose_a_test_defined_hardware_limit() {
    // A large symbolic module name checks that the test suite does not
    // introduce an arbitrary universal name-length ceiling. This does
    // not claim that every external artifact format accepts every name.
    let name = format!("large_module_{}", "x".repeat(4096));
    let module = IrModule::new(name);

    let output = X86Backend
        .generate(&module)
        .expect("generation must not panic because of a test-defined size cap");

    assert!(
        !output.is_empty(),
        "successful generation must return output"
    );
}

#[test]
fn successful_dispatch_returns_consistent_output_metadata() {
    let targets = [
        CompilationTarget::X86_64Linux,
        CompilationTarget::Wasm32,
        CompilationTarget::QASM,
        CompilationTarget::NanoControl,
        CompilationTarget::RiscV,
        CompilationTarget::MTSBytecode,
    ];

    for target in targets {
        let output = generate_target(target);

        assert!(!output.target.trim().is_empty());
        assert!(!output.extension.trim().is_empty());

        assert_eq!(
            output.size_bytes,
            output.source.as_bytes().len(),
            "reported output size must be measured in bytes"
        );
    }
}