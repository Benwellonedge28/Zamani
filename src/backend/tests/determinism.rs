
 //! Backend determinism contract tests.
 //!
 //! Path: src/backend/tests/determinism.rs
 //!
 //! Purpose:
 //! Verify that repeated backend generation produces identical output for
 //! identical IR and configuration, and that cloning the input does not
 //! change the generated result.
 //!
 //! Ownership:
 //! These tests verify backend output determinism only. They do not claim
 //! semantic correctness, target compatibility, valid machine code, or
 //! successful execution.
 //!
 //! Integration:
 //! - src/backend.rs: CodeGenerator and CodeGenOutput
 //! - src/ir_gen.rs: IrModule, IrFunction, IrInstruction, IrType, IrValue
 //! - src/compiler_types.rs: CompilationTarget and CompilerConfig
 //!
 //! Safety: safe Rust only; no unsafe code.
 //!
 //! Important:
 //! The legacy LLVM backend writes files and prints to the console.
 //! It is deliberately excluded from these side-effect-free determinism
 //! tests until its output and side-effect contracts have been repaired.
 //! LLVM needs dedicated tests using an explicit artifact destination.
 //!
 //! Universal IR exporters are also excluded because their determinism
 //! depends on each exporter's contract and implementation.

use crate::backend::CodeGenerator;
use crate::compiler_types::{CompilationTarget, CompilerConfig};
use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrType, IrValue};

/// Construct a small, reproducible IR input using the existing public API.
///
/// The fixture is deliberately target-independent. Target-specific lowering
/// correctness is tested separately from repeatability.
fn fixture_module() -> IrModule {
    let mut module = IrModule::new("determinism_fixture");

    let mut main = IrFunction::new("main", Vec::new(), IrType::I64);

    let result = crate::ir_gen::IrRegister::new("result", IrType::I64);

    main.push(IrInstruction::Add(
        result.clone(),
        IrValue::ConstInt(19, IrType::I64),
        IrValue::ConstInt(23, IrType::I64),
    ));

    main.push(IrInstruction::Comment(
        "determinism regression fixture".to_owned(),
    ));

    main.push(IrInstruction::Ret(Some(IrValue::Reg(result))));

    module.add_function(main);
    module
}

/// Produce an input containing a caller-selected number of comments.
///
/// The count is a test workload parameter, not a language-wide or hardware
/// limit. The test does not interpret it as a maximum supported module size.
fn fixture_module_with_workload(instruction_count: usize) -> IrModule {
    let mut module = IrModule::new("determinism_workload");

    let mut main = IrFunction::new("main", Vec::new(), IrType::I64);

    for index in 0..instruction_count {
        main.push(IrInstruction::Comment(format!(
            "generated deterministic workload item {index}"
        )));
    }

    main.push(IrInstruction::Ret(Some(IrValue::ConstInt(
        0,
        IrType::I64,
    ))));

    module.add_function(main);
    module
}

/// Targets whose legacy generation path does not intentionally write an
/// artifact to a fixed filesystem location.
///
/// Keep this list aligned with the existing CompilationTarget API. A new
/// target should be added after its output and side-effect contracts have
/// been reviewed.
fn deterministic_text_targets() -> Vec<CompilationTarget> {
    vec![
        CompilationTarget::X86_64Linux,
        CompilationTarget::Wasm32,
        CompilationTarget::QASM,
        CompilationTarget::NanoControl,
        CompilationTarget::MTSBytecode,
        CompilationTarget::RiscV,
    ]
}

fn config_for(target: CompilationTarget) -> CompilerConfig {
    CompilerConfig {
        target,
        ..CompilerConfig::default()
    }
}

#[test]
fn repeated_generation_produces_identical_output() {
    let module = fixture_module();

    for target in deterministic_text_targets() {
        let config = config_for(target);

        let first = CodeGenerator::new(config.clone())
            .generate(&module)
            .unwrap_or_else(|error| {
                panic!(
                    "first generation failed for target {:?}: {}",
                    config.target, error.message
                )
            });

        let second = CodeGenerator::new(config.clone())
            .generate(&module)
            .unwrap_or_else(|error| {
                panic!(
                    "second generation failed for target {:?}: {}",
                    config.target, error.message
                )
            });

        assert_eq!(
            first.target, second.target,
            "target identity changed between repeated generations for {:?}",
            config.target
        );

        assert_eq!(
            first.extension, second.extension,
            "artifact extension changed between repeated generations for {:?}",
            config.target
        );

        assert_eq!(
            first.source, second.source,
            "generated output changed between repeated generations for {:?}",
            config.target
        );

        assert_eq!(
            first.size_bytes, second.size_bytes,
            "reported output size changed between repeated generations for {:?}",
            config.target
        );

        assert_eq!(
            first.size_bytes,
            first.source.len(),
            "reported size must equal the UTF-8 byte length for {:?}",
            config.target
        );
    }
}

#[test]
fn cloned_ir_produces_identical_output() {
    let original = fixture_module();
    let cloned = original.clone();

    assert_eq!(
        original.to_ir_string(),
        cloned.to_ir_string(),
        "cloning the IR must preserve its serialized representation"
    );

    for target in deterministic_text_targets() {
        let config = config_for(target);

        let original_output = CodeGenerator::new(config.clone())
            .generate(&original)
            .unwrap_or_else(|error| {
                panic!(
                    "generation of original IR failed for {:?}: {}",
                    config.target, error.message
                )
            });

        let cloned_output = CodeGenerator::new(config.clone())
            .generate(&cloned)
            .unwrap_or_else(|error| {
                panic!(
                    "generation of cloned IR failed for {:?}: {}",
                    config.target, error.message
                )
            });

        assert_eq!(
            original_output.target, cloned_output.target,
            "cloning IR changed target identity for {:?}",
            config.target
        );

        assert_eq!(
            original_output.extension, cloned_output.extension,
            "cloning IR changed the output extension for {:?}",
            config.target
        );

        assert_eq!(
            original_output.source, cloned_output.source,
            "cloning IR changed generated output for {:?}",
            config.target
        );

        assert_eq!(
            original_output.size_bytes, cloned_output.size_bytes,
            "cloning IR changed reported output size for {:?}",
            config.target
        );
    }
}

#[test]
fn independent_generators_produce_identical_output() {
    let module = fixture_module();

    for target in deterministic_text_targets() {
        let first_generator = CodeGenerator::new(config_for(target.clone()));
        let second_generator = CodeGenerator::new(config_for(target.clone()));

        let first = first_generator
            .generate(&module)
            .unwrap_or_else(|error| {
                panic!(
                    "first independent generator failed for {:?}: {}",
                    target, error.message
                )
            });

        let second = second_generator
            .generate(&module)
            .unwrap_or_else(|error| {
                panic!(
                    "second independent generator failed for {:?}: {}",
                    target, error.message
                )
            });

        assert_eq!(
            first.source, second.source,
            "independent generator instances disagree for {:?}",
            target
        );

        assert_eq!(
            first.target, second.target,
            "independent generator instances disagree on target for {:?}",
            target
        );

        assert_eq!(
            first.extension, second.extension,
            "independent generator instances disagree on extension for {:?}",
            target
        );
    }
}

#[test]
fn generation_does_not_mutate_the_input_ir() {
    let module = fixture_module();
    let before = module.to_ir_string();
    let before_instruction_count = module.instruction_count();

    for target in deterministic_text_targets() {
        let config = config_for(target.clone());

        CodeGenerator::new(config)
            .generate(&module)
            .unwrap_or_else(|error| {
                panic!(
                    "generation failed for {:?}: {}",
                    target, error.message
                )
            });

        assert_eq!(
            module.to_ir_string(),
            before,
            "generation mutated the IR for {:?}",
            target
        );

        assert_eq!(
            module.instruction_count(),
            before_instruction_count,
            "generation changed the instruction count for {:?}",
            target
        );
    }
}

#[test]
fn generated_workloads_are_repeatable_at_multiple_sizes() {
    // These are representative test workloads, not maximum-size limits.
    // Extend the workload through stress tests and caller-configured budgets
    // instead of treating any value here as a universal capacity ceiling.
    let workloads = [0, 1, 7, 31];

    for instruction_count in workloads {
        let module = fixture_module_with_workload(instruction_count);

        assert_eq!(
            module.instruction_count(),
            instruction_count + 1,
            "fixture instruction count is incorrect for workload {instruction_count}"
        );

        for target in deterministic_text_targets() {
            let config = config_for(target.clone());

            let first = CodeGenerator::new(config.clone())
                .generate(&module)
                .unwrap_or_else(|error| {
                    panic!(
                        "first generation failed for {:?}, workload {}: {}",
                        target, instruction_count, error.message
                    )
                });

            let second = CodeGenerator::new(config)
                .generate(&module)
                .unwrap_or_else(|error| {
                    panic!(
                        "second generation failed for {:?}, workload {}: {}",
                        target, instruction_count, error.message
                    )
                });

            assert_eq!(
                first.source, second.source,
                "output was not deterministic for {:?}, workload {}",
                target, instruction_count
            );

            assert_eq!(
                first.size_bytes,
                first.source.len(),
                "reported size is inconsistent for {:?}, workload {}",
                target, instruction_count
            );
        }
    }
}

#[test]
fn repeated_ir_serialization_is_stable() {
    let module = fixture_module();

    let first = module.to_ir_string();
    let second = module.to_ir_string();
    let third = module.to_ir_string();

    assert_eq!(first, second);
    assert_eq!(second, third);
}

#[test]
fn repeated_generation_preserves_the_selected_configuration() {
    for target in deterministic_text_targets() {
        let config = config_for(target.clone());

        let before_target = config.target.clone();

        let generator = CodeGenerator::new(config.clone());
        let module = fixture_module();

        let first = generator.generate(&module).unwrap_or_else(|error| {
            panic!(
                "first generation failed for {:?}: {}",
                target, error.message
            )
        });

        let second = generator.generate(&module).unwrap_or_else(|error| {
            panic!(
                "second generation failed for {:?}: {}",
                target, error.message
            )
        });

        assert_eq!(
            config.target, before_target,
            "generation changed the caller's configuration"
        );

        assert_eq!(first.source, second.source);
    }
}
