mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
mod database;
mod requests_io;
use aes_gcm::aes::cipher;
use base64::{engine::general_purpose, Engine};
use xrpl::wallet::Wallet;
use crate::database::{insert_wallet_into_db, get_wallets_from_db};
use crate::{encryption::encrypt_plaintext, wallet_functionality::TXRPWallet};

fn main(){
    println!("Program starting");
    file_io::initialize_directories();
    menu_navigation::home_menu();
    //let fee = TXRPWallet::get_open_ledger_fee().unwrap();
    //println!("{}", fee);
}