use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString}, Algorithm, Argon2, Params, Version};
use rand::rngs::OsRng;
use aes_gcm::{aead::Nonce, Key};
use serde::Deserialize;
use serde::Serialize;


const M_COST: u32 = 30_000;
const T_COST: u32 = 5;
const P_COST: u32 = 1;
const OUTPUT_LEN: usize = 32;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptionData {
    salt: String,
    nonce: String,
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: usize,
    argon_algorithm: String,
    argon_version: String,
}

pub fn create_key(password: &str, salt: &[u8]) {
    let argon_parameters = Params::new(M_COST, T_COST, P_COST, Some(OUTPUT_LEN)).expect("Error creating argon parameters.");
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon_parameters);
}