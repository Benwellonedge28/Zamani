#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! Backend regression tests for Zamani.
//!
//! Path: src/backend/tests/regression.rs
//! Edition: Rust 2021
//! Minimum Rust version: 1.97
//! Safety: unsafe Rust is forbidden.
//!
//! # Purpose
//!
//! Preserve minimal, deterministic reproductions of backend defects that
//! could violate target identity, semantic preservation, output integrity,
//! or POCO-REAF compatibility.
//!
//! # Architectural authority
//!
//! - grammar/Zamani.g4: grammar composition root.
//! - grammar/specification/grammar-authority.md: language ownership.
//! - grammar/specification/semantic-model.md: source-program meaning.
//! - grammar/specification/compilation-model.md: compilation contracts.
//! - grammar/specification/compatibility.md: compatibility contracts.
//! - grammar/specification/scalability-model.md: resource-parametric scaling.
//! - grammar/specification/poco-reaf.md: portability requirements.
//! - src/compiler_types.rs: CompilationTarget and CompilerConfig.
//! - src/ir_gen.rs: legacy IrModule and instruction representation.
//! - src/ir_verify.rs: legacy IR verification.
//! - src/backend.rs: current legacy backend dispatch and emitters.
//! - src/backend/target.rs: target descriptors, after module integration.
//! - src/backend/pipeline.rs: canonical backend pipeline, after integration.
//!
//! These tests exercise the existing public backend API. They must not
//! introduce a competing backend trait, registry, IR, or target dispatcher.
//!
//! # Integration prerequisite
//!
//! The repository currently contains both src/backend.rs and
//! src/backend/mod.rs. Rust module ownership must be resolved before
//! these tests are registered. Until then, src/backend.rs remains the
//! legacy implementation root.
//!
//! Register this file from src/backend.rs with:
//!
//! #[cfg(test)]
//! #[path = "backend/tests/regression.rs"]
//! mod regression_tests;
//!
//! Register it exactly once. Do not compile it through a second backend
//! module root.
//!
//! # Regression policy
//!
//! A test that exposes a known defect is an acceptance test, not a reason
//! to weaken the expected behavior. A failing test must remain reproducible
//! until the corresponding implementation defect is fixed.
//!
//! Tests do not claim that every target supports every operation. Unsupported
//! semantics must be rejected explicitly rather than represented as comments
//! or silently discarded.
//!
//! # Scalability policy
//!
//! Test workloads are finite examples, not language or hardware ceilings.
//! The tests do not prescribe a maximum function count, instruction count,
//! identifier length, number of devices, or artifact size. Actual resource
//! limits belong to explicit operational policies and the relevant toolchains.

use crate::backend::{
    Backend,
    CodeGenError,
    CodeGenerator,
    LlvmIrBackend,
    X86Backend,
};
use crate::compiler_types::{
    CompilationTarget,
    CompilerConfig,
    OptimizationLevel,
};
use crate::ir_gen::{
    IrFunction,
    IrInstruction,
    IrModule,
    IrRegister,
    IrType,
    IrValue,
};

// ---------------------------------------------------------------------------
// Shared fixtures
// ---------------------------------------------------------------------------

fn config_for(target: CompilationTarget) -> CompilerConfig {
    CompilerConfig {
        target,
        opt_level: OptimizationLevel::None,
        debug_info: false,
        verify: true,
        emit_ir: false,
        parallel: false,
    }
}

fn minimal_module(name: &str) -> IrModule {
    let mut module = IrModule::new(name);
    let mut main = IrFunction::new("main", Vec::new(), IrType::I64);

    main.push(IrInstruction::Ret(Some(IrValue::ConstInt(
        0,
        IrType::I64,
    ))));

    module.add_function(main);
    module
}

fn comment_module(name: &str, comments: usize) -> IrModule {
    let mut module = IrModule::new(name);
    let mut function =
        IrFunction::new("regression_probe", Vec::new(), IrType::Void);

    for index in 0..comments {
        function.push(IrInstruction::Comment(format!(
            "regression-marker-{index}"
        )));
    }

    function.push(IrInstruction::Ret(None));
    module.add_function(function);
    module
}

fn assert_explicitly_unsupported(error: &CodeGenError) {
    let message = error.message.to_ascii_lowercase();

    let unsupported = [
        "unsupported",
        "unavailable",
        "not implemented",
        "not supported",
    ]
    .iter()
    .any(|marker| message.contains(marker));

    assert!(
        unsupported,
        "an unsupported target or operation must be reported explicitly; \
         target={:?}, message={:?}",
        error.target,
        error.message
    );
}

// ---------------------------------------------------------------------------
// Regression: architecture dispatch
// ---------------------------------------------------------------------------

/// Regression: the legacy dispatcher previously routed Arm64 to X86Backend.
///
/// A correct implementation must either generate an artifact identified as
/// AArch64 or explicitly reject the target until an implementation is
/// available. Returning an x86-64 artifact is never acceptable.
#[test]
fn arm64_never_silently_dispatches_to_x86_64() {
    let generator =
        CodeGenerator::new(config_for(CompilationTarget::Arm64));

    let result = generator.generate(&minimal_module("arm64_regression"));

    match result {
        Ok(output) => {
            assert!(
                output.target.to_ascii_lowercase().contains("aarch64")
                    || output.target.to_ascii_lowercase().contains("arm64"),
                "Arm64 compilation returned a different target: {:?}",
                output.target
            );

            assert_eq!(
                output.size_bytes,
                output.source.len(),
                "reported output size must equal UTF-8 byte length"
            );
        }
        Err(error) => {
            assert_explicitly_unsupported(&error);

            assert!(
                !error.target.to_ascii_lowercase().contains("x86"),
                "an AArch64 request must not be reported as an x86 target"
            );
        }
    }
}

/// Regression: the existing x86-64 target must retain its identity and
/// extension when the dispatcher selects its actual implementation.
#[test]
fn x86_64_dispatch_preserves_target_identity() {
    let generator =
        CodeGenerator::new(config_for(CompilationTarget::X86_64Linux));

    let output = generator
        .generate(&minimal_module("x86_dispatch_regression"))
        .expect("the legacy x86-64 emitter should accept the minimal fixture");

    assert_eq!(output.target, "x86_64");
    assert_eq!(output.extension, ".s");
    assert_eq!(output.size_bytes, output.source.len());
    assert!(output.source.contains("main"));
}

// ---------------------------------------------------------------------------
// Regression: unsupported semantics must not become comments
// ---------------------------------------------------------------------------

/// Regression: the legacy native emitter can fall through to emitting
/// unimplemented instructions as assembly comments.
///
/// A comment is not an implementation of a quantum operation. The backend
/// must reject unsupported semantics or produce a genuine, explicitly
/// defined lowering; it must not silently claim that comment-only output
/// implements the operation.
#[test]
fn x86_does_not_silently_comment_out_quantum_operations() {
    let mut module = IrModule::new("unsupported_quantum_regression");
    let mut function =
        IrFunction::new("quantum_probe", Vec::new(), IrType::Void);

    function.push(IrInstruction::QuantumGate(
        IrRegister::new("quantum_result", IrType::I64),
        "H".to_owned(),
        Vec::new(),
    ));
    function.push(IrInstruction::Ret(None));
    module.add_function(function);

    let result = X86Backend.generate(&module);

    match result {
        Err(error) => assert_explicitly_unsupported(&error),
        Ok(output) => {
            let lowered_instruction = output
                .source
                .lines()
                .any(|line| {
                    let line = line.trim();
                    line.contains("__quantum_rt_h")
                        && !line.starts_with('#')
                });

            assert!(
                lowered_instruction,
                "quantum semantics must not be represented only by an \
                 assembly comment; generated output:\n{}",
                output.source
            );
        }
    }
}

/// Regression: a backend must preserve every explicitly supported comment
/// marker in output. This catches accidental instruction loss and truncation
/// without requiring physical hardware or an external toolchain.
#[test]
fn x86_output_preserves_regression_markers() {
    let marker_count = 41;
    let module = comment_module("marker_preservation_regression", marker_count);

    let output = X86Backend
        .generate(&module)
        .expect("the x86 emitter should preserve comment-only fixtures");

    for index in 0..marker_count {
        let marker = format!("regression-marker-{index}");

        assert!(
            output.contains(&marker),
            "generated output lost marker {marker:?}"
        );
    }

    assert!(output.contains("regression_probe:"));
}

// ---------------------------------------------------------------------------
// Regression: output metadata and determinism
// ---------------------------------------------------------------------------

/// Regression: output-size metadata must describe encoded bytes, not
/// Unicode scalar values or grapheme counts.
#[test]
fn generated_output_size_uses_utf8_bytes() {
    let mut module = IrModule::new("utf8_size_regression");
    let mut function =
        IrFunction::new("utf8_probe", Vec::new(), IrType::Void);

    function.push(IrInstruction::Comment(
        "Unicode regression: λ 漢字".to_owned(),
    ));
    function.push(IrInstruction::Ret(None));
    module.add_function(function);

    let output = CodeGenerator::new(config_for(CompilationTarget::X86_64Linux))
        .generate(&module)
        .expect("the x86 emitter should accept the UTF-8 fixture");

    assert!(output.source.contains("λ 漢字"));
    assert_eq!(output.size_bytes, output.source.as_bytes().len());
}

/// Regression: identical inputs and configuration must not produce
/// different results merely because compilation was repeated.
#[test]
fn identical_inputs_produce_identical_x86_output() {
    let module = comment_module("determinism_regression", 73);
    let generator =
        CodeGenerator::new(config_for(CompilationTarget::X86_64Linux));

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

/// Regression: separately constructed equivalent inputs must not depend
/// on object identity, allocation addresses, or incidental construction
/// history.
#[test]
fn equivalent_modules_produce_identical_output() {
    let first_module = comment_module("equivalent_regression", 29);
    let second_module = comment_module("equivalent_regression", 29);

    let generator =
        CodeGenerator::new(config_for(CompilationTarget::X86_64Linux));

    let first = generator
        .generate(&first_module)
        .expect("first equivalent module should compile");
    let second = generator
        .generate(&second_module)
        .expect("second equivalent module should compile");

    assert_eq!(first.source, second.source);
    assert_eq!(first.size_bytes, second.size_bytes);
}

// ---------------------------------------------------------------------------
// Regression: boundary inputs
// ---------------------------------------------------------------------------

/// Regression: an empty module must not panic or invent a mandatory function
/// count. Whether an empty module is executable is a separate target contract.
#[test]
fn empty_module_does_not_panic() {
    let module = IrModule::new("empty_module_regression");
    let generator =
        CodeGenerator::new(config_for(CompilationTarget::X86_64Linux));

    let output = generator
        .generate(&module)
        .expect("the legacy x86 emitter should handle an empty module");

    assert_eq!(output.target, "x86_64");
    assert_eq!(output.extension, ".s");
    assert_eq!(output.size_bytes, output.source.len());
}

/// Regression: larger symbolic names must not be rejected because of an
/// arbitrary backend-defined identifier ceiling. This is a representation
/// test, not a claim that every external assembler accepts every identifier.
#[test]
fn long_function_names_are_not_truncated_by_backend_dispatch() {
    let name = format!("function_{}", "scale".repeat(512));
    let mut module = IrModule::new("long_name_regression");
    let mut function =
        IrFunction::new(name.clone(), Vec::new(), IrType::Void);

    function.push(IrInstruction::Ret(None));
    module.add_function(function);

    let output = CodeGenerator::new(config_for(CompilationTarget::X86_64Linux))
        .generate(&module)
        .expect("the legacy emitter should preserve the symbolic name");

    assert!(
        output.source.contains(&name),
        "generated output truncated the function identifier"
    );
    assert_eq!(output.size_bytes, output.source.len());
}

// ---------------------------------------------------------------------------
// Integration notes
// ---------------------------------------------------------------------------

// This file intentionally does not invoke LlvmIrBackend::generate to test
// filesystem purity: the current legacy implementation writes to a
// target/ path as a side effect, and changing the process working directory
// inside parallel tests would introduce cross-test interference.
//
// Once LLVM emission is moved behind an explicit artifact/output boundary,
// add an isolated test proving that in-memory generation performs no
// filesystem writes and that returned LLVM IR passes the configured verifier.
//
// The following integration checks also belong in their authoritative suites:
// - target descriptors and registry: tests/targets.rs and tests/registry.rs;
// - capability compatibility: tests/compatibility.rs;
// - resource budgets: tests/scalability.rs;
// - artifact format and validation: tests/ artifacts.rs (correct the existing
//   leading-space filename during repository cleanup);
// - pipeline verification and failure propagation: pipeline.rs contract tests;
// - grammar and semantic meaning: frontend/domain conformance tests.
//
// Do not make these tests pass by changing the language meaning, weakening
// target identity, silently dropping unsupported operations, or treating
// unverified output as verified.