//! Zenith Universal Meta-Compiler (UMC) Toolchain Components
//!
//! This module aggregates and manages all toolchain-related components
//! for Zenith, including code generation, self-evolution, and hardware
//! description language (HDL) integration.

pub mod autonomous_toolchain;
pub mod build;
pub mod debug_info;
pub mod hyper_evolution;
pub mod ide_support;
pub mod interoperability;
pub mod lang_integration;
pub mod meta_programming;
pub mod package_manager;
pub mod zbe_connector;
pub mod zenith_debug;
pub mod zenith_lsp;
pub mod zenith_test; // Autonomous Code Generation
                     // (gated) pub mod self_evolution;
pub mod formal_verification; // Formal Verification Engine
pub mod hyper_ascension; // New: For 1,000,000x recursive self-improvement

/// Initializes all toolchain components.
pub fn initialize_toolchain() {
    println!("Initializing Zenith Toolchain...");
    meta_programming::init_meta_programming();
    formal_verification::init_formal_verification();
    hyper_ascension::init_hyper_ascension(); // Initialize Hyper-Ascension
    autonomous_toolchain::init_autonomous_toolchain();
    build::init_build();
    debug_info::init_debug_info();
    hyper_evolution::init_hyper_evolution();
    ide_support::init_ide_support();
    interoperability::init_interoperability();
    lang_integration::init_lang_integration();
    package_manager::init_package_manager();
    self_evolution::init_self_evolution();
    zbe_connector::init_zbe_connector();
    zenith_debug::init_zenith_debug();
    zenith_lsp::init_zenith_lsp();
    zenith_test::init_zenith_test();
    println!("Zenith Toolchain initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain() {
    println!("Shutting down Zenith Toolchain...");
    hyper_ascension::shutdown_hyper_ascension(); // Shutdown Hyper-Ascension
    formal_verification::shutdown_formal_verification();
    meta_programming::shutdown_meta_programming();
    zenith_test::shutdown_zenith_test();
    zenith_lsp::shutdown_zenith_lsp();
    zenith_debug::shutdown_zenith_debug();
    zbe_connector::shutdown_zbe_connector();
    self_evolution::shutdown_self_evolution();
    package_manager::shutdown_package_manager();
    lang_integration::shutdown_lang_integration();
    interoperability::shutdown_interoperability();
    ide_support::shutdown_ide_support();
    hyper_evolution::shutdown_hyper_evolution();
    debug_info::shutdown_debug_info();
    build::shutdown_build();
    autonomous_toolchain::shutdown_autonomous_toolchain();
    println!("Zenith Toolchain shut down.");
}
