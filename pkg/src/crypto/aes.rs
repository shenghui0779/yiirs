use anyhow::{anyhow, Result};
use openssl::symm::{decrypt_aead, encrypt_aead, Cipher, Crypter, Mode};

// AES-CBC pkcs#7
// CBC(key, iv)
pub struct CBC<'a>(pub &'a [u8], pub &'a [u8]);

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

    // 填充字节, 默认: BlockSize(16)
    pub fn encrypt(&self, data: &[u8], padding_size: Option<usize>) -> Result<Vec<u8>> {
        let t = self.cipher()?;
        let CBC(key, iv) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, Some(iv))?;
        c.pad(false);

        let v = pkcs7_padding(data, padding_size.unwrap_or(t.block_size()));
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;
        let CBC(key, iv) = *self;
        let mut c = Crypter::new(t, Mode::Decrypt, key, Some(iv))?;
        c.pad(false);

        let mut out = vec![0; data.len() + t.block_size()];
        let count = c.update(data, &mut out)?;
        out.truncate(count);

        Ok(pkcs7_unpadding(&out))
    }
}

// AES-ECB pkcs#7
// ECB(key)
pub struct ECB<'a>(pub &'a [u8]);

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

    // 填充字节, 默认: BlockSize(16)
    pub fn encrypt(&self, data: &[u8], padding_size: Option<usize>) -> Result<Vec<u8>> {
        let t = self.cipher()?;
        let ECB(key) = *self;
        let mut c = Crypter::new(t, Mode::Encrypt, key, None)?;
        c.pad(false);

        let v = pkcs7_padding(data, padding_size.unwrap_or(t.block_size()));
        let mut out = vec![0; v.len() + t.block_size()];
        let count = c.update(&v, &mut out)?;
        out.truncate(count);

        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;
        let ECB(key) = *self;
        let mut c = Crypter::new(t, Mode::Decrypt, key, None)?;
        c.pad(false);

        let mut out = vec![0; data.len() + t.block_size()];
        let count = c.update(data, &mut out)?;
        out.truncate(count);

        Ok(pkcs7_unpadding(&out))
    }
}

// AES-GCM
// GCM(key, nonce) -> (cipher, tag)
pub struct GCM<'a>(pub &'a [u8], pub &'a [u8]);

impl<'a> GCM<'a> {
    fn cipher(&self) -> Result<Cipher> {
        let cipher = match self.0.len() {
            16 => Cipher::aes_128_gcm(),
            24 => Cipher::aes_192_gcm(),
            32 => Cipher::aes_256_gcm(),
            _ => return Err(anyhow!("crypto/aes: invalid key size")),
        };
        Ok(cipher)
    }

    // tag_size, 默认: 16, 可取范围: [12, 16]
    pub fn encrypt(
        &self,
        data: &[u8],
        aad: &[u8],
        tag_size: Option<usize>,
    ) -> Result<(Vec<u8>, Vec<u8>)> {
        let t = self.cipher()?;
        let GCM(key, iv) = *self;
        let mut tag = vec![0; tag_size.unwrap_or(16)];
        let out = encrypt_aead(t, key, Some(iv), aad, data, &mut tag)?;
        Ok((out, tag))
    }

    pub fn decrypt(&self, data: &[u8], aad: &[u8], tag: &[u8]) -> Result<Vec<u8>> {
        let t = self.cipher()?;
        let GCM(key, iv) = *self;
        let out = decrypt_aead(t, key, Some(iv), aad, data, tag)?;
        Ok(out)
    }
}

fn pkcs7_padding(data: &[u8], block_size: usize) -> Vec<u8> {
    let mut padding = block_size - data.len() % block_size;
    if padding == 0 {
        padding = block_size
    }
    let mut b = [padding as u8; 1].repeat(padding);
    let mut v = data.to_vec();
    v.append(&mut b);
    v
}

fn pkcs7_unpadding(data: &[u8]) -> Vec<u8> {
    let len = data.len();
    let padding = data[len - 1] as usize;
    data[..len - padding].to_vec()
}

#[cfg(test)]
mod tests {
    use base64::{prelude::BASE64_STANDARD, Engine};

    use crate::crypto::aes::{CBC, ECB, GCM};

    #[test]
    fn aes_cbc() {
        let key = b"AES256Key-32Characters1234567890";
        let cbc = CBC(key, &key[..16]);

        // 默认填充
        let cipher = cbc.encrypt(b"ILoveYiigo", None).unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "kyJ6t0cpUYpoWaewhTwDwQ==");

        let plain = cbc.decrypt(&cipher).unwrap();
        assert_eq!(plain, b"ILoveYiigo");

        // 32字节填充
        let cipher2 = cbc.encrypt(b"ILoveYiigo", Some(32)).unwrap();
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

        // 默认填充
        let cipher = ecb.encrypt(b"ILoveYiigo", None).unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "8+evCMirn78a5l2mCCdJug==");

        let plain = ecb.decrypt(&cipher).unwrap();
        assert_eq!(plain, b"ILoveYiigo");

        // 32字节填充
        let cipher2 = ecb.encrypt(b"ILoveYiigo", Some(32)).unwrap();
        assert_eq!(
            BASE64_STANDARD.encode(&cipher2),
            "FqrgSRCY4zBRYBOg4Pe3Vbpl6eN3wP/L8phJTP4aWFE="
        );

        let plain2 = ecb.decrypt(&cipher2).unwrap();
        assert_eq!(plain2, b"ILoveYiigo");
    }

    #[test]
    fn aes_gcm() {
        let key = b"AES256Key-32Characters1234567890";
        let gcm = GCM(key, &key[..12]);

        // 默认 tag_size
        let (cipher, tag) = gcm.encrypt(b"ILoveYiigo", b"IIInsomnia", None).unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher), "qciumnRZKY42HQ==");
        assert_eq!(BASE64_STANDARD.encode(&tag), "WOeD9xSN3RX44lkHpnBEXw==");

        let plain = gcm.decrypt(&cipher, b"IIInsomnia", &tag).unwrap();
        assert_eq!(plain, b"ILoveYiigo");

        // 指定 tag_size
        let (cipher2, tag2) = gcm.encrypt(b"ILoveYiigo", b"IIInsomnia", Some(12)).unwrap();
        assert_eq!(BASE64_STANDARD.encode(&cipher2), "qciumnRZKY42HQ==");
        assert_eq!(BASE64_STANDARD.encode(&tag2), "WOeD9xSN3RX44lkH");

        let plain = gcm.decrypt(&cipher2, b"IIInsomnia", &tag2).unwrap();
        assert_eq!(plain, b"ILoveYiigo");
    }
}
