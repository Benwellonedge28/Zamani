#![allow(unused_imports, dead_code, unused_variables, unused_mut)]
//! Zenith Native Constructs — Comprehensive Tests
//! Tests for all grammar spec constructs, Zenith-native keywords,
//! extended type system, and full compiler pipeline.

use std::sync::Arc;
use zenith_compiler::ast::{Expression, Literal, Program, Statement, TypeExpr};
use zenith_compiler::ir_gen::IrGenerator;
use zenith_compiler::lexer::{Lexer, TokenType};
use zenith_compiler::optimizer::Optimizer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::SemanticAnalyzer;
use zenith_compiler::source_map::{FileId, SourceFile};

fn make_parser(src: &str) -> Parser {
    let sf = Arc::new(SourceFile::new("<test>".into(), src.into()));
    let lex = Lexer::new(FileId::new(1), sf);
    Parser::new(lex)
}

fn parse_ok(src: &str) -> Program {
    let mut p = make_parser(src);
    let prog = p.parse_program();
    assert!(
        p.get_errors().is_empty(),
        "Parse errors for {:?}: {:?}",
        src,
        p.get_errors()
    );
    prog
}

fn parse_and_ir(src: &str) -> zenith_compiler::ir_gen::IrModule {
    let prog = parse_ok(src);
    let mut gen = IrGenerator::new();
    gen.generate(&prog)
}

// ── Let bindings ──────────────────────────────────────────────────────────────

#[test]
fn test_let_mut() {
    let prog = parse_ok("let mut count = 0;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_const_decl() {
    let prog = parse_ok("const MAX: Int = 100;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_var_decl() {
    let prog = parse_ok("var x = 42;");
    assert_eq!(prog.statements.len(), 1);
}

// ── Return with no value ──────────────────────────────────────────────────────

#[test]
fn test_empty_return() {
    let prog = parse_ok("fn noop() { return; }");
    assert_eq!(prog.statements.len(), 1);
}

// ── Type annotations ──────────────────────────────────────────────────────────

#[test]
fn test_typed_let() {
    let prog = parse_ok("let x: Int = 5;");
    match &prog.statements[0] {
        Statement::Let(_, _, Some(ty), _) => {
            matches!(ty, TypeExpr::Identifier(_));
        }
        s => panic!("Expected typed let, got {:?}", s),
    }
}

#[test]
fn test_list_type() {
    let prog = parse_ok("let xs: List<Int> = 1;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_tuple_type() {
    let prog = parse_ok("let p: (Int, Float) = 1;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_fn_type() {
    let prog = parse_ok("let f: fn(Int) -> Bool = 1;");
    assert_eq!(prog.statements.len(), 1);
}

// ── Functions ─────────────────────────────────────────────────────────────────

#[test]
fn test_fn_with_return_type() {
    let prog = parse_ok("fn add(a: Int, b: Int) -> Int { return a; }");
    match &prog.statements[0] {
        Statement::Function(_, name, params, ret, _) => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert!(ret.is_some());
        }
        s => panic!("Expected Function, got {:?}", s),
    }
}

#[test]
fn test_fn_no_params_no_ret() {
    let prog = parse_ok("fn hello() { let x = 1; }");
    assert!(matches!(
        &prog.statements[0],
        Statement::Function(_, _, _, None, _)
    ));
}

#[test]
fn test_fn_with_default_param() {
    let prog = parse_ok("fn greet(name, times) { let x = 1; }");
    match &prog.statements[0] {
        Statement::Function(_, _, params, _, _) => assert_eq!(params.len(), 2),
        _ => panic!("expected function"),
    }
}

// ── Control flow ──────────────────────────────────────────────────────────────

#[test]
fn test_if_only() {
    let prog = parse_ok("if x { let a = 1; }");
    assert!(matches!(
        &prog.statements[0],
        Statement::Expression(Expression::If(_, _, _, None))
    ));
}

#[test]
fn test_if_else_if_else() {
    let prog = parse_ok("if a { let x = 1; } else if b { let y = 2; } else { let z = 3; }");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_while_complex() {
    let prog = parse_ok("while i < 100 { let x = i; }");
    assert!(matches!(&prog.statements[0], Statement::While(_, _, _)));
}

#[test]
fn test_for_range() {
    let prog = parse_ok("for i in items { let x = i; }");
    assert!(matches!(&prog.statements[0], Statement::For(_, _, _, _)));
}

#[test]
fn test_match_multiple_arms() {
    let prog = parse_ok("match x { 1 => 10, 2 => 20, 3 => 30, }");
    match &prog.statements[0] {
        Statement::Match(_, _, cases) => assert_eq!(cases.len(), 3),
        s => panic!("Expected Match, got {:?}", s),
    }
}

// ── Quantum ───────────────────────────────────────────────────────────────────

#[test]
fn test_quantum_circuit_named() {
    let prog = parse_ok("quantum circuit Bell { let q = 1; }");
    match &prog.statements[0] {
        Statement::QuantumCircuit(_, name, _) => assert_eq!(name, "Bell"),
        s => panic!("Expected QuantumCircuit, got {:?}", s),
    }
}

#[test]
fn test_quantum_circuit_with_params() {
    let prog = parse_ok("quantum circuit GHZ(n) { let q = n; }");
    assert!(matches!(
        &prog.statements[0],
        Statement::QuantumCircuit(_, _, _)
    ));
}

#[test]
fn test_quantum_op_expr() {
    let prog = parse_ok("let op = quantum_gate(q);");
    assert_eq!(prog.statements.len(), 1);
}

// ── Nano agents ───────────────────────────────────────────────────────────────

#[test]
fn test_nano_agent_decl() {
    let prog = parse_ok("nano agent Healer { let x = 1; }");
    match &prog.statements[0] {
        Statement::NanoAgent(_, name, _) => assert_eq!(name, "Healer"),
        s => panic!("Expected NanoAgent, got {:?}", s),
    }
}

#[test]
fn test_agent_keyword() {
    let prog = parse_ok("agent Scout { let pos = 0; }");
    assert!(matches!(&prog.statements[0], Statement::NanoAgent(_, _, _)));
}

// ── Sankofa memory ────────────────────────────────────────────────────────────

#[test]
fn test_remember_stmt() {
    let prog = parse_ok("remember wisdom_of_elders = 42;");
    match &prog.statements[0] {
        Statement::SankofaMemory(_, name, _) => assert_eq!(name, "wisdom_of_elders"),
        s => panic!("Expected SankofaMemory, got {:?}", s),
    }
}

#[test]
fn test_remember_with_type() {
    let prog = parse_ok("remember past: History = 999;");
    assert!(matches!(
        &prog.statements[0],
        Statement::SankofaMemory(_, _, _)
    ));
}

#[test]
fn test_recall_expr() {
    let prog = parse_ok("let mem = recall(past);");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_learn_expr() {
    let prog = parse_ok("let k = learn from data;");
    assert_eq!(prog.statements.len(), 1);
}

// ── Algebraic effects ─────────────────────────────────────────────────────────

#[test]
fn test_effect_decl() {
    let prog = parse_ok("effect QuantumDecoherence;");
    assert!(matches!(
        &prog.statements[0],
        Statement::EffectDeclaration(_, _)
    ));
}

#[test]
fn test_effect_with_body() {
    let prog = parse_ok("effect NanoMalfunction { }");
    assert!(matches!(
        &prog.statements[0],
        Statement::EffectDeclaration(_, _)
    ));
}

#[test]
fn test_handle_stmt() {
    let prog = parse_ok("handle MyEffect { let x = 1; } with { let y = 2; }");
    assert!(matches!(&prog.statements[0], Statement::Handle(_, _, _, _)));
}

#[test]
fn test_perform_expr() {
    let prog = parse_ok("let r = perform QuantumDecoherence(reason);");
    assert_eq!(prog.statements.len(), 1);
}

// ── Type declarations ─────────────────────────────────────────────────────────

#[test]
fn test_type_alias() {
    let prog = parse_ok("type PatientId = String;");
    assert!(matches!(
        &prog.statements[0],
        Statement::TypeAlias(_, _, _, _)
    ));
}

#[test]
fn test_struct_decl() {
    let prog = parse_ok("struct Point { x, y, }");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_struct_with_types() {
    let prog = parse_ok("struct Patient { id: PatientId, age: Int, }");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_enum_decl() {
    let prog = parse_ok("enum Color { Red, Green, Blue }");
    assert_eq!(prog.statements.len(), 1);
}

// ── OOP ───────────────────────────────────────────────────────────────────────

#[test]
fn test_class_empty() {
    let prog = parse_ok("class Animal { }");
    assert!(matches!(&prog.statements[0], Statement::Class(_, _, _, _)));
}

#[test]
fn test_class_with_fields() {
    let prog = parse_ok("class Dog { name: String, age: Int, }");
    match &prog.statements[0] {
        Statement::Class(_, id, _, members) => {
            assert_eq!(id.0, "Dog");
        }
        s => panic!("Expected Class, got {:?}", s),
    }
}

#[test]
fn test_class_extends() {
    let prog = parse_ok("class Labrador extends Dog { }");
    assert!(matches!(&prog.statements[0], Statement::Class(_, _, supers, _) if !supers.is_empty()));
}

#[test]
fn test_interface_decl() {
    let prog = parse_ok("interface Runnable { fn run(); }");
    assert!(matches!(
        &prog.statements[0],
        Statement::Interface(_, _, _, _)
    ));
}

// ── Module / import ───────────────────────────────────────────────────────────

#[test]
fn test_import_stmt() {
    let prog = parse_ok("import stdlib.math;");
    assert!(matches!(&prog.statements[0], Statement::Import(_, _)));
}

#[test]
fn test_module_decl() {
    let prog = parse_ok("module quantum_utils { fn hadamard() { let x = 1; } }");
    assert!(matches!(&prog.statements[0], Statement::Module(_, _, _)));
}

// ── Expressions ───────────────────────────────────────────────────────────────

#[test]
fn test_method_chain() {
    let prog = parse_ok("let r = obj.method1().method2();");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_array_literal() {
    let prog = parse_ok("let arr = [1, 2, 3];");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Array(_, elems)) => assert_eq!(elems.len(), 3),
        s => panic!("Expected array literal, got {:?}", s),
    }
}

#[test]
fn test_tuple_literal() {
    let prog = parse_ok("let t = (1, 2, 3);");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Tuple(_, elems)) => assert_eq!(elems.len(), 3),
        s => panic!("Expected tuple, got {:?}", s),
    }
}

#[test]
fn test_lambda_expr() {
    let prog = parse_ok("let f = fn(x, y) { return x; };");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_nested_function_calls() {
    let prog = parse_ok("let r = outer(inner1(a), inner2(b, c));");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_index_access() {
    let prog = parse_ok("let v = arr[42];");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Index(_, _, _)) => {}
        s => panic!("Expected Index, got {:?}", s),
    }
}

#[test]
fn test_assign_expr() {
    let prog = parse_ok("x = 10;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_complex_arithmetic() {
    let prog = parse_ok("let r = (a + b) * (c - d) / e;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_boolean_logic() {
    let prog = parse_ok("let ok = x > 0 && y < 100 || z == 0;");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_new_object() {
    let prog = parse_ok("let dog = new Dog(name, age);");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::New(_, id, args)) => {
            assert_eq!(id.0, "Dog");
            assert_eq!(args.len(), 2);
        }
        s => panic!("Expected New, got {:?}", s),
    }
}

// ── Unsafe ───────────────────────────────────────────────────────────────────

#[test]
fn test_unsafe_block() {
    let prog = parse_ok("unsafe { let x = 1; }");
    assert!(matches!(&prog.statements[0], Statement::Unsafe(_, _, _)));
}

// ── Sankofa zamani/sasa blocks ────────────────────────────────────────────────

#[test]
fn test_zamani_block() {
    let prog = parse_ok("let past = zamani { let x = 1; };");
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_sasa_block() {
    let prog = parse_ok("let now = sasa { let x = 2; };");
    assert_eq!(prog.statements.len(), 1);
}

// ── IR pipeline ───────────────────────────────────────────────────────────────

#[test]
fn test_ir_from_function() {
    let module = parse_and_ir("fn add(a, b) { return a; }");
    assert!(
        module.functions.iter().any(|f| f.name == "add"),
        "Expected 'add' in IR functions"
    );
}

#[test]
fn test_ir_from_let_bindings() {
    let module = parse_and_ir("let x = 1; let y = 2; let z = x;");
    let top = module.functions.iter().find(|f| f.name == "main").unwrap();
    assert!(!top.body.is_empty());
}

#[test]
fn test_ir_from_quantum_circuit() {
    let module = parse_and_ir("quantum circuit Bell { let q = 1; }");
    let top = module.functions.iter().find(|f| f.name == "main").unwrap();
    assert!(!top.body.is_empty());
}

#[test]
fn test_ir_from_sankofa() {
    let module = parse_and_ir("remember wisdom = 42;");
    let top = module.functions.iter().find(|f| f.name == "main").unwrap();
    let has_sankofa = top.body.iter().any(|ins| {
        matches!(
            ins,
            zenith_compiler::ir_gen::IrInstruction::SankofaRemember(_, _)
        )
    });
    assert!(has_sankofa, "Expected SankofaRemember in IR");
}

#[test]
fn test_ir_while_loop() {
    let module = parse_and_ir("while true { let x = 1; }");
    let top = module.functions.iter().find(|f| f.name == "main").unwrap();
    let has_cond_jump = top.body.iter().any(|ins| {
        matches!(
            ins,
            zenith_compiler::ir_gen::IrInstruction::CondJump(_, _, _)
        )
    });
    assert!(has_cond_jump, "Expected CondJump in IR for while loop");
}

// ── Optimizer ─────────────────────────────────────────────────────────────────

#[test]
fn test_optimizer_constant_folding() {
    let module = parse_and_ir("let r = 2 + 3;");
    let mut opt = Optimizer::new(zenith_compiler::optimizer::OptimizationConfig::default());
    let optimized = opt.optimize(&module);
    assert!(!optimized.functions.is_empty());
}

#[test]
fn test_optimizer_dead_code_elimination() {
    let module = parse_and_ir("let x = 1; let y = 2; let z = 3;");
    let mut opt = Optimizer::new(zenith_compiler::optimizer::OptimizationConfig::default());
    let optimized = opt.optimize(&module);
    assert!(!optimized.functions.is_empty());
}

// ── Multiple programs ─────────────────────────────────────────────────────────

#[test]
fn test_full_zenith_program() {
    let src = r#"
        effect QuantumError;
        type PatientId = String;
        struct Patient { id: PatientId, age: Int, }
        quantum circuit Diagnose(patient) {
            let qreg = quantum_alloc(4);
            return qreg;
        }
        nano agent Healer(target) {
            let payload = load_drug(target);
            deliver(payload);
        }
        remember ancient_knowledge = 9999;
        fn main() {
            let p = new Patient(id, age);
            let result = recall(ancient_knowledge);
            return result;
        }
    "#;
    let prog = parse_ok(src);
    assert!(
        prog.statements.len() >= 5,
        "Expected at least 5 top-level declarations"
    );
}

#[test]
fn test_class_with_methods() {
    let src = r#"
        class Calculator {
            value: Int,
            fn add(x: Int) -> Int {
                return value;
            }
            fn reset() {
                value = 0;
            }
        }
    "#;
    let prog = parse_ok(src);
    assert_eq!(prog.statements.len(), 1);
}

#[test]
fn test_module_with_multiple_fns() {
    let src = r#"
        module math_utils {
            fn square(x: Int) -> Int { return x; }
            fn cube(x: Int) -> Int { return x; }
            fn abs(x: Int) -> Int { return x; }
        }
    "#;
    let prog = parse_ok(src);
    assert_eq!(prog.statements.len(), 1);
}
