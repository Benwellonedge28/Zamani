
//! Zenith Universal Meta-Compiler (UMC) Standard Library: Cryptography Module
//!
//! This module aggregates and manages all cryptography-related components for Zenith.

pub mod symmetric_encryption;
pub mod asymmetric_encryption;
pub mod hashing;
pub mod signature;
pub mod tls;
pub mod quantum_safe_primitives;

// Re-export core types to simplify usage in other modules
pub use self::hashing::{Crypto, Hash};
pub use self::symmetric_encryption::AesKey;
pub use self::asymmetric_encryption::{PublicKey, PrivateKey};
pub use self::quantum_safe_primitives::QuantumKey;
pub use self::signature::Signature;
pub use self::tls::SecureCommunicationChannel;
pub use self::quantum_identity::ZeroKnowledgeProof; // Need to ensure ZeroKnowledgeProof is publicly accessible

/// Initializes all cryptography components.
pub fn init_crypto() {
    println!("Initializing Zenith Cryptography Module...");
    symmetric_encryption::init_symmetric_encryption();
    asymmetric_encryption::init_asymmetric_encryption();
    hashing::init_hashing();
    signature::init_signature();
    tls::init_tls();
    quantum_safe_primitives::init_quantum_safe_primitives(); // Initialize Quantum Identity
    println!("Zenith Cryptography Module initialized.");
}

/// Shuts down all cryptography components.
pub fn shutdown_crypto() {
    println!("Shutting down Zenith Cryptography Module..."); // Shutdown Quantum Identity
    quantum_safe_primitives::shutdown_quantum_safe_primitives();
    tls::shutdown_tls();
    signature::shutdown_signature();
    hashing::shutdown_hashing();
    asymmetric_encryption::shutdown_asymmetric_encryption();
    symmetric_encryption::shutdown_symmetric_encryption();
    println!("Zenith Cryptography Module shut down.");
}
