use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString}, Algorithm, Argon2, Params, Version};
use rand::{RngCore, rngs::OsRng};
use aes_gcm::{aead::Nonce, Key};
use serde::Deserialize;
use serde::Serialize;


const M_COST: u32 = 30_000;
const T_COST: u32 = 5;
const P_COST: u32 = 1;
const OUTPUT_LEN: usize = 32;

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgonVersion{
    V0x13 = 19,
    V0x10 = 16
}

impl From<ArgonVersion> for Version {
    fn from(version: ArgonVersion) -> Self {
        match version {
            ArgonVersion::V0x10 => return Version::V0x10,
            ArgonVersion::V0x13 => return Version::V0x13,
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArgonAlgorithm{
    Argon2i,
    Argon2d,
    Argon2id,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionData {
    salt: String,
    nonce: String,
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: usize,
    argon_algorithm: ArgonAlgorithm,
    argon_version: ArgonVersion,
}
 fn create_key_from_password_and_salt(password: &str, salt: &[u8]) -> [u8; 32] {
    let argon_parameters = Params::new(M_COST, T_COST, P_COST, Some(OUTPUT_LEN)).expect("Error creating argon parameters.");
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_parameters);
    let mut key =[0u8; 32];
    argon.hash_password_into(password.as_bytes(), salt, &mut key).expect("Error with argon hasing password into key");
    return key;
}

fn create_ciphertext_from_key_and_password(key: &[u8], password: &str) {
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce.from_slice(&nonce_bytes);
}