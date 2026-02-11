use std::usize;
use serde_json::{json, Value};
use xrpl::wallet::Wallet;
use bip39::{Language, Mnemonic};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use xrpl::core::keypairs::generate_seed;
use crate::encryption;
use crate::encryption::EncryptionData;
use crate::encrypt_plaintext;
use crate::encryption::{M_COST, T_COST, P_COST, OUTPUT_LEN, ARGON_ALGORITHM, ARGON_VERSION};
use crate::encryption::{ArgonAlgorithm, ArgonVersion};
use crate::requests_io::{xrp_ledger_call};
use xrpl::clients::XRPLSyncClient;
use xrpl::clients::json_rpc::JsonRpcClient;
use xrpl::models::requests::account_info::AccountInfo;
use xrpl::models::results::account_info::AccountInfoVersionMap;
use reqwest::Client;
use url::Url;

const WORD_COUNT: usize = 12;

//TXRP will save wallets as seeds (and the mnemonics that generated them, if applicable)
//and recreate them via the xrpl-rust library when the user wants to transact. 
//The encryption flag will tell TXRP whether the data needs to be decrypted before use or not.

#[derive(Serialize, Deserialize, Debug)]
pub struct TXRPWallet {
    pub classic_address: String,
    pub seed: String,
    pub mnemonic: Option<String>,
    pub encryption_enabled: bool,
    pub name: String,
    pub encryption_data: Option<EncryptionData>,
}

impl TXRPWallet {

    pub fn generate_without_mnemonic_or_seed(wallet_name: String, encryption_password: Option<String>) -> TXRPWallet {
        let mnemonic = generate_mnemonic(WORD_COUNT);
        let mnemonic_string = mnemonic.to_string();
        let wallet = TXRPWallet::generate_from_mnemonic(wallet_name, &mnemonic_string, encryption_password);
        return wallet;
    }


    pub fn generate_from_mnemonic(wallet_name: String, mnemonic_string: &str, encryption_password: Option<String>) -> TXRPWallet {
        let mnemonic = Mnemonic::parse(mnemonic_string).expect("Error: Provided mnemonic doesn't correctly parse into a Mnemonic type.");
        let seed = generate_family_seed_from_mnemonic(&mnemonic);
        let classic_address = Wallet::new(&seed, 0).expect("Error converting TXRPWallet to XRPL Wallet").classic_address.clone();
        
        match encryption_password {
            None => {
                let wallet = TXRPWallet{classic_address: classic_address, name: wallet_name, seed: seed, mnemonic: Some(mnemonic_string.to_string()), encryption_enabled: false, encryption_data: None};
                return wallet;
            },
            Some(password) => {
                let (seed_ciphertext, seed_salt, seed_nonce) = encrypt_plaintext(&seed, &password);
                let (mnemonic_ciphertext, mnemonic_salt, mnemonic_nonce) = encrypt_plaintext(&mnemonic_string, &password);
                
                let encryption_data = EncryptionData {
                    salt_mnemonic: mnemonic_salt.to_vec(),
                    salt_seed: seed_salt.to_vec(),
                    nonce_mnemonic: mnemonic_nonce,
                    nonce_seed: seed_nonce,
                    m_cost: encryption::M_COST,
                    t_cost: encryption::T_COST,
                    p_cost: encryption::P_COST,
                    output_len: encryption::OUTPUT_LEN,
                    argon_algorithm: ArgonAlgorithm::from(encryption::ARGON_ALGORITHM),
                    argon_version: ArgonVersion::from(encryption::ARGON_VERSION),
                    };
                
                
                let wallet = TXRPWallet{classic_address: classic_address, name: wallet_name, seed: seed_ciphertext, mnemonic: Some(mnemonic_ciphertext), encryption_enabled: true, encryption_data: Some(encryption_data)};
                return wallet;
            },
        }
    }

    pub fn view_balance_of_address(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
         let request_body = json!({"account": address, "ledger_index": "validated"});
         let response = xrp_ledger_call("account_info", request_body);
         match response {
            Ok(valid_response) => {
                let drops_string = valid_response["result"]["account_data"]["Balance"].as_str().unwrap_or("0");
                let drops: f64 = drops_string.parse().expect("Error parsing drops_string into u64.");
                let xrp = (drops as f64) / 1_000_000.0;
                return Ok(xrp);
            },
            Err(error_response) => {
                println!("Error looking up balance of XRP address. Is your internet connection working?");
                return Err(error_response);
            }
         }
    }

    pub fn view_balance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let address = self.classic_address.clone();
        return TXRPWallet::view_balance_of_address(&address);
    }

    pub fn get_sequence(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let request_body = json!({"account": self.classic_address.clone(), "ledger_index": "validated"});
        let response = xrp_ledger_call("account_info", request_body);
        match response {
            Ok(valid_response) => {
                let sequence = valid_response["result"]["account_data"]["Sequence"].as_u64().expect("Error unwrapping account_data sequence.");
                return Ok(sequence);
            },
            Err(error_response) => {
                println!("XRP ledger response returned an error. Is your internet working?");
                return Err(error_response);
            }
        }
        //let sequence = response["result"]["account_data"]["Sequence"].as_u64();
    }


    pub fn send_xrp(amount: f64, destination_address: &str) {
        
    }

    pub fn to_json(&self) -> String {
        let string = serde_json::to_string_pretty(&self).expect("Error converting TXRPWallet to json.");
        return string;
    }

    pub fn from_json(json_wallet: &str) -> TXRPWallet {
        let wallet_in_json_format: TXRPWallet = serde_json::from_str(&json_wallet).expect("Error converting JSON to TXRPWallet object.");
        return wallet_in_json_format;
    }
}

pub fn generate_mnemonic(word_amount: usize) -> Mnemonic {
    let mut rng = OsRng;
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, word_amount).expect("Error generating mnemonic.");
    return mnemonic;
}

pub fn generate_family_seed_from_mnemonic(mnemonic: &Mnemonic) -> String{
    let entropy_vector = mnemonic.to_entropy();
    let slice_of_entropy_vector = entropy_vector.as_slice();
    let entropy_array: [u8; 16] = slice_of_entropy_vector.try_into().expect("Error pushing entropy vector into 16 byte array.");
    let seed = generate_seed(Some(entropy_array), None).expect("Error generating seed from entropy array of mnemonic.");
    return seed;
}

pub fn generate_xrp_wallet_without_mnemonic_or_seed(word_amount: Option<usize>) -> Wallet {
    let word_count = word_amount.unwrap_or(12);
    generate_mnemonic(word_count);
    let wallet = Wallet::create(None).expect("Error generating wallet from generate_xrp_wallet_without_mnemonic_or_seed() function.");
    return wallet;
}