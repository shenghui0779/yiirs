use anyhow::{anyhow, Result};
use crypto::{
    aes::{self, KeySize},
    blockmodes,
    buffer::{self, BufferResult, ReadBuffer, WriteBuffer},
    symmetriccipher::{
        Decryptor, Encryptor,
        SymmetricCipherError::{InvalidLength, InvalidPadding},
    },
};

#[allow(dead_code)]
pub enum AES<'a> {
    // PKCS#5(key_size, key, iv)
    CBC(KeySize, &'a [u8], &'a [u8]),

    // PKCS#5(key_size, key)
    ECB(KeySize, &'a [u8]),
}

use AES::*;

#[allow(dead_code)]
impl<'a> AES<'a> {
    pub fn encrypt(self, plain: &[u8]) -> Result<Vec<u8>> {
        match self {
            CBC(key_size, key, iv) => {
                let encryptor = aes::cbc_encryptor(key_size, key, iv, blockmodes::PkcsPadding);
                AES::encode(encryptor, plain)
            }
            ECB(key_size, key) => {
                let encryptor = aes::ecb_encryptor(key_size, key, blockmodes::PkcsPadding);
                AES::encode(encryptor, plain)
            }
        }
    }

    pub fn decrypt(self, cipher: &[u8]) -> Result<Vec<u8>> {
        match self {
            CBC(key_size, key, iv) => {
                let decryptor = aes::cbc_decryptor(key_size, key, iv, blockmodes::PkcsPadding);
                AES::decode(decryptor, cipher)
            }
            ECB(key_size, key) => {
                let decryptor = aes::ecb_decryptor(key_size, key, blockmodes::PkcsPadding);
                AES::decode(decryptor, cipher)
            }
        }
    }

    fn encode(mut encryptor: Box<dyn Encryptor>, plain: &[u8]) -> Result<Vec<u8>> {
        let mut buffer = [0; 4096];
        let mut cipher = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(plain);
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let ret_enc = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);

            match ret_enc {
                Err(e) => match e {
                    InvalidLength => {
                        return Err(anyhow!(
                            "crypto: invalid length, please check if key or iv mismatch key_size"
                        ))
                    }
                    InvalidPadding => {
                        return Err(anyhow!(
                            "crypto: invalid padding, please check if key or iv mismatch key_size"
                        ))
                    }
                },
                Ok(ret_buf) => {
                    cipher.extend(
                        write_buffer
                            .take_read_buffer()
                            .take_remaining()
                            .iter()
                            .map(|&i| i),
                    );

                    match ret_buf {
                        BufferResult::BufferUnderflow => break,
                        BufferResult::BufferOverflow => {}
                    }
                }
            }
        }

        Ok(cipher)
    }

    fn decode(mut decryptor: Box<dyn Decryptor>, cipher: &[u8]) -> Result<Vec<u8>> {
        let mut buffer = [0; 4096];
        let mut plain = Vec::<u8>::new();
        let mut read_buffer = buffer::RefReadBuffer::new(cipher);
        let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

        loop {
            let ret_dec = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);

            match ret_dec {
                Err(e) => match e {
                    InvalidLength => {
                        return Err(anyhow!(
                            "crypto: invalid length, please check if key or iv mismatch key_size"
                        ))
                    }
                    InvalidPadding => {
                        return Err(anyhow!(
                            "crypto: invalid padding, please check if key or iv mismatch key_size"
                        ))
                    }
                },
                Ok(ret_buf) => {
                    plain.extend(
                        write_buffer
                            .take_read_buffer()
                            .take_remaining()
                            .iter()
                            .map(|&i| i),
                    );

                    match ret_buf {
                        BufferResult::BufferUnderflow => break,
                        BufferResult::BufferOverflow => {}
                    }
                }
            }
        }

        Ok(plain)
    }
}

#[cfg(test)]
mod tests {
    use super::AES::*;
    use base64::{prelude::BASE64_STANDARD, Engine};

    #[test]
    fn aes_cbc() {
        let key = b"190000bf3cdad8cc075571b56feae191";

        // encrypt
        let cipher = CBC(crypto::aes::KeySize::KeySize256, key, &key[..16])
            .encrypt(b"shenghui")
            .unwrap();

        assert_eq!(BASE64_STANDARD.encode(&cipher), "T2N4WfGRBRisF0KM604ZWg==");

        // decrypt
        let plain = CBC(crypto::aes::KeySize::KeySize256, key, &key[..16])
            .decrypt(&cipher)
            .unwrap();

        assert_eq!(plain, b"shenghui");
    }

    #[test]
    fn aes_ecb() {
        let key = b"190000bf3cdad8cc075571b56feae191";

        // encrypt
        let cipher = ECB(crypto::aes::KeySize::KeySize256, key)
            .encrypt(b"shenghui")
            .unwrap();

        assert_eq!(BASE64_STANDARD.encode(&cipher), "C5ba7G4RVXRAroQYorCPPw==");

        // decrypt
        let plain = ECB(crypto::aes::KeySize::KeySize256, key)
            .decrypt(&cipher)
            .unwrap();

        assert_eq!(plain, b"shenghui");
    }
}
