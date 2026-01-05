mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
mod database;
use crate::wallet_functionality::TXRPWallet;

fn main(){
    file_io::initialize_directories();
    //menu_navigation::home_menu();
    let wallet = TXRPWallet::generate_without_mnemonic_or_seed("wallet_name".to_string(), false);
    println!("Wallet as string:\n{}", wallet.to_json());
}