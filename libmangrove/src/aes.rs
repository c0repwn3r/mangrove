use aes::cipher::BlockSizeUser;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use arrayref::array_ref;

// !!
// !! WARNING! THIS CODE HAS *NOT* BEEN INDEPENDENTLY VERIFIED.
// !! IT RELIES ON THE `aes` CRATE TO DO THE ACTUAL CIPHER, WHICH HAS BEEN AUDITED.
// !! USE AT YOUR OWN RISK!
//

pub struct AES128Cipher {
    key: [u8; 16],
    bs: usize,
}

impl AES128Cipher {
    pub fn new(key: [u8; 16]) -> AES128Cipher {
        AES128Cipher {
            key,
            bs: Aes128::block_size(),
        }
    }

    pub fn encrypt(self: &mut AES128Cipher, data: &[u8]) -> Vec<u8> {
        let mut dvec = data.to_vec();
        self.pad(&mut dvec);
        let cipher = Aes128::new(&GenericArray::from(self.key));
        let mut encrypted: Vec<u8> = vec![];
        for block in dvec.chunks(self.bs) {
            // Encrypt the raw block
            let mut block_ga = GenericArray::from(array_ref![block, 0, 16].to_owned());
            cipher.encrypt_block(&mut block_ga);
            encrypted.append(&mut block_ga.to_vec());
        }

        encrypted
    }

    pub fn decrypt(self: &mut AES128Cipher, data: &[u8]) -> Vec<u8> {
        let cipher = Aes128::new(&GenericArray::from(self.key));
        let mut decrypted: Vec<u8> = vec![];
        for block in data.chunks(self.bs) {
            // Encrypt the raw block
            let mut block_ga = GenericArray::from(array_ref![block, 0, 16].to_owned());
            cipher.decrypt_block(&mut block_ga);
            decrypted.append(&mut block_ga.to_vec());
        }
        
        self.unpad(decrypted)
    }

    fn pad(self: &AES128Cipher, data: &mut Vec<u8>) {
        // Explaination:
        // Determine the amount of padding required to get to the required block size (self.bs),
        // then convert that number into a char and add it to the end the required amount of times
        // This allows us to unpad by looking at the last char and converting it to a usize,
        // telling us how much padding there is
        if data.len() % self.bs == 0 {
            // no padding required
            return
        }
        let pad_amt = self.bs - (data.len() % self.bs);
        let pad_byte = pad_amt as u8;
        let mut pad_array = vec![pad_byte; pad_amt];
        data.append(&mut pad_array);
    }

    fn unpad(self: &AES128Cipher, data: Vec<u8>) -> Vec<u8> {
        // Explaination:
        // The `pad` function above uses the amount of padding converted to a char
        // for the padding, so simply convert the last element of the array to a usize,
        // then remove that many elements from the end
        let pad_amt = data[data.len() - 1] as usize;
        if pad_amt > data.len() || data[data.len() - pad_amt] != pad_amt as u8 {
            // no padding present
            return data
        }
        data[0..pad_amt].to_vec()
    }
}
