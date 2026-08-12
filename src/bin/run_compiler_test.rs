use zamani_compiler::compiler;

fn main() {
    match compiler::compile("test_sample.zn") {
        Ok(bytes) => {
            println!("Compilation successful! Output length: {} bytes", bytes.len());
            let s = String::from_utf8_lossy(&bytes);
            println!("--- Generated Assembly / IR Output ---");
            println!("{}", s);
        }
        Err(e) => {
            eprintln!("Compilation failed: {}", e);
            std::process::exit(1);
        }
    }
}
