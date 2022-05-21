use serde::{Deserialize, Serialize};
/// Basic 認証を行う
// use actix_web::{error, Error};
// use actix_web_httpauth::extractors::basic::BasicAuth;
// use dotenv::dotenv;
// use std::env;
use urlencoding;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String, // id
    pub name: String,
    pub password: String,
}

impl User {
    pub fn create(user_name: &str, password: &str) -> Self {
        let filepath = urlencoding::encode(&user_name);
        Self {
            id: filepath.to_string(),
            name: user_name.to_string(),
            password: password.to_string(),
        }
    }

    /// check the user has the given name and password
    pub fn check(&self, user_name: &str, password: &str) -> bool {
        user_name == self.name && password == self.password
    }

    /// check the user has the given password
    pub fn validate(&self, password: &str) -> bool {
        password == self.password
    }
}
