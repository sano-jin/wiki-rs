/// Basic 認証を行う
// use actix_web::{error, Error};
// use actix_web_httpauth::extractors::basic::BasicAuth;
// use dotenv::dotenv;
// use std::env;
use urlencoding;

#[derive(Debug)]
pub struct User {
    pub path: String, // id
    pub name: String,
    pub password: String,
}

impl User {
    pub fn create(user_name: &str, password: &str) -> Self {
        let filepath = urlencoding::encode(&user_name);
        Self {
            path: filepath.to_string(),
            name: user_name.to_string(),
            password: password.to_string(),
        }
    }

    /// check the user has the given id and password
    pub fn check(&self, user_name: &str, password: &str) -> bool {
        user_name == self.name && password == self.password
    }
}
