
//! Conceptual Tests: Parser
//!
//! This module provides conceptual unit tests for the Zenith Parser.
//! It verifies that the parser correctly constructs the Abstract Syntax Tree (AST)
//! for various Zenith, Sankofa, and Nimbus language constructs, including:
//! - Basic `let` and `return` statements
//! - Function declarations
//! - Classical, Quantum, Nano-agent, and MTS expressions
//! - Control flow statements (`if`, `while`, `for`, `match`)
//! - Zenith-specific statements like `quantum circuit`, `nano agent`, `remember`, `effect`, `handle`.
//! - Type expressions including generics, arrays, dependent types, linear/affine, and effectful types.

use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::{Parser, ParserError};
use zenith_compiler::ast::{Program, Statement, Expression, Literal, Identifier, TypeExpr, Parameter, MatchCase};
use zenith_compiler::tokens::{Span, BytePos, TokenType};
use zenith_compiler::source_map::FileId;

// Helper function for creating a FileId for tests
fn test_file_id() -> FileId { FileId::new(1) }

// Helper to create dummy Span for comparisons where exact span details don't matter
fn span_for_test(start: u32, end: u32) -> Span {
    Span::new(test_file_id(), BytePos(start), BytePos(end), 1, 1) // Line/column simplified
}

fn parse_and_check(input: &str) -> Result<Program, Vec<ParserError>> {
    let lexer = Lexer::new(test_file_id(), input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if parser.get_errors().is_empty() {
        Ok(program)
    } else {
        Err(parser.get_errors().to_vec())
    }
}

#[test]
fn test_parser_let_statement() {
    let input = "let x: int = 5;";
    let program = parse_and_check(input).expect("Parsing 'let' statement should succeed");

    assert_eq!(program.statements.len(), 1);
    let expected_stmt = Statement::Let(
        span_for_test(0, 15), // Simplified span
        "x".to_string(),
        Some(TypeExpr::Base(Identifier("int".to_string(), span_for_test(6, 9)))),
        Expression::Literal(Literal::Integer("5".to_string(), span_for_test(13, 14))),
    );
    assert_eq!(program.statements[0], expected_stmt);
}

#[test]
fn test_parser_function_declaration() {
    let input = "fn add(a: int, b: int) -> int { return a + b; }";
    let program = parse_and_check(input).expect("Parsing function declaration should succeed");

    assert_eq!(program.statements.len(), 1);
    let Statement::Function(_, name, params, return_type, body) = &program.statements[0] else { panic!("Expected function statement") };
    assert_eq!(name, "add");
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name.0, "a");
    assert_eq!(params[0].typ, Some(TypeExpr::Base(Identifier("int".to_string(), span_for_test(9, 12)))));
    assert_eq!(params[1].name.0, "b");
    assert_eq!(params[1].typ, Some(TypeExpr::Base(Identifier("int".to_string(), span_for_test(17, 20)))));
    assert_eq!(return_type.as_ref().unwrap(), &TypeExpr::Base(Identifier("int".to_string(), span_for_test(26, 29))));
    // Further check body structure
}

#[test]
fn test_parser_quantum_circuit() {
    let input = "quantum circuit Q { let q = Qubit::new(); q.h(); }";
    let program = parse_and_check(input).expect("Parsing quantum circuit should succeed");
    
    assert_eq!(program.statements.len(), 1);
    let Statement::QuantumCircuit(_, name, body) = &program.statements[0] else { panic!("Expected quantum circuit") };
    assert_eq!(name, "Q");
    // Further check body structure
}

#[test]
fn test_parser_nano_agent() {
    let input = "nano agent MyAgent { let x = 1; }";
    let program = parse_and_check(input).expect("Parsing nano agent should succeed");
    
    assert_eq!(program.statements.len(), 1);
    let Statement::NanoAgent(_, name, body) = &program.statements[0] else { panic!("Expected nano agent") };
    assert_eq!(name, "MyAgent");
    // Further check body structure
}

#[test]
fn test_parser_handle_statement() {
    let input = "handle MyEffect { call_effect(); } with { |e: int| { return 0; } }";
    let program = parse_and_check(input).expect("Parsing handle statement should succeed");

    assert_eq!(program.statements.len(), 1);
    let Statement::Handle(_, effect_id, body, handler) = &program.statements[0] else { panic!("Expected handle statement") };
    assert_eq!(effect_id.0, "MyEffect");
    // Further check body and handler structures
}

#[test]
fn test_parser_type_expressions() {
    let input = "
        type MyType = List<int>;
        type ComplexFn = fn(int, string) -> bool with effects { E1, E2 };
        type QArr = QReg[8];
        type DepPi = Π(x: A) B(x);
        type LinearQ = linear Qubit;
    ";
    let program = parse_and_check(input).expect("Parsing complex type expressions should succeed");
    assert_eq!(program.statements.len(), 5);
    // Add specific assertions for each type expression
}

#[test]
fn test_parser_control_flow() {
    let input = "
        if (x > 0) { return 1; } else { return 0; }
        while (true) { break; }
        for i in arr { continue; }
        match val { case 1 -> { return 1; } case _ -> { return 0; } }
    ";
    let program = parse_and_check(input).expect("Parsing control flow statements should succeed");
    assert_eq!(program.statements.len(), 4);
    // Add specific assertions for each control flow statement
}

#[test]
fn test_parser_member_access_and_calls() {
    let input = "
        my_object.field;
        my_array[index];
        func(arg1, arg2);
        q.h().measure(); // Chained quantum operations
    ";
    let program = parse_and_check(input).expect("Parsing member access and calls should succeed");
    assert_eq!(program.statements.len(), 4);
    // Add specific assertions for each
}

#[test]
fn test_parser_error_handling() {
    let input = "let x = ;"; // Missing expression after '='
    let result = parse_and_check(input);
    assert!(result.is_err(), "Parser should report an error for incomplete statement.");
    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "Parser should collect at least one error.");
    println!("Parser errors: {:?}", errors);
}
