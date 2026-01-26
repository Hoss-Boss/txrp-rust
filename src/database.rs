use crossterm::cursor;
use rusqlite::Connection;
use rusqlite::params;

use crate::{file_io::get_application_path, wallet_functionality::TXRPWallet};

fn get_db_connection() -> Connection {
    let file = get_application_path().join("wallets.db");
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    return connection;
}

pub fn apply_schema_to_db_file() {
    let connection = get_db_connection();
    let query = "CREATE TABLE IF NOT EXISTS `Wallet` 
    (`ID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `Name` TEXT NOT NULL,
    `Mnemonic` TEXT,
    `Seed` TEXT NOT NULL,
    `EncryptionEnabled` INTEGER NOT NULL DEFAULT 0
    )
    ".to_string();
    let query_2 = "CREATE TABLE IF NOT EXISTS `EncryptionData` 
    (`ID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `WalletID` INTEGER NOT NULL,
    `MnemonicSalt` TEXT NOT NULL,
    `MnemonicNonce` TEXT NOT NULL,
    `SeedSalt` TEXT NOT NULL,
    `SeedNonce` TEXT NOT NULL,
    `M_COST` INTEGER NOT NULL,
    `T_COST` INTEGER NOT NULL,
    `P_COST` INTEGER NOT NULL,
    `OUTPUT_LEN` INTEGER NOT NULL,
    `ArgonAlgorithm` TEXT NOT NULL,
    `ArgonVersion` TEXT NOT NULL,

    FOREIGN KEY (WalletID) REFERENCES Wallet(ID)
    ON UPDATE CASCADE
    ON DELETE CASCADE
    )
    ".to_string();
    connection.execute(&query, []).expect("Error writing to wallets.db");
    connection.execute(&query_2, []).expect("Error writing to wallets.db");
}

pub fn insert_wallet_into_db(wallet: &TXRPWallet) {
    let mut connection = get_db_connection();
    let mut transaction = connection.transaction().expect("Error initializing transaction."); // atomic: wallet + encryptiondata together
    if wallet.mnemonic.is_none() {
        let query = 
        "INSERT INTO `Wallet` (`Name`, `Seed`, `EncryptionEnabled`)
        VALUES (?1, ?2, ?3, ?4);
        ".to_string();
        let parameters = params![wallet.name, wallet.seed, wallet.encryption_enabled];
        transaction.execute(&query, parameters).expect("Error inserting Wallet row.");

        if (wallet.encryption_enabled) {
            let wallet_id: i64 = transaction.last_insert_rowid();
            let encryption_data = wallet.encryption_data.as_ref().expect("EncryptionData for wallet doesn't exist, despite teh fact that encryption_enabled is true.");
            let output_len: u8 = u8::try_from(encryption_data.output_len).expect("Error converting wallet.encryption_data.output_len to u8.");
            let query = 
            "INSERT INTO `EncryptionData` (`WalletID`, `MnemonicSalt`, `MnemonicNonce`, `SeedSalt`, `SeedNonce`, `M_COST`, `T_COST`, `P_COST`, `OUTPUT_LEN`, `ArgonAlgorithm`, `ArgonVersion`)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ".to_string();
            let parameters = params![wallet_id, encryption_data.salt_mnemonic, encryption_data.nonce_mnemonic, encryption_data.salt_seed, encryption_data.nonce_seed, encryption_data.m_cost, encryption_data.t_cost, encryption_data.p_cost, output_len, encryption_data.argon_algorithm.to_string(), encryption_data.argon_version.to_string()];
            transaction.execute(&query, parameters).expect("Error inserting EncryptionData row.");
        }
        transaction.commit().expect("Error commiting transaction(s).");

    }
    else {
        let query = 
        "INSERT INTO `Wallet` (`Name`, `Mnemonic`, `Seed`, `EncryptionEnabled`)
        VALUES (?, ?, ?, ?);
        ".to_string();
    }
}


pub fn attempt_test_data_insertion() {
    let file = get_application_path().join("wallets.db");
    let query = r#"INSERT INTO `Wallet` (`Name`, `Seed`, `EncryptionEnabled`) VALUES ("Hoss", "123", 0)"#;
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    connection.execute(&query, []).expect("Error writing to wallets.db");
}