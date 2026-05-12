
//! Zenith Universal Meta-Compiler (UMC) Toolchain: Formal Verification Module
//!
//! This module aggregates and manages all formal verification-related components
//! for Zenith, ensuring provable correctness and security for critical code and systems.

pub mod verifier; // Core Formal Verifier
pub mod model_checker; // Model Checking for Temporal Properties
pub mod theorem_prover; // Automated Theorem Proving
pub mod attestation_engine; // New: Continuous Formal Verification Attestation Engine

/// Initializes all formal verification components.
pub fn init_formal_verification() {
    println!("Initializing Zenith Formal Verification Module...");
    verifier::init_verifier();
    model_checker::init_model_checker();
    theorem_prover::init_theorem_prover();
    attestation_engine::init_attestation_engine(); // Initialize Attestation Engine
    println!("Zenith Formal Verification Module initialized.");
}

/// Shuts down all formal verification components.
pub fn shutdown_formal_verification() {
    println!("Shutting down Zenith Formal Verification Module...");
    attestation_engine::shutdown_attestation_engine(); // Shutdown Attestation Engine
    theorem_prover::shutdown_theorem_prover();
    model_checker::shutdown_model_checker();
    verifier::shutdown_verifier();
    println!("Zenith Formal Verification Module shut down.");
}
