use arrayref::array_ref;
use ed25519_dalek::{Keypair, PublicKey as VerifyingKey, Signature, Signer, Verifier};
use rand_dalek::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs::File, io};

use crate::aes::AES256Cipher;

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
    pub key_data: VerifyingKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateKey {
    pub name: String,
    pub key_data: Keypair,
}

impl PrivateKey {
    pub fn generate(name: String) -> PrivateKey {
        let mut rng = OsRng {};
        let keypair = Keypair::generate(&mut rng);
        PrivateKey {
            name,
            key_data: keypair,
        }
    }

    pub fn derive(&self) -> PublicKey {
        PublicKey {
            name: self.name.clone(),
            key_data: self.key_data.public,
        }
    }
}

pub fn mcrypt_sha256_raw(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let data = hasher.finalize();
    data.to_vec()
}

pub fn encrypt_package(key: &PrivateKey, data: &[u8]) -> Result<Vec<u8>, String> {
    // Encrypted package format:
    // field  value         description
    //
    // magic  0x4d475645   'MGVE' ascii, this is the magic
    // s_len  0x??          Signature length (in bytes)
    // s_dat  0x??*s_len    Signature data (s_len bytes)
    // s_sep  0x00          Null byte seperator
    // d_len  0x????????    Data length (in bytes)
    // d_dat  0x??*d_len    Package data (d_len bytes)
    // p_val  0x42          End sentinel
    let signature: Signature = key.key_data.sign(data);
    if key.key_data.verify(data, &signature).is_err() {
        panic!("Signature immediately failed verification");
    }
    let mut signature_b = signature.to_bytes().to_vec();
    let aes_key = mcrypt_sha256_raw(&signature_b);
    let key = array_ref!(aes_key, 0, 32);
    println!("{:x?}", key);
    let mut aes_cipher = AES256Cipher::new(*key);
    let data_arr: &[u8] = data;
    let mut enc_data = aes_cipher.encrypt(data_arr);
    let mut header: Vec<u8> = vec![0x4d, 0x47, 0x56, 0x45];
    header.push(signature_b.len() as u8);
    header.append(&mut signature_b);
    header.push(0x00);
    header.append(&mut (enc_data.len() as u32).to_be_bytes().to_vec());
    header.append(&mut enc_data);
    println!("{}", header.len() - 1);
    print!("offset of p_val: {} to ", header.len());
    header.push(0x42u8);
    println!("{}", header.len() - 1);
    Ok(header)
}

pub fn decrypt_package(vkey: &PublicKey, data: Vec<u8>) -> Result<Vec<u8>, String> {
    // Check for the magic
    if data[0] != 0x4d || data[1] != 0x47 || data[2] != 0x56 || data[3] != 0x45 {
        return Err("Not an encrypted package (magic missing)".to_string());
    }
    // Get signature length
    let s_len = data[4] as usize;
    // Get signature bytes
    let s_dat = &data[5..5 + s_len];
    // Validate package data
    if data[5 + s_len] != 0x0u8 {
        return Err("Package has been corrupt (s/d sentinel missing)".to_string());
    }
    // Derive key from signature
    let raw_key = mcrypt_sha256_raw(s_dat);
    let key = array_ref!(raw_key, 0, 32);
    let mut cipher = AES256Cipher::new(*key);
    let d_len = u32::from_be_bytes(*array_ref!(data[6 + s_len..10 + s_len], 0, 4)) as usize;
    let d_dat = &data[10 + s_len..10 + s_len + d_len];
    // Validate package data
    if data[10 + s_len + d_len] != 0x42u8 || data[data.len() - 1] != 0x42u8 {
        return Err("Package has been corrupt (end sentinel missing)".to_string());
    }
    // Package meets the proper structure
    // Decrypt package data
    let d_dat_dec = cipher.decrypt(d_dat);
    // Validate digital signature
    let sig = match Signature::from_bytes(s_dat) {
        Ok(sig) => sig,
        Err(err) => return Err(format!("Failed to load signature data: {}", err)),
    };
    match vkey.key_data.verify(&d_dat_dec, &sig) {
        Ok(_) => (),
        Err(err) => return Err(format!("The digital signature is invalid: {}", err)),
    }
    // Signature is valid, strip extra data and return
    Ok(d_dat_dec)
}
