extern crate crypto;
extern crate ring;
extern crate hex;

use std::u8;
use hex::FromHex;
use crypto::{sha2, hkdf, aes, blockmodes, buffer};

use x25519_dalek::x25519;


#[derive(Debug)]
pub struct SharedSecret {
    pub value: String
}

#[derive(Debug)]
pub struct IdentityKey {
    salt: String,
    prk: String,
    pub value: String
}

#[derive(Debug)]
pub struct TemporaryKey {
    data: String,
    value: String
}

#[derive(Debug)]
pub struct EphemeralID {
    rotation_exponent: u64,
    beacon_time_seconds: u64,
    temporary_key: String,
    data: String,
    pub value: String
}


impl SharedSecret {
    pub fn new(private_key: String, beacon_public_key: String) -> Self {
        let shared_secret = x25519(
            <[u8; 32]>::from_hex(private_key).unwrap(),
            <[u8; 32]>::from_hex(beacon_public_key).unwrap()
        );

        SharedSecret {
            value: hex::encode(shared_secret)
        }
    }
}

impl IdentityKey {
    pub fn new(shared_secret: String, service_public_key: String, beacon_public_key: String) -> Self {
        let salt = [
            <[u8; 32]>::from_hex(service_public_key).unwrap(),
            <[u8; 32]>::from_hex(&beacon_public_key).unwrap()
        ].concat();

        let mut identity_key: [u8; 16] = [0; 16];
        let mut prk: [u8; 32] = [0; 32];

        let hasher = sha2::Sha256::new();

        hkdf::hkdf_extract(hasher, &salt, &hex::decode(&shared_secret).unwrap(), &mut prk);
        hkdf::hkdf_expand(hasher, &prk, b"", &mut identity_key);

        IdentityKey {
            salt: hex::encode(&salt),
            prk: hex::encode(&prk),
            value: hex::encode(&identity_key)
        }
    }
}

impl TemporaryKey {
    pub fn new(identity_key: String, counter: u64) -> Self {

        let mut temporary_key_data = [0; 16];
        temporary_key_data[11] = u8::from_str_radix("FF", 16).unwrap();
        temporary_key_data[14] = ((counter / u64::pow(2, 24)) % 256) as u8;
        temporary_key_data[15] = ((counter / u64::pow(2, 16)) % 256) as u8;

        let mut aes_ecb = aes::ecb_encryptor(aes::KeySize::KeySize128, &hex::decode(&identity_key).unwrap(), blockmodes::NoPadding);

        let mut temporary_key = [0; 16];
        aes_ecb.encrypt(
            &mut buffer::RefReadBuffer::new(&mut temporary_key_data),
            &mut buffer::RefWriteBuffer::new(&mut temporary_key),
            true
        ).unwrap();

        TemporaryKey {
            data: hex::encode(temporary_key_data),
            value: hex::encode(temporary_key)
        }
    }
}


impl EphemeralID {
    pub fn new(identity_key: String, scaler: u32, counter: u64) -> Self {

        let rotation_exponent = u64::pow(2, scaler);
        let beacon_time_seconds = (counter / rotation_exponent) * rotation_exponent;

        let mut ephemeral_id_data = [0; 16];
        ephemeral_id_data[11] = scaler as u8;
        ephemeral_id_data[12] = ((beacon_time_seconds / u64::pow(2, 24)) % 256) as u8;
        ephemeral_id_data[13] = ((beacon_time_seconds / u64::pow(2, 16)) % 256) as u8;
        ephemeral_id_data[14] = ((beacon_time_seconds / u64::pow(2, 8)) % 256) as u8;
        ephemeral_id_data[15] = ((beacon_time_seconds / u64::pow(2, 0)) % 256) as u8;

        let temporary_key = TemporaryKey::new(identity_key, counter);

        let mut aes_ecb = aes::ecb_encryptor(aes::KeySize::KeySize128, &hex::decode(&temporary_key.value).unwrap(), blockmodes::NoPadding);

        let mut ephemeral_id = [0; 8];
        aes_ecb.encrypt(
            &mut buffer::RefReadBuffer::new(&mut ephemeral_id_data),
            &mut buffer::RefWriteBuffer::new(&mut ephemeral_id),
            true
        ).unwrap();

        EphemeralID {
            rotation_exponent: rotation_exponent,
            beacon_time_seconds: beacon_time_seconds,
            temporary_key: hex::encode(temporary_key.value),
            data:  hex::encode(ephemeral_id_data),
            value: hex::encode(ephemeral_id)
        }
    }
}

#[cfg(test)]
mod tests_ephemeral_id {
    use super::SharedSecret;
    use super::IdentityKey;
    use super::TemporaryKey;
    use super::EphemeralID;

    #[test]
    fn test_ephemeral_id() {

        let shared_secret = SharedSecret::new(
            "4041414141414141414141414141414141414141414141414141414141414141".to_string(),
            "5f2ff6357762b9c188343259a9bd899a9a667d170143c0bc1ae905e877914a0e".to_string()
        );
        assert_eq!("80722c34967ab7d613c5549224c662aed7cdf5369ec051bcede788a4a29b7677", &shared_secret.value);

        let identity_key = IdentityKey::new(
            shared_secret.value,
            "7a1a4e709bf085ac494aba0469b9b1eda0ab1f78b16aabb79ffeda90623e8522".to_string(),
            "5f2ff6357762b9c188343259a9bd899a9a667d170143c0bc1ae905e877914a0e".to_string()
        );
        assert_eq!("7c91330e61dfea4606b5b3ecb4457d76", &identity_key.value);

        let temporary_key = TemporaryKey::new(
            identity_key.value.clone(),
            0
        );
        assert_eq!("4d7ad123b3f67bb3c7ac3ffd02599f6a", &temporary_key.value);

        let ephemeral_id = EphemeralID::new(
            identity_key.value.clone(),
            0,
            0
        );
        assert_eq!("436bfe57d5a09505", &ephemeral_id.value);
    }
}
