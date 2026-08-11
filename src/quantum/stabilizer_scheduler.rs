//! Stabilizer Scheduler for Fault-Tolerant Quantum Circuits
//! Schedules syndrome extraction and parity checks for surface code patches.

use crate::ir_gen::{IrFunction, IrInstruction, IrModule, IrValue, IrRegister, IrType};

pub struct StabilizerScheduler {
    pub patch_name: String,
    pub distance: usize,
}

impl StabilizerScheduler {
    pub fn new(patch_name: impl Into<String>, distance: usize) -> Self {
        StabilizerScheduler {
            patch_name: patch_name.into(),
            distance,
        }
    }

    /// Schedule syndrome extraction rounds for X and Z stabilizers
    pub fn schedule_rounds(&self, func: &mut IrFunction, rounds: usize) {
        func.push(IrInstruction::Comment(format!(
            "--- Begin Stabilizer Scheduling for Patch: {} (Distance: {}) ---",
            self.patch_name, self.distance
        )));

        for r in 1..=rounds {
            func.push(IrInstruction::Comment(format!("Round {} / {}: X-Stabilizer Parity Check", r, rounds)));
            // Emit X-stabilizer ancilla initialization and entangling gates
            let ancilla_x = IrRegister(format!("ancilla_x_{}", r));
            func.push(IrInstruction::QuantumGate(ancilla_x, "H".into(), vec![]));

            func.push(IrInstruction::Comment(format!("Round {} / {}: Z-Stabilizer Parity Check", r, rounds)));
            let ancilla_z = IrRegister(format!("ancilla_z_{}", r));
            func.push(IrInstruction::QuantumGate(ancilla_z, "Reset".into(), vec![]));
        }

        func.push(IrInstruction::Comment("--- End Stabilizer Scheduling ---".into()));
    }
}
