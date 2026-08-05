//! Zamani Universal Meta-Compiler (UMC) Toolchain Components
//!
//! This module aggregates and manages all toolchain-related components
//! for Zamani, including code generation, self-evolution, and hardware
//! description language (HDL) integration.

// Components with dedicated, lighter-weight feature flags so they can be
// built (and containerized) independently of the rest of the aspirational
// `full` toolchain surface, most of which does not compile yet.
#[cfg(any(feature = "buildsystem", feature = "full"))]
pub mod build;
#[cfg(any(feature = "buildsystem", feature = "full"))]
pub mod package_manager;
#[cfg(any(feature = "lsp", feature = "full"))]
pub mod zamani_lsp;

// Everything below is only wired in under the full aspirational build.
#[cfg(feature = "full")]
pub mod autonomous_toolchain;
#[cfg(feature = "full")]
pub mod debug_info;
#[cfg(feature = "full")]
pub mod formal_verification; // Formal Verification Engine
#[cfg(feature = "full")]
pub mod hyper_ascension;
#[cfg(feature = "full")]
pub mod hyper_evolution;
#[cfg(feature = "full")]
pub mod ide_support;
#[cfg(feature = "full")]
pub mod interoperability;
#[cfg(feature = "full")]
pub mod lang_integration;
#[cfg(feature = "full")]
pub mod meta_programming;
#[cfg(feature = "full")]
pub mod self_evolution;
#[cfg(feature = "full")]
pub mod zamani_debug;
#[cfg(feature = "full")]
pub mod zamani_test;
#[cfg(feature = "full")]
pub mod zbe_connector; // Autonomous Code Generation // New: For 1,000,000x recursive self-improvement

/// Initializes all toolchain components.
///
/// Components exposing free-standing `init_*`/`shutdown_*` functions are
/// initialized here directly; components that model themselves as structs
/// (e.g. `ZamaniLsp::new()`, `AutonomousToolchain::new()`) are constructed
/// by their own callers on demand instead, since they carry per-session
/// state that doesn't belong in a single global instance.
pub fn initialize_toolchain() {
    println!("Initializing Zamani Toolchain...");
    #[cfg(feature = "full")]
    {
        meta_programming::init_meta_programming();
        formal_verification::init_formal_verification();
        hyper_ascension::init_hyper_ascension();
        debug_info::init_debug_info_gen();
        ide_support::init_ide_support();
        interoperability::init_interoperability_layer();
    }
    #[cfg(any(feature = "buildsystem", feature = "full"))]
    {
        build::init_build_system();
        package_manager::init_package_manager();
    }
    println!("Zamani Toolchain initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain() {
    println!("Shutting down Zamani Toolchain...");
    #[cfg(feature = "full")]
    {
        formal_verification::shutdown_formal_verification();
        meta_programming::shutdown_meta_programming();
        interoperability::shutdown_interoperability_layer();
        ide_support::shutdown_ide_support();
        debug_info::shutdown_debug_info_gen();
    }
    #[cfg(any(feature = "buildsystem", feature = "full"))]
    {
        package_manager::shutdown_package_manager();
        build::shutdown_build_system();
    }
    println!("Zamani Toolchain shut down.");
}
