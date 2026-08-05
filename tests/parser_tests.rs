#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unreachable_code,
    unused_comparisons
)]

//! Zamani Parser — Comprehensive Integration Tests

use std::sync::Arc;
use zamani_compiler::ast::{Expression, Literal, Program, Statement};
use zamani_compiler::lexer::{Lexer, TokenType};
use zamani_compiler::parser::Parser;
use zamani_compiler::source_map::{FileId, SourceFile};

fn make_parser(source: &str) -> Parser {
    let sf = Arc::new(SourceFile::new("<test>".to_string(), source.to_string()));
    let lex = Lexer::new(FileId::new(1), sf);
    Parser::new(lex)
}

fn parse(source: &str) -> Program {
    let mut p = make_parser(source);
    let prog = p.parse_program();
    assert!(
        p.get_errors().is_empty(),
        "Unexpected parse errors for {:?}: {:?}",
        source,
        p.get_errors()
    );
    prog
}

// ── Let bindings ──────────────────────────────────────────────────────────────

#[test]
fn test_parse_let_integer() {
    let prog = parse("let x = 42;");
    assert_eq!(prog.statements.len(), 1);
    match &prog.statements[0] {
        Statement::Let(_, name, _, expr) => {
            assert_eq!(name, "x");
            assert!(matches!(expr, Expression::Literal(Literal::Integer(42, _))));
        }
        s => panic!("Expected Let, got {:?}", s),
    }
}

#[test]
fn test_parse_let_float() {
    let prog = parse("let pi = 3.14;");
    match &prog.statements[0] {
        Statement::Let(_, name, _, Expression::Literal(Literal::Float(_, _))) => {
            assert_eq!(name, "pi");
        }
        s => panic!("Expected Let float, got {:?}", s),
    }
}

#[test]
fn test_parse_let_string() {
    let prog = parse(r#"let msg = "hello";"#);
    match &prog.statements[0] {
        Statement::Let(_, name, _, Expression::Literal(Literal::String(s, _))) => {
            assert_eq!(name, "msg");
        }
        s => panic!("Expected Let string, got {:?}", s),
    }
}

#[test]
fn test_parse_let_boolean_true() {
    let prog = parse("let flag = true;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Literal(Literal::Boolean(b, _))) => {
            assert!(*b, "Expected true");
        }
        s => panic!("Expected Let bool, got {:?}", s),
    }
}

#[test]
fn test_parse_let_boolean_false() {
    let prog = parse("let flag = false;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Literal(Literal::Boolean(b, _))) => {
            assert!(!b, "Expected false");
        }
        _ => panic!("Expected Let bool false"),
    }
}

// ── Arithmetic expressions ────────────────────────────────────────────────────

#[test]
fn test_parse_addition() {
    let prog = parse("let r = 1 + 2;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Infix(_, _, op, _)) => {
            assert_eq!(*op, TokenType::Plus);
        }
        s => panic!("Expected infix add, got {:?}", s),
    }
}

#[test]
fn test_parse_subtraction() {
    let prog = parse("let r = 10 - 3;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Infix(_, _, op, _)) => {
            assert_eq!(*op, TokenType::Minus);
        }
        _ => panic!("Expected infix sub"),
    }
}

#[test]
fn test_parse_multiplication() {
    let prog = parse("let r = 4 * 5;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Infix(_, _, op, _)) => {
            assert_eq!(*op, TokenType::Star);
        }
        _ => panic!("Expected infix mul"),
    }
}

#[test]
fn test_parse_division() {
    let prog = parse("let r = 8 / 2;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Infix(_, _, op, _)) => {
            assert_eq!(*op, TokenType::Slash);
        }
        _ => panic!("Expected infix div"),
    }
}

// ── Function declarations ─────────────────────────────────────────────────────

#[test]
fn test_parse_function_no_params() {
    let prog = parse("fn greet() { let x = 1; }");
    match &prog.statements[0] {
        Statement::Function(_, name, params, _, _) => {
            assert_eq!(name, "greet");
            assert!(params.is_empty());
        }
        s => panic!("Expected Function, got {:?}", s),
    }
}

#[test]
fn test_parse_function_with_params() {
    let prog = parse("fn add(a, b) { let r = a + b; }");
    match &prog.statements[0] {
        Statement::Function(_, name, params, _, _) => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
        }
        s => panic!("Expected Function with params, got {:?}", s),
    }
}

#[test]
fn test_parse_function_return() {
    let prog = parse("fn double(x) { return x; }");
    match &prog.statements[0] {
        Statement::Function(_, name, params, _, body) => {
            assert_eq!(name, "double");
        }
        s => panic!("Expected Function with return, got {:?}", s),
    }
}

// ── Control flow ──────────────────────────────────────────────────────────────

#[test]
fn test_parse_if_else() {
    let prog = parse("if x { let a = 1; } else { let b = 2; }");
    match &prog.statements[0] {
        Statement::Expression(Expression::If(_, _, _, else_branch)) => {
            assert!(else_branch.is_some(), "Expected else branch");
        }
        s => panic!("Expected If expression, got {:?}", s),
    }
}

#[test]
fn test_parse_while_loop() {
    let prog = parse("while true { let x = 1; }");
    assert!(matches!(prog.statements[0], Statement::While(_, _, _)));
}

#[test]
fn test_parse_for_loop() {
    let prog = parse("for item in collection { let x = item; }");
    match &prog.statements[0] {
        Statement::For(_, var, _, _) => assert_eq!(var.0, "item"),
        s => panic!("Expected For, got {:?}", s),
    }
}

#[test]
fn test_parse_break_continue() {
    let prog = parse("break; continue;");
    assert!(matches!(prog.statements[0], Statement::Break(_)));
    assert!(matches!(prog.statements[1], Statement::Continue(_)));
}

// ── Zamani-specific ───────────────────────────────────────────────────────────

#[test]
fn test_parse_quantum_circuit() {
    let prog = parse("quantum circuit Bell { let q = 1; }");
    match &prog.statements[0] {
        Statement::QuantumCircuit(_, name, _) => assert_eq!(name, "Bell"),
        s => panic!("Expected QuantumCircuit, got {:?}", s),
    }
}

#[test]
fn test_parse_sankofa_remember() {
    let prog = parse("remember mem_past = 42;");
    match &prog.statements[0] {
        Statement::SankofaMemory(_, name, _) => assert_eq!(name, "mem_past"),
        s => panic!("Expected SankofaMemory, got {:?}", s),
    }
}

#[test]
fn test_parse_multiple_statements() {
    let prog = parse("let a = 1; let b = 2; let c = 3;");
    assert_eq!(prog.statements.len(), 3);
}

#[test]
fn test_parse_nested_function_call() {
    let prog = parse("let r = f(g(1));");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::Call(_, func, args)) => {
            assert_eq!(args.len(), 1);
            assert!(matches!(&args[0], Expression::Call(_, _, _)));
        }
        s => panic!("Expected nested call, got {:?}", s),
    }
}

#[test]
fn test_parse_member_access() {
    let prog = parse("let v = obj.field;");
    match &prog.statements[0] {
        Statement::Let(_, _, _, Expression::MemberAccess(_, _, member)) => {
            assert_eq!(member.0, "field");
        }
        s => panic!("Expected MemberAccess, got {:?}", s),
    }
}
