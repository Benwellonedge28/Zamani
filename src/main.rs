#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

//! Zenith Universal Meta-Compiler (UMC) - Main Entry Point
//!
//! Entry point for the Zenith compiler CLI. Reads a `.zn` source file,
//! runs it through the real compiler pipeline (lex -> parse -> semantic
//! analysis -> IR generation -> optimization -> codegen), and either
//! writes the generated output to disk or reports diagnostics with
//! file/line/column information.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::Instant;

use zenith_compiler::backend::CodeGenerator;
use zenith_compiler::compiler_types::{CompilationTarget, CompilerConfig, OptimizationLevel};
use zenith_compiler::ir_gen::IrGenerator;
use zenith_compiler::lexer::Lexer;
use zenith_compiler::optimizer::Optimizer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::SemanticAnalyzer;
use zenith_compiler::source_map::{FileId, SourceFile};
use zenith_compiler::VERSION;

struct Args {
    source_file: String,
    target: CompilationTarget,
    opt_level: OptimizationLevel,
    output: Option<String>,
    emit_ir: bool,
    emit_ast: bool,
}

fn print_banner() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  Zenith Universal Meta-Compiler (UMC) v{:<10}║", VERSION);
    println!("║  The Omniversal AGI Language & Operating Intel.   ║");
    println!("╚══════════════════════════════════════════════════╝");
    println!();
}

fn print_help() {
    println!("Zenith UMC — Usage:");
    println!();
    println!("  zenith <file.zn> [options]   Compile a Zenith source file");
    println!("  zenith --version             Print version information");
    println!("  zenith --help                Show this help message");
    println!();
    println!("Options:");
    println!("  -o, --output <path>          Write generated code to <path>");
    println!("                                (default: <file> with target extension)");
    println!("  -t, --target <target>        Codegen target. One of:");
    println!("                                  llvm (default), x86-64, arm64, wasm32,");
    println!("                                  qasm, riscv, mts, nano");
    println!("  -O, --opt <level>             Optimization level. One of:");
    println!("                                  none, basic (default), aggressive, ultra");
    println!("      --emit-ir                 Print unoptimized IR to stdout before codegen");
    println!("      --emit-ast                Print the parsed AST (debug form) to stdout");
    println!();
    println!("Language features:");
    println!("  Quantum computing primitives  (quantum, circuit keywords)");
    println!("  Nano-agent programming        (nano, @atom, @molecule)");
    println!("  Linear & affine types         (linear, affine keywords)");
    println!("  Algebraic effects             (handle, effect, perform)");
    println!("  Formal verification           (#[prove], invariant, post_condition)");
    println!("  Multi-temporal semantics      (zamani, sasa, ancestor)");
}

fn parse_target(s: &str) -> Option<CompilationTarget> {
    match s.to_ascii_lowercase().as_str() {
        "llvm" | "llvm-ir" | "llvmir" => Some(CompilationTarget::LLVMIR),
        "x86" | "x86-64" | "x86_64" | "x86-64-linux" => Some(CompilationTarget::X86_64Linux),
        "arm64" | "aarch64" => Some(CompilationTarget::Arm64),
        "wasm" | "wasm32" | "wat" => Some(CompilationTarget::Wasm32),
        "qasm" | "quantum" => Some(CompilationTarget::QASM),
        "riscv" | "risc-v" | "rv64" => Some(CompilationTarget::RiscV),
        "mts" | "mts-bytecode" => Some(CompilationTarget::MTSBytecode),
        "nano" | "nanocontrol" => Some(CompilationTarget::NanoControl),
        _ => None,
    }
}

fn parse_opt_level(s: &str) -> Option<OptimizationLevel> {
    match s.to_ascii_lowercase().as_str() {
        "none" | "0" => Some(OptimizationLevel::None),
        "basic" | "1" => Some(OptimizationLevel::Basic),
        "aggressive" | "2" => Some(OptimizationLevel::Aggressive),
        "ultra" | "ultra-agi" | "3" => Some(OptimizationLevel::UltraAGI),
        _ => None,
    }
}

fn opt_level_to_u8(level: &OptimizationLevel) -> u8 {
    match level {
        OptimizationLevel::None => 0,
        OptimizationLevel::Basic => 1,
        OptimizationLevel::Aggressive => 2,
        OptimizationLevel::UltraAGI => 3,
    }
}

fn parse_args(raw: &[String]) -> Result<Args, String> {
    let mut source_file: Option<String> = None;
    let mut target = CompilationTarget::LLVMIR;
    let mut opt_level = OptimizationLevel::Basic;
    let mut output: Option<String> = None;
    let mut emit_ir = false;
    let mut emit_ast = false;

    let mut i = 0;
    while i < raw.len() {
        let arg = &raw[i];
        match arg.as_str() {
            "-o" | "--output" => {
                i += 1;
                let val = raw.get(i).ok_or("--output requires a path argument")?;
                output = Some(val.clone());
            }
            "-t" | "--target" => {
                i += 1;
                let val = raw.get(i).ok_or("--target requires a value")?;
                target = parse_target(val).ok_or_else(|| format!("Unknown target: {}", val))?;
            }
            "-O" | "--opt" => {
                i += 1;
                let val = raw.get(i).ok_or("--opt requires a value")?;
                opt_level =
                    parse_opt_level(val).ok_or_else(|| format!("Unknown opt level: {}", val))?;
            }
            "--emit-ir" => emit_ir = true,
            "--emit-ast" => emit_ast = true,
            other if other.starts_with('-') => {
                return Err(format!("Unknown flag: {}", other));
            }
            other => {
                if source_file.is_some() {
                    return Err(format!("Unexpected extra argument: {}", other));
                }
                source_file = Some(other.to_string());
            }
        }
        i += 1;
    }

    let source_file = source_file.ok_or("No source file provided")?;
    Ok(Args {
        source_file,
        target,
        opt_level,
        output,
        emit_ir,
        emit_ast,
    })
}

fn default_output_path(source_file: &str, extension: &str) -> PathBuf {
    // Backend `file_extension()` impls return a leading-dot form (e.g.
    // ".ll", ".wat") since that's convenient for display purposes.
    // `Path::with_extension` adds its own separating dot, so strip any
    // leading dot here to avoid producing "name..ll".
    let ext = extension.trim_start_matches('.');
    let path = Path::new(source_file);
    path.with_extension(ext)
}

fn run(args: Args) -> Result<(), i32> {
    let start = Instant::now();

    let source_path = Path::new(&args.source_file);
    let source = fs::read_to_string(source_path).map_err(|e| {
        eprintln!(
            "error: could not read source file '{}': {}",
            args.source_file, e
        );
        1
    })?;

    println!("Compiling: {}", args.source_file);
    println!();

    // ── [1/5] Lexical analysis ────────────────────────────────────────────
    println!("[1/5] Lexical analysis...");
    let file_id = FileId::new(1);
    let source_file =
        std::sync::Arc::new(SourceFile::new(args.source_file.clone(), source.clone()));
    let lexer = Lexer::new(file_id, source_file);

    // ── [2/5] Parsing ──────────────────────────────────────────────────────
    println!("[2/5] Parsing...");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let parse_errors = parser.get_errors().clone();
    if !parse_errors.is_empty() {
        eprintln!();
        eprintln!("error: {} parse error(s):", parse_errors.len());
        for e in &parse_errors {
            eprintln!(
                "  {}:{}:{}: {}",
                args.source_file, e.span.start_line, e.span.start_column, e.message
            );
        }
        return Err(1);
    }

    if args.emit_ast {
        println!();
        println!("--- AST ---");
        println!("{:#?}", program);
        println!();
    }

    // ── [3/5] Semantic analysis ────────────────────────────────────────────
    println!("[3/5] Semantic analysis...");
    let mut sem = SemanticAnalyzer::new();
    let sem_errors = sem.analyze(&program);
    if !sem_errors.is_empty() {
        eprintln!();
        eprintln!("error: {} semantic error(s):", sem_errors.len());
        for e in &sem_errors {
            eprintln!(
                "  {}:{}:{}: {}",
                args.source_file, e.span.start_line, e.span.start_column, e.message
            );
        }
        return Err(1);
    }

    // ── [4/5] IR generation & optimization ─────────────────────────────────
    println!("[4/5] IR generation & optimization...");
    let mut ir_gen = IrGenerator::new();
    let module = ir_gen.generate(&program);

    if args.emit_ir {
        println!();
        println!("--- Unoptimized IR ---");
        println!("{}", module.to_ir_string());
    }

    let mut optimizer = Optimizer::with_level(opt_level_to_u8(&args.opt_level));
    let optimized_module = optimizer.optimize(&module);

    // ── [5/5] Code generation ───────────────────────────────────────────────
    println!("[5/5] Code generation...");
    let config = CompilerConfig {
        target: args.target.clone(),
        opt_level: args.opt_level.clone(),
        debug_info: true,
        verify: false,
        emit_ir: args.emit_ir,
        parallel: false,
    };
    let generator = CodeGenerator::new(config);
    let output = generator.generate(&optimized_module).map_err(|e| {
        eprintln!(
            "error: code generation failed for target '{}': {}",
            e.target, e.message
        );
        1
    })?;

    let out_path = args
        .output
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output_path(&args.source_file, &output.extension));
    fs::write(&out_path, &output.source).map_err(|e| {
        eprintln!("error: could not write output file '{:?}': {}", out_path, e);
        1
    })?;

    let elapsed = start.elapsed();
    println!();
    println!("✓ Compilation complete.");
    println!(
        "  target:     {}  ({} functions, {} instructions)",
        output.target,
        optimized_module.functions.len(),
        optimized_module.instruction_count()
    );
    println!(
        "  output:     {} ({} bytes)",
        out_path.display(),
        output.size_bytes
    );
    println!("  opt level:  {:?}", args.opt_level);
    println!("  time:       {:.2?}", elapsed);

    Ok(())
}

fn main() {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    if raw_args.is_empty() {
        print_banner();
        eprintln!("Usage: zenith <source_file.zn> [options]");
        eprintln!("       zenith --version");
        eprintln!("       zenith --help");
        process::exit(1);
    }

    match raw_args[0].as_str() {
        "--version" | "-V" => {
            println!("zenith {} (edition 2026)", VERSION);
            return;
        }
        "--help" | "-h" => {
            print_banner();
            print_help();
            return;
        }
        _ => {}
    }

    print_banner();

    let args = match parse_args(&raw_args) {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("error: {}", msg);
            eprintln!();
            eprintln!("Usage: zenith <source_file.zn> [options]");
            eprintln!("Run `zenith --help` for details.");
            process::exit(1);
        }
    };

    match run(args) {
        Ok(()) => process::exit(0),
        Err(code) => process::exit(code),
    }
}
