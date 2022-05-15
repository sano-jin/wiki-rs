// use crate::users;
// use crate::controllers::appstate::AppState;
use crate::gateways;
use crate::gateways::db::Database;
/// Basic 認証を行う
// use actix_web::{error, web, Error};
use actix_web::{error, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
// use crate::usecases::users::User;
// use dotenv::dotenv;
// use std::env;

/// Basic 認証を行う
pub fn authenticate<T: Clone + Database>(db: &T, auth: BasicAuth) -> Result<(), Error> {
    // println!("auth: {:?}", auth);

    let user_id = auth.user_id().to_string();
    let password = auth.password().expect("password").to_string();
    println!("user_id: {}, password: {}", user_id, password);

    // load users from database
    if let Ok(user) = gateways::users::get_user(db, &user_id) {
        if user.check(&user_id, &password) {
            println!("authentication succeeded");
            return Ok(());
        }
    }

    println!("authentication failed");
    return Err(error::ErrorUnauthorized("authentication failed"));
}

// /// .env からユーザ ID と password を取得してくる
// /// TODO: 現状リクエスト時に毎回これを取得してくるようになっているので，要改良
// pub fn load() -> User {
//     dotenv().ok();
//     let user_id = env::var("USER_ID").expect("USER_ID must be set");
//     let password = env::var("PASSWORD").expect("PASSWORD must be set");
//
//     User::create(user_id, password)
// }
