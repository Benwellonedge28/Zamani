//! Comprehensive Test Suite for Quantum Entanglement and Conditional Gate Execution

use zamani::compile;
use zamani::ast::{Statement, Expression};

#[test]
fn test_parse_quantum_entangle() {
    // Test that entangle(q1, q2) parses successfully into an Entangle AST node
    let source = "omniversal simulate TestSim { quantum circuit QC { let q1 = |0>; let q2 = |0>; entangle(q1, q2); } }";
    let module = compile(source);
    assert!(module.is_ok(), "Quantum simulation with entangle should compile successfully: {:?}", module);
}

#[test]
fn test_quantum_conditional_execution() {
    // Test quantum entanglement combined with measurement and conditional branching
    let source = r#"
        omniversal simulate BellStateSim {
            quantum circuit BellQC {
                let q1 = |0>;
                let q2 = |0>;
                H(q1);
                entangle(q1, q2);
                let res = measure(q1);
                if res == 1 {
                    X(q2);
                }
            }
        }
    "#;
    let module = compile(source);
    assert!(module.is_ok(), "Quantum conditional execution should compile successfully: {:?}", module);
    
    let ir = module.unwrap().to_ir_string();
    assert!(ir.contains("__quantum_rt_cnot"), "IR should lower entangle into CNOT runtime call");
    assert!(ir.contains("Omniversal Block: BellStateSim"), "IR should include simulation block metadata");
}

#[test]
fn test_multi_qubit_circuit_pipeline() {
    let source = r#"
        omniversal simulate MultiQubitSim {
            quantum circuit GHZState {
                let a = |0>;
                let b = |0>;
                let c = |0>;
                H(a);
                entangle(a, b);
                entangle(b, c);
            }
        }
    "#;
    let module = compile(source);
    assert!(module.is_ok(), "Multi-qubit GHZ state circuit should compile: {:?}", module);
    let ir = module.unwrap().to_ir_string();
    assert!(ir.matches("__quantum_rt_cnot").count() >= 2, "GHZ state should emit multiple CNOT calls");
}
