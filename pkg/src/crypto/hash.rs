use digest::{crypto_common::BlockSizeUser, Digest, Mac};
use hmac::{Hmac, SimpleHmac};
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;

pub fn md5(b: &[u8]) -> String {
    let mut h = Md5::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn sha1(b: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn sha256(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn hash<D: Digest>(b: &[u8]) -> String {
    let mut h = D::new();
    h.update(b);
    const_hex::encode(h.finalize())
}

pub fn hmac_sha1(key: &[u8], b: &[u8]) -> String {
    let mut h = Hmac::<Sha1>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

pub fn hmac_sha256(key: &[u8], b: &[u8]) -> String {
    let mut h = Hmac::<Sha256>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

pub fn hmac<D: Digest + BlockSizeUser>(key: &[u8], b: &[u8]) -> String {
    let mut h = SimpleHmac::<D>::new_from_slice(key).unwrap();
    h.update(b);
    const_hex::encode(h.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use md5::Md5;
    use sha1::Sha1;
    use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256};

    use crate::crypto::hash::{hash, hmac, hmac_sha1, hmac_sha256};

    #[test]
    fn digest_hash() {
        assert_eq!(hash::<Md5>(b"shenghui"), "ff7f89cbe5c489ff2825d97c4e7b6f7c");
        assert_eq!(
            hash::<Sha1>(b"shenghui"),
            "5d06bcf2a58b4e2ae3280e031f84baa8a28db3aa"
        );
        assert_eq!(
            hash::<Sha224>(b"shenghui"),
            "a79fee2960ea91b511556f393e3bbdc1da5aa17253b029c36adf0ef3"
        );
        assert_eq!(
            hash::<Sha256>(b"shenghui"),
            "c6f540373c19d5cc0564fdce042b74d7e57c4fc352878f8128a7d513bac76568"
        );
        assert_eq!(
            hash::<Sha384>(b"shenghui"),
            "1ad756ef7fbc0912b56d2609a646a2887ce34f70cbb0144a86a2f394a121dee88d09d0b47e0b99f039f36e7dba06e90d"
        );
        assert_eq!(
            hash::<Sha512>(b"shenghui"),
            "42071eb6241a2a19c01c1cb7cad9aa5730c1d15de8b54ff4f333e7c9e5854640084f20a1406bf362c22131725c432b387832a9431859eb031b914890ddd01671"
        );
        assert_eq!(
            hash::<Sha512_224>(b"shenghui"),
            "25ecca889865b41d2386b08d71e84bd4bb6dc9bfb4bda5127462ad90"
        );
        assert_eq!(
            hash::<Sha512_256>(b"shenghui"),
            "f12bb32e3b8cf30102b9b2a316e84bc69ee009623197a17a97ed33dc8a71a872"
        );
    }

    #[test]
    fn digest_hmac() {
        assert_eq!(
            hmac::<Md5>(b"IIInsomnia", b"shenghui"),
            "cac9160ed60eb1bcca32c7460b5ca238"
        );
        assert_eq!(
            hmac::<Sha1>(b"IIInsomnia", b"shenghui"),
            "750583660d10fbadf8004f462aa7ef1d9f18cd91"
        );
        assert_eq!(
            hmac::<Sha224>(b"IIInsomnia", b"shenghui"),
            "c2b5456bf70ab7be63de54c055a66554d0ee558f1c6985a5325f2b0a"
        );
        assert_eq!(
            hmac::<Sha256>(b"IIInsomnia", b"shenghui"),
            "6ea90a066be004ca5ac384d79605d8a2403cc8a9b14ffc988822bf85be12b038"
        );
        assert_eq!(
            hmac::<Sha384>(b"IIInsomnia", b"shenghui"),
            "04faa29cd8da1e4d18d9890006242a90dfcb127e5914ceb18226857bdb04e106af54473afd6a061c9f6f16c70990d73c"
        );
        assert_eq!(
            hmac::<Sha512>(b"IIInsomnia", b"shenghui"),
            "094f0911af5717643188cce2537528f36212473a4756a110606b7c98bdcc5d0dcd64ee03acb7a2f8e91b6c46bd78ac82279ed9889834e52433da90a57c8ef506"
        );
        assert_eq!(
            hmac::<Sha512_224>(b"IIInsomnia", b"shenghui"),
            "94732693878898c638f449a4c3c2bc6d0ed73d43d2c1c2233aeedfa2"
        );
        assert_eq!(
            hmac::<Sha512_256>(b"IIInsomnia", b"shenghui"),
            "9863f2c13c3218265d374f82605ef368d6577e4d292d122117fa07c72839b71e"
        );
        assert_eq!(
            hmac_sha1(b"IIInsomnia", b"shenghui"),
            "750583660d10fbadf8004f462aa7ef1d9f18cd91"
        );
        assert_eq!(
            hmac_sha256(b"IIInsomnia", b"shenghui"),
            "6ea90a066be004ca5ac384d79605d8a2403cc8a9b14ffc988822bf85be12b038"
        );
    }
}
