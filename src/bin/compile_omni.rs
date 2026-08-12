use std::env;
use std::fs;
use zamani::compile;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let source_path = &args[1];
    let source = fs::read_to_string(source_path).expect("Failed to read source file");

    println!("--- Compiling Zamani Source: {} ---", source_path);
    match compile(&source) {
        Ok(module) => {
            println!("Compilation Successful!");
            println!("\n--- Generated Zamani LLVM IR ---");
            println!("{}", module.to_ir_string());
        }
        Err(errors) => {
            eprintln!("Compilation Failed with {} errors:", errors.len());
            for err in errors {
                eprintln!("  {}", err);
            }
            std::process::exit(1);
        }
    }
}
