mod watchword;

pub use crate::sec::watchword::login as watchword_login;
pub use crate::sec::watchword::User;

pub enum AuthType {
    Watchword(User),
}

/*
    Check user exists, check if user can be auth'd from the given info, then return an access token. else, reject login.
*/
pub async fn login(identity: AuthType) -> Result<String, ()> {
    let success = match identity {
        AuthType::Watchword(u) => watchword_login(u).await?,
    };
    Ok("totallylegitJWT".to_string())
}
