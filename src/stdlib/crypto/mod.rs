//! Zenith Universal Meta-Compiler (UMC) Standard Library: Cryptography Module
//!
//! This module aggregates and manages all cryptography-related components for Zenith.

pub mod asymmetric_encryption;
pub mod hashing;
pub mod quantum_identity; // Quantum-Secure Identity & Trust Fabric
pub mod quantum_safe_primitives;
pub mod signature;
pub mod symmetric_encryption;
pub mod tls;

// Re-export core types to simplify usage in other modules
pub use self::asymmetric_encryption::{PrivateKey, PublicKey};
pub use self::hashing::{Crypto, Hash};
pub use self::quantum_identity::ZeroKnowledgeProof;
pub use self::quantum_safe_primitives::QuantumKey;
pub use self::signature::Signature;
pub use self::symmetric_encryption::AesKey;
pub use self::tls::SecureCommunicationChannel; // Need to ensure ZeroKnowledgeProof is publicly accessible

/// Initializes all cryptography components.
pub fn init_crypto() {
    println!("Initializing Zenith Cryptography Module...");
    symmetric_encryption::init_symmetric_encryption();
    asymmetric_encryption::init_asymmetric_encryption();
    hashing::init_hashing();
    signature::init_signature();
    tls::init_tls();
    quantum_safe_primitives::init_quantum_safe_primitives(); // Initialize Quantum Identity
    quantum_identity::init_quantum_identity();
    println!("Zenith Cryptography Module initialized.");
}

/// Shuts down all cryptography components.
pub fn shutdown_crypto() {
    println!("Shutting down Zenith Cryptography Module...");
    quantum_identity::shutdown_quantum_identity(); // Shutdown Quantum Identity
    quantum_safe_primitives::shutdown_quantum_safe_primitives();
    tls::shutdown_tls();
    signature::shutdown_signature();
    hashing::shutdown_hashing();
    asymmetric_encryption::shutdown_asymmetric_encryption();
    symmetric_encryption::shutdown_symmetric_encryption();
    println!("Zenith Cryptography Module shut down.");
}
