use zamani_compiler::{compiler_types::{CompilerConfig, CompilationTarget}, backend::CodeGenerator, ir_gen::{IrModule, IrFunction, IrType, IrInstruction, IrValue}};

fn main() {
    let mut config = CompilerConfig::default();
    config.target = CompilationTarget::UniversalIRExport("vrml".to_string());

    let code_gen = CodeGenerator::new(config);
    let mut module = IrModule::new("test_mod");
    let mut func = IrFunction::new("main", vec![], IrType::I32);
    func.push(IrInstruction::Ret(Some(IrValue::ConstInt(42, IrType::I32))));
    module.add_function(func);

    match code_gen.generate(&module) {
        Ok(output) => {
            println!("Target: {}", output.target);
            println!("Generated Output:\n{}", output.source);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
