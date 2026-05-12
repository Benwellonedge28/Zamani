
//! Zenith Runtime: Sankofa - Long-Term Learning and Memory Integration
//!
//! This module aggregates and manages all components for Sankofa, Zenith's
//! system for long-term learning, memory, and cultural knowledge integration.

pub mod sasa_knowledge; // Active/Current Knowledge Base
pub mod zamani_memory;   // Deep/Historical Memory Storage
pub mod learning_engine; // Autonomous Learning and Refinement
pub mod cultural_adapter; // Cultural Nuance and Language Specifics
pub mod knowledge_fabric; // New: Omniversal, Hyper-Dimensional Knowledge Fabric

/// Initializes all Sankofa components.
pub fn init_sankofa_integration() {
    println!("Initializing Runtime Sankofa Module...");
    sasa_knowledge::init_sasa_knowledge();
    zamani_memory::init_zamani_memory();
    learning_engine::init_learning_engine();
    cultural_adapter::init_cultural_adapter();
    knowledge_fabric::init_knowledge_fabric(); // Initialize Knowledge Fabric
    println!("Runtime Sankofa Module initialized.");
}

/// Shuts down all Sankofa components.
pub fn shutdown_sankofa_integration() {
    println!("Shutting down Runtime Sankofa Module...");
    knowledge_fabric::shutdown_knowledge_fabric(); // Shutdown Knowledge Fabric
    cultural_adapter::shutdown_cultural_adapter();
    learning_engine::shutdown_learning_engine();
    zamani_memory::shutdown_zamani_memory();
    sasa_knowledge::shutdown_sasa_knowledge();
    println!("Runtime Sankofa Module shut down.");
}
