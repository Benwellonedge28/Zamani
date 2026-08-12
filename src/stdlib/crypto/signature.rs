#![allow(unused_imports, dead_code, unused_variables)]
//! Zamani — digital signature module implementation

use ed25519_dalek::{SigningKey, VerifyingKey, Signature as EdSignature, Signer, Verifier};
use rand::RngCore;

pub fn init_signature() {
    println!("  - Initializing Signature...");
}
pub fn shutdown_signature() {
    println!("  - Shutting down Signature...");
}

pub fn generate_ed25519_keypair() -> (SigningKey, VerifyingKey) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let signing_key = SigningKey::from_bytes(&bytes);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

pub fn sign_ed25519(signing_key: &SigningKey, message: &[u8]) -> EdSignature {
    signing_key.sign(message)
}

pub fn verify_ed25519(verifying_key: &VerifyingKey, message: &[u8], signature: &EdSignature) -> bool {
    verifying_key.verify(message, signature).is_ok()
}
