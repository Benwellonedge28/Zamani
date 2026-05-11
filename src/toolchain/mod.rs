
//! Zenith Universal Meta-Compiler (UMC) Toolchain Integration
//!
//! This module defines the conceptual interfaces and components for integrating
//! the Zenith UMC with external development tools and systems. It encompasses
//! aspects like build management, package resolution, IDE support, and debugging.
//!
//! Key responsibilities include:
//! - **Build System Integration:** Defining how Zenith projects are built, compiled,
//!   and linked using tools like `cargo` (for Rust host) or a custom Zenith builder.
//! - **Package Management:** Conceptual support for `zenith-pkg` for dependency
//!   resolution and library management across paradigms.
//! - **IDE Integration:** Protocols and APIs for language servers (LSP), debugging (DAP),
//!   and other features within Integrated Development Environments.
//! - **Debugging Support:** Generating and consuming debug information.
//! - **Cross-Language Interoperability:** Mechanisms for interfacing Zenith code with
//!   components written in other languages, especially within the Nimbus ecosystem.

pub mod build;
pub mod package_manager;
pub mod ide_support;
pub mod debug_info;
pub mod interoperability;
pub mod zenith_project_config; // Expose the project config module

/// Initializes the conceptual Zenith Toolchain Integration components.
pub fn init_toolchain_integration() {
    println!("Initializing Zenith UMC Toolchain Integration...");
    build::init_build_system();
    package_manager::init_package_manager();
    ide_support::init_ide_support();
    debug_info::init_debug_info_gen();
    interoperability::init_interoperability_layer();
    // zenith_project_config::init_config_system(); // No-op for now
    println!("Zenith UMC Toolchain Integration initialized.");
}

/// Shuts down the conceptual Zenith Toolchain Integration components.
pub fn shutdown_toolchain_integration() {
    println!("Shutting down Zenith UMC Toolchain Integration...");
    // zenith_project_config::shutdown_config_system(); // No-op for now
    interoperability::shutdown_interoperability_layer();
    debug_info::shutdown_debug_info_gen();
    ide_support::shutdown_ide_support();
    package_manager::shutdown_package_manager();
    build::shutdown_build_system();
    println!("Zenith UMC Toolchain Integration shut down.");
}
