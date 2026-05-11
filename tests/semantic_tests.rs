
//! Conceptual Tests: Semantic Analyzer
//!
//! This module provides conceptual unit tests for the Zenith Semantic Analyzer.
//! It verifies that the analyzer correctly performs:
//! - Type checking for classical operations.
//! - Scope resolution and symbol management.
//! - Type validation for Zenith-specific constructs (quantum, nano, MTS, Sankofa).
//! - Conceptual checks for linear/affine type usage.
//! - Algebraic effect handling verification.
//! - Proper error reporting for semantic issues.

use zenith_compiler::lexer::Lexer;
use zenith_compiler::parser::Parser;
use zenith_compiler::semantic::{SemanticAnalyzer, SemanticError};
use zenith_compiler::source_map::FileId;
use zenith_compiler::compiler_types::{Type, IntWidth}; // For expected types

// Helper function for creating a FileId for tests
fn test_file_id() -> FileId { FileId::new(1) }

fn analyze_and_check(input: &str) -> Result<(), Vec<SemanticError>> {
    let lexer = Lexer::new(test_file_id(), input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    if !parser.get_errors().is_empty() {
        return Err(parser.get_errors().into_iter().map(|e| SemanticError{message: e.message, span:e.span}).collect());
    }

    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(&program)
}

#[test]
fn test_semantic_basic_type_checking() {
    let input = r#"
        fn main() -> int {
            let x: int = 10;
            let y: float = 20.0;
            let z: int = x + 5;
            return z;
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Basic type checking should pass: {:?}", result.unwrap_err());
}

#[test]
fn test_semantic_type_mismatch_error() {
    let input = r#"
        fn main() -> int {
            let x: int = 10;
            let y: bool = x + true; // Type mismatch
            return 0;
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_err(), "Type mismatch should produce an error.");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Incompatible types for operator"));
}

#[test]
fn test_semantic_unresolved_identifier_error() {
    let input = r#"
        fn main() -> int {
            let x = unknown_var; // Unresolved identifier
            return 0;
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_err(), "Unresolved identifier should produce an error.");
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].message.contains("Unresolved identifier 'unknown_var'"));
}

#[test]
fn test_semantic_quantum_ops() {
    let input = r#"
        quantum circuit MyCircuit {
            let q1 = Qubit::new();
            q1.h();
            q1.cnot(q1); // Should allow self-cnot conceptually, real quantum would error
            let b: bool = q1.measure();
            // let invalid_op = q1 + 10; // This should be a semantic error
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Valid quantum operations should pass: {:?}", result.unwrap_err());
}

#[test]
fn test_semantic_nano_ops() {
    let input = r#"
        nano agent Builder {
            let blueprint = "brick_builder";
            let components = ["arm", "laser"];
            let my_agent = NanoAgent::assemble(blueprint, components);
            my_agent.perform_action("build_wall");
            // let invalid_agent_op = my_agent + 5; // Should be semantic error
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Valid nano-agent operations should pass: {:?}", result.unwrap_err());
}

#[test]
fn test_semantic_sankofa_memory_ops() {
    let input = r#"
        remember initial_state = "initial string";
        let fact_id = "initial_state";
        let historical_data = ZamaniFact::access(fact_id);
        if historical_data.is_some() {
            let data: String = historical_data.unwrap().get_content::<String>();
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Valid Sankofa memory operations should pass: {:?}", result.unwrap_err());
}

#[test]
fn test_semantic_effect_handling() {
    let input = r#"
        effect MyError;
        effect MyOtherEffect;

        fn risky_op() {
            perform MyError("Failed!");
        }

        handle MyError {
            risky_op();
            perform MyOtherEffect("Also failed!"); // MyOtherEffect is not handled
        } with { |msg: String| {
            println("Handled error: " + msg);
        }}
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_err(), "Unhandled effect should produce an error.");
    let errors = result.unwrap_err();
    assert!(errors.len() == 1, "Expected one unhandled effect error.");
    assert!(errors[0].message.contains("Effect 'MyOtherEffect' is performed but not handled"));
}

#[test]
fn test_semantic_unsafe_with_proof() {
    let input = r#"
        effect EvasProofId; // Declare a dummy proof ID for testing
        fn main() {
            unsafe!(evas:EvasProofId) {
                // Potentially unsafe code
            }
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Unsafe block with declared proof ID should pass: {:?}", result.unwrap_err());
}

#[test]
fn test_semantic_unsafe_unrecognized_proof() {
    let input = r#"
        fn main() {
            unsafe!(evas:UnknownProofId) { // Unknown proof ID
                // Potentially unsafe code
            }
        }
    "#;
    let result = analyze_and_check(input);
    assert!(result.is_err(), "Unsafe block with unknown proof ID should error.");
    let errors = result.unwrap_err();
    assert!(errors.len() == 1);
    assert!(errors[0].message.contains("Unrecognized safety proof ID 'UnknownProofId'"));
}

#[test]
fn test_semantic_linear_type_usage_conceptual() {
    let input = r#"
        type LinearInt = linear int;
        fn process_resource(res: LinearInt) -> int {
            // res is used exactly once here conceptually
            let x = res + 1; // First conceptual use
            // let y = res + 2; // This would be a second use, an error for linear
            return x;
        }

        fn main() -> int {
            let my_linear_res: LinearInt = 10;
            let result = process_resource(my_linear_res); // Transfer ownership, one use
            // let z = my_linear_res + 5; // Semantic error: used after move/consumption
            return result;
        }
    "#;
    // For now, this test just checks for conceptual passes. Full linear check is complex.
    let result = analyze_and_check(input);
    assert!(result.is_ok(), "Conceptual linear type usage should pass (current implementation only counts): {:?}", result.unwrap_err());
    // A real test would inject errors for double-use.
}
