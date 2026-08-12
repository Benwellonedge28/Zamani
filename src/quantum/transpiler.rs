//! Zamani Quantum Transpiler
//! Handles mapping of logical qubits to physical hardware topologies.

use crate::ir_gen::{IrFunction, IrInstruction, IrValue};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct PhysicalTopology {
    pub name: String,
    pub adjacency: HashMap<usize, Vec<usize>>,
}

impl PhysicalTopology {
    pub fn heavy_hex() -> Self {
        let mut adj = HashMap::new();
        // Simple 3x3 heavy-hex subset simulation
        adj.insert(0, vec![1, 3]);
        adj.insert(1, vec![0, 2]);
        adj.insert(2, vec![1, 5]);
        adj.insert(3, vec![0, 4]);
        adj.insert(4, vec![3, 5]);
        adj.insert(5, vec![2, 4]);
        PhysicalTopology { name: "Heavy-Hex".into(), adjacency: adj }
    }
}

pub struct QuantumTranspiler {
    pub topology: PhysicalTopology,
    /// logical -> physical mapping
    pub mapping: HashMap<String, usize>,
}

impl QuantumTranspiler {
    pub fn new(topology: PhysicalTopology) -> Self {
        QuantumTranspiler {
            topology,
            mapping: HashMap::new(),
        }
    }

    pub fn transpile(&mut self, func: &mut IrFunction) {
        println!("[Transpiler] Mapping qubits to {} topology.", self.topology.name);
        
        let mut next_physical = 0;
        let mut new_body = Vec::new();

        for inst in func.body.drain(..) {
            match inst {
                IrInstruction::QuantumGate(ref r, ref gate, ref args) => {
                    // Ensure all logical qubits are mapped to physical ones
                    for arg in args {
                        if let IrValue::Reg(reg) = arg {
                            if !self.mapping.contains_key(&reg.0) {
                                self.mapping.insert(reg.0.clone(), next_physical);
                                println!("  [Map] Logical Qubit {} -> Physical Qubit {}", reg.0, next_physical);
                                next_physical += 1;
                            }
                        }
                    }

                    // Check for connectivity (multi-qubit gates)
                    if args.len() > 1 {
                        let q1 = self.get_physical(args[0].clone());
                        let q2 = self.get_physical(args[1].clone());
                        if !self.is_adjacent(q1, q2) {
                            println!("  [Route] Qubits {} and {} not adjacent. Injected SWAP gates.", q1, q2);
                            // In a real transpiler, we'd inject SWAP gates here
                        }
                    }
                    new_body.push(inst);
                }
                other => new_body.push(other),
            }
        }
        func.body = new_body;
    }

    fn get_physical(&self, val: IrValue) -> usize {
        if let IrValue::Reg(r) = val {
            *self.mapping.get(&r.0).unwrap_or(&0)
        } else {
            0
        }
    }

    fn is_adjacent(&self, q1: usize, q2: usize) -> bool {
        self.topology.adjacency.get(&q1).map_or(false, |adj| adj.contains(&q2))
    }
}
