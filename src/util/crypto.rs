use anyhow::{anyhow, Result};
use openssl::symm::{decrypt, encrypt, Cipher, Crypter, Mode};

// AES-CBC (key, iv)
pub struct AesCBC<'a>(pub &'a [u8], pub &'a [u8]);

#[allow(dead_code)]
impl<'a> AesCBC<'a> {
    fn cipher(&self) -> Result<Cipher> {
        let cipher = match self.0.len() {
            16 => Cipher::aes_128_cbc(),
            24 => Cipher::aes_192_cbc(),
            32 => Cipher::aes_256_cbc(),
            _ => return Err(anyhow!("crypto/aes: invalid key size")),
        };

        Ok(cipher)
    }

    pub fn encrypt_pkcs5(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesCBC(key, iv) = *self;
        let out = encrypt(cipher, key, Some(iv), data)?;

        Ok(out)
    }

    pub fn encrypt_pkcs7(&self, data: &[u8], padding: usize) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesCBC(key, iv) = *self;
        let mut c = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))?;
        let mut out = vec![0; data.len() + padding];
        let count = c.update(data, &mut out)?;
        let rest = c.finalize(&mut out[count..])?;

        out.truncate(count + rest);

        Ok(out)
    }

    pub fn decrypt_pkcs5(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesCBC(key, iv) = *self;
        let out = decrypt(cipher, key, Some(iv), data)?;

        Ok(out)
    }

    pub fn decrypt_pkcs7(&self, data: &[u8], padding: usize) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesCBC(key, iv) = *self;
        let mut c = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))?;
        let mut out = vec![0; data.len() + padding];
        let count = c.update(data, &mut out)?;
        let rest = c.finalize(&mut out[count..])?;

        out.truncate(count + rest);

        Ok(out)
    }
}

pub struct AesECB<'a>(pub &'a [u8]);

#[allow(dead_code)]
impl<'a> AesECB<'a> {
    fn cipher(&self) -> Result<Cipher> {
        let cipher = match self.0.len() {
            16 => Cipher::aes_128_ecb(),
            24 => Cipher::aes_192_ecb(),
            32 => Cipher::aes_256_ecb(),
            _ => return Err(anyhow!("crypto/aes: invalid key size")),
        };

        Ok(cipher)
    }

    pub fn encrypt_pkcs5(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesECB(key) = *self;
        let out = encrypt(cipher, key, None, data)?;

        Ok(out)
    }

    pub fn encrypt_pkcs7(&self, data: &[u8], padding: usize) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesECB(key) = *self;
        let mut c = Crypter::new(cipher, Mode::Encrypt, key, None)?;
        let mut out = vec![0; data.len() + padding];
        let count = c.update(data, &mut out)?;
        let rest = c.finalize(&mut out[count..])?;

        out.truncate(count + rest);

        Ok(out)
    }

    pub fn decrypt_pkcs5(&self, data: &[u8]) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesECB(key) = *self;
        let out = decrypt(cipher, key, None, data)?;

        Ok(out)
    }

    pub fn decrypt_pkcs7(&self, data: &[u8], padding: usize) -> Result<Vec<u8>> {
        let cipher = self.cipher()?;

        let AesECB(key) = *self;
        let mut c = Crypter::new(cipher, Mode::Decrypt, key, None)?;
        let mut out = vec![0; data.len() + padding];
        let count = c.update(data, &mut out)?;
        let rest = c.finalize(&mut out[count..])?;

        out.truncate(count + rest);

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::crypto::{AesCBC, AesECB};

    use base64::{prelude::BASE64_STANDARD, Engine};

    #[test]
    fn aes_cbc() {
        let key = b"190000bf3cdad8cc075571b56feae191";
        let cbc = AesCBC(key, &key[..16]);

        // encrypt
        let cipher = cbc.encrypt_pkcs5(b"shenghui").unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "T2N4WfGRBRisF0KM604ZWg==");

        // decrypt
        let plain = cbc.decrypt_pkcs5(&cipher).unwrap();
        assert_eq!(plain, b"shenghui");
    }

    #[test]
    fn aes_ecb() {
        let key = b"190000bf3cdad8cc075571b56feae191";
        let ecb = AesECB(key);

        // encrypt
        let cipher = ecb.encrypt_pkcs5(b"shenghui").unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "C5ba7G4RVXRAroQYorCPPw==");

        // decrypt
        let plain = ecb.decrypt_pkcs5(&cipher).unwrap();
        assert_eq!(plain, b"shenghui");
    }
}
