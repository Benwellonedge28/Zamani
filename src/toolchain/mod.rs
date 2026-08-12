//! Zamani Universal Meta-Compiler (UMC) Toolchain Components
//!
//! This module aggregates and manages all toolchain-related components
//! for Zamani, including code generation, self-evolution, and hardware
//! description language (HDL) integration.

pub mod build;
pub mod package_manager;
pub mod zamani_lsp;
pub mod autonomous_toolchain;
pub mod debug_info;
pub mod formal_verification; // Formal Verification Engine
pub mod causality_checker;   // Temporal Causality Checker
pub mod hyper_ascension;
pub mod hyper_evolution;
pub mod ide_support;
pub mod interoperability;
pub mod lang_integration;
pub mod meta_programming;
pub mod self_evolution;
pub mod zamani_debug;
pub mod zamani_test;
pub mod zbe_connector; // Autonomous Code Generation // New: For 1,000,000x recursive self-improvement

/// Initializes all toolchain components.
pub fn initialize_toolchain() {
    println!("Initializing Zamani Toolchain...");
    meta_programming::init_meta_programming();
    formal_verification::init_formal_verification();
    hyper_ascension::init_hyper_ascension();
    debug_info::init_debug_info_gen();
    ide_support::init_ide_support();
    interoperability::init_interoperability_layer();
    zamani_lsp::init_lsp();
    zamani_debug::init_debugger();
    build::init_build_system();
    package_manager::init_package_manager();
    println!("Zamani Toolchain initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain() {
    println!("Shutting down Zamani Toolchain...");
    formal_verification::shutdown_formal_verification();
    meta_programming::shutdown_meta_programming();
    interoperability::shutdown_interoperability_layer();
    ide_support::shutdown_ide_support();
    debug_info::shutdown_debug_info_gen();
    zamani_debug::shutdown_debugger();
    zamani_lsp::shutdown_lsp();
    package_manager::shutdown_package_manager();
    build::shutdown_build_system();
    println!("Zamani Toolchain shut down.");
}
