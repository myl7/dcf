//! Embedded PRGs.
//!
//! Including the ones from AES-series, combined with one-way compression functions.

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes128, Aes256};

use crate::owcf::{Hirose, MatyasMeyerOseas};
use crate::PrgBytes;

pub struct Aes128MatyasMeyerOseas {
    ciphers: Vec<Aes128MatyasMeyerOseasCipher>,
}

impl Aes128MatyasMeyerOseas {
    pub fn new(keys: &[&[u8; 16]]) -> Self {
        Self {
            ciphers: keys
                .iter()
                .map(|&key| Aes128MatyasMeyerOseasCipher::new(key))
                .collect(),
        }
    }
}

impl PrgBytes for Aes128MatyasMeyerOseas {
    fn gen(&self, buf: &mut [u8], src: &[u8]) {
        assert_eq!(src.len(), buf.len());
        assert_eq!(src.len() % 16, 0);
        assert_eq!(src.len() / 16, self.ciphers.len());
        buf.array_chunks_mut::<16>()
            .zip(src.array_chunks::<16>())
            .zip(&self.ciphers)
            .for_each(|((buf, src), cipher)| cipher.gen_blk(buf, src));
    }
}

struct Aes128MatyasMeyerOseasCipher {
    cipher: Aes128,
}

impl Aes128MatyasMeyerOseasCipher {
    fn new(key: &[u8; 16]) -> Self {
        Self {
            cipher: Aes128::new(GenericArray::from_slice(key)),
        }
    }
}

impl MatyasMeyerOseas<16> for Aes128MatyasMeyerOseasCipher {
    fn enc_blk(&self, buf: &mut [u8; 16], input: &[u8; 16]) {
        let in_blk = GenericArray::from_slice(input);
        let buf_blk = GenericArray::from_mut_slice(buf);
        self.cipher.encrypt_block_b2b(in_blk, buf_blk)
    }
}

pub struct Aes256Hirose {
    ciphers: Vec<Aes256HiroseCipher>,
}

impl Aes256Hirose {
    pub fn new(keys: &[&[u8; 32]]) -> Self {
        Self {
            ciphers: keys
                .iter()
                .map(|&key| Aes256HiroseCipher::new(key))
                .collect(),
        }
    }
}

impl PrgBytes for Aes256Hirose {
    fn gen(&self, buf: &mut [u8], src: &[u8]) {
        assert_eq!(src.len() * 2, buf.len());
        assert_eq!(src.len() % 16, 0);
        assert_eq!(src.len() / 16, self.ciphers.len());
        buf.array_chunks_mut::<32>()
            .zip(src.array_chunks::<16>())
            .zip(&self.ciphers)
            .for_each(|((buf, src), cipher)| {
                let mut iter = buf.array_chunks_mut::<16>();
                let buf1 = iter.next().unwrap();
                let buf2 = iter.next().unwrap();
                assert_eq!(iter.next(), None);
                cipher.gen_blk([buf1, buf2], src);
            });
    }
}

struct Aes256HiroseCipher {
    cipher: Aes256,
}

impl Aes256HiroseCipher {
    fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: Aes256::new(GenericArray::from_slice(key)),
        }
    }
}

impl Hirose<16> for Aes256HiroseCipher {
    fn enc_blk(&self, buf: &mut [u8; 16], input: &[u8; 16]) {
        let in_blk = GenericArray::from_slice(input);
        let buf_blk = GenericArray::from_mut_slice(buf);
        self.cipher.encrypt_block_b2b(in_blk, buf_blk)
    }
}
