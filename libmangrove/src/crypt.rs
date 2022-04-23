use ed25519_dalek::{Keypair, PublicKey as VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs::File, io};

pub fn mcrypt_sha256_file(filename: &String) -> Result<String, String> {
    let file_r = File::open(filename);
    let mut file_ptr = match file_r {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Unable to open file for reading: {}", err)),
    };
    let mut hasher = Sha256::new();
    match io::copy(&mut file_ptr, &mut hasher) {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to copy file data: {}", err)),
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn mcrypt_sha256_verify_file(filename: &String, expect: &String) -> Result<(), String> {
    let sha256 = match mcrypt_sha256_file(filename) {
        Ok(hash) => hash,
        Err(error) => return Err(error),
    };
    if &sha256 != expect {
        return Err(format!("Hash of {} does not match {}", sha256, expect));
    } else {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub name: String,
    pub fingerprint: String,
    pub key_data: VerifyingKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateKey {
    pub name: String,
    pub fingerprint: String,
    pub key_data: Keypair,
}
/* 
pub fn encrypt_package(key: &PrivateKey, data: Vec<u8>) -> Result<Vec<u8>, String> {
    let signature: Signature = key.key_data.sign(&data);
    let aes_key = match argon2::hash_raw(&signature.to_bytes(), b"mbs-mgve", &Config::default()) {
        Ok(key) => key,
        Err(err) => return Err(format!("Failed to derive encryption key: {}", err))
    };
    
    Ok(vec![])
}

pub fn decrypt_package(key: &PublicKey, data: Vec<u8>) -> Result<Vec<u8>, String> {
    Ok(vec![])
}*/
