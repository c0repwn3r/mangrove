//! # High-level pure-rust AES implementation using the `aes` crate as a backend
//! This is a very high-level aes cipher implementation that is used in libmangrove. It uses the `aes` crate as a backend.
//! WARNING: This code itself has not been independently verified to be proper usage.
//! The `aes` crate HAS received an independent security audit and no major issues were found.
//! Use at your own risk.

use crate::aes_cipher;
use aes::cipher::BlockSizeUser;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::{Aes128, Aes192, Aes256};
use arrayref::array_ref;
// !!
// !! WARNING! THIS CODE HAS *NOT* BEEN INDEPENDENTLY VERIFIED.
// !! IT RELIES ON THE `aes` CRATE TO DO THE ACTUAL CIPHER, WHICH HAS BEEN AUDITED.
// !! USE AT YOUR OWN RISK!
//

aes_cipher!(AES128Cipher, Aes128, 16);
aes_cipher!(AES192Cipher, Aes192, 24);
aes_cipher!(AES256Cipher, Aes256, 32);

// aes_cipher!
/// This macro is used to generate the AES???Cipher structs. It is not useful otherwise.
#[macro_export]
#[allow(clippy::module_name_repetitions)]
macro_rules! aes_cipher {
    ($struct_name:ident,$crypto_backend:tt,$ks:expr) => {
        /// A wrapper around a bare AES construct.
        /// You shouldn't construct this directly; use the `new` function instead
        pub struct $struct_name {
            key: [u8; $ks],
            bs: usize,
        }

        impl $struct_name {
            // new
            /// Given a fixed-size array containing a key, create a new AES construct.
            /// # Example
            /// ```rust
            /// // Due to an annoying limitation with triple-slash doc comments, I have to pick one cipher to use here. Any work
            /// use libmangrove::aes::AES128Cipher;
            /// let cipher: AES128Cipher = AES128Cipher::new([0; 16]);
            /// ```
            pub fn new(key: [u8; $ks]) -> Self {
                Self {
                    key,
                    bs: <$crypto_backend>::block_size(),
                }
            }

            // encrypt
            /// Given an array of data, encrypt it and return the result as a byte vector.
            /// # Example
            /// ```rust
            /// // Due to an annoying limitation with triple-slash doc comments, I have to pick one cipher to use here. Any work
            /// use libmangrove::aes::AES128Cipher;
            /// let mut cipher: AES128Cipher = AES128Cipher::new([0; 16]);
            /// let data = [0; 16];
            /// let enc_data = cipher.encrypt(&data);
            /// assert_eq!(enc_data, vec![102, 233, 75, 212, 239, 138, 44, 59, 136, 76, 250, 89, 202, 52, 43, 46]);
            /// ```
            pub fn encrypt(self: &mut $struct_name, data: &[u8]) -> Vec<u8> {
                let mut dvec = data.to_vec();
                self.pad(&mut dvec);
                let cipher = <$crypto_backend>::new(&GenericArray::from(self.key));
                let mut encrypted: Vec<u8> = vec![];
                for block in dvec.chunks(self.bs) {
                    // Encrypt the raw block
                    let mut block_ga = GenericArray::from(array_ref![block, 0, 16].to_owned());
                    cipher.encrypt_block(&mut block_ga);
                    encrypted.append(&mut block_ga.to_vec());
                }

                encrypted
            }

            // decrypt
            /// Given an array of data, decrypt it and return the result as a byte vector.
            /// # Example
            /// ```rust
            /// use libmangrove::aes::AES128Cipher;
            /// let key = [42u8; 16];
            /// let mut cipher = AES128Cipher::new(key);
            /// let data = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)!@#$%^&*("
            ///             .to_string()
            ///             .into_bytes();
            /// let encrypted = cipher.encrypt(&data);
            /// let decrypted = cipher.decrypt(&encrypted);
            /// assert_eq!(data, decrypted);
            /// ```
            pub fn decrypt(self: &mut $struct_name, data: &[u8]) -> Vec<u8> {
                let cipher = <$crypto_backend>::new(&GenericArray::from(self.key));
                let mut decrypted: Vec<u8> = vec![];
                for block in data.chunks(self.bs) {
                    // Encrypt the raw block
                    let mut block_ga = GenericArray::from(array_ref![block, 0, 16].to_owned());
                    cipher.decrypt_block(&mut block_ga);
                    decrypted.append(&mut block_ga.to_vec());
                }

                self.unpad(decrypted)
            }

            // pad
            /// Apply PKCS#7 padding to the given data vector to extend it to the required block size.
            /// This will mutate **the original vector** and will not return anything.
            /// You probably don't need to use this directly.
            fn pad(self: &$struct_name, data: &mut Vec<u8>) {
                // Explaination:
                // Determine the amount of padding required to get to the required block size (self.bs),
                // then convert that number into a char and add it to the end the required amount of times
                // This allows us to unpad by looking at the last char and converting it to a usize,
                // telling us how much padding there is
                if data.len() % self.bs == 0 {
                    // no padding required
                    return;
                }
                let pad_amt = self.bs - (data.len() % self.bs);
                let pad_byte = pad_amt as u8;
                let mut pad_array = vec![pad_byte; pad_amt];
                data.append(&mut pad_array);
            }

            // unpad
            /// Remove PKCS#7 padding from a given data vector.
            /// This does not mutate the original vector, and instead returns a new one.
            /// You probably don't need to use this directly.
            fn unpad(self: &$struct_name, data: Vec<u8>) -> Vec<u8> {
                // Explaination:
                // The `pad` function above uses the amount of padding converted to a char
                // for the padding, so simply convert the last element of the array to a usize,
                // then remove that many elements from the end
                let pad_amt = data[data.len() - 1] as usize;
                if pad_amt > data.len() || data[data.len() - pad_amt] != pad_amt as u8 {
                    // no padding present
                    return data;
                }
                // Make sure that the entire last n bytes are the same, otherwise this might be a false padding
                for x in &data[(data.len() - pad_amt)..] {
                    if x != &(pad_amt as u8) {
                        // no padding present
                        return data;
                    }
                }
                data[0..(data.len() - pad_amt)].to_vec()
            }
        }
    };
}
