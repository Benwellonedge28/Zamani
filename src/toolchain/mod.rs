
//! Zenith Universal Meta-Compiler (UMC) Toolchain Integration
//!
//! This module orchestrates the integration of various toolchain components
//! with the UMC. It provides interfaces for debugging, IDE support,
//! static analysis, and formal verification, ensuring a comprehensive
//! development experience for Zenith programmers.

pub mod debug_info;
pub mod ide_support;
pub mod formal_verification;
pub mod hdl;
pub mod self_evolution;
pub mod lang_integration; // New module for Language Integration

/// Initializes all toolchain components.
pub fn init_toolchain_integration() {
    println!("Initializing Zenith UMC Toolchain Integration...");
    debug_info::init_debug_info();
    ide_support::init_ide_support();
    formal_verification::init_formal_verification();
    hdl::init_hdl();
    self_evolution::init_self_evolution();
    lang_integration::init_lang_integration(); // Initialize Language Integration module
    println!("Zenith UMC Toolchain Integration initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain_integration() {
    println!("Shutting down Zenith UMC Toolchain Integration...");
    lang_integration::fn_shutdown_lang_integration(); // Shutdown Language Integration module
    self_evolution::shutdown_self_evolution(); 
    hdl::shutdown_hdl(); 
    formal_verification::shutdown_formal_verification();
    ide_support::shutdown_ide_support();
    debug_info::shutdown_debug_info();
    println!("Zenith UMC Toolchain Integration shut down.");
}
