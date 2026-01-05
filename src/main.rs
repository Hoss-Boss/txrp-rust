use std::io;
use xrpl::wallet::Wallet;
use crate::wallet_functionality::TXRPWallet;
mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
mod database;
fn main(){
    file_io::initialize_directories();
    //menu_navigation::home_menu();
    let wallet = TXRPWallet::generate_without_mnemonic_or_seed("wallet_name".to_string(), false);
    println!("Wallet as string:\n{}", wallet.to_json());
}