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
    connection.execute(&query, []).expect("Error writing to wallets.db");
}

pub fn attempt_test_data_insertion() {
    let file = get_application_path().join("wallets.db");
    let query = r#"INSERT INTO `Wallet` (`Name`, `Seed`, `EncryptionEnabled`) VALUES ("Hoss", "123", 0)"#;
    let connection = rusqlite::Connection::open(file).expect("Error opening wallets.db");
    connection.execute(&query, []).expect("Error writing to wallets.db");
}