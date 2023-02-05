use rustvrty::{login, migration, AuthType, User};
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), argon2::password_hash::Error> {
    migration().await;
    println!("Enter username");
    let mut username = String::new();
    stdin().read_line(&mut username).expect("readline");
    println!("Enter password");
    let mut password = String::new();
    stdin().read_line(&mut password).expect("readline");
    let username_and_password = AuthType::Watchword(User { username, password });
    dbg! {login(username_and_password).await.unwrap()};
    Ok(())
}
