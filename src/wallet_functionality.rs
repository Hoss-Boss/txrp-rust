use std::usize;
use aes_gcm::Error;
use serde_json::{error, json, Value};
use xrpl::models::requests::fee;
use xrpl::models::results::tx::Transaction;
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
use std::sync::{OnceLock};
use std::thread::Thread;
use xrpl::models::transactions::payment::Payment;
use xrpl::transaction::sign;
use xrpl::models::XRPAmount;
use xrpl::models::Amount;

const WORD_COUNT: usize = 12;

//TXRP will save wallets as seeds (and the mnemonics that generated them, if applicable)
//and recreate them via the xrpl-rust library when the user wants to transact. 
//The encryption flag will tell TXRP whether the data needs to be decrypted before use or not.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXRPWallet {
    pub classic_address: String,
    pub seed: String,
    pub mnemonic: Option<String>,
    pub encryption_enabled: bool,
    pub name: String,
    pub encryption_data: Option<EncryptionData>,
}

impl TXRPWallet {

    pub fn generate_from_nothing(wallet_name: String, encryption_password: Option<String>) -> TXRPWallet {
        let mnemonic = generate_mnemonic(WORD_COUNT);
        let mnemonic_string = mnemonic.to_string();
        let wallet = TXRPWallet::generate_from_mnemonic(wallet_name, &mnemonic_string, encryption_password);
        return wallet;
    }

    pub fn generate_from_seed(wallet_name: String, seed: &str, encryption_password: Option<String>) -> Result<TXRPWallet, Box<dyn std::error::Error>> {
        let xrpl_wallet = Wallet::new(seed, 0);
        match xrpl_wallet {
            Err(invalid_wallet_err) => return Err(format!("Error: The seed provided isn't a valid XRP wallet seed.").into()),
            Ok(valid_xrpl_wallet) => {
                let address = valid_xrpl_wallet.classic_address.clone();
                match encryption_password {
                    None => {
                    let wallet = TXRPWallet{classic_address: address, name: wallet_name, seed: seed.to_string(), mnemonic: None, encryption_enabled: false, encryption_data: None};
                    return Ok(wallet);
                    },
                    Some(password) => {
                        let (seed_ciphertext, seed_salt, seed_nonce) = encrypt_plaintext(&seed, &password);
                        
                        let encryption_data = EncryptionData {
                            salt_mnemonic: None,
                            salt_seed: seed_salt.to_vec(),
                            nonce_mnemonic: None,
                            nonce_seed: seed_nonce,
                            m_cost: encryption::M_COST,
                            t_cost: encryption::T_COST,
                            p_cost: encryption::P_COST,
                            output_len: encryption::OUTPUT_LEN,
                            argon_algorithm: ArgonAlgorithm::from(encryption::ARGON_ALGORITHM),
                            argon_version: ArgonVersion::from(encryption::ARGON_VERSION),
                        };
                
                
                let wallet = TXRPWallet{classic_address: address, name: wallet_name, seed: seed_ciphertext, mnemonic: None, encryption_enabled: true, encryption_data: Some(encryption_data)};
                return Ok(wallet);
                }
            }
            }
        }
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
                    salt_mnemonic: Some(mnemonic_salt.to_vec()),
                    salt_seed: seed_salt.to_vec(),
                    nonce_mnemonic: Some(mnemonic_nonce),
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

    pub fn encrypt(&self, password: &str) -> Result<TXRPWallet, Box<dyn std::error::Error>> {
        let current_wallet_is_encrypted = self.encryption_enabled;
        if (current_wallet_is_encrypted) {
            return Err(format!("Error: convert_unencrypted_wallet_to_encrypted wallet failed because current wallet is encrypted.").into());
        }
        let mnemonic_exists = self.mnemonic.is_some();
        let wallet_name = self.name.clone();
        if (mnemonic_exists) {
            let mnemonic_string = self.mnemonic.as_ref().expect("Error: mnemonic doens't exist despite the fact that we're in the mnemonic exists branch.").to_string();
            let encrypted_wallet = TXRPWallet::generate_from_mnemonic(wallet_name, mnemonic_string.as_str(), Some(password.to_string()));
            return Ok(encrypted_wallet);
        }
        else {
            return Ok(TXRPWallet::generate_from_seed(wallet_name, self.seed.as_str(), Some(password.to_string())).expect("Error: The seed somehow couldn't be used to create another wallet - even though it's already set in an existing wallet."));
        }
    }

    pub fn decrypt(&self, password: &str) -> Result<TXRPWallet, Box<dyn std::error::Error>> {
        let current_wallet_isnt_encrypted = !self.encryption_enabled;
        if (current_wallet_isnt_encrypted) {
            return Err(format!("Error: decrypt() failed because current wallet isn't encrypted.").into());
        }
        else {
            let wallet = encryption::decrypt_wallet(self.clone(), password);
            match wallet {
                Err(_) => return Err(format!("Error decryptiong wallet. Perhaps the password is wrong?").into()),
                Ok(valid_wallet) => return Ok(valid_wallet)
            }
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
        return TXRPWallet::get_sequence_of_address(&self.classic_address.clone());
    }

     pub fn get_sequence_of_address(address: &str) -> Result<u64, Box<dyn std::error::Error>> {
        let request_body = json!({"account": address, "ledger_index": "validated"});
        let response = xrp_ledger_call("account_info", request_body);
        match response {
            Ok(valid_response) => {
                let sequence = valid_response["result"]["account_data"]["Sequence"].as_u64();
                match sequence {
                    Some(valid_u64_sequence) => return Ok(valid_u64_sequence),
                    None => return Err(format!("Error: get_sequence_of_address couldn't a sequence from {}.", address).into()),
                }
            },
            Err(error_response) => {
                println!("XRP ledger response returned an error. Is your internet working?");
                return Err(error_response);
            }
        }
    }

    pub fn get_last_xrp_ledger_index(advance: u32) -> Result<u32, Box<dyn std::error::Error>> {
        let resp = xrp_ledger_call("ledger_current", json!({}))?;
        let current = resp["result"]["ledger_current_index"].as_u64().ok_or("Error getting XRP latest ledger index.")?;
        Ok(current as u32 + advance)
    }

    pub fn drops_to_xrp(drops: f64) -> f64 {
        return drops/1_000_000.0;
    }

    pub fn xrp_to_drops(xrp_amount: f64) -> f64 {
        return xrp_amount * 1_000_000.0;
    }

    pub fn get_ledger_minimum_fee() -> Result<f64, Box<dyn std::error::Error>> {
        let response = xrp_ledger_call("fee", json!({}));
        match response {
            Ok(valid_response) => {
                let minimum_fee = valid_response["result"]["drops"]["minimum_fee"].as_str();
                match minimum_fee {
                    Some(valid_minimum_fee) => {
                        let fee_f64: f64 = valid_minimum_fee.parse().expect("Error unwrapping fee_string to f64.");
                        return Ok(fee_f64);
                    },
                    None => return Err(format!("Error: minimum fee couldn't be found.").into())
                }
            },
            Err(invalid_response) => return Err(invalid_response)
        }
    }

    pub fn get_open_ledger_fee() -> Result<f64, Box<dyn std::error::Error>> {
        let response = xrp_ledger_call("fee", json!({}));
        match response {
            Ok(valid_response) => {
                let open_ledger_fee = valid_response["result"]["drops"]["open_ledger_fee"].as_str();
                match open_ledger_fee {
                    Some(valid_open_ledger_fee) => {
                        let fee_f64: f64 = valid_open_ledger_fee.parse().expect("Error unwrapping fee_string to f64.");
                        return Ok(fee_f64);
                    },
                    None => return Err(format!("Error: minimum fee couldn't be found.").into())
                }
            },
            Err(invalid_response) => return Err(invalid_response)
        }
    }

    pub fn send_xrp(&self, amount_in_drops: u32, destination: &str) -> Result<String, Box<dyn std::error::Error>> {
        let sequence = TXRPWallet::get_sequence_of_address(&self.classic_address.clone()).expect(format!("Error getting sequence of {}", self.classic_address.clone()).as_str());
        let fee = TXRPWallet::get_open_ledger_fee().expect("Error getting open ledger fee.");
        let last_ledger_sequence = TXRPWallet::get_last_xrp_ledger_index(5).expect("Error getting last ledger sequence of XRP ledger.");
        let mut payment = Payment::new(
        self.classic_address.clone().into(),
        None,
        Some(XRPAmount::from(fee.to_string())),
        None,
        Some(last_ledger_sequence),
        None,
        Some(sequence as u32),
        None,
        None,
        None,
        Amount::XRPAmount(amount_in_drops.to_string().into()),
        destination.to_string().into(),
        None,
        None,
        None,
        None,
        None,
    );

    let xrpl_wallet = Wallet::new(self.seed.as_str(), 0).expect("Error converting TXRPWallet to XRPL Wallet. Did we accidentally pass in an encrypted wallet?");
    sign(&mut payment, &xrpl_wallet, false).expect("Error signing transaction.");
    let response = xrp_ledger_call("submit", json!({ "tx_json": payment }));
    match response {
        Ok(valid_response) => {
            let hash = valid_response["result"]["tx_json"]["hash"].as_str();
            match hash {
                Some(valid_hash) => return Ok(String::from(valid_hash)),
                None => return Err(format!("Error: the response from our hash request returned nothing.").into())
            }
        },
        Err(invalid_response) => return Err(format!("Error getting response from transaction.").into())
    }


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