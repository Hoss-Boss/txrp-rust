use crate::file_io::get_application_path;

pub fn apply_schema_to_db_file() {
    let file = get_application_path().join("wallets.db");
    let query = "CREATE TABLE IF NOT EXISTS `Wallet` 
    (`ID` INTEGER PRIMARY KEY AUTOINCREMENT,
    `Name` TEXT NOT NULL,
    `Mnemonic` TEXT,
    `Seed` TEXT NOT NULL,
    `EncryptionEnabled` INTEGER NOT NULL DEFAULT 0
    )
    ".to_string();
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
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

pub fn attempt_test_data_insertion() {
    let file = get_application_path().join("wallets.db");
    let query = r#"INSERT INTO `Wallet` (`Name`, `Seed`, `EncryptionEnabled`) VALUES ("Hoss", "123", 0)"#;
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    connection.execute(&query, []).expect("Error writing to wallets.db");
}