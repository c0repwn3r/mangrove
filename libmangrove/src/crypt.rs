//! # Various cryptographic helper functions to remove repetitive code

use arrayref::array_ref;
use ed25519_dalek::{Keypair, PublicKey as VerifyingKey, Signature, Signer, Verifier};
use rand_dalek::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs::File, io};

use crate::aes::AES256Cipher;

// mcrypt_sha256_file
/// Get the sha256 hash of the given file
/// ```
/// use std::env;
/// use libmangrove::crypt::mcrypt_sha256_file;
/// let string_hash: String = mcrypt_sha256_file(&String::from("../test/hash.txt")).unwrap();
/// assert_eq!(string_hash, "1805ddd21da13e7038470a391cc082680b7e680a2fc40ed7db01ee32a8c6cbd6");
/// ```
//
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

// mcrypt_sha256_verify_file
/// Validate that the provided file matches the sha256 hash
/// ```
/// use libmangrove::crypt::mcrypt_sha256_verify_file;
/// mcrypt_sha256_verify_file(&String::from("../test/hash.txt"), &String::from("1805ddd21da13e7038470a391cc082680b7e680a2fc40ed7db01ee32a8c6cbd6")).expect("");
/// ```
//
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

// PublicKey
/// Represents a verifying key, with a name and the actual key data
//
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    pub name: String,
    pub key_data: VerifyingKey,
}

// PrivateKey
/// Represents a signing key, with a name and the actual key data
//
#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateKey {
    pub name: String,
    pub key_data: Keypair,
}

impl PrivateKey {
    // generate
    /// Generate a randomized PrivateKey
    /// ```
    /// use libmangrove::crypt::PrivateKey;
    /// let private_key = PrivateKey::generate(String::from("test_key"));
    /// ```
    //
    pub fn generate(name: String) -> PrivateKey {
        let mut rng = OsRng {};
        let keypair = Keypair::generate(&mut rng);
        PrivateKey {
            name,
            key_data: keypair,
        }
    }

    // derive
    /// Derive a PublicKey from this PrivateKey
    /// ```
    /// use libmangrove::crypt::PrivateKey;
    /// let private_key = PrivateKey::generate(String::from("test_key"));
    /// let public_key = private_key.derive();
    /// ```
    //
    pub fn derive(&self) -> PublicKey {
        PublicKey {
            name: self.name.clone(),
            key_data: self.key_data.public,
        }
    }
}

// mcrypt_sha256_raw
/// Given a raw byte array, get the sha256 hash of it and return its digest in bytes
/// ```
/// use libmangrove::crypt::mcrypt_sha256_raw;
/// let input_bytes: [u8; 10] = [0x42u8;10];
/// let expected_bytes: Vec<u8> = vec![78, 93, 84, 245, 3, 112, 185, 54, 83, 61, 252, 178, 243, 84, 10, 36, 43, 93, 241, 47, 192, 99, 28, 222, 28, 41, 4, 146, 247, 189, 155, 254];
/// assert_eq!(mcrypt_sha256_raw(&input_bytes), expected_bytes);
/// ```
//
pub fn mcrypt_sha256_raw(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let data = hasher.finalize();
    data.to_vec()
}

// encrypt_package
/// Given a PrivateKey and any arbitrary data array, encrypt it using the Signed Package format and return the result as a byte array
/// ```
/// use libmangrove::crypt::{encrypt_package, PrivateKey};
/// let private_key = PrivateKey::generate(String::from("test_key"));
/// let data_to_encrypt: [u8; 5] = [0x42u8;5];
/// let encrypted_data = encrypt_package(&private_key, &data_to_encrypt).unwrap();
/// ```
//
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
    let mut aes_cipher = AES256Cipher::new(*key);
    let data_arr: &[u8] = data;
    let mut enc_data = aes_cipher.encrypt(data_arr);
    let mut header: Vec<u8> = vec![0x4d, 0x47, 0x56, 0x45];
    header.push(signature_b.len() as u8);
    header.append(&mut signature_b);
    header.push(0x00);
    header.append(&mut (enc_data.len() as u32).to_be_bytes().to_vec());
    header.append(&mut enc_data);
    header.push(0x42u8);
    Ok(header)
}

// decrypt_package
/// Validate and decrypt a package in the Signed Package format
/// ```
/// use libmangrove::crypt::{decrypt_package, encrypt_package, PrivateKey};
/// let private_key = PrivateKey::generate(String::from("test_key"));
/// let public_key = private_key.derive();
///
/// let data_to_encrypt: [u8; 5] = [0x42u8;5];
///
/// let encrypted_data = encrypt_package(&private_key, &data_to_encrypt).unwrap();
/// let decrypted_data = decrypt_package(&public_key, encrypted_data).unwrap();
/// assert_eq!(data_to_encrypt.to_vec(), decrypted_data);
/// ```
//
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

// debug_dump_package
/// Dump the provided encrypted data in the Signed Package Format to a string.
/// Optionally, decrypt the data if the correct public key is provided.
//
pub fn debug_dump_package(data: Vec<u8>, vkey: Option<&PublicKey>) -> String {
    let mut result = String::from("== Begin Package Dump ==\n");
    // Check for the magic
    if data[0] != 0x4d || data[1] != 0x47 || data[2] != 0x56 || data[3] != 0x45 {
        result += "| Magic: Not Present\n";
        result += "| Package State: INVALID\n";
        result += "== End package Dump ==";
        return result;
    }
    result += "| Magic: Present\n";
    // Get signature length
    let s_len = data[4] as usize;
    result += &*format!("| Signature Length: {s_len}\n");
    // Get signature bytes
    let s_dat = &data[5..5 + s_len];
    result += &*format!("| Signature Data: {:x?}\n", s_dat);
    // Validate package data
    result += "| Signature/Data Sentinel: ";
    if data[5 + s_len] != 0x0u8 {
        result += "Not Present\n";
        result += "| Package State: INVALID\n";
        result += "== End Package Dump ==";
    }
    result += "Present\n";
    // Derive key from signature
    let raw_key = mcrypt_sha256_raw(s_dat);
    let key = array_ref!(raw_key, 0, 32);
    result += &*format!("| Cipher Key: {:x?}", raw_key);
    let mut cipher = AES256Cipher::new(*key);
    let d_len = u32::from_be_bytes(*array_ref!(data[6 + s_len..10 + s_len], 0, 4)) as usize;
    result += &*format!("| Encrypted Data Length: {d_len}\n");
    let d_dat = &data[10 + s_len..10 + s_len + d_len];
    result += &*format!("| Encrypted Data: {:x?}\n", d_dat);
    // Validate package data
    result += "| End Sentinel: ";
    if data[10 + s_len + d_len] != 0x42u8 || data[data.len() - 1] != 0x42u8 {
        result += "Not Present\n";
        result += "| Package State: INVALID\n";
        result += "== End Package Dump ==";
        return result;
    }
    result += "Present\n";
    result += "| Package Structure: OK\n";
    // Package meets the proper structure
    // Decrypt package data
    let d_dat_dec = cipher.decrypt(d_dat);

    result += &*format!("| Decrypted Data: {:x?}\n", d_dat_dec);
    // Validate digital signature
    let sig = match Signature::from_bytes(s_dat) {
        Ok(sig) => sig,
        Err(_) => {
            result += "| Signature Load: Failure\n";
            result += "| Package State: INVALID\n";
            result += "== End Package Dump ==";
            return result;
        },
    };
    result += "| Signature Load: Success\n";
    if vkey.is_none() {
        result += "| Data Signature: Skipped (no public key)\n";
        result += "| Package State: OK\n";
        result += "== End Package Dump ==";
        return result;
    }
    match vkey.unwrap().key_data.verify(&d_dat_dec, &sig) {
        Ok(_) => (),
        Err(_) => {
            result += "| Data Signature: Failure\n";
            result += "| Package State: INVALID\n";
            result += "== End Package Dump ==";
            return result;
        }
    }
    result += "| Data Signature: OK\n";
    result += "| Package State: OK\n";
    result += "== End Package Dump ==";
    return result;
}

// is_signed_package
/// Determine if the provided data array has the correct structure of a package in the Signed Package Format.
/// Does not perform signature checks.
//
pub fn is_signed_package(data: Vec<u8>) -> bool {
    // Check for the magic
    if data[0] != 0x4d || data[1] != 0x47 || data[2] != 0x56 || data[3] != 0x45 {
        return false;
    }
    let s_len = data[4] as usize;
    // Get signature bytes
    let s_dat = &data[5..5 + s_len];

    if data[5 + s_len] != 0x0u8 {
        return false;
    }
    // Derive key from signature
    let raw_key = mcrypt_sha256_raw(s_dat);
    let key = array_ref!(raw_key, 0, 32);
    let mut cipher = AES256Cipher::new(*key);
    let d_len = u32::from_be_bytes(*array_ref!(data[6 + s_len..10 + s_len], 0, 4)) as usize;
    let d_dat = &data[10 + s_len..10 + s_len + d_len];
    // Validate package data
    if data[10 + s_len + d_len] != 0x42u8 || data[data.len() - 1] != 0x42u8 {
        return false;
    }
    // Package meets the proper structure
    // Validate digital signature
    match Signature::from_bytes(s_dat) {
        Ok(sig) => sig,
        Err(_) => {
            return false;
        },
    };
    true
}

