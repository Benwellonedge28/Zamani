use std::fs;
use zamani::compile;
use zamani::optimizer::{Optimizer, OptimizationConfig};

fn main() {
    let source = fs::read_to_string("benchmark.zn").expect("Failed to read benchmark.zn");

    // 1. Unoptimized Compilation
    let module = compile(&source).expect("Unoptimized compilation failed");
    let unopt_ir = module.to_ir_string();
    let unopt_count = count_quantum_calls(&unopt_ir);

    // 2. Optimized Compilation
    let mut optimizer = Optimizer::with_level(2);
    let opt_module = optimizer.optimize(&module);
    let opt_ir = opt_module.to_ir_string();
    let opt_count = count_quantum_calls(&opt_ir);

    // 3. Results Output
    println!("--- Zamani Quantum IR Benchmark ---");
    println!("Circuit: DeepQuantumSim::ComplexCircuit");
    println!("");
    println!("Instruction Type | Unoptimized | Optimized | Reduction");
    println!("-----------------|-------------|-----------|----------");
    println!("Quantum Gates    | {:<11} | {:<9} | {:.1}%", 
        unopt_count, 
        opt_count, 
        (1.0 - (opt_count as f64 / unopt_count as f64)) * 100.0
    );
    
    let unopt_cost = calculate_cost(&unopt_ir);
    let opt_cost = calculate_cost(&opt_ir);
    println!("Execution Cost*  | {:<11} | {:<9} | {:.1}%", 
        unopt_cost, 
        opt_cost, 
        (1.0 - (opt_cost as f64 / unopt_cost as f64)) * 100.0
    );
    println!("");
    println!("*Cost model: 1-qubit gate = 1, 2-qubit gate = 10, measurement = 5");
}

fn count_quantum_calls(ir: &str) -> usize {
    ir.lines()
        .filter(|line| line.contains("@__quantum_rt_"))
        .count()
}

fn calculate_cost(ir: &str) -> usize {
    let mut cost = 0;
    for line in ir.lines() {
        if line.contains("@__quantum_rt_") {
            if line.contains("_cnot") {
                cost += 10;
            } else if line.contains("_measure") {
                cost += 5;
            } else {
                cost += 1; // H, X, Y, Z, etc.
            }
        }
    }
    cost
}
