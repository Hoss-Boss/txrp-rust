mod wallet_functionality;
mod file_io;
mod menu_navigation;
mod encryption;
mod database;
use aes_gcm::aes::cipher;
use base64::{engine::general_purpose, Engine};


use crate::{encryption::encrypt_plaintext, wallet_functionality::TXRPWallet};

fn main(){
    file_io::initialize_directories();
    menu_navigation::home_menu();
    //let plaintext = String::from("My Secret");
    //let (ciphertext, salt, nonce) = encrypt_plaintext(&plaintext, "bosco");
    //println!("Ciphertext: {}", ciphertext);
    //let back_to_plaintext = encryption::decrypt_ciphertext(&ciphertext, "bosco", &salt, &nonce);
    //println!("converted back: {}", back_to_plaintext.expect("Wrong password"));

}