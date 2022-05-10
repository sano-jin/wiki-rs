// use crate::users;
use crate::usecases::users::User;
/// Basic 認証を行う
use actix_web::{error, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use dotenv::dotenv;
use std::env;

/// Basic 認証を行う
pub fn authenticate(user: User, auth: BasicAuth) -> Result<(), Error> {
    // println!("auth: {:?}", auth);

    let user_id = auth.user_id().to_string();
    let password = auth.password().expect("password").to_string();
    println!("user id: {}, password: {}", user_id, password);
    if !user.check(&user_id, &password) {
        println!("authentication failed");
        return Err(error::ErrorUnauthorized("authentication failed"));
    }
    Ok(())
}

/// .env からユーザ ID と password を取得してくる
/// TODO: 現状リクエスト時に毎回これを取得してくるようになっているので，要改良
pub fn load() -> User {
    dotenv().ok();
    let user_id = env::var("USER_ID").expect("USER_ID must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");

    User::create(user_id, password)
}
