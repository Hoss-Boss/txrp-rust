use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use xrpl::wallet::Wallet;
use std::env::consts::OS;
use crate::wallet_functionality::TXRPWallet;
use crate::database::apply_schema_to_db_file;


pub fn get_application_path() -> PathBuf {
    let directory = ProjectDirs::from("com", "TXRP", "txrp-rust")
    .expect("Failure getting data directory.");
    let directory_path = directory.data_dir().to_path_buf();
    return directory_path;
    }

pub fn check_for_existence_of_wallets_db_file() -> bool {
    let path = get_application_path().join("wallets.db");
    if path.is_file() {
        println!("wallets.db exists!");
        return true;
    }  
    else {
        println!("wallets.db is missing. We should create it.");
    }
    return false;
}

pub fn initialize_directories() {
    let path = get_application_path();
    fs::create_dir_all(&path).expect("There's some error in the app's directory structure.\nHas the project files been tampered with?\nClosing app for safety.");
    let file_path = get_application_path().join("wallets.db");
    let wallets_db_file_exists = check_for_existence_of_wallets_db_file();
    if (wallets_db_file_exists == false) {
        fs::File::create(&file_path).expect("Error creating wallets.db in application directory.");  
    }
    apply_schema_to_db_file();
}
