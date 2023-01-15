use futures::executor::block_on;
use rustvrty::{login, User};
use std::io::stdin;

fn main() -> Result<(), argon2::password_hash::Error> {
    block_on(async_main())
}

async fn async_main() -> Result<(), argon2::password_hash::Error> {
    println!("Enter password");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("readline");
    let res = salt_and_hash_to_string(&input)?;
    println!("Enter password again");
    input = String::new();
    stdin().read_line(&mut input).expect("readline");
    let are_the_same = verify_password_hashes(input, &res)?;
    dbg!(&are_the_same);
    Ok(())
}
