use anyhow::{anyhow, Result};
use openssl::symm::{Cipher, Crypter, Mode};

// AES-CBC pkcs#7
// CBC(key, iv)
pub struct CBC<'a>(pub &'a [u8], pub &'a [u8]);

#[allow(dead_code)]
impl<'a> CBC<'a> {
    fn cipher(&self) -> Result<Cipher> {
        let cipher = match self.0.len() {
            16 => Cipher::aes_128_cbc(),
            24 => Cipher::aes_192_cbc(),
            32 => Cipher::aes_256_cbc(),
            _ => return Err(anyhow!("crypto/aes: invalid key size")),
        };

        Ok(cipher)
    }

    pub fn encrypt(&self, b: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let CBC(key, iv) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, Some(iv))?;
        c.pad(false);

        let v = pkcs7_padding(b, t.block_size());
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn encrypt_with_padding_size(&self, b: &[u8], padding_size: usize) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let CBC(key, iv) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, Some(iv))?;
        c.pad(false);

        let v = pkcs7_padding(b, padding_size);
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn decrypt(&self, b: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let CBC(key, iv) = *self;
        let mut c = Crypter::new(t, Mode::Decrypt, key, Some(iv))?;
        c.pad(false);

        let mut out = vec![0; b.len() + t.block_size()];
        let count = c.update(b, &mut out)?;
        out.truncate(count);

        Ok(pkcs7_unpadding(&out))
    }
}

// AES-ECB pkcs#7
// ECB(key)
#[allow(dead_code)]
pub struct ECB<'a>(pub &'a [u8]);

#[allow(dead_code)]
impl<'a> ECB<'a> {
    fn cipher(&self) -> Result<Cipher> {
        let cipher = match self.0.len() {
            16 => Cipher::aes_128_ecb(),
            24 => Cipher::aes_192_ecb(),
            32 => Cipher::aes_256_ecb(),
            _ => return Err(anyhow!("crypto/aes: invalid key size")),
        };

        Ok(cipher)
    }

    pub fn encrypt(&self, b: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let ECB(key) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, None)?;
        c.pad(false);

        let v = pkcs7_padding(b, t.block_size());
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn encrypt_with_padding_size(&self, b: &[u8], padding_size: usize) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let ECB(key) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, None)?;
        c.pad(false);

        let v = pkcs7_padding(b, padding_size);
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn decrypt(&self, b: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;

        let ECB(key) = *self;
        let mut c = Crypter::new(t, Mode::Decrypt, key, None)?;
        c.pad(false);

        let mut out = vec![0; b.len() + t.block_size()];
        let count = c.update(b, &mut out)?;
        out.truncate(count);

        Ok(pkcs7_unpadding(&out))
    }
}

fn pkcs7_padding(b: &[u8], block_size: usize) -> Vec<u8> {
    let mut padding = block_size - b.len() % block_size;
    if padding == 0 {
        padding = block_size
    }

    let mut b = [padding as u8; 1].repeat(padding);
    let mut v = b.to_vec();
    v.append(&mut b);

    v
}

fn pkcs7_unpadding(b: &[u8]) -> Vec<u8> {
    let len = b.len();
    let padding = b[len - 1] as usize;

    b[..len - padding].to_vec()
}

#[cfg(test)]
mod tests {
    use base64::{prelude::BASE64_STANDARD, Engine};

    use crate::util::aes::{CBC, ECB};

    #[test]
    fn aes_cbc() {
        let key = b"AES256Key-32Characters1234567890";
        let cbc = CBC(key, &key[..16]);

        let cipher = cbc.encrypt(b"ILoveYiigo").unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "kyJ6t0cpUYpoWaewhTwDwQ==");

        let plain = cbc.decrypt(&cipher).unwrap();
        assert_eq!(plain, b"ILoveYiigo");

        let cipher2 = cbc.encrypt_with_padding_size(b"ILoveYiigo", 32).unwrap();
        assert_eq!(
            BASE64_STANDARD.encode(&cipher2),
            "hSXsKUV2fbG8F2JlVcnra876xvKxyXwoJvaebTtWGzQ="
        );

        let plain2 = cbc.decrypt(&cipher2).unwrap();
        assert_eq!(plain2, b"ILoveYiigo");
    }

    #[test]
    fn aes_ecb() {
        let key = b"AES256Key-32Characters1234567890";
        let ecb = ECB(key);

        let cipher = ecb.encrypt(b"ILoveYiigo").unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "8+evCMirn78a5l2mCCdJug==");

        let plain = ecb.decrypt(&cipher).unwrap();
        assert_eq!(plain, b"ILoveYiigo");

        let cipher2 = ecb.encrypt_with_padding_size(b"ILoveYiigo", 32).unwrap();
        assert_eq!(
            BASE64_STANDARD.encode(&cipher2),
            "FqrgSRCY4zBRYBOg4Pe3Vbpl6eN3wP/L8phJTP4aWFE="
        );

        let plain2 = ecb.decrypt(&cipher2).unwrap();
        assert_eq!(plain2, b"ILoveYiigo");
    }
}
