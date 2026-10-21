//! Backend contract tests for Zamani.
//!
//! Path: src/backend/tests/contract.rs
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Establishes shared behavioral contracts for the existing backend API:
//!
//! - generated output must have consistent metadata;
//! - declared target names and extensions must be meaningful;
//! - repeated generation from identical inputs must be deterministic;
//! - backends must be Send + Sync;
//! - target dispatch must preserve target identity;
//! - architecture mismatches must not silently select another architecture;
//! - output-size accounting must use the actual returned bytes;
//! - empty modules must be handled without panics;
//! - large symbolic identifiers must not be constrained by arbitrary
//!   backend-test limits.
//!
//! These tests deliberately exercise the existing `crate::backend` API.
//! They do not assume that the proposed modular registry, artifact model,
//! or capability-negotiation API already exists.
//!
//! # Integration contract
//!
//! This file is a child test module of the authoritative `backend` module.
//! In the current repository, `src/backend.rs` is the implementation file.
//! The test module must therefore be wired from that file until module
//! ownership is migrated deliberately.
//!
//! Add the following declaration to src/backend.rs:
//!
//! ```ignore
//! #[cfg(test)]
//! #[path = "backend/tests/contract.rs"]
//! mod contract_tests;
//! ```
//!
//! Do not also compile the same tests through a competing
//! src/backend/mod.rs module root.
//!
//! # Important limitations
//!
//! The legacy `Backend` trait accepts an `IrModule` and returns a String.
//! It does not yet expose structured artifacts, capability evidence,
//! validation reports, or an explicit output destination.
//!
//! Consequently, these tests cannot establish full semantic preservation,
//! artifact validity, absence of hidden filesystem side effects, or
//! hardware execution correctness. Those require stronger interfaces and
//! dedicated integration tests.
//!
//! Tests for known defects are intentionally written as acceptance tests.
//! They may fail against the current implementation until the associated
//! production defect is fixed.

#![forbid(unsafe_code)]

use crate::backend::{
    Backend,
    CodeGenerator,
    CompilationTarget,
    CompilerConfig,
    MtsBackend,
    NanoBackend,
    NewWasmBackendAdapter,
    QasmBackend,
    RiscVBackend,
    WasmBackend,
    X86Backend,
};
use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType, IrValue};

/// Create a small, structurally simple module.
///
/// This fixture is intentionally independent of the parser and semantic
/// analyzer. Backend contract tests should be able to isolate backend
/// behavior from frontend behavior.
fn minimal_module() -> IrModule {
    let mut module = IrModule::new("backend_contract");

    let mut main = IrFunction::new("main", Vec::new(), IrType::I64);
    main.push(IrInstruction::Ret(Some(IrValue::ConstInt(
        0,
        IrType::I64,
    ))));

    module.add_function(main);
    module
}

/// Create an empty module for boundary tests.
///
/// An empty module is not necessarily executable, but backend handling of
/// an empty collection must be explicit and must not panic.
fn empty_module() -> IrModule {
    IrModule::new("empty_backend_contract")
}

/// Exercise common invariants shared by the legacy Backend trait.
///
/// This helper intentionally avoids assumptions about the syntax of every
/// output format. Format-specific validators belong in target-specific
/// tests and must use the corresponding authoritative toolchain.
fn assert_common_backend_contract(backend: &dyn Backend) {
    let module = minimal_module();

    let target = backend.target_name();
    let extension = backend.file_extension();

    assert!(
        !target.trim().is_empty(),
        "backend target name must not be empty"
    );

    assert!(
        !extension.trim().is_empty(),
        "backend file extension must not be empty"
    );

    assert!(
        extension.starts_with('.'),
        "backend extension should include its leading dot: {extension:?}"
    );

    let first = backend
        .generate(&module)
        .unwrap_or_else(|error| {
            panic!(
                "backend {:?} rejected the minimal contract fixture: {}",
                target, error
            )
        });

    assert!(
        !first.trim().is_empty(),
        "backend {:?} returned empty output for the minimal fixture",
        target
    );

    let second = backend
        .generate(&module)
        .unwrap_or_else(|error| {
            panic!(
                "backend {:?} failed on repeated generation: {}",
                target, error
            )
        });

    assert_eq!(
        first, second,
        "backend {:?} must generate deterministic output for identical \
         inputs and unchanged configuration",
        target
    );
}

/// Every backend implementation used by this suite must be safe to share
/// between threads. This is a compile-time contract, not a concurrency
/// performance guarantee.
fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn existing_backends_satisfy_send_sync_contract() {
    assert_send_sync::<X86Backend>();
    assert_send_sync::<WasmBackend>();
    assert_send_sync::<NewWasmBackendAdapter>();
    assert_send_sync::<QasmBackend>();
    assert_send_sync::<RiscVBackend>();
    assert_send_sync::<NanoBackend>();
    assert_send_sync::<MtsBackend>();
}

#[test]
fn x86_backend_satisfies_common_contract() {
    assert_common_backend_contract(&X86Backend);
}

#[test]
fn legacy_wasm_backend_satisfies_common_contract() {
    assert_common_backend_contract(&WasmBackend);
}

#[test]
fn production_wasm_adapter_satisfies_common_contract() {
    assert_common_backend_contract(&NewWasmBackendAdapter);
}

#[test]
fn qasm_backend_satisfies_common_contract() {
    assert_common_backend_contract(&QasmBackend);
}

#[test]
fn riscv_backend_satisfies_common_contract() {
    assert_common_backend_contract(&RiscVBackend);
}

#[test]
fn nano_backend_satisfies_common_contract() {
    assert_common_backend_contract(&NanoBackend);
}

#[test]
fn mts_backend_satisfies_common_contract() {
    assert_common_backend_contract(&MtsBackend);
}

#[test]
fn backend_handles_empty_module_without_panicking() {
    let module = empty_module();

    let backends: Vec<Box<dyn Backend>> = vec![
        Box::new(X86Backend),
        Box::new(WasmBackend),
        Box::new(NewWasmBackendAdapter),
        Box::new(QasmBackend),
        Box::new(RiscVBackend),
        Box::new(NanoBackend),
        Box::new(MtsBackend),
    ];

    for backend in &backends {
        let result = backend.generate(&module);

        assert!(
            result.is_ok(),
            "backend {:?} must explicitly handle an empty module; got {:?}",
            backend.target_name(),
            result.as_ref().err()
        );
    }
}

#[test]
fn code_generator_reports_actual_output_size() {
    let config = CompilerConfig::default();
    let generator = CodeGenerator::new(config);
    let module = minimal_module();

    let output = generator
        .generate(&module)
        .expect("default backend should generate the minimal fixture");

    assert_eq!(
        output.size_bytes,
        output.source.len(),
        "size_bytes must equal the number of UTF-8 bytes in source"
    );

    assert!(
        !output.target.trim().is_empty(),
        "successful output must identify its target"
    );

    assert!(
        !output.extension.trim().is_empty(),
        "successful output must identify its artifact extension"
    );
}

#[test]
fn code_generator_preserves_requested_riscv_target_identity() {
    let config = CompilerConfig {
        target: CompilationTarget::RiscV,
        ..CompilerConfig::default()
    };

    let generator = CodeGenerator::new(config);

    let output = generator
        .generate(&minimal_module())
        .expect("RISC-V generation should succeed for the minimal fixture");

    assert_eq!(output.target, "riscv64");
    assert_eq!(output.extension, ".rv64.s");
}

#[test]
fn code_generator_does_not_route_arm64_to_x86_backend() {
    let config = CompilerConfig {
        target: CompilationTarget::Arm64,
        ..CompilerConfig::default()
    };

    let generator = CodeGenerator::new(config);

    let output = generator
        .generate(&minimal_module())
        .expect(
            "the ARM64 target must either generate valid ARM64 output \
             or return an explicit unsupported-target error",
        );

    assert_ne!(
        output.target, "x86_64",
        "ARM64 compilation must never return x86-64 output"
    );

    assert_ne!(
        output.extension, ".rv64.s",
        "ARM64 compilation must not return RISC-V assembly"
    );
}

#[test]
fn code_generator_rejects_no_output_metadata_on_success() {
    let generator = CodeGenerator::new(CompilerConfig::default());

    let output = generator
        .generate(&minimal_module())
        .expect("minimal fixture should compile for the default target");

    assert!(
        !output.target.trim().is_empty(),
        "successful compilation must identify the target"
    );

    assert!(
        !output.extension.trim().is_empty(),
        "successful compilation must identify the artifact format"
    );

    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn repeated_code_generator_calls_are_deterministic() {
    let generator = CodeGenerator::new(CompilerConfig::default());
    let module = minimal_module();

    let first = generator
        .generate(&module)
        .expect("first generation should succeed");

    let second = generator
        .generate(&module)
        .expect("second generation should succeed");

    assert_eq!(first.target, second.target);
    assert_eq!(first.extension, second.extension);
    assert_eq!(first.source, second.source);
    assert_eq!(first.size_bytes, second.size_bytes);
}

#[test]
fn backend_handles_long_module_names_without_test_defined_limits() {
    // This is an operational stress fixture, not a promise that every
    // target's external format permits every possible identifier.
    //
    // Target-specific name restrictions must be validated and diagnosed
    // by the corresponding backend rather than by a universal arbitrary
    // identifier-length ceiling in the common contract.
    let name = format!("module_{}", "x".repeat(4096));
    let module = IrModule::new(name);

    let output = X86Backend
        .generate(&module)
        .expect("x86 backend should handle the fixture without panicking");

    assert!(
        !output.is_empty(),
        "successful generation must produce output"
    );
}

#[test]
fn backend_output_size_is_measured_in_bytes_not_characters() {
    let mut module = IrModule::new("unicode_contract");

    // Comments are used only to test output encoding and byte accounting.
    // They do not define source-language Unicode semantics.
    let mut function = IrFunction::new("main", Vec::new(), IrType::I64);
    function.push(IrInstruction::Comment(
        "Unicode: λ, 世界, 🚀".to_owned(),
    ));
    function.push(IrInstruction::Ret(Some(IrValue::ConstInt(
        0,
        IrType::I64,
    ))));

    module.add_function(function);

    let output = CodeGenerator::new(CompilerConfig::default())
        .generate(&module)
        .expect("Unicode fixture should generate");

    assert_eq!(output.size_bytes, output.source.as_bytes().len());
}

#[test]
fn unsupported_or_invalid_generation_is_not_reported_as_successful_empty_output() {
    // The legacy trait does not yet expose a uniform capability negotiation
    // API. This test establishes the narrower invariant that a successful
    // generation result must contain an actual artifact representation.
    //
    // A future typed artifact API should extend this test to validate the
    // declared artifact format and target compatibility.
    let module = minimal_module();

    let backends: Vec<Box<dyn Backend>> = vec![
        Box::new(X86Backend),
        Box::new(WasmBackend),
        Box::new(NewWasmBackendAdapter),
        Box::new(QasmBackend),
        Box::new(RiscVBackend),
        Box::new(NanoBackend),
        Box::new(MtsBackend),
    ];

    for backend in backends {
        match backend.generate(&module) {
            Ok(output) => assert!(
                !output.trim().is_empty(),
                "backend {:?} returned success without an artifact",
                backend.target_name()
            ),
            Err(error) => assert!(
                !error.message.trim().is_empty(),
                "backend {:?} returned an error without a useful message",
                backend.target_name()
            ),
        }
    }
}