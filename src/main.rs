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
    file_io::initialize_directories();
    //menu_navigation::home_menu();
    let wallet = TXRPWallet::generate_without_mnemonic_or_seed("Dan".to_string(), None);
    let balance = wallet.view_balance();
    println!("Balance is {}", balance.expect("Error getting balance"));
    //insert_wallet_into_db(&wallet);
    //let wallets = get_wallets_from_db();
    //let wallet_0 = &wallets[0];
    //let xrpl_wallet = Wallet::new(wallet_0.seed.as_str(), 0).expect("Error converting TXRPWallet to XRPL Wallet");
    //println!("{:?}", wallets);
}