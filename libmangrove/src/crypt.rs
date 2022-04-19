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
