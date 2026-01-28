use bip39::Mnemonic;
use crossterm::cursor;
use rusqlite::Connection;
use rusqlite::params;
use rusqlite::Row;
use crate::encryption::ArgonAlgorithm;
use crate::encryption::ArgonVersion;
use crate::encryption::EncryptionData;
use crate::{file_io::get_application_path, wallet_functionality::TXRPWallet};

enum DBError {
    ReadFromFileError,
}

fn get_db_connection() -> Connection {
    let file = get_application_path().join("wallets.db");
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    return connection;
}

pub fn apply_schema_to_db_file() {
    let connection = get_db_connection();
    let query = "CREATE TABLE IF NOT EXISTS `Wallet` 
    (`ID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `ClassicAddress` TEXT NOT NULL,
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
        "INSERT INTO `Wallet` (`Name`, `ClassicAddress` `Seed`, `EncryptionEnabled`)
        VALUES (?, ?, ?, ?);
        ".to_string();
        let parameters = params![wallet.name, wallet.classic_address, wallet.seed, wallet.encryption_enabled];
        transaction.execute(&query, parameters).expect("Error inserting Wallet row.");
    }
    else {
        let query = 
        "INSERT INTO `Wallet` (`Name`, `ClassicAddress`, `Mnemonic`, `Seed`, `EncryptionEnabled`)
        VALUES (?, ?, ?, ?, ?);
        ".to_string();
        let mnemonic = wallet.mnemonic.as_ref().expect("Error: no mnemonic despite wallet.mnemonic.is_none() being false.");
        let parameters = params![wallet.name, wallet.classic_address, mnemonic, wallet.seed, wallet.encryption_enabled];
        transaction.execute(&query, parameters).expect("Error inserting Wallet row.");
    }

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

fn row_to_wallet(row: &Row) -> rusqlite::Result<TXRPWallet> {
    let id: u16 = row.get(0).expect("Error: DB row has no ID.");
    let classic_address = row.get(1).expect("Error: DB row has no classic_address.");
    let name: String = row.get(2).expect("Error: DB row has no name.");
    let mnemonic: Option<String> = row.get(3).expect("Error: Failed reading mnemonic column.");
    let seed: String = row.get(4).expect("Error: DB row has no seed.");
    let encryption_enabled: bool = row.get(5).expect("Error: DB row has no encryption_enabled data.");
    let mut wallet = TXRPWallet{classic_address: classic_address, name: name, seed: seed, mnemonic: mnemonic, encryption_enabled: encryption_enabled, encryption_data: None};
    if encryption_enabled {
        let mnemonic_salt = row.get(8).expect("Error getting MnemonicSalt from DB row.");
        let mnemonic_nonce = row.get(9).expect("Error getting MnemonicNonce from DB row.");
        let seed_salt = row.get(10).expect("Error getting SeedSalt from DB row.");
        let seed_nonce = row.get(11).expect("Error getting SeedNonce from DB row.");
        let m_cost: u32 = row.get(12).expect("Error getting M_COST from DB row.");
        let t_cost: u32 = row.get(13).expect("Error getting T_COST from DB row.");
        let p_cost: u32 = row.get(14).expect("Error getting P_COST from DB row.");
        let output_len_u32: u32 = row.get(15).expect("Error getting OUTPUT_LEN from DB row.");
        let output_len = output_len_u32 as usize;
        let argon_algorithm_string: String = row.get(16).expect("Error getting MnemonicNonce from DB row.");
        let argon_version_string: String = row.get(17).expect("Error getting ArgonVersion from DB Row");
        let argon_algorithm = ArgonAlgorithm::from_string(&argon_algorithm_string).expect("Error: ArgonAlgorithmString isn't a valid ArgonAlgorithm");
        let argon_version = ArgonVersion::from_string(&argon_version_string).expect("Error: ArgonVersionString isn't a valid ArgonVersion");
        let encryption_data = EncryptionData{salt_mnemonic: mnemonic_salt, salt_seed: seed_salt, nonce_mnemonic: mnemonic_nonce, nonce_seed: seed_nonce, m_cost: m_cost, t_cost: t_cost, p_cost: p_cost, output_len: output_len, argon_algorithm: argon_algorithm, argon_version: argon_version};
        wallet.encryption_data = Some(encryption_data);
    }
    return Ok(wallet);
}

pub fn get_wallets_from_db() -> Vec<TXRPWallet> {
    let mut connection = get_db_connection();
    let query = 
    "SELECT
    Wallet.*,
    EncryptionData.*
    FROM Wallet
    LEFT JOIN EncryptionData
    ON EncryptionData.WalletID = Wallet.ID
    AND Wallet.EncryptionEnabled = 1;
    ".to_string();
    let mut statement = connection.prepare(&query).expect("Error preparing statement");
    let wallets_iterator = statement.query_map([], row_to_wallet).expect("Error mapping rows to function.");
    let wallets: Vec<TXRPWallet> = wallets_iterator.collect::<rusqlite::Result<Vec<TXRPWallet>>>().expect("Error collecting wallets into Vec");
    return wallets;
}


pub fn attempt_test_data_insertion() {
    let file = get_application_path().join("wallets.db");
    let query = r#"INSERT INTO `Wallet` (`Name`, `Seed`, `EncryptionEnabled`) VALUES ("Hoss", "123", 0)"#;
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    connection.execute(&query, []).expect("Error writing to wallets.db");
}