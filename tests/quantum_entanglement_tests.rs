//! Comprehensive Test Suite for Quantum Entanglement and Conditional Gate Execution

use zamani::compile;
use zamani::optimizer::{Optimizer, OptimizationConfig};

#[test]
fn test_parse_quantum_entangle() {
    let source = "omniversal simulate TestSim { quantum circuit QC { let q1 = |0>; let q2 = |0>; entangle(q1, q2); } }";
    let module = compile(source);
    assert!(module.is_ok(), "Quantum simulation with entangle should compile successfully: {:?}", module);
}

#[test]
fn test_quantum_conditional_execution() {
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
fn test_quantum_gate_optimization() {
    // Adjacent H * H on the same qubit should be eliminated by the quantum optimizer
    let source = r#"
        omniversal simulate OptSim {
            quantum circuit RedundantQC {
                let q = |0>;
                H(q);
                H(q);
                X(q);
            }
        }
    "#;
    let module = compile(source).unwrap();
    let mut opt = Optimizer::with_level(2);
    let optimized_module = opt.optimize(&module);
    let ir = optimized_module.to_ir_string();
    
    // The double H should cancel out, leaving only X(q)
    assert!(!ir.contains("__quantum_rt_h"), "Redundant Hadamard gates should be optimized out");
    assert!(ir.contains("__quantum_rt_x"), "X gate should remain");
}
