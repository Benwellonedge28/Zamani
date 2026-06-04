
//! Zenith Universal Meta-Compiler (UMC) Toolchain Components
//!
//! This module aggregates and manages all toolchain-related components
//! for Zenith, including code generation, self-evolution, and hardware
//! description language (HDL) integration.

pub mod meta_programming; // Autonomous Code Generation
// (gated) pub mod self_evolution;
pub mod formal_verification; // Formal Verification Engine
pub mod autonomous_toolchain; // The Orchestrator for the entire toolchain
pub mod zbe_connector; // Zenith Bridge Extension (ZBE) Connector for IDE/Editor integration
pub mod hyper_ascension; // New: For 1,000,000x recursive self-improvement

/// Initializes all toolchain components.
pub fn initialize_toolchain() {
    println!("Initializing Zenith Toolchain...");
    meta_programming::init_meta_programming();
    hdl::init_hdl();
    formal_verification::init_formal_verification();
    autonomous_toolchain::init_autonomous_toolchain();
    zbe_connector::init_zbe_connector();
    hyper_ascension::init_hyper_ascension(); // Initialize Hyper-Ascension
    println!("Zenith Toolchain initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain() {
    println!("Shutting down Zenith Toolchain...");
    hyper_ascension::shutdown_hyper_ascension(); // Shutdown Hyper-Ascension
    zbe_connector::shutdown_zbe_connector();
    autonomous_toolchain::shutdown_autonomous_toolchain();
    formal_verification::shutdown_formal_verification();
    hdl::shutdown_hdl();
    meta_programming::shutdown_meta_programming();
    println!("Zenith Toolchain shut down.");
}
