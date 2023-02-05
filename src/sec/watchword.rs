use crate::db::connect;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand_core::OsRng;
use sea_orm::prelude::Uuid;

pub struct User {
    pub username: String,
    pub password: String,
}

struct UuidPasswordHash {
    uuid: Uuid,
    hash: String,
}

fn salt_and_hash_to_string(input: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(input.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

fn verify_password_hashes(
    raw: String,
    hashed: &String,
) -> Result<bool, argon2::password_hash::Error> {
    Ok(Argon2::default()
        .verify_password(raw.as_bytes(), &PasswordHash::new(hashed)?)
        .is_ok())
}

async fn get_user(username: String) -> Result<UuidPasswordHash, ()> {
    let db_connection = connect().await;
    match db_connection {
        Ok(connection) => {
            
        }
        Err(error) => eprintln!("{error}"),
    }
    todo! {};
}

pub async fn login(user: User) -> Result<bool, ()> {
    // get user id
    let db_user = get_user(user.username).await?;
    let authed = verify_password_hashes(user.password, &db_user.hash);

    Ok(false)
}
