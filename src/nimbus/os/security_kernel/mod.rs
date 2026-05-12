
//! Nimbus OS: Security Kernel - Core Security Components
//!
//! This module aggregates and manages the core security components for Nimbus OS,
//! providing sandboxing, access control, encryption, and threat monitoring.

pub mod sandbox_manager; // Secure Sandboxing and Isolation
pub mod access_control; // Fine-grained Access Control
pub mod encryption_services; // Cryptographic Services
pub mod threat_intelligence; // New: Autonomous Threat Intelligence

// Re-export core types to simplify usage in other modules
pub use self::sandbox_manager::{SecureExecutionEnvironment, SandboxPolicy, IsolationLevel};

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
