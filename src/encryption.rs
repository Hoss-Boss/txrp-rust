use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString}, Algorithm, Argon2, Params, Version};
use rand::{RngCore, rngs::OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, aead::{Aead, Nonce}, aes::Aes256};
use serde::Deserialize;
use serde::Serialize;
use base64::{engine::general_purpose, Engine};


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
    salt: Vec<u8>,
    nonce: Vec<u8>,
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
    let mut key = [0u8; 32];
    argon.hash_password_into(password.as_bytes(), salt, &mut key).expect("Error with argon hasing password into key");
    return key;
}


fn encrypt(plaintext: &str)  -> String {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let key = create_key_from_password_and_salt(&plaintext, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Error initializng cipher.");
    let plaintext_bytes = plaintext.as_bytes();
    let ciphertext = cipher.encrypt(&nonce, plaintext_bytes).expect("Error creating cyphertext from nonce and plaintext bytes.");
    let ciphertext_base64 = general_purpose::STANDARD.encode(ciphertext);
    println!("{:#?}", ciphertext_base64);
    return ciphertext_base64;
    
}

