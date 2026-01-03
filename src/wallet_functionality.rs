use std::usize;
use xrpl::wallet::Wallet;
use bip39::{Language, Mnemonic};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use xrpl::core::keypairs::generate_seed;

const WORD_COUNT: usize = 12;

//TXRP will save wallets as seeds (and the mnemonics that generated them, if applicable)
//and recreate them via the xrpl-rust library when the user wants to transact. 
//The encryption flag will tell TXRP whether the data needs to be decrypted before use or not.


#[derive(Serialize, Deserialize, Debug)]
pub struct TXRPWallet {
    pub seed: String,
    pub mnemonic: Option<Mnemonic>,
    pub encryption_enabled: bool,
    pub name: String
}

impl TXRPWallet {

    pub fn generate_without_mnemonic_or_seed(wallet_name: String, encryption_enabled: bool) -> TXRPWallet {
        //let encryption_enabled = encryption_enabled.unwrap_or(false);
        let mnemonic = generate_mnemonic(WORD_COUNT);
        let seed = generate_family_seed_from_mnemonic(&mnemonic);
        
        if (encryption_enabled == false) {
            let hoss_wallet = TXRPWallet{name: wallet_name, seed: seed, mnemonic: Some(mnemonic), encryption_enabled: encryption_enabled};
            return hoss_wallet;
        }
        else {
            return TXRPWallet{name: wallet_name, seed: seed, mnemonic: Some(mnemonic), encryption_enabled: encryption_enabled};
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