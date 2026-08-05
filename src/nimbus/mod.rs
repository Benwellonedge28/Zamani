#![allow(dead_code, unused_variables, unused_imports)]
//! NIMBUS — Networked Intelligence Multi-Base Universal System
//! The distributed runtime substrate for Zamani AGI instances.

pub mod admin_interface;
pub mod os;

pub use admin_interface::{AdminInterface, SystemMetrics};

#[derive(Debug, Clone, PartialEq)]
pub enum NimbusNodeRole {
    Primary,
    Secondary,
    Observer,
    Guardian,
    BootstrapNode,
}

#[derive(Debug, Clone)]
pub struct NimbusNode {
    pub id: String,
    pub role: NimbusNodeRole,
    pub address: String,
    pub capabilities: Vec<String>,
    pub alignment_score: f64,
    pub online: bool,
}

pub struct NimbusCluster {
    nodes: std::collections::HashMap<String, NimbusNode>,
    consensus_threshold: f64,
}

impl NimbusCluster {
    pub fn new(consensus_threshold: f64) -> Self {
        NimbusCluster {
            nodes: std::collections::HashMap::new(),
            consensus_threshold,
        }
    }

    pub fn register_node(&mut self, node: NimbusNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn online_count(&self) -> usize {
        self.nodes.values().filter(|n| n.online).count()
    }

    pub fn consensus_alignment(&self) -> f64 {
        let online: Vec<&NimbusNode> = self.nodes.values().filter(|n| n.online).collect();
        if online.is_empty() {
            return 0.0;
        }
        online.iter().map(|n| n.alignment_score).sum::<f64>() / online.len() as f64
    }

    pub fn has_quorum(&self) -> bool {
        let online = self.online_count() as f64;
        let total = self.nodes.len() as f64;
        total == 0.0 || (online / total) >= self.consensus_threshold
    }
}

impl Default for NimbusCluster {
    fn default() -> Self {
        Self::new(0.67)
    }
}
