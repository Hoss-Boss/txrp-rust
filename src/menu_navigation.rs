use std::{io, process::exit};
use crossterm::{execute,terminal::{Clear, ClearType},cursor::MoveTo};

use crate::wallet_functionality::TXRPWallet;
use xrpl::wallet::Wallet;

enum MenuAction {
    Back,
    Home
}

fn clear_screen() {
    execute!(
        std::io::stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
    ).unwrap();
}

pub fn home_menu() {
loop {
    println!("Welcome to TXRP - Rusty edition! v1.0");
    println!("What do you want to do?");
    println!("1. Create a wallet");
    println!("2. View and transact with existing wallets");
    println!("3. Import wallet via family seed or mnemonic");
    println!("4. Configure options");
    println!("5. Support the project");
    println!("b. Exit");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error when reading user input");
    let user_input_trimmed = user_input.trim();

    match user_input_trimmed {
        "1" =>  {
            clear_screen();
            option_1_create_a_wallet();
            continue;
        },
        "b" => {
            exit(0);
        }
        _ => {
            clear_screen();
            println!("Try again");
        }
    }
}
}

fn option_1_create_a_wallet() -> MenuAction {
loop{
    println!("Choose a name for the wallet. Enter b to go back. Enter h to go to the main menu.");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error while reading user input");
    let user_input_trimmed = user_input.trim();
    match user_input_trimmed.to_lowercase().as_str() {
        "b" => {
            clear_screen();
            return MenuAction::Back;
            },
        "h" => {
            clear_screen();
            return MenuAction::Home;
        },
        _ => {
            clear_screen();
            let wallet_name = user_input_trimmed.to_string();
            println!("Do you want to encrypt the wallet's mnemonics and seed with a password?");
            println!("Enter y or n");
            loop {
            user_input.clear();
            std::io::stdin().read_line(&mut user_input).expect("Error while reading user input");
            let user_input_trimmed = user_input.trim();
            match user_input_trimmed.to_lowercase().as_str() {
                "y" => {
                    
                },
                "n" => {
                    let wallet = TXRPWallet::generate_without_mnemonic_or_seed(wallet_name, false);
                    println!("Creating wallet with the following info:");
                    println!("Name: {}", wallet.name);
                    println!("Mnemonics: {}", wallet.mnemonic.unwrap().to_string());
                    println!("Seed: {}", wallet.seed);
                    let xrpl_wallet = Wallet::new(&wallet.seed, 0).expect("Error generating XRPL wallet from seed");
                    println!("Address: {}", xrpl_wallet.classic_address);
                    println!("Press enter to return home.");
                    let mut user_confirmation = String::new();
                    std::io::stdin().read_line(&mut user_confirmation).expect("Error while reading user input.");
                    clear_screen();
                    return MenuAction::Home;
                }
                _ => {
                    println!("Try again. Do you want to encrypt this wallet with a password?\nEnter y or n.");
                    continue;
                }
            }
        }


            
        }
    }
}
}


