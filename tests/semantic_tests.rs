//! Zenith Semantic Analyser — Comprehensive Integration Tests

use std::sync::Arc;
use zenith_compiler::compiler_types::{FloatWidth, IntWidth, Type};
use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::SemanticAnalyzer;
use zenith_compiler::source_map::{FileId, SourceFile};

fn analyze(source: &str) -> (Vec<String>, SemanticAnalyzer) {
    let sf = Arc::new(SourceFile::new("<test>".to_string(), source.to_string()));
    let lex = Lexer::new(FileId::new(1), sf);
    let mut parser = Parser::new(lex);
    let prog = parser.parse_program();
    let mut sem = SemanticAnalyzer::new();
    let errs = sem.analyze(&prog);
    (errs.iter().map(|e| e.message.clone()).collect(), sem)
}

fn analyze_ok(source: &str) -> SemanticAnalyzer {
    let (errs, sem) = analyze(source);
    assert!(
        errs.is_empty(),
        "Expected no semantic errors for {:?}: {:?}",
        source,
        errs
    );
    sem
}

fn analyze_err(source: &str) -> Vec<String> {
    let (errs, _) = analyze(source);
    assert!(
        !errs.is_empty(),
        "Expected semantic errors for {:?}",
        source
    );
    errs
}

// ── Type inference ────────────────────────────────────────────────────────────

#[test]
fn test_infer_integer_literal() {
    let sem = analyze_ok("let x = 42;");
    match sem.symbols.lookup("x") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert_eq!(*ty, Type::Int(IntWidth::I64));
        }
        _ => panic!("Expected variable x with Int type"),
    }
}

#[test]
fn test_infer_float_literal() {
    let sem = analyze_ok("let f = 3.14;");
    match sem.symbols.lookup("f") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert_eq!(*ty, Type::Float(FloatWidth::F64));
        }
        _ => panic!("Expected float variable"),
    }
}

#[test]
fn test_infer_boolean_literal() {
    let sem = analyze_ok("let flag = true;");
    match sem.symbols.lookup("flag") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert_eq!(*ty, Type::Bool);
        }
        _ => panic!("Expected bool variable"),
    }
}

#[test]
fn test_infer_string_literal() {
    let sem = analyze_ok(r#"let s = "hello";"#);
    match sem.symbols.lookup("s") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert_eq!(*ty, Type::String);
        }
        _ => panic!("Expected string variable"),
    }
}

// ── Arithmetic type rules ─────────────────────────────────────────────────────

#[test]
fn test_int_addition_type() {
    let sem = analyze_ok("let r = 1 + 2;");
    match sem.symbols.lookup("r") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert!(ty.is_numeric(), "Expected numeric result");
        }
        _ => panic!("Expected numeric variable r"),
    }
}

#[test]
fn test_bool_comparison_type() {
    let sem = analyze_ok("let b = 1 == 1;");
    match sem.symbols.lookup("b") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert_eq!(*ty, Type::Bool);
        }
        _ => panic!("Expected bool result"),
    }
}

// ── Symbol resolution ─────────────────────────────────────────────────────────

#[test]
fn test_undefined_symbol_error() {
    let errs = analyze_err("let x = undefined_var;");
    assert!(
        errs.iter()
            .any(|e| e.contains("Undefined symbol") || e.contains("undefined_var")),
        "Expected undefined symbol error, got: {:?}",
        errs
    );
}

#[test]
fn test_symbol_shadowing() {
    // Variables can be re-bound in Zenith
    let sem = analyze_ok("let x = 1; let x = 2;");
    match sem.symbols.lookup("x") {
        Some(zenith_compiler::semantic::Symbol::Variable(_)) => {}
        _ => panic!("Expected x to be defined"),
    }
}

// ── Function analysis ─────────────────────────────────────────────────────────

#[test]
fn test_function_registered_in_scope() {
    let sem = analyze_ok("fn greet() { let x = 1; }");
    match sem.symbols.lookup("greet") {
        Some(zenith_compiler::semantic::Symbol::Function(_, _)) => {}
        s => panic!("Expected Function symbol, got {:?}", s),
    }
}

#[test]
fn test_builtin_print_available() {
    let sem = analyze_ok(r#"let r = print("hello");"#);
    // print should be in builtins — no undefined error
    assert!(sem.errors.is_empty());
}

// ── Zenith-specific semantics ─────────────────────────────────────────────────

#[test]
fn test_quantum_circuit_registered() {
    let sem = analyze_ok("quantum circuit Bell { let q = 1; }");
    match sem.symbols.lookup("Bell") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert!(matches!(ty, Type::Quantum(_)), "Expected Quantum type");
        }
        _ => panic!("Expected Bell quantum circuit in scope"),
    }
}

#[test]
fn test_sankofa_memory_type() {
    let sem = analyze_ok("remember mem_ancient = 42;");
    match sem.symbols.lookup("mem_ancient") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert!(matches!(ty, Type::Sankofa(_)), "Expected Sankofa type");
        }
        _ => panic!("Expected ancient Sankofa in scope"),
    }
}

#[test]
fn test_nano_agent_registered() {
    let sem = analyze_ok("agent Scout { let x = 1; }");
    match sem.symbols.lookup("Scout") {
        Some(zenith_compiler::semantic::Symbol::Variable(ty)) => {
            assert!(matches!(ty, Type::Nano(_)), "Expected Nano type");
        }
        _ => panic!("Expected Scout nano in scope"),
    }
}

// ── Control flow type rules ───────────────────────────────────────────────────

#[test]
fn test_multiple_lets_all_typed() {
    let sem = analyze_ok("let a = 1; let b = 2; let c = 3;");
    for name in &["a", "b", "c"] {
        assert!(
            sem.symbols.lookup(name).is_some(),
            "Expected {} in scope",
            name
        );
    }
}

#[test]
fn test_no_errors_on_clean_program() {
    let (errs, _) = analyze("fn compute(x, y) { let sum = x + y; return sum; }");
    // May have unknown-type warnings but no hard errors
    let hard_errors: Vec<_> = errs.iter().filter(|e| e.contains("Undefined")).collect();
    assert!(
        hard_errors.is_empty(),
        "Unexpected hard errors: {:?}",
        hard_errors
    );
}
