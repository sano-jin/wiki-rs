/// Basic 認証を行う
use actix_web::{error, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct User {
    pub user_id: String,
    pub password: String,
}

impl User {
    pub fn create(user_id: String, password: String) -> User {
        User {
            user_id: user_id,
            password: password,
        }
    }

    pub fn check(&self, user_id: &str, password: &str) -> bool {
        user_id == self.user_id && password == self.password
    }

    pub fn authenticate(&self, auth: BasicAuth) -> Result<(), Error> {
        println!("auth: {:?}", auth);

        let user_id = auth.user_id().to_string();
        let password = auth.password().expect("password").to_string();
        println!("user id: {}, password: {}", user_id, password);
        if !self.check(&user_id, &password) {
            println!("authentication failed");
            return Err(error::ErrorUnauthorized("authentication failed"));
        }
        Ok(())
    }

    pub fn load() -> User {
        dotenv().ok();
        let user_id = env::var("USER_ID").expect("USER_ID must be set");
        let password = env::var("PASSWORD").expect("PASSWORD must be set");

        User::create(user_id, password)
    }
}
