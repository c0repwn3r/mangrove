//! # Various cryptographic helper functions to remove repetitive code

use std::{fs::File, io};
use std::error::Error;

use arrayref::array_ref;
use ed25519_dalek::{Keypair, PublicKey as VerifyingKey, Signature, Signer, Verifier};
use rand_dalek::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::aes::AES256Cipher;
use crate::trustcache::Trustcache;

// mcrypt_sha256_file
/// Get the sha256 hash of the given file
/// ```
/// use std::env;
/// use libmangrove::crypt::mcrypt_sha256_file;
/// let string_hash: String = mcrypt_sha256_file(&String::from("../test/hash.txt")).unwrap();
/// assert_eq!(string_hash, "1805ddd21da13e7038470a391cc082680b7e680a2fc40ed7db01ee32a8c6cbd6");
/// ```
/// # Errors
/// `mcrypt_sha256_file` may return an error for the following reasons:
/// - if it fails to open the provided file
/// - if it fails to copy data from the file to the hasher
pub fn mcrypt_sha256_file(filename: &String) -> Result<String, Box<dyn Error>> {
    let file_r = File::open(filename);
    let mut file_ptr = match file_r {
        Ok(ptr) => ptr,
        Err(err) => return Err(format!("Unable to open file for reading: {}", err).into()),
    };
    let mut hasher = Sha256::new();
    match io::copy(&mut file_ptr, &mut hasher) {
        Ok(_) => (),
        Err(err) => return Err(format!("Unable to copy file data: {}", err).into()),
    }
    // IntelliJ platform users: Ignore the warning here. This is a bug in the IntelliJ Rust plugin.
    Ok(format!("{:x}", hasher.finalize()))
}

// mcrypt_sha256_verify_file
/// Validate that the provided file matches the sha256 hash
/// ```
/// use libmangrove::crypt::mcrypt_sha256_verify_file;
/// mcrypt_sha256_verify_file(&String::from("../test/hash.txt"), &String::from("1805ddd21da13e7038470a391cc082680b7e680a2fc40ed7db01ee32a8c6cbd6")).expect("");
/// ```
/// # Errors
/// This function will error if:
/// - the file could not be hashed (see `mcrypt_sha256_file`)
/// - the hash does not match
//
pub fn mcrypt_sha256_verify_file(filename: &String, expect: &String) -> Result<(), Box<dyn Error>> {
    let sha256 = match mcrypt_sha256_file(filename) {
        Ok(hash) => hash,
        Err(error) => return Err(error),
    };
    if &sha256 != expect {
        return Err(format!("Hash of {} does not match {}", sha256, expect).into());
    }
    Ok(())
}

// PublicKey
/// Represents a verifying key, with a name and the actual key data
//
#[derive(Serialize, Deserialize, Debug)]
pub struct PublicKey {
    /// The key name, if loaded from a public key file. This is almost always unused, and will most of the time be \_\_anonymous\_\_
    pub name: String,
    /// The ed25519 public key data
    pub key_data: VerifyingKey,
}

// PrivateKey
/// Represents a signing key, with a name and the actual key data
//
#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateKey {
    /// The key name, if loaded from a private key file. This is almost always unused, and will most of the time be \_\_anonymous\_\_
    pub name: String,
    /// The ed25519 keypair. This contains the public key as well and as such can be used to derive a public key.
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

    // to_anonymous
    /// Serialize this PrivateKey into a base64-encoded anonymous private key.
    pub fn to_anonymous(&self) -> String {
        base64::encode(&self.key_data.to_bytes())
    }

    // from_anonymous
    /// Attempt to create a new PrivateKey from an anonymous base64-encoded private key.
    /// # Errors
    /// This function may error if:
    /// - an error occured trying to convert "__anonymous__" to a String
    /// - an error occured while decoding the base64 key
    /// - an error occured while loading the base64 key into a Keypair
    pub fn from_anonymous(anonymous: &String) -> Result<Self, Box<dyn Error>> {
        Ok(PrivateKey {
            name: "__anonymous__".parse()?,
            key_data: Keypair::from_bytes(&*base64::decode(anonymous)?)?
        })
    }
}

impl PublicKey {
    // to_anonymous
    /// Serialize this PublicKey into a base64-encoded anonymous public key.
    pub fn to_anonymous(&self) -> String {
        base64::encode(&self.key_data.to_bytes())
    }

    // from_anonymous
    /// Attempt to create a new PublicKey from an anonymous base64-encoded public key.
    /// # Errors
    /// This function may error if:
    /// - an error occured trying to convert "__anonymous__" to a String
    /// - an error occured while decoding the base64 key
    /// - an error occured while loading the base64 key into a VerifyingKey
    pub fn from_anonymous(anonymous: &String) -> Result<Self, Box<dyn Error>> {
        Ok(PublicKey {
            name: "__anonymous__".parse()?,
            key_data: VerifyingKey::from_bytes(&*base64::decode(anonymous)?)?
        })
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
/// # Errors
/// This function may return an error if the signature fails sanity checks or the data length is over
//
pub fn encrypt_package(key: &PrivateKey, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
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
        return Err("Signature failed basic sanity checks".into())
    }
    let mut signature_b = signature.to_bytes().to_vec();
    let aes_key = mcrypt_sha256_raw(&signature_b);
    let key = array_ref!(aes_key, 0, 32);
    let mut aes_cipher = AES256Cipher::new(*key);
    let data_arr: &[u8] = data;
    let mut enc_data = aes_cipher.encrypt(data_arr);
    let mut header: Vec<u8> = vec![0x4d, 0x47, 0x56, 0x45];
    #[allow(clippy::cast_possible_truncation)] // Signature length will always be 64, which is within the range of u8
    header.push(signature_b.len() as u8);
    header.append(&mut signature_b);
    header.push(0x00);

    /* Fixes a data truncation error caused by package data lengths above 4294967295 bytes. */
    if enc_data.len() > u32::MAX as usize {
        return Err(format!("Data length {} is above maximum of {}", enc_data.len(), u32::MAX).into());
    }
    #[allow(clippy::cast_possible_truncation)] // bounds checked above
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
/// let decrypted_data = decrypt_package(&public_key, &encrypted_data[..]).unwrap();
/// assert_eq!(data_to_encrypt.to_vec(), decrypted_data);
/// ```
/// # Errors
/// This function will error if:
/// - the magic is missing
/// - the start/data sentinel is missing
/// - end sentinel is missing
/// - the signature could not be loaded
/// - the digital signature was invalid
//
pub fn decrypt_package(vkey: &PublicKey, data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    // Check for the magic
    if data[0] != 0x4d || data[1] != 0x47 || data[2] != 0x56 || data[3] != 0x45 {
        return Err("Not an encrypted package (magic missing)".into());
    }
    // Get signature length
    let s_len = data[4] as usize;
    // Get signature bytes
    let s_dat = &data[5..5 + s_len];
    // Validate package data
    if data[5 + s_len] != 0x0u8 {
        return Err("Package has been corrupt (s/d sentinel missing)".into());
    }
    // Derive key from signature
    let raw_key = mcrypt_sha256_raw(s_dat);
    let key = array_ref!(raw_key, 0, 32);
    let mut cipher = AES256Cipher::new(*key);
    let d_len = u32::from_be_bytes(*array_ref!(data[6 + s_len..10 + s_len], 0, 4)) as usize;
    let d_dat = &data[10 + s_len..10 + s_len + d_len];
    // Validate package data
    if data[10 + s_len + d_len] != 0x42u8 || data[data.len() - 1] != 0x42u8 {
        return Err("Package has been corrupt (end sentinel missing)".into());
    }
    // Package meets the proper structure
    // Decrypt package data
    let d_dat_dec = cipher.decrypt(d_dat);
    // Validate digital signature
    let sig = match Signature::from_bytes(s_dat) {
        Ok(sig) => sig,
        Err(err) => return Err(format!("Failed to load signature data: {}", err).into()),
    };
    match vkey.key_data.verify(&d_dat_dec, &sig) {
        Ok(_) => (),
        Err(err) => return Err(format!("The digital signature is invalid: {}", err).into()),
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
    result += &*format!("| Cipher Key: {:x?}\n", raw_key);
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
    let sig = if let Ok(sig) = Signature::from_bytes(s_dat) { sig } else {
        result += "| Signature Load: Failure\n";
        result += "| Package State: INVALID\n";
        result += "== End Package Dump ==";
        return result;
    };
    result += "| Signature Load: Success\n";
    if vkey.is_none() {
        result += "| Data Signature: Skipped (no public key)\n";
        result += "| Package State: OK\n";
        result += "== End Package Dump ==";
        return result;
    }
    let vkeyv = match vkey {
        Some(k) => k,
        None => return result
    };
    if vkeyv.key_data.verify(&d_dat_dec, &sig).is_err() {
        result += "| Data Signature: Failure\n";
        result += "| Package State: INVALID\n";
        result += "== End Package Dump ==";
        return result;
    }
    result += "| Data Signature: OK\n";
    result += "| Package State: OK\n";
    result += "== End Package Dump ==";
    result
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
    let d_len = u32::from_be_bytes(*array_ref!(data[6 + s_len..10 + s_len], 0, 4)) as usize;
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

// find_key
/// Try every public key in the trustcache against the provided SPF data and try to find the key it is encrypted with.
//
pub fn find_key(data: &[u8], trustcache: &Trustcache) -> Option<PublicKey> {
    // try known public keys
    for key in &trustcache.keydb.known_pubkeys {
        // load __anonymous__ key
        let pk = match PublicKey::from_anonymous(&key.clone()) {
            Ok(k) => k,
            Err(_) => return None
        };
        if decrypt_package(&pk, data).is_ok() {
            return Some(pk);
        }
    }
    // try known private keys
    for key in &trustcache.keydb.known_privkeys {
        // load __anonymous__ key
        let sk = match PrivateKey::from_anonymous(&key.clone()) {
            Ok(k) => k,
            Err(_) => return None
        };
        let pk = sk.derive();
        if decrypt_package(&pk, data).is_ok() {
            return Some(pk);
        }
    }
    None
}