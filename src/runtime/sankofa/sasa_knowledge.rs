#![allow(dead_code, unused_variables, unused_imports)]
//! Sasa Knowledge — present-tense active knowledge management (Swahili: "now").
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SasaFact {
    pub key: String,
    pub value: String,
    pub confidence: f64,
    pub active_since: u64,
    pub last_validated: u64,
}

#[derive(Debug, Clone)]
pub struct SasaContext {
    pub agent_id: u64,
    pub active_goals: Vec<String>,
    pub current_environment: HashMap<String, String>,
    pub ethical_state: f64,
}

pub struct SasaKnowledgeBase {
    facts: HashMap<String, SasaFact>,
    context: SasaContext,
    tick: u64,
}

impl SasaKnowledgeBase {
    pub fn new(agent_id: u64) -> Self {
        SasaKnowledgeBase {
            facts: HashMap::new(),
            context: SasaContext {
                agent_id,
                active_goals: Vec::new(),
                current_environment: HashMap::new(),
                ethical_state: 1.0,
            },
            tick: 0,
        }
    }

    pub fn assert_fact(&mut self, key: &str, value: &str, confidence: f64) {
        self.facts.insert(
            key.to_string(),
            SasaFact {
                key: key.to_string(),
                value: value.to_string(),
                confidence,
                active_since: self.tick,
                last_validated: self.tick,
            },
        );
    }

    pub fn query_fact(&self, key: &str) -> Option<&SasaFact> {
        self.facts.get(key)
    }

    pub fn retract_fact(&mut self, key: &str) -> bool {
        self.facts.remove(key).is_some()
    }

    pub fn update_context(&mut self, env_key: &str, env_val: &str) {
        self.context
            .current_environment
            .insert(env_key.to_string(), env_val.to_string());
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        // Decay confidence of stale facts
        for fact in self.facts.values_mut() {
            if self.tick - fact.last_validated > 1000 {
                fact.confidence *= 0.99;
            }
        }
    }

    pub fn fact_count(&self) -> usize {
        self.facts.len()
    }
}

impl Default for SasaKnowledgeBase {
    fn default() -> Self {
        Self::new(0)
    }
}
