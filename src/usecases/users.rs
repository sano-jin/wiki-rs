/// Basic 認証を行う
// use actix_web::{error, Error};
// use actix_web_httpauth::extractors::basic::BasicAuth;
// use dotenv::dotenv;
// use std::env;

#[derive(Debug)]
pub struct User {
    pub user_id: String,
    pub password: String,
}

impl User {
    pub fn create(user_id: String, password: String) -> Self {
        Self {
            user_id: user_id,
            password: password,
        }
    }

    /// check the user has the given id and password
    pub fn check(&self, user_id: &str, password: &str) -> bool {
        user_id == self.user_id && password == self.password
    }
}
