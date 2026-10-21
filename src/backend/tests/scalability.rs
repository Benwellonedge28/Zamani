
#![forbid(unsafe_code)]

//! Backend scalability and resource-behaviour contract tests.
//!
//! Integration target:
//!     src/backend/tests/scalability.rs
//!
//! Current production APIs exercised:
//!     crate::backend::CodeGenerator
//!     crate::backend::CodeGenOutput
//!     crate::compiler_types::{CompilationTarget, CompilerConfig}
//!     crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType}
//!
//! These tests deliberately exercise the existing IR/backend boundary.
//! They do not introduce a second IR, backend registry, target-selection
//! mechanism, or resource-management implementation.
//!
//! # Scalability contract
//!
//! 1. Workloads grow with the resources available to the test process.
//! 2. Test workload sizes are test inputs, not language or hardware limits.
//! 3. Every generated test instruction must remain represented in the IR.
//! 4. Successful output must report its actual UTF-8 byte length.
//! 5. Repeated compilation of an unchanged input must produce identical
//!    output when the selected backend is deterministic.
//! 6. Empty, small, and multi-function modules must not require a minimum
//!    artificial workload size.
//! 7. Tests must not assume that every target supports every IR operation.
//! 8. Backend tests must not require physical quantum devices, GPUs,
//!    FPGAs, network clusters, or other optional execution hardware.
//!
//! # Integration prerequisites
//!
//! The repository currently has both `src/backend.rs` and
//! `src/backend/mod.rs`. Resolve that module-layout collision before
//! registering this file in the canonical backend module tree.
//!
//! Once the module layout is resolved, register this file through the
//! appropriate test module declaration. Do not add duplicate public
//! backend implementations to make these tests compile.
//!
//! # Resource policy
//!
//! The workloads below are finite regression workloads selected to keep
//! routine CI practical. They do not establish a maximum supported
//! instruction count, function count, output size, memory size, or
//! compilation duration.
//!
//! Allocation failure, process termination, and operating-system resource
//! exhaustion cannot all be converted into ordinary Rust test failures.
//! Production resource-budget enforcement belongs in the backend and
//! compiler APIs; these tests must not pretend otherwise.

use crate::backend::CodeGenerator;
use crate::compiler_types::{CompilationTarget, CompilerConfig, OptimizationLevel};
use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType};

/// Builds a module containing exactly `comment_count` comment instructions
/// distributed across a single function.
///
/// Comments are used because they exercise IR traversal and backend output
/// growth without introducing target-specific arithmetic or memory
/// semantics into a generic scalability test.
fn comment_module(module_name: &str, comment_count: usize) -> IrModule {
    let mut module = IrModule::new(module_name);

    let mut function = IrFunction::new("scalability_probe", Vec::new(), IrType::Void);

    for index in 0..comment_count {
        function.push(IrInstruction::Comment(format!(
            "scalability-probe-{index}"
        )));
    }

    function.push(IrInstruction::Ret(None));
    module.add_function(function);

    module
}

/// Builds a module with instructions distributed across multiple functions.
///
/// Each function receives a distinct marker. This lets the tests detect
/// accidental omission, reordering, or cross-function mixing in generated
/// output without relying on a particular machine's capacity.
fn multi_function_module(
    module_name: &str,
    function_count: usize,
    comments_per_function: usize,
) -> IrModule {
    let mut module = IrModule::new(module_name);

    for function_index in 0..function_count {
        let mut function = IrFunction::new(
            format!("scalability_function_{function_index}"),
            Vec::new(),
            IrType::Void,
        );

        for instruction_index in 0..comments_per_function {
            function.push(IrInstruction::Comment(format!(
                "function-{function_index}-instruction-{instruction_index}"
            )));
        }

        function.push(IrInstruction::Ret(None));
        module.add_function(function);
    }

    module
}

/// Creates a deliberately explicit configuration for the existing x86-64
/// backend.
///
/// This tests the current backend API, not a future registry or capability
/// negotiation API. Target-independent selection tests should be added when
/// that contract exists in the repository.
fn x86_config() -> CompilerConfig {
    CompilerConfig {
        target: CompilationTarget::X86_64Linux,
        opt_level: OptimizationLevel::None,
        debug_info: false,
        verify: true,
        emit_ir: false,
        parallel: false,
    }
}

fn generate_x86(module: &IrModule) -> crate::backend::CodeGenOutput {
    CodeGenerator::new(x86_config())
        .generate(module)
        .expect("the existing x86-64 backend should generate this comment-only IR")
}

#[test]
fn tiny_module_is_supported() {
    let module = comment_module("tiny_scalability_module", 0);

    assert_eq!(
        module.instruction_count(),
        1,
        "the terminating return is an IR instruction"
    );

    let output = generate_x86(&module);

    assert_eq!(output.target, "x86_64");
    assert_eq!(output.size_bytes, output.source.len());
    assert!(
        output.source.contains("scalability_probe"),
        "generated output must retain the function identity"
    );
}

#[test]
fn generated_instruction_count_matches_the_requested_workload() {
    // These are deliberately varied test workloads, not supported-capacity
    // boundaries. Their purpose is to detect dropped or duplicated work.
    let workload_sizes = [0_usize, 1, 7, 31, 127, 509];

    for comment_count in workload_sizes {
        let module = comment_module("instruction_count_module", comment_count);

        assert_eq!(
            module.instruction_count(),
            comment_count + 1,
            "all comments and the return must remain in the IR; \
             comment_count={comment_count}"
        );

        let function = module
            .functions
            .first()
            .expect("the test helper always creates one function");

        assert_eq!(
            function.body.len(),
            comment_count + 1,
            "the function body must preserve every requested instruction"
        );
    }
}

#[test]
fn increasing_workloads_preserve_all_instruction_markers() {
    let workload_sizes = [1_usize, 16, 128, 1024];

    for comment_count in workload_sizes {
        let module = comment_module("marker_preservation_module", comment_count);
        let output = generate_x86(&module);

        for index in 0..comment_count {
            let marker = format!("scalability-probe-{index}");

            assert!(
                output.source.contains(&marker),
                "backend output lost {marker} at workload size {comment_count}"
            );
        }

        assert!(
            output.source.contains("scalability_probe:"),
            "generated output must include the function entry label"
        );

        assert_eq!(output.size_bytes, output.source.len());
    }
}

#[test]
fn multiple_functions_preserve_function_and_instruction_identity() {
    let function_count = 19;
    let comments_per_function = 23;

    let module = multi_function_module(
        "multi_function_scalability_module",
        function_count,
        comments_per_function,
    );

    assert_eq!(module.functions.len(), function_count);

    assert_eq!(
        module.instruction_count(),
        function_count * (comments_per_function + 1),
        "each function contains its comments and a terminating return"
    );

    let output = generate_x86(&module);

    for function_index in 0..function_count {
        assert!(
            output
                .source
                .contains(&format!("scalability_function_{function_index}:")),
            "missing function {function_index}"
        );

        for instruction_index in 0..comments_per_function {
            let marker =
                format!("function-{function_index}-instruction-{instruction_index}");

            assert!(
                output.source.contains(&marker),
                "missing instruction marker {marker}"
            );
        }
    }

    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn output_size_is_measured_in_bytes_not_characters() {
    let mut module = IrModule::new("utf8_scalability_module");
    let mut function =
        IrFunction::new("utf8_probe", Vec::new(), IrType::Void);

    // UTF-8 makes this a useful regression test for byte accounting.
    // It does not assert that arbitrary comment text is valid source syntax.
    function.push(IrInstruction::Comment(
        "unicode-marker: λ 漢字".to_owned(),
    ));
    function.push(IrInstruction::Ret(None));
    module.add_function(function);

    let output = generate_x86(&module);

    assert!(output.source.contains("unicode-marker: λ 漢字"));
    assert_eq!(
        output.size_bytes,
        output.source.as_bytes().len(),
        "size_bytes must report encoded bytes"
    );
}

#[test]
fn repeated_compilation_of_the_same_module_is_deterministic() {
    let module = multi_function_module(
        "determinism_scalability_module",
        11,
        37,
    );

    let first = generate_x86(&module);
    let second = generate_x86(&module);

    assert_eq!(first.target, second.target);
    assert_eq!(first.extension, second.extension);
    assert_eq!(first.source, second.source);
    assert_eq!(first.size_bytes, second.size_bytes);
    assert_eq!(first.size_bytes, first.source.len());
}

#[test]
fn independently_constructed_equivalent_modules_generate_identical_output() {
    let first_module = comment_module("equivalent_module", 64);
    let second_module = comment_module("equivalent_module", 64);

    let first = generate_x86(&first_module);
    let second = generate_x86(&second_module);

    assert_eq!(first_module.instruction_count(), second_module.instruction_count());
    assert_eq!(first.source, second.source);
    assert_eq!(first.size_bytes, second.size_bytes);
}

#[test]
fn larger_ir_produces_non_decreasing_output_for_comment_only_workloads() {
    let workload_sizes = [0_usize, 1, 8, 64, 256, 1024];

    let mut previous_size = None;

    for comment_count in workload_sizes {
        let module = comment_module("output_growth_module", comment_count);
        let output = generate_x86(&module);

        if let Some(previous) = previous_size {
            assert!(
                output.size_bytes >= previous,
                "adding comment instructions must not reduce output size: \
                 previous={previous}, current={}, comments={comment_count}",
                output.size_bytes
            );
        }

        previous_size = Some(output.size_bytes);
    }
}

#[test]
fn empty_module_is_handled_without_artificial_minimum_function_count() {
    let module = IrModule::new("empty_scalability_module");

    assert_eq!(module.functions.len(), 0);
    assert_eq!(module.instruction_count(), 0);

    let output = generate_x86(&module);

    assert_eq!(output.target, "x86_64");
    assert_eq!(output.size_bytes, output.source.len());
}

#[test]
fn generated_ir_text_preserves_growth_before_backend_lowering() {
    let workload_sizes = [0_usize, 1, 10, 100, 500];

    let mut previous_length = None;

    for comment_count in workload_sizes {
        let module = comment_module("ir_growth_module", comment_count);
        let ir_text = module.to_ir_string();

        assert!(
            ir_text.contains("scalability_probe"),
            "IR serialization must retain the function"
        );

        for index in 0..comment_count {
            assert!(
                ir_text.contains(&format!("scalability-probe-{index}")),
                "IR serialization lost comment {index}"
            );
        }

        if let Some(previous) = previous_length {
            assert!(
                ir_text.len() >= previous,
                "serialized IR length must not shrink as comments are added"
            );
        }

        previous_length = Some(ir_text.len());
    }
}

#[test]
fn instruction_growth_across_functions_is_accounted_for_exactly() {
    let function_counts = [1_usize, 2, 5, 13];

    for function_count in function_counts {
        let comments_per_function = 17;

        let module = multi_function_module(
            "distributed_instruction_growth",
            function_count,
            comments_per_function,
        );

        let expected = function_count * (comments_per_function + 1);

        assert_eq!(
            module.instruction_count(),
            expected,
            "instruction accounting must include all functions"
        );

        assert!(
            module.functions.iter().all(|function| {
                function.body.len() == comments_per_function + 1
            }),
            "every generated function must preserve its complete body"
        );
    }
}

#[test]
fn repeated_generation_does_not_mutate_the_input_ir() {
    let module = multi_function_module(
        "immutable_input_module",
        7,
        29,
    );

    let before_ir = module.to_ir_string();
    let before_count = module.instruction_count();
    let before_functions = module.functions.len();

    let _first = generate_x86(&module);
    let _second = generate_x86(&module);

    assert_eq!(module.to_ir_string(), before_ir);
    assert_eq!(module.instruction_count(), before_count);
    assert_eq!(module.functions.len(), before_functions);
}

#[test]
fn generated_output_contains_each_function_marker_once() {
    let function_count = 9;
    let module = multi_function_module(
        "unique_function_markers",
        function_count,
        3,
    );

    let output = generate_x86(&module);

    for function_index in 0..function_count {
        let marker = format!("scalability_function_{function_index}:");

        assert_eq!(
            output.source.matches(&marker).count(),
            1,
            "each function entry label must appear exactly once"
        );
    }
}

#[test]
fn test_workload_sizes_do_not_define_backend_capacity_limits() {
    // This is an architectural guard expressed through test design:
    // a larger workload is another ordinary input, not a new target limit.
    //
    // Avoid tests such as `assert!(count <= MAX_INSTRUCTIONS)` or a fixed
    // maximum function count. Those would encode artificial language limits
    // rather than verify scalable behaviour.

    let small = comment_module("capacity_independent_module", 3);
    let larger = comment_module("capacity_independent_module", 300);

    assert!(larger.instruction_count() > small.instruction_count());

    let small_output = generate_x86(&small);
    let larger_output = generate_x86(&larger);

    assert!(larger_output.size_bytes > small_output.size_bytes);
    assert_eq!(small_output.size_bytes, small_output.source.len());
    assert_eq!(larger_output.size_bytes, larger_output.source.len());
}
