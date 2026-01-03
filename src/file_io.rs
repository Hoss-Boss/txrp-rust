use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use xrpl::wallet::Wallet;
use std::env::consts::OS;
use crate::wallet_functionality::TXRPWallet;


fn get_application_path() -> PathBuf {
    let directory = ProjectDirs::from("com", "TXRP", "txrp-rust")
    .expect("Failure getting data directory.");
    let directory_path = directory.data_dir().to_path_buf();
    return directory_path;
    }

pub fn check_for_existence_of_wallet_json_file() -> bool {
    let path = get_application_path().join("wallets.json");
    if path.is_file() {
        println!("wallets.json exists!");
        return true;
    }  
    else {
        println!("wallets.json is missing");
    }
    return false;
}

pub fn initialize_directories() {
    let path = get_application_path();
    fs::create_dir_all(&path).expect("There's some error in the app's directory structure.\nHas the project files been tampered with?\nClosing app for safety.");
    let file_path = get_application_path().join("wallets.json");   
}

pub fn load_json_files_into_wallets_vector(wallets: &Vec<TXRPWallet>) {
    let path = get_application_path();
    //let wallets_json_file = 
}
