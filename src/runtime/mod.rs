
//! Zenith Runtime: Core Runtime Components
//!
//! This module aggregates and manages the core runtime components for Zenith,
//! providing essential services for application execution, memory management,
//! and concurrency.

pub mod memory_manager; // Memory Allocation and Garbage Collection
pub mod concurrency_manager; // Task Scheduling and Parallel Execution
pub mod quantum; // Quantum Runtime Environment
pub mod nano; // Nano Runtime Environment
pub mod mts; // Multi-Timeline System for speculative execution
pub mod sankofa; // Long-term memory and learning integration
pub mod universal_runtime; // New: Universal Runtime & POCO-REAF Engine

/// Initializes all runtime components.
pub fn initialize_runtime() {
    println!("Initializing Zenith Runtime...");
    memory_manager::init_memory_manager();
    concurrency_manager::init_concurrency_manager();
    quantum::init_quantum_runtime();
    nano::init_nano_runtime();
    mts::init_mts_runtime();
    sankofa::init_sankofa_integration();
    universal_runtime::init_universal_runtime(); // Initialize Universal Runtime
    println!("Zenith Runtime initialized.");
}

/// Shuts down all runtime components.
pub fn shutdown_runtime() {
    println!("Shutting down Zenith Runtime...");
    universal_runtime::shutdown_universal_runtime(); // Shutdown Universal Runtime
    sankofa::shutdown_sankofa_integration();
    mts::shutdown_mts_runtime();
    nano::shutdown_nano_runtime();
    quantum::shutdown_quantum_runtime();
    concurrency_manager::shutdown_concurrency_manager();
    memory_manager::shutdown_memory_manager();
    println!("Zenith Runtime shut down.");
}
