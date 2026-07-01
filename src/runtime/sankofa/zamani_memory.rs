#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Memory — long-term ancestral/historical memory (Swahili: "the distant past").
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ZamaniRecord {
    pub id: u64,
    pub era: String,
    pub content: String,
    pub author: Option<String>,
    pub timestamp: i64,  // Can be negative for historical records
    pub verified: bool,
    pub cultural_tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AncestralChain {
    pub lineage_name: String,
    pub records: Vec<u64>, // ZamaniRecord IDs
    pub generation_span: u32,
}

pub struct ZamaniMemory {
    records: HashMap<u64, ZamaniRecord>,
    lineages: HashMap<String, AncestralChain>,
    next_id: u64,
}

impl ZamaniMemory {
    pub fn new() -> Self {
        ZamaniMemory { records: HashMap::new(), lineages: HashMap::new(), next_id: 1 }
    }

    pub fn store(&mut self, era: &str, content: &str, timestamp: i64, author: Option<&str>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.records.insert(id, ZamaniRecord {
            id, era: era.to_string(), content: content.to_string(),
            author: author.map(String::from), timestamp, verified: false,
            cultural_tags: Vec::new(),
        });
        id
    }

    pub fn recall(&self, id: u64) -> Option<&ZamaniRecord> {
        self.records.get(&id)
    }

    pub fn recall_era(&self, era: &str) -> Vec<&ZamaniRecord> {
        self.records.values().filter(|r| r.era == era).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&ZamaniRecord> {
        let q = query.to_lowercase();
        self.records.values()
            .filter(|r| r.content.to_lowercase().contains(&q))
            .collect()
    }

    pub fn register_lineage(&mut self, name: &str) -> &AncestralChain {
        self.lineages.insert(name.to_string(), AncestralChain {
            lineage_name: name.to_string(), records: Vec::new(), generation_span: 0
        });
        self.lineages.get(name).unwrap()
    }

    pub fn total_records(&self) -> usize { self.records.len() }
}

impl Default for ZamaniMemory { fn default() -> Self { Self::new() } }
