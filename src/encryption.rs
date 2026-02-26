use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString}, Algorithm, Argon2, Params, Version};
use rand::{RngCore, rngs::OsRng};
use aes_gcm::{aead::{Aead, Nonce}, aes::Aes256, AeadCore, Aes256Gcm, Key, KeyInit};
use serde::Deserialize;
use serde::Serialize;
use base64::{engine::general_purpose, Engine};
use std::error::Error;

use crate::wallet_functionality::TXRPWallet;

pub const M_COST: u32 = 30_000;
pub const T_COST: u32 = 5;
pub const P_COST: u32 = 1;
pub const OUTPUT_LEN: usize = 32;
pub const ARGON_ALGORITHM: argon2::Algorithm = argon2::Algorithm::Argon2id;
pub const ARGON_VERSION: argon2::Version = argon2::Version::V0x13;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ArgonVersion{
    V0x13 = 19,
    V0x10 = 16
}

#[derive(Debug)]
pub enum ArgonParseError {
    StringMatchError(String),
}



impl ArgonVersion {
    pub fn to_string(&self) -> String {
        match self {
            ArgonVersion::V0x10 => return "V0x10".to_string(),
            ArgonVersion::V0x13 => return "V0x13".to_string(),
        }
    }

    pub fn from_string(string: &str) -> Result<ArgonVersion, ArgonParseError> {
        match string {
            "V0x10" => return Ok(ArgonVersion::V0x10),
            "V0x13" => return Ok(ArgonVersion::V0x13),
            _ => return Err(ArgonParseError::StringMatchError((string.to_string()))),
        }
    }

}

impl From<ArgonVersion> for Version {
    fn from(version: ArgonVersion) -> Self {
        match version {
            ArgonVersion::V0x10 => return argon2::Version::V0x10,
            ArgonVersion::V0x13 => return argon2::Version::V0x13,
        }
    }
}

impl From<argon2::Version> for ArgonVersion {
    fn from(version: argon2::Version) -> Self {
        match version {
            argon2::Version::V0x10 => return ArgonVersion::V0x10,
            argon2::Version::V0x13 => return ArgonVersion::V0x13,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ArgonAlgorithm{
    Argon2i,
    Argon2d,
    Argon2id,
}

impl ArgonAlgorithm {
    pub fn to_string(&self) -> String {
        match self {
            ArgonAlgorithm::Argon2i => return "Argon2i".to_string(),
            ArgonAlgorithm::Argon2d => return "Argon2d".to_string(),
            ArgonAlgorithm::Argon2id => return "Argon2id".to_string(),
        }
    }

    pub fn from_string(string: &str) -> Result<ArgonAlgorithm, ArgonParseError> {
        match string {
            "Argon2i" => return Ok(ArgonAlgorithm::Argon2i),
            "Argon2d" => return Ok(ArgonAlgorithm::Argon2d),
            "Argon2id" => return Ok(ArgonAlgorithm::Argon2id),
            _ => return Err(ArgonParseError::StringMatchError((string.to_string()))),
        }
    }
}

impl From<ArgonAlgorithm> for Algorithm {
    fn from(a: ArgonAlgorithm) -> Self {
        match a {
            ArgonAlgorithm::Argon2i  => argon2::Algorithm::Argon2i,
            ArgonAlgorithm::Argon2d  => argon2::Algorithm::Argon2d,
            ArgonAlgorithm::Argon2id => argon2::Algorithm::Argon2id,
        }
    }
}

impl From<argon2::Algorithm> for ArgonAlgorithm {
    fn from(a: argon2::Algorithm) -> Self {
        match a {
            argon2::Algorithm::Argon2i  => ArgonAlgorithm::Argon2i,
            argon2::Algorithm::Argon2d  => ArgonAlgorithm::Argon2d,
            argon2::Algorithm::Argon2id => ArgonAlgorithm::Argon2id,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptionData {
    pub salt_mnemonic: Option<Vec<u8>>,
    pub salt_seed: Vec<u8>,
    pub nonce_mnemonic: Option<Vec<u8>>,
    pub nonce_seed: Vec<u8>,
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
    pub output_len: usize,
    pub argon_algorithm: ArgonAlgorithm,
    pub argon_version: ArgonVersion,
}

 fn create_key_from_password_and_salt(password: &str, salt: &[u8]) -> [u8; 32] {
    let argon_parameters = Params::new(M_COST, T_COST, P_COST, Some(OUTPUT_LEN)).expect("Error creating argon parameters.");
    let argon = Argon2::new(argon2::Algorithm::from(ARGON_ALGORITHM), argon2::Version::from(ARGON_VERSION), argon_parameters);
    let mut key = [0u8; 32];
    argon.hash_password_into(password.as_bytes(), salt, &mut key).expect("Error with argon hasing password into key");
    return key;
}

fn create_key_from_password_and_salt_and_encryption_data(password: &str, salt: &[u8], encryption_data: &EncryptionData) -> [u8; 32] {
    let argon_version_converted = argon2::Version::from(encryption_data.argon_version);
    let argon_algorithm_converted = argon2::Algorithm::from(encryption_data.argon_algorithm);
    let argon_parameters = Params::new(encryption_data.m_cost, encryption_data.t_cost, encryption_data.p_cost, Some(encryption_data.output_len)).expect("Error creating argon parameters.");
    let argon = Argon2::new(argon_algorithm_converted, argon_version_converted, argon_parameters);
    let mut key = [0u8; 32];
    argon.hash_password_into(password.as_bytes(), salt, &mut key).expect("Error with argon hasing password into key");
    return key;
}

pub fn decrypt_wallet(wallet: TXRPWallet, password: &str) -> Result<TXRPWallet, Box<dyn std::error::Error>> {
    let encryption_data_exists = (wallet.encryption_enabled == true && wallet.encryption_data.is_some());
    match encryption_data_exists {
        false => {
            return Err(format!("Error: decrypt_wallet called on an unencrypted wallet.").into())
        },
        true => {
            let encryption_data = wallet.encryption_data.as_ref().expect("Error: encryption_data doesn't exist despute being in the encryption_data existing branch.");
            let seed_nonce = &encryption_data.nonce_seed;
            let seed_salt = &encryption_data.salt_seed;
            let seed_ciphertext = wallet.seed.as_str();
            let seed = decrypt_ciphertext_from_encryption_data(seed_ciphertext, password, seed_salt, seed_nonce, encryption_data);
            match seed {
                Ok(valid_decrypted_seed) => {
                    let decrypted_wallet  = TXRPWallet::generate_from_seed(wallet.name, valid_decrypted_seed.as_str(), None).expect("Error: the seed that was presented on an encrypted wallet somehow doesn't work on an unencrypted wallet. Perhaps there was an error decrypting the seed?");
                    return Ok(decrypted_wallet);
                },
                Err(error) => {
                    println!("Oh no");
                    return Err(format!("Error: password for decryption is likely incorrect.").into());
                }
            }
        
        }

    }
}


fn encrypt_from_salt_and_nonce(plaintext: &str, password: &str, salt: &[u8], nonce: &[u8]) -> String {
    let nonce_converted = Nonce::<Aes256Gcm>::from_slice(&nonce);
    let key = create_key_from_password_and_salt(&password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Error initializng cipher.");
    let plaintext_bytes = plaintext.as_bytes();
    let ciphertext = cipher.encrypt(&nonce_converted, plaintext_bytes).expect("Error creating cyphertext from nonce and plaintext bytes.");
    let ciphertext_base64 = general_purpose::STANDARD.encode(ciphertext);
    //println!("{:#?}", ciphertext_base64);
    return ciphertext_base64;
}

//returns ciphertext, salt, nonce
pub fn encrypt_plaintext(plaintext: &str, password: &str) -> (String, [u8; 32], Vec<u8>) {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng).to_vec();
    let ciphertext = encrypt_from_salt_and_nonce(plaintext, password, &salt, &nonce);
    return (ciphertext, salt, nonce);
}

pub fn decrypt_ciphertext(ciphertext: &str, password: &str, salt: &[u8], nonce: &[u8]) -> Result<String, aes_gcm::Error> {
    let ciphertext_as_bytes: Vec<u8> = general_purpose::STANDARD.decode(ciphertext.trim()).expect("Error converting base64 to bytes");
    let key = create_key_from_password_and_salt(password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Error initializng cipher.");
    let nonce_converted =  Nonce::<Aes256Gcm>::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce_converted, ciphertext_as_bytes.as_ref());
    match plaintext {
         Ok(plaintext) => {
            let plaintext_string = String::from_utf8(plaintext).expect("Error converting plaintext bytes to a string.");
            return Ok(plaintext_string);
        },
        Err(e) => return Err(e),
    }
    
}

pub fn decrypt_ciphertext_from_encryption_data(ciphertext: &str, password: &str, salt: &[u8], nonce:&[u8], encryption_data: &EncryptionData) -> Result<String, aes_gcm::Error> {
    let ciphertext_as_bytes: Vec<u8> = general_purpose::STANDARD.decode(ciphertext.trim()).expect("Error converting base64 to bytes");
    let key = create_key_from_password_and_salt_and_encryption_data(password, salt, encryption_data);
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Error initializng cipher.");
    let nonce_converted =  Nonce::<Aes256Gcm>::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce_converted, ciphertext_as_bytes.as_ref());
    match plaintext {
         Ok(plaintext) => {
            let plaintext_string = String::from_utf8(plaintext).expect("Error converting plaintext bytes to a string.");
            return Ok(plaintext_string);
        },
        Err(e) => return Err(e.into()),
    }

}