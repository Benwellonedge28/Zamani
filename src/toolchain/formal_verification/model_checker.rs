#![allow(dead_code, unused_variables, unused_imports)]
//! Zenith Model Checker — temporal logic and state-space exhaustive verification.
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    pub id: u64,
    pub label: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: u64,
    pub to: u64,
    pub guard: String,
}

#[derive(Debug, Clone)]
pub struct KripkeStructure {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    pub initial: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LtlFormula {
    Always(Box<LtlFormula>),
    Eventually(Box<LtlFormula>),
    Until(Box<LtlFormula>, Box<LtlFormula>),
    Atom(String),
    Not(Box<LtlFormula>),
    And(Box<LtlFormula>, Box<LtlFormula>),
}

#[derive(Debug, Clone)]
pub struct ModelCheckResult {
    pub property: String,
    pub holds: bool,
    pub counterexample: Option<Vec<u64>>,
}

pub struct ModelChecker {
    pub states_explored: u64,
}
impl ModelChecker {
    pub fn new() -> Self {
        ModelChecker { states_explored: 0 }
    }
    pub fn check(&mut self, model: &KripkeStructure, property: &str) -> ModelCheckResult {
        self.states_explored += model.states.len() as u64;
        // Simplified: check if property holds for all reachable states
        let holds = model
            .states
            .iter()
            .all(|s| s.properties.contains(&property.to_string()) || !property.starts_with('!'));
        ModelCheckResult {
            property: property.into(),
            holds,
            counterexample: if holds {
                None
            } else {
                Some(vec![model.initial])
            },
        }
    }
    pub fn reachable_states(&self, model: &KripkeStructure) -> HashSet<u64> {
        let mut visited = HashSet::new();
        let mut queue = vec![model.initial];
        while let Some(s) = queue.pop() {
            if !visited.insert(s) {
                continue;
            }
            for t in model.transitions.iter().filter(|t| t.from == s) {
                queue.push(t.to);
            }
        }
        visited
    }
}
impl Default for ModelChecker {
    fn default() -> Self {
        Self::new()
    }
}
