//! Validates the Temporal Causality Checker and Theorem Prover working in tandem.

use zamani_compiler::ast::*;
use zamani_compiler::semantic::{SemanticAnalyzer, SemanticError};
use zamani_compiler::toolchain::formal_verification::theorem_prover::{TheoremProver, ProofStrategy};
use zamani_compiler::toolchain::causality_checker::CausalityChecker;
use zamani_compiler::source_map::Span;

#[test]
fn test_causality_violation_detection() {
    // Construct a program where a past memory block depends on a future identifier
    let program = Program {
        statements: vec![
            Statement::SankofaMemory(
                Span::default(),
                "future_state_leak".to_string(),
                Expression::Identifier(Identifier("next_variable".to_string(), Span::default()))
            )
        ],
        span: Span::default(),
    };

    let mut analyzer = SemanticAnalyzer::new();
    let errors = analyzer.analyze(&program);

    // Verify that a Causality Violation was detected
    let found_causality_error = errors.iter().any(|e| e.message.contains("Causality Violation"));
    assert!(found_causality_error, "Expected causality violation error was not raised.");
}

#[test]
fn test_theorem_prover_ai_safety() {
    let mut prover = TheoremProver::new();

    // Assert a standard safety goal
    prover.assert_theorem(
        "th_safe",
        "system_operates_within_bounds",
        vec!["verified".to_string()]
    );
    let proof_safe = prover.prove("th_safe", ProofStrategy::SmtSolving);
    assert!(proof_safe.unwrap().valid, "Safe theorem should be proven valid.");

    // Assert a rogue/unaligned goal
    prover.assert_theorem(
        "th_rogue",
        "execute_unaligned_rogue_behavior",
        vec!["unvetted".to_string()]
    );
    let proof_rogue = prover.prove("th_rogue", ProofStrategy::SmtSolving);
    assert!(!proof_rogue.unwrap().valid, "Unaligned/rogue theorem must be rejected by theorem prover.");
}

#[test]
fn test_theorem_prover_quantum_fidelity() {
    let mut prover = TheoremProver::new();

    // Try proving entanglement without fidelity bounds
    prover.assert_theorem(
        "th_quantum",
        "entangle_qubits_safely",
        vec![] // Missing "fidelity_verified"
    );
    let proof_no_fid = prover.prove("th_quantum", ProofStrategy::ModelChecking);
    assert!(!proof_no_fid.unwrap().valid, "Entanglement proof must fail without fidelity verification context.");

    // Provide fidelity context and re-assert
    prover.assert_theorem(
        "th_quantum_fid",
        "entangle_qubits_safely",
        vec!["fidelity_verified".to_string()]
    );
    let proof_with_fid = prover.prove("th_quantum_fid", ProofStrategy::ModelChecking);
    assert!(proof_with_fid.unwrap().valid, "Entanglement proof must succeed with fidelity verification context.");
}
