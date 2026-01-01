use argon2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::rngs::OsRng;

pub fn encrypt(data: &str, password: &str) {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    //let hash = argon2.hash_password(data, &salt)?.to_string();
    
}