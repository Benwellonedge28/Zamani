//! Zamani Runtime: Sankofa - Long-Term Learning and Memory Integration
//!
//! This module aggregates and manages all components for Sankofa, Zamani's
//! system for long-term learning, memory, and cultural knowledge integration.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub mod cultural_adapter;
pub mod knowledge_fabric; // Omniversal Knowledge Fabric
pub mod learning_engine; // Autonomous Learning and Refinement
pub mod sasa_knowledge; // Active/Current Knowledge Base
pub mod zamani_memory; // Deep/Historical Memory Storage // Cultural Nuance and Language Specifics

/// Initializes all Sankofa components.
///
/// `cultural_adapter`, `learning_engine`, `sasa_knowledge`, and `zamani_memory`
/// each model themselves as structs with their own `::new()` constructors
/// (constructed on demand by their callers, since they carry per-session
/// state), so only the free-standing `knowledge_fabric` init/shutdown pair
/// and this module's own process-wide runtime are handled globally here.
pub fn init_sankofa_integration() {
    println!("Initializing Runtime Sankofa Module...");
    knowledge_fabric::init_knowledge_fabric();
    init_sankofa_runtime();
    println!("Runtime Sankofa Module initialized.");
}

/// Shuts down all Sankofa components.
pub fn shutdown_sankofa_integration() {
    println!("Shutting down Runtime Sankofa Module...");
    shutdown_sankofa_runtime();
    knowledge_fabric::shutdown_knowledge_fabric();
    println!("Runtime Sankofa Module shut down.");
}

// ── merged from flat_backup ────

static mut SANKOFA_RUNTIME_STATE_INSTANCE: Option<Arc<Mutex<SankofaRuntimeState>>> = None;

#[derive(Clone)]
pub struct ZamaniFactRecord {
    pub fact_id: String,
    pub content: Vec<u8>,
    pub timestamp_recorded: u64, // The immutable point in time it was recorded
    pub provenance: String, // Source of the fact (e.g., "observer_A", "quantum_measurement_device")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SasaKnowledgeVersion {
    pub version_id: u64, // Unique ID for this version
    pub knowledge_id: String,
    pub content: Vec<u8>,
    pub timestamp_valid_from: u64, // When this version became active
    pub timestamp_valid_to: Option<u64>, // When this version was superseded (None if current)
    pub causal_predecessors: Vec<u64>, // Version IDs that directly led to this update
    pub learning_agent_id: Option<String>, // Which agent generated/updated this knowledge
}

pub struct ZamaniStore {
    facts: HashMap<String, ZamaniFactRecord>, // Map fact_id to record
}

pub struct SasaStore {
    knowledge_versions: HashMap<String, Vec<SasaKnowledgeVersion>>, // Map knowledge_id to ordered list of versions
    next_version_id: u64,
}

pub struct SankofaRuntimeState {
    pub zamani_store: ZamaniStore,
    pub sasa_store: SasaStore,
    // Add conceptual learning agents, causality graph, inter-memory interfaces
    causality_graph: HashMap<u64, Vec<u64>>, // Maps version ID to list of its causal dependents
    learning_agents: HashMap<String, Box<dyn LearningAgent + Send + Sync>>, // Conceptual plug-in agents
    pub inter_memory_interfaces: HashMap<String, Box<dyn InterMemoryInterface + Send + Sync>>, // For other languages
}

pub trait LearningAgent {
    fn process_temporal_data(&self, data: &SasaKnowledgeVersion) -> Option<SasaKnowledgeVersion>;
}

pub trait InterMemoryInterface {
    fn access_external_memory(&self, query: &str) -> Option<Vec<u8>>;
    fn write_external_memory(&self, data: &[u8]) -> Result<(), String>;
}

pub fn init_sankofa_runtime() -> Arc<Mutex<SankofaRuntimeState>> {
    println!("  - Initializing Sankofa Memory Runtime (Temporal Storage, Knowledge Management)...");
    let runtime_state = Arc::new(Mutex::new(SankofaRuntimeState {
        zamani_store: ZamaniStore::new(),
        sasa_store: SasaStore::new(),
        causality_graph: HashMap::new(),
        learning_agents: HashMap::new(),
        inter_memory_interfaces: HashMap::new(),
    }));

    // Register conceptual learning agents
    let mut state_guard = runtime_state.lock().unwrap();
    state_guard.learning_agents.insert(
        "temporal_pattern_matcher".to_string(),
        Box::new(TemporalPatternMatcherAgent),
    );
    state_guard.learning_agents.insert(
        "causal_inference_engine".to_string(),
        Box::new(CausalInferenceEngine),
    );
    drop(state_guard);

    unsafe {
        SANKOFA_RUNTIME_STATE_INSTANCE = Some(Arc::clone(&runtime_state));
    }

    println!("    -> Sankofa Runtime initialized.");
    runtime_state
}

pub fn shutdown_sankofa_runtime() {
    println!("  - Shutting down Sankofa Memory Runtime...");
    // Conceptual: Persist all pending knowledge, close database connections.
    unsafe {
        SANKOFA_RUNTIME_STATE_INSTANCE = None;
    }
}

pub fn record_zamani_fact(
    runtime_state_arc: &Arc<Mutex<SankofaRuntimeState>>,
    fact_id: String,
    content: Vec<u8>,
    timestamp: u64,
    provenance: String,
) {
    let mut state = runtime_state_arc.lock().unwrap();
    let fact = ZamaniFactRecord {
        fact_id: fact_id.clone(),
        content,
        timestamp_recorded: timestamp,
        provenance,
    };
    state.zamani_store.record_fact(fact);
    println!(
        "    -> Sankofa Runtime: Recorded Zamani fact '{}' at {}.",
        fact_id, timestamp
    );
}

pub fn access_zamani_fact(
    runtime_state_arc: &Arc<Mutex<SankofaRuntimeState>>,
    fact_id: &str,
) -> Option<ZamaniFactRecord> {
    let state = runtime_state_arc.lock().unwrap();
    state.zamani_store.get_fact(fact_id).cloned()
}

pub fn update_sasa_knowledge(
    runtime_state_arc: &Arc<Mutex<SankofaRuntimeState>>,
    knowledge_id: String,
    content: Vec<u8>,
    timestamp: u64,
    causal_predecessors: Vec<u64>,
) -> SasaKnowledgeVersion {
    let mut state = runtime_state_arc.lock().unwrap();
    let new_version = state.sasa_store.record_knowledge_update(
        knowledge_id.clone(),
        content,
        timestamp,
        causal_predecessors,
        None,
    );

    // Conceptual: Update causality graph
    // For each predecessor, add new_version as a dependent.
    // For simplicity, just adding self for now.
    state
        .causality_graph
        .entry(new_version.version_id)
        .or_default()
        .push(new_version.version_id);

    println!(
        "    -> Sankofa Runtime: Updated Sasa knowledge '{}' to version {} at {}.",
        knowledge_id, new_version.version_id, timestamp
    );
    new_version
}

pub fn get_sasa_knowledge_at_time(
    runtime_state_arc: &Arc<Mutex<SankofaRuntimeState>>,
    knowledge_id: &str,
    timestamp: u64,
) -> Option<SasaKnowledgeVersion> {
    let state = runtime_state_arc.lock().unwrap();
    state
        .sasa_store
        .get_knowledge_at_time(knowledge_id, timestamp)
        .cloned()
}

pub fn temporal_learn(
    runtime_state_arc: &Arc<Mutex<SankofaRuntimeState>>,
    knowledge_id: &str,
    timestamp_range_start: u64,
    timestamp_range_end: u64,
) {
    println!(
        "    -> Sankofa Runtime: Initiating temporal learning for '{}' over range {}-{}.",
        knowledge_id, timestamp_range_start, timestamp_range_end
    );
    let state_guard = runtime_state_arc.lock().unwrap();

    if let Some(versions) = state_guard.sasa_store.knowledge_versions.get(knowledge_id) {
        for version in versions.iter().filter(|v| {
            v.timestamp_valid_from >= timestamp_range_start
                && (v.timestamp_valid_to.is_none()
                    || v.timestamp_valid_to.unwrap() <= timestamp_range_end)
        }) {
            for agent in state_guard.learning_agents.values() {
                if let Some(new_knowledge) = agent.process_temporal_data(version) {
                    // This would normally trigger an update_sasa_knowledge, requiring mut self
                    // For conceptual, just log the potential new knowledge.
                    println!("      -> Learning Agent processed version {}. Potential new knowledge: {:?}", version.version_id, new_knowledge);
                }
            }
        }
    }
    drop(state_guard);
    println!(
        "    -> Sankofa Runtime: Temporal learning completed for '{}'.",
        knowledge_id
    );
}

impl ZamaniStore {
    pub fn new() -> Self {
        ZamaniStore {
            facts: HashMap::new(),
        }
    }

    pub fn record_fact(&mut self, fact: ZamaniFactRecord) {
        self.facts.insert(fact.fact_id.clone(), fact);
    }

    pub fn get_fact(&self, fact_id: &str) -> Option<&ZamaniFactRecord> {
        self.facts.get(fact_id)
    }
}

impl Default for ZamaniStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SasaStore {
    pub fn new() -> Self {
        SasaStore {
            knowledge_versions: HashMap::new(),
            next_version_id: 1,
        }
    }

    /// Records a new version of a piece of knowledge, closing out the
    /// previously-current version (if any) by setting its `valid_to`.
    #[allow(clippy::too_many_arguments)]
    pub fn record_knowledge_update(
        &mut self,
        knowledge_id: String,
        content: Vec<u8>,
        timestamp: u64,
        causal_predecessors: Vec<u64>,
        learning_agent_id: Option<String>,
    ) -> SasaKnowledgeVersion {
        let version_id = self.next_version_id;
        self.next_version_id += 1;

        let versions = self
            .knowledge_versions
            .entry(knowledge_id.clone())
            .or_default();
        if let Some(last) = versions.last_mut() {
            if last.timestamp_valid_to.is_none() {
                last.timestamp_valid_to = Some(timestamp);
            }
        }

        let new_version = SasaKnowledgeVersion {
            version_id,
            knowledge_id,
            content,
            timestamp_valid_from: timestamp,
            timestamp_valid_to: None,
            causal_predecessors,
            learning_agent_id,
        };
        versions.push(new_version.clone());
        new_version
    }

    pub fn get_knowledge_at_time(
        &self,
        knowledge_id: &str,
        timestamp: u64,
    ) -> Option<&SasaKnowledgeVersion> {
        self.knowledge_versions
            .get(knowledge_id)
            .and_then(|versions| {
                versions.iter().find(|v| {
                    v.timestamp_valid_from <= timestamp
                        && v.timestamp_valid_to.map(|t| timestamp < t).unwrap_or(true)
                })
            })
    }
}

impl Default for SasaStore {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TemporalPatternMatcherAgent;

impl LearningAgent for TemporalPatternMatcherAgent {
    fn process_temporal_data(&self, data: &SasaKnowledgeVersion) -> Option<SasaKnowledgeVersion> {
        // Conceptual: a real implementation would detect recurring patterns
        // across the knowledge_id's version history. For now, this reports
        // that it observed the version without synthesizing new knowledge.
        println!(
            "      -> TemporalPatternMatcherAgent observed version {} of '{}'.",
            data.version_id, data.knowledge_id
        );
        None
    }
}

pub struct CausalInferenceEngine;

impl LearningAgent for CausalInferenceEngine {
    fn process_temporal_data(&self, data: &SasaKnowledgeVersion) -> Option<SasaKnowledgeVersion> {
        // Conceptual: a real implementation would infer causal links between
        // this version and its `causal_predecessors`. For now, this reports
        // the predecessor count it would reason over.
        println!(
            "      -> CausalInferenceEngine analyzing version {} ({} causal predecessors).",
            data.version_id,
            data.causal_predecessors.len()
        );
        None
    }
}

// -----------------------------------------------------------------------------
// Additional conceptual handles used by higher-level stdlib modules (e.g. MGNS)
// that plug into the Sankofa (Zamani/Sasa) memory system.
// -----------------------------------------------------------------------------

/// Identifies a single stored piece of Sankofa knowledge.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct KnowledgeId(pub String);

/// A high-level, queryable handle onto the evolving Sasa (present) knowledge
/// store, distinct from the lower-level `SasaStore` record cache.
#[derive(Debug, Clone, Default)]
pub struct SasaKnowledge {
    pub entries: crate::stdlib::collections::Map<String, String>,
}

impl SasaKnowledge {
    pub fn new() -> Self {
        SasaKnowledge {
            entries: crate::stdlib::collections::Map::new(),
        }
    }

    /// Records a conjecture fact (from the mathematical invention engine)
    /// into the knowledge store, keyed by its predicate.
    pub fn store_conjecture(&mut self, fact: crate::stdlib::ai_reasoning::Fact) {
        self.entries.insert(
            format!("conjecture:{}", fact.predicate),
            format!("{:?}", fact),
        );
    }

    /// Records empirical evidence gathered while exploring a conjecture,
    /// keyed by the conjecture's identifier.
    pub fn store_empirical_evidence<T: std::fmt::Debug>(
        &mut self,
        id: crate::ast::Identifier,
        evidence: T,
    ) {
        self.entries
            .insert(format!("empirical:{}", id.0), format!("{:?}", evidence));
    }

    /// Records a completed formal proof for a conjecture.
    pub fn store_proof<T: std::fmt::Debug>(&mut self, id: crate::ast::Identifier, proof: T) {
        self.entries
            .insert(format!("proof:{}", id.0), format!("{:?}", proof));
    }

    /// Records a counterexample that falsifies a conjecture.
    pub fn store_counterexample<T: std::fmt::Debug>(
        &mut self,
        id: crate::ast::Identifier,
        counterexample: T,
    ) {
        self.entries.insert(
            format!("counterexample:{}", id.0),
            format!("{:?}", counterexample),
        );
    }

    /// Retrieves knowledge entries relevant to a topic keyword, returning up
    /// to `limit` matches as a Map of key->value.  Used by the documentation
    /// system for RAG-style context gathering.
    pub fn retrieve_relevant_knowledge(
        &self,
        topic: &str,
        limit: usize,
    ) -> Result<crate::stdlib::collections::Map<String, crate::stdlib::meta_ops::MetaValue>, String>
    {
        let mut results = crate::stdlib::collections::Map::new();
        let mut count = 0;
        for (key, value) in self.entries.iter() {
            if key.contains(topic) || value.contains(topic) {
                results.insert(
                    key.clone(),
                    crate::stdlib::meta_ops::MetaValue::String(value.clone()),
                );
                count += 1;
                if count >= limit {
                    break;
                }
            }
        }
        Ok(results)
    }

    /// Retrieves stored knowledge entries whose key starts with the given
    /// prefix, returning up to `limit` matches as a List of MetaValue.
    pub fn retrieve_knowledge(
        &self,
        prefix: &str,
        limit: usize,
    ) -> Option<crate::stdlib::collections::List<crate::stdlib::meta_ops::MetaValue>> {
        let mut results = crate::stdlib::collections::List::new();
        for (key, value) in self.entries.iter() {
            if key.starts_with(prefix) {
                results.push(crate::stdlib::meta_ops::MetaValue::String(value.clone()));
                if results.len() >= limit {
                    break;
                }
            }
        }
        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    /// Recursively queries the knowledge store for all entries related to
    /// a target identifier, following causal chains.  Used by the
    /// documentation system for deep knowledge extraction.
    pub fn query_recursive(
        &self,
        target_id: &crate::ast::Identifier,
    ) -> Result<crate::stdlib::collections::List<String>, String> {
        let mut results = crate::stdlib::collections::List::new();
        let prefix = &target_id.0;
        for (key, value) in self.entries.iter() {
            if key.contains(prefix) || value.contains(prefix) {
                results.push(format!("{}: {}", key, value));
            }
        }
        Ok(results)
    }
}

/// A conceptual graph of related knowledge nodes, used for higher-order
/// reasoning over Sankofa memory.
#[derive(Debug, Clone, Default)]
pub struct ConceptualGraph {
    pub nodes: crate::stdlib::collections::List<String>,
}

impl ConceptualGraph {
    pub fn new() -> Self {
        ConceptualGraph {
            nodes: crate::stdlib::collections::List::new(),
        }
    }
}
