
//! Zenith Universal Meta-Compiler (UMC) Toolchain Integration
//!
//! This module orchestrates the integration of various toolchain components
//! with the UMC. It provides interfaces for debugging, IDE support,
//! static analysis, and formal verification, ensuring a comprehensive
//! development experience for Zenith programmers.

pub mod debug_info;
pub mod ide_support;
pub mod formal_verification;
pub mod hdl; // New module for HDL

/// Initializes all toolchain components.
pub fn init_toolchain_integration() {
    println!("Initializing Zenith UMC Toolchain Integration...");
    debug_info::init_debug_info();
    ide_support::init_ide_support();
    formal_verification::init_formal_verification();
    hdl::init_hdl(); // Initialize HDL module
    println!("Zenith UMC Toolchain Integration initialized.");
}

/// Shuts down all toolchain components.
pub fn shutdown_toolchain_integration() {
    println!("Shutting down Zenith UMC Toolchain Integration...");
    hdl::shutdown_hdl(); // Shutdown HDL module
    formal_verification::shutdown_formal_verification();
    ide_support::shutdown_ide_support();
    debug_info::shutdown_debug_info();
    println!("Zenith UMC Toolchain Integration shut down.");
}
