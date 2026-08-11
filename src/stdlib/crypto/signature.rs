#![allow(unused_imports, dead_code, unused_variables)]
//! Zamani — digital signature module implementation

use ed25519_dalek::{Keypair, Signer, Verifier, Signature, PublicKey, SecretKey};
use rand::rngs::OsRng;

pub fn init_signature() {}
pub fn shutdown_signature() {}

pub fn generate_ed25519_keypair() -> Keypair {
    let mut csprng = OsRng;
    Keypair::generate(&mut csprng)
}

pub fn sign_ed25519(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn verify_ed25519(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    public_key.verify(message, signature).is_ok()
}
