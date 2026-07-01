#![allow(dead_code, unused_variables, unused_imports)]
//! Sankofa Learning Engine — temporal pattern learning and wisdom extraction.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LearningRecord {
    pub source: String,
    pub knowledge: String,
    pub weight: f64,
    pub timestamp: u64,
    pub verified: bool,
}

#[derive(Debug, Clone)]
pub struct WisdomPattern {
    pub pattern: String,
    pub confidence: f64,
    pub supporting_records: usize,
    pub temporal_span: u64,
}

pub struct LearningEngine {
    records: Vec<LearningRecord>,
    patterns: Vec<WisdomPattern>,
    learning_rate: f64,
}

impl LearningEngine {
    pub fn new(learning_rate: f64) -> Self {
        LearningEngine { records: Vec::new(), patterns: Vec::new(), learning_rate }
    }

    pub fn learn(&mut self, source: &str, knowledge: &str, weight: f64, timestamp: u64) {
        self.records.push(LearningRecord {
            source: source.to_string(),
            knowledge: knowledge.to_string(),
            weight: weight * self.learning_rate,
            timestamp,
            verified: false,
        });
        self.extract_patterns();
    }

    pub fn learn_from_ancestor(&mut self, ancestor_name: &str, wisdom: &str, ts: u64) {
        self.learn(ancestor_name, wisdom, 1.5, ts); // Ancestral wisdom gets higher weight
    }

    fn extract_patterns(&mut self) {
        // Group by similar knowledge patterns
        let mut freq: HashMap<String, usize> = HashMap::new();
        for r in &self.records {
            let key = r.knowledge[..r.knowledge.len().min(20)].to_string();
            *freq.entry(key).or_insert(0) += 1;
        }
        for (pattern, count) in freq {
            if count >= 3 {
                self.patterns.push(WisdomPattern {
                    pattern,
                    confidence: (count as f64 / self.records.len() as f64).min(1.0),
                    supporting_records: count,
                    temporal_span: 0,
                });
            }
        }
    }

    pub fn recall_relevant(&self, query: &str) -> Vec<&LearningRecord> {
        self.records.iter()
            .filter(|r| r.knowledge.contains(query))
            .collect()
    }

    pub fn top_patterns(&self, n: usize) -> Vec<&WisdomPattern> {
        let mut sorted = self.patterns.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        sorted.into_iter().take(n).collect()
    }
}

impl Default for LearningEngine { fn default() -> Self { Self::new(0.01) } }
