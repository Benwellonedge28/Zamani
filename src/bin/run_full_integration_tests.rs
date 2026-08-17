use zamani_compiler::{compiler_types::{CompilerConfig, CompilationTarget}, backend::CodeGenerator, ir_gen::{IrModule, IrFunction, IrType, IrInstruction, IrValue}};

fn main() {
    println!("======================================================================");
    println!("Zamani Universal IR Hub — Comprehensive Integration Test Suite");
    println!("======================================================================");

    let sample_targets = vec![
        "llvm", "qir", "mlir", "spirv", "ebpf",
        "wasm", "onnx", "solidity", "verilog_structural", "riscv_vector",
        "cuda_ptx", "tflite", "graphql", "protobuf", "dockerfile",
        "c_minus_minus", "coq_gallina", "dafny", "firrtl", "ghc_core",
        "hlo", "rust_mir", "stablehlo", "tvm_tir", "swift_sil",
        "aig", "bolt_ir", "boogie", "capnproto", "cil",
        "dex", "gimple", "haxe_ir", "lua", "ocaml_lambda",
        "abb_rapid", "abc_notation", "vrml", "midi", "latex"
    ];

    let mut success_count = 0;
    let mut failure_count = 0;

    let mut module = IrModule::new("integration_test_mod");
    let mut func = IrFunction::new("main", vec![], IrType::I32);
    func.push(IrInstruction::Comment("Integration test execution".into()));
    func.push(IrInstruction::Ret(Some(IrValue::ConstInt(100, IrType::I32))));
    module.add_function(func);

    for target in &sample_targets {
        let mut config = CompilerConfig::default();
        config.target = CompilationTarget::UniversalIRExport(target.to_string());
        let code_gen = CodeGenerator::new(config);

        match code_gen.generate(&module) {
            Ok(output) => {
                if !output.source.is_empty() {
                    println!(
                        "[PASS] Target '{:<20}' -> Generated {} bytes",
                        target, output.size_bytes
                    );
                    success_count += 1;
                } else {
                    println!("[FAIL] Target '{:<20}' -> Generated empty output", target);
                    failure_count += 1;
                }
            }
            Err(e) => {
                println!("[FAIL] Target '{:<20}' -> Error: {:?}", target, e);
                failure_count += 1;
            }
        }
    }

    println!("======================================================================");
    println!(
        "Test Summary: Total: {}, Passed: {}, Failed: {}",
        success_count + failure_count,
        success_count,
        failure_count
    );
    println!("======================================================================");

    if failure_count > 0 {
        std::process::exit(1);
    }
}
