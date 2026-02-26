use std::{io, num::ParseIntError, process::exit, ptr::read_unaligned, thread::sleep, time::Duration};
use crossterm::{execute,terminal::{Clear, ClearType},cursor::MoveTo};
use crate::database::{insert_wallet_into_db, get_wallets_from_db};
use crate::wallet_functionality::TXRPWallet;
use xrpl::{asynch::wallet, wallet::Wallet};

struct UserOptions {
    fee_preference: FeePreference,
}
enum FeePreference {
    MinimumFees,
    HigherFees,
}
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
    println!("4. Delete a wallet");
    println!("5. Options");
    println!("6. Support the project");
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
        "2" => {
            clear_screen();
            option_2_view_and_transact_with_wallets();
        },
        "3" => {
            clear_screen();
            option_3_import_wallet();
        }
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
                        let wallet = TXRPWallet::generate_from_nothing(wallet_name, Some(user_input_2));
                        insert_wallet_into_db(&wallet);
                        return MenuAction::Home;
                    }

                }//loop
                },
                "n" => {
                    let wallet = TXRPWallet::generate_from_nothing(wallet_name, None);
                    insert_wallet_into_db(&wallet);
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

fn option_2_view_and_transact_with_wallets() -> MenuAction{
loop{
    println!("Select a wallet ID. Enter b or h to go back home.");
    let wallets = get_wallets_from_db();
    for (index, wallet) in wallets.iter().enumerate() {
        println!("ID: {}, Name: {}, Address: {}", index, wallet.name, wallet.classic_address);
    }
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error getting user input.");
    let user_input_trimmed = user_input.trim();
    match user_input_trimmed.to_lowercase().as_str() {
        "b" => return MenuAction::Back,
        "h" => return MenuAction::Home,
        _ => {
            let parsed_value = user_input_trimmed.to_lowercase().parse::<usize>();
            if (parsed_value.is_err()) {
                clear_screen();
                println!("The input didn't parse into a number or menu action. Try again.");
                continue;
            }
            else {
                clear_screen();
                let index = parsed_value.expect("parsed_value failed to unwrap despite the fact that it was already checked.");
                if (index >= wallets.len()) {
                    clear_screen();
                    println!("Error: Number entered is larger than any wallet ID. Try again.");
                    continue;
                }
                let wallet = &wallets[index];
            }
        },
    }
    
}

}

fn option_3_import_wallet() -> MenuAction {
    loop {
    clear_screen();
    println!("How do you want to import your wallet?");
    println!("1. family seed");
    println!("2. 12 word seed phrase/mnemonic");
    println!("b. Back");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Error getting user input");
    user_input = user_input.trim().to_string();
    match user_input.to_lowercase().as_str() {
        "1" => {
            loop{
            clear_screen();
            user_input.clear();
            println!("Enter your XRP wallet's family seed. Enter b to go back, h to go home.");
            std::io::stdin().read_line(&mut user_input).expect("Error getting user input");
            user_input = user_input.trim().to_string();
            if (&user_input.to_lowercase() == "b" || &user_input.to_lowercase() == "h") {
                return MenuAction::Home;
            }
            
            println!("Attempting to create a wallet from this seed phrase.");
            let seed_validity = TXRPWallet::generate_from_seed("Placeholder".to_string(), &user_input, None);
            let mut seed = String::new();
            match seed_validity {
                Ok(wallet_from_valid_seet) => {
                    seed = wallet_from_valid_seet.seed;
                    user_input.clear();
                    println!("Enter a name for this wallet. Enter b to go back, h to go home.");
                    std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");
                    user_input = user_input.trim().to_string();

                    if &user_input == "b" || &user_input == "B" {
                        continue;
                    }

                    else if &user_input == "h" || &user_input == "H" {
                        return MenuAction::Home;
                    }

                    let name = user_input.clone();
                    
                    loop {
                    user_input.clear();
                    clear_screen();
                    println!("Will this wallet use encryption? Enter y or n. Enter b to go back, h to go home.");
                    std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");
                    match user_input.trim().to_lowercase().as_str() {
                        "n" => {
                            let wallet = TXRPWallet::generate_from_seed(name.clone(), &seed, None).expect("Error: seed was thought to be valid, but an error occured while generating the wallet.");
                            insert_wallet_into_db(&wallet);
                            println!("Wallet {} ({}) inserted into database. Press enter to go back home.", wallet.name, wallet.classic_address);
                            std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");
                            return MenuAction::Home;
                        },
                        "y" => {
                            loop {
                            clear_screen();
                            user_input.clear();
                            println!("Choose a password to encrypt the wallet's sensitive info. Enter b to go back, h to go home.");
                            std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");
                            user_input = user_input.trim().to_string();
                            if user_input == "b" {
                                continue;
                            }
                            else if user_input == "h" {
                                return MenuAction::Home;
                            }

                            let wallet = TXRPWallet::generate_from_seed(name, &seed, Some(user_input.clone())).expect("Error: seed was thought to be valid, but an error occured while generating the wallet.");
                            insert_wallet_into_db(&wallet);
                            println!("Wallet {} ({}) imported! Press enter go go back home.", wallet.name, wallet.classic_address);
                            std::io::stdin().read_line(&mut user_input).expect("Error reading user input.");
                            return MenuAction::Home;

                            }
                        },
                        "h" => return MenuAction::Home,
                        "b" => break,
                        _ => continue
                    }
                    }



                },
                Err(invalid_wallet) => {
                    println!("This seed phrase doesn't correlate to a valid wallet.\nPress enter to continue.");
                    let mut placeholder_buffer = String::new();
                    std::io::stdin().read_line(&mut placeholder_buffer).expect("Error reading user input.");
                    continue;
                }
            }

        }//loop

        },
        "2" => {

        },
        "b" => return MenuAction::Home,
        "h" => return MenuAction::Home,
        _ => {
            continue;
        }
    }
}//loop
}//option_3_import_wallet
