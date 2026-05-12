
//! Zenith Universal Meta-Compiler (UMC) Toolchain Components
//!
//! This module aggregates and manages all toolchain-related components
//! for Zenith, including code generation, self-evolution, and hardware
//! description language (HDL) integration.

pub mod meta_programming; // Autonomous Code Generation
pub mod self_evolution;   // Autonomous Self-Adjustment and Improvement
pub mod hdl;              // Hardware Description Language Integration
pub mod formal_verification; // Formal Verification Engine
pub mod autonomous_toolchain; // The Orchestrator for the entire toolchain
pub mod zbe_connector; // New: Zenith Bridge Extension (ZBE) Connector for IDE/Editor integration

/// Initializes all toolchain components.
pub fn initialize_toolchain() {
    println!("Initializing Zenith Toolchain...");
    meta_programming::init_meta_programming();
    self_evolution::init_self_evolution();
    hdl::init_hdl();
    formal_verification::init_formal_verification();
    autonomous_toolchain::init_autonomous_toolchain();
    zbe_connector::init_zbe_connector(); // Initialize ZBE Connector
    println!("Zenith Toolchain initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain() {
    println!("Shutting down Zenith Toolchain...");
    zbe_connector::shutdown_zbe_connector(); // Shutdown ZBE Connector
    autonomous_toolchain::shutdown_autonomous_toolchain();
    formal_verification::shutdown_formal_verification();
    hdl::shutdown_hdl();
    self_evolution::shutdown_self_evolution();
    meta_programming::shutdown_meta_programming();
    println!("Zenith Toolchain shut down.");
}
