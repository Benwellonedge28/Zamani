//! Nimbus OS: Security Kernel - Core Security Components
//!
//! This module aggregates and manages the core security components for Nimbus OS,
//! providing sandboxing, access control, encryption, and threat monitoring.

pub mod access_control; // Fine-grained Access Control
pub mod encryption_services;
pub mod sandbox_manager;
pub mod threat_intelligence; // Autonomous Threat Intelligence // Secure Sandboxing and Isolation // Cryptographic Services
#[cfg(feature = "full")]
// Re-export core types to simplify usage in other modules
// Note: `IsolationLevel` and `SandboxPolicy` were previously (mis)re-exported
// here but neither is defined by `sandbox_manager` nor used by any consumer
// under those names — the real sandboxing config type is `SandboxLevel`
// and the real access-policy type is `crate::nimbus_os::SandboxPolicy`, a
// different, unrelated concept. Only `SecureExecutionEnvironment` (an alias
// for `SandboxManager`) is genuinely constructed by consumers.
pub use self::sandbox_manager::SecureExecutionEnvironment;

/// Initializes all security kernel components.
pub fn init_security_kernel() {
    println!("Initializing Nimbus OS Security Kernel...");
    sandbox_manager::init_sandbox_manager();
    access_control::init_access_control();
    encryption_services::init_encryption_services();
    threat_intelligence::init_threat_intelligence(); // Initialize Threat Intelligence
    println!("Nimbus OS Security Kernel initialized.");
}

/// Shuts down all security kernel components.
pub fn shutdown_security_kernel() {
    println!("Shutting down Nimbus OS Security Kernel...");
    threat_intelligence::shutdown_threat_intelligence(); // Shutdown Threat Intelligence
    encryption_services::shutdown_encryption_services();
    access_control::shutdown_access_control();
    sandbox_manager::shutdown_sandbox_manager();
    println!("Nimbus OS Security Kernel shut down.");
}
