
//! Zenith Universal Meta-Compiler (UMC) Runtime
//!
//! This module aggregates and manages all core runtime components for Zenith.
//! It provides the execution environment for Zenith programs, orchestrating
//! interactions between the Nimbus OS, multi-paradigm hardware (Z-MMP),
//! and the various standard library and application components.
//!
//! This includes initialization and shutdown procedures for all major runtime subsystems.

pub mod quantum;
pub mod nano;
pub mod mts;
pub mod sankofa;
pub mod nimbus_os_interface; // Provides high-level interfaces to Nimbus OS
pub mod cloud_network_security; // New module for Autonomous Cloud & Network Security

/// Initializes all Zenith runtime components.
pub fn initialize_runtime() {
    println!("Initializing Zenith UMC Runtime...");
    nimbus_os_interface::init_nimbus_os_interface(); // Initialize the OS interface first
    quantum::init_quantum_runtime();
    nano::init_nano_runtime();
    mts::init_mts_runtime();
    sankofa::init_sankofa_runtime();
    cloud_network_security::init_cloud_network_security(); // Initialize Cloud & Network Security module
    println!("Zenith UMC Runtime initialized.");
}

/// Shuts down all Zenith runtime components.
pub fn shutdown_runtime() {
    println!("Shutting down Zenith UMC Runtime...");
    cloud_network_security::shutdown_cloud_network_security(); // Shutdown Cloud & Network Security module
    sankofa::shutdown_sankofa_runtime();
    mts::shutdown_mts_runtime();
    nano::shutdown_nano_runtime();
    quantum::shutdown_quantum_runtime();
    nimbus_os_interface::shutdown_nimbus_os_interface();
    println!("Zenith UMC Runtime shut down.");
}
