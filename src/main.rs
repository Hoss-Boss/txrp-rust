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
    //menu_navigation::home_menu();
    let u_wallet = TXRPWallet::generate_from_nothing("u1".to_string(), None);
    let e_wallet = u_wallet.encrypt( "test").unwrap();
    let u_wallet_2 = e_wallet.decrypt("test").unwrap();
    println!("u_wallet seed: {}", u_wallet.seed);
    println!("e_wallet seed: {}", e_wallet.seed);
    println!("u_wallet seed: {}", u_wallet_2.seed);
}