#![allow(unused_imports, dead_code, unused_variables)]
//! Zamani — asymmetric encryption module implementation

use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt};
use rand::thread_rng;

pub fn init_asymmetric_encryption() {}
pub fn shutdown_asymmetric_encryption() {}

pub fn generate_rsa_keys(bits: usize) -> Result<(RsaPrivateKey, RsaPublicKey), String> {
    let mut rng = thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, bits).map_err(|e| format!("Key generation error: {:?}", e))?;
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((priv_key, pub_key))
}

pub fn rsa_encrypt(pub_key: &RsaPublicKey, data: &[u8]) -> Result<Vec<u8>, String> {
    let mut rng = thread_rng();
    pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, data).map_err(|e| format!("Encryption error: {:?}", e))
}

pub fn rsa_decrypt(priv_key: &RsaPrivateKey, ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    priv_key.decrypt(Pkcs1v15Encrypt, ciphertext).map_err(|e| format!("Decryption error: {:?}", e))
}
