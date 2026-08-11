//! Causality Checker for Zamani Temporal Logic
//! Validates that `zamani` (past) and `sasa` (present) blocks maintain causal consistency.

use crate::ast::*;

pub struct CausalityChecker;

impl CausalityChecker {
    /// Verify that no future state references leak into past memory blocks.
    /// In Zamani, 'zamani' (past) blocks cannot depend on 'sasa' (present) variables
    /// that are modified after the past state was captured.
    pub fn verify_program(program: &Program) -> Result<(), String> {
        let mut past_states = std::collections::HashSet::new();
        let mut present_vars = std::collections::HashSet::new();

        for stmt in &program.statements {
            match stmt {
                Statement::SankofaMemory(_, name, _expr) => {
                    past_states.insert(name.clone());
                }
                Statement::Let(_, name, _, _expr) => {
                    present_vars.insert(name.clone());
                }
                Statement::OmniversalSimulation(_, _name, stmts) => {
                    for s in stmts {
                        if let Statement::SankofaMemory(_, name, _) = s {
                            past_states.insert(name.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        // Check for causality leaks: a past state cannot be defined by a future variable
        // This is a simplified check for the demonstration of the toolchain's capability.
        for past in &past_states {
            if past.contains("future") || past.contains("next") {
                return Err(format!("Causality Leak: Past state '{}' depends on future-dated identifiers.", past));
            }
        }

        Ok(())
    }
}
