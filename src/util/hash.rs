use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

#[allow(dead_code)]
pub enum Algo {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA512_224,
    SHA512_256,
}

use Algo::*;

#[allow(dead_code)]
pub struct Hash(pub Algo);

#[allow(dead_code)]
impl Hash {
    pub fn from_str(self, s: &str) -> String {
        self.to_string(s.as_bytes())
    }

    pub fn from_bytes(self, b: &[u8]) -> String {
        self.to_string(b)
    }

    pub fn from_string(self, s: String) -> String {
        self.to_string(s.as_bytes())
    }

    fn to_string(self, b: &[u8]) -> String {
        let Hash(algo) = self;

        match algo {
            MD5 => {
                let mut h = Md5::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA1 => {
                let mut h = Sha1::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA224 => {
                let mut h = Sha224::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA384 => {
                let mut h = Sha384::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA256 => {
                let mut h = Sha256::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA512 => {
                let mut h = Sha512::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA512_224 => {
                let mut h = Sha512_224::new();

                h.update(b);
                hex::encode(h.finalize())
            }
            SHA512_256 => {
                let mut h = Sha512_256::new();

                h.update(b);
                hex::encode(h.finalize())
            }
        }
    }
}

#[allow(dead_code)]
pub struct HMAC<'a>(pub Algo, pub &'a str);

#[allow(dead_code)]
impl<'a> HMAC<'a> {
    pub fn from_str(self, s: &str) -> String {
        self.to_string(s.as_bytes())
    }

    pub fn from_bytes(self, b: &[u8]) -> String {
        self.to_string(b)
    }

    pub fn from_string(self, s: String) -> String {
        self.to_string(s.as_bytes())
    }

    fn to_string(self, b: &[u8]) -> String {
        let HMAC(algo, key) = self;

        match algo {
            MD5 => {
                let mut h = Hmac::<Md5>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA1 => {
                let mut h = Hmac::<Sha1>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA224 => {
                let mut h = Hmac::<Sha224>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA256 => {
                let mut h = Hmac::<Sha256>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA384 => {
                let mut h = Hmac::<Sha384>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA512 => {
                let mut h = Hmac::<Sha512>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA512_224 => {
                let mut h = Hmac::<Sha512_224>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
            SHA512_256 => {
                let mut h = Hmac::<Sha512_256>::new_from_slice(key.as_bytes())
                    .expect("HMAC can take key of any size");

                h.update(b);
                hex::encode(h.finalize().into_bytes())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::hash::{Algo::*, Hash, HMAC};

    #[test]
    fn hash() {
        assert_eq!(
            Hash(MD5).from_str("shenghui"),
            "ff7f89cbe5c489ff2825d97c4e7b6f7c"
        );
        assert_eq!(
            Hash(SHA1).from_str("shenghui"),
            "5d06bcf2a58b4e2ae3280e031f84baa8a28db3aa"
        );
        assert_eq!(
            Hash(SHA224).from_str("shenghui"),
            "a79fee2960ea91b511556f393e3bbdc1da5aa17253b029c36adf0ef3"
        );
        assert_eq!(
            Hash(SHA256).from_str("shenghui"),
            "c6f540373c19d5cc0564fdce042b74d7e57c4fc352878f8128a7d513bac76568"
        );
        assert_eq!(
            Hash(SHA384).from_str("shenghui"),
            "1ad756ef7fbc0912b56d2609a646a2887ce34f70cbb0144a86a2f394a121dee88d09d0b47e0b99f039f36e7dba06e90d"
        );
        assert_eq!(
            Hash(SHA512).from_str("shenghui"),
            "42071eb6241a2a19c01c1cb7cad9aa5730c1d15de8b54ff4f333e7c9e5854640084f20a1406bf362c22131725c432b387832a9431859eb031b914890ddd01671"
        );
        assert_eq!(
            Hash(SHA512_224).from_str("shenghui"),
            "25ecca889865b41d2386b08d71e84bd4bb6dc9bfb4bda5127462ad90"
        );
        assert_eq!(
            Hash(SHA512_256).from_str("shenghui"),
            "f12bb32e3b8cf30102b9b2a316e84bc69ee009623197a17a97ed33dc8a71a872"
        );
    }

    #[test]
    fn hmac() {
        assert_eq!(
            HMAC(MD5, "IIInsomnia").from_str("shenghui"),
            "cac9160ed60eb1bcca32c7460b5ca238"
        );
        assert_eq!(
            HMAC(SHA1, "IIInsomnia").from_str("shenghui"),
            "750583660d10fbadf8004f462aa7ef1d9f18cd91"
        );
        assert_eq!(
            HMAC(SHA224, "IIInsomnia").from_str("shenghui"),
            "c2b5456bf70ab7be63de54c055a66554d0ee558f1c6985a5325f2b0a"
        );
        assert_eq!(
            HMAC(SHA256, "IIInsomnia").from_str("shenghui"),
            "6ea90a066be004ca5ac384d79605d8a2403cc8a9b14ffc988822bf85be12b038"
        );
        assert_eq!(
            HMAC(SHA384, "IIInsomnia").from_str("shenghui"),
            "04faa29cd8da1e4d18d9890006242a90dfcb127e5914ceb18226857bdb04e106af54473afd6a061c9f6f16c70990d73c"
        );
        assert_eq!(
            HMAC(SHA512, "IIInsomnia").from_str("shenghui"),
            "094f0911af5717643188cce2537528f36212473a4756a110606b7c98bdcc5d0dcd64ee03acb7a2f8e91b6c46bd78ac82279ed9889834e52433da90a57c8ef506"
        );
        assert_eq!(
            HMAC(SHA512_224, "IIInsomnia").from_str("shenghui"),
            "94732693878898c638f449a4c3c2bc6d0ed73d43d2c1c2233aeedfa2"
        );
        assert_eq!(
            HMAC(SHA512_256, "IIInsomnia").from_str("shenghui"),
            "9863f2c13c3218265d374f82605ef368d6577e4d292d122117fa07c72839b71e"
        );
    }
}
