//! Zenith Runtime: Core Runtime Components
//!
//! This module aggregates and manages the core runtime components for Zenith,
//! providing essential services for application execution, memory management,
//! and concurrency.

pub mod cloud_network_security;
pub mod core;
pub mod distributed;
pub mod memory_manager; // Memory Allocation and Garbage Collection
pub mod mts; // Multi-Timeline System for speculative execution
pub mod nano; // Nano Runtime Environment
pub mod nimbus_os;
pub mod nimbus_os_interface;
pub mod quantum; // Quantum Runtime Environment
pub mod sankofa;
pub mod universal_runtime; // Long-term memory and learning integration

/// Initializes all runtime components.
pub fn initialize_runtime() {
    println!("Initializing Zenith Runtime...");
    memory_manager::init_memory_manager();
    quantum::init_quantum_runtime();
    nano::init_nano_runtime();
    mts::init_mts_runtime();
    sankofa::init_sankofa_integration(); // Initialize Universal Runtime
    cloud_network_security::init_cloud_network_security();
    core::init_core();
    distributed::init_distributed();
    nimbus_os::init_nimbus_os();
    nimbus_os_interface::init_nimbus_os_interface();
    universal_runtime::init_universal_runtime();
    println!("Zenith Runtime initialized.");
}

/// Shuts down all runtime components.
pub fn shutdown_runtime() {
    println!("Shutting down Zenith Runtime..."); // Shutdown Universal Runtime
    sankofa::shutdown_sankofa_integration();
    mts::shutdown_mts_runtime();
    nano::shutdown_nano_runtime();
    quantum::shutdown_quantum_runtime();
    memory_manager::shutdown_memory_manager();
    universal_runtime::shutdown_universal_runtime();
    nimbus_os_interface::shutdown_nimbus_os_interface();
    nimbus_os::shutdown_nimbus_os();
    distributed::shutdown_distributed();
    core::shutdown_core();
    cloud_network_security::shutdown_cloud_network_security();
    println!("Zenith Runtime shut down.");
}
