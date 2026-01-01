use std::io;
use xrpl::wallet::Wallet;

use crate::wallet_functionality::TXRPWallet;
mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
fn main(){
    file_io::initialize_directories();
    menu_navigation::home_menu();
}