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
    clear_screen();
    println!("Welcome to TXRP - Rusty edition! v1.0");
    println!("What do you want to do?");
    println!("1. Create a wallet");
    println!("2. View and transact with existing wallets");
    println!("3. Import wallet via family seed or mnemonic");
    println!("4. Support the project");
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
    println!("Choose a name for the wallet. Enter b or h to go back.");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error while reading user input");
    let user_input_trimmed = user_input.trim();
    match user_input_trimmed.to_lowercase().as_str() {
        "b" => {
            return MenuAction::Back;
            },
        "h" => {
            return MenuAction::Home;
        },
        _ => {
            clear_screen();
            let wallet_name = user_input_trimmed.to_string();
            loop {
            user_input.clear();
            println!("Do you want to encrypt the wallet's mnemonics and seed with a password?");
            println!("Enter y or n. Enter b to go back or h to go home.");
            std::io::stdin().read_line(&mut user_input).expect("Error while reading user input");
            let user_input_trimmed = user_input.trim();
            match user_input_trimmed.to_lowercase().as_str() {
                "b" => {
                    clear_screen();
                    break;
                },
                "h" => return MenuAction::Home,
                "y" => {
                    loop {
                    clear_screen();
                    println!("Create a password to encrypt the wallet's mnemonics and seed. Enter b to go back or h to go to the home menu.");
                    user_input.clear();
                    std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");

                    if (user_input.trim() == "b") {
                        break;
                    }
                    else if (user_input.trim() == "h") {
                        return MenuAction::Home;
                    }

                    println!("Confirm your password by entering it again.");
                    let mut user_input_2 = String::new();
                    std::io::stdin().read_line(&mut user_input_2).expect("Error reading user input.");
                    if (user_input.trim() != user_input_2.trim()) {
                        clear_screen();
                        println!("Passwords don't match. Try again.")
                    }
                    else {
                        let wallet = TXRPWallet::generate_without_mnemonic_or_seed(wallet_name, Some(user_input_2));
                        return MenuAction::Home;
                    }

                }//loop
                },
                "n" => {
                    let wallet = TXRPWallet::generate_without_mnemonic_or_seed(wallet_name, None);
                    println!("Creating wallet with the following info:");
                    println!("Name: {}", wallet.name);
                    println!("Mnemonics: {}", wallet.mnemonic.expect("Error unwrapping wallet.mnemonic").to_string());
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
                    clear_screen();
                    println!("Try again.\n");
                    continue;
                }
            }
        }


            
        }
    }
}
}


