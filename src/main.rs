mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
mod database;
use aes_gcm::aes::cipher;
use base64::{engine::general_purpose, Engine};
use crate::database::insert_wallet_into_db;
use crate::{encryption::encrypt_plaintext, wallet_functionality::TXRPWallet};

fn main(){
    file_io::initialize_directories();
    //menu_navigation::home_menu();
    let wallet = TXRPWallet::generate_without_mnemonic_or_seed("Dan2".to_string(), None);
    insert_wallet_into_db(&wallet);
}