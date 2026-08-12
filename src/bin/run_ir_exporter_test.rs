use zamani_compiler::{compiler, compiler_types::{CompilerConfig, CompilationTarget}};

fn main() {
    let mut config = CompilerConfig::default();
    config.target = CompilationTarget::UniversalIRExport("abb_rapid".to_string());

    let code_gen = zamani_compiler::backend::CodeGenerator::new(config);
    // Let's test compiling test_sample.zn using the abb_rapid Universal IR exporter
    match zamani_compiler::compiler::compile("test_sample.zn") {
        Ok(bytes) => {
            println!("Universal IR Export successful!");
            println!("{}", String::from_utf8_lossy(&bytes));
        }
        Err(e) => {
            eprintln!("Export failed: {}", e);
        }
    }
}
