// Validate JWT

use crate::controllers::appstate::AppState;
use crate::controllers::login::{Claims, JWT_COOKIE_KEY, JWT_SECRET};
use crate::gateways;
use crate::gateways::db::Database;
use crate::usecases::users::User;
use actix_web::{error, web, Error};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

//
fn dec_jwt(secret: &String, jwt: &String) -> Option<String> {
    let validation = jsonwebtoken::Validation::default();
    match jsonwebtoken::decode::<Claims>(
        &jwt,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(c) => Option::Some(c.claims.uuid),
        _ => Option::None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId {
    pub name: String,
    pub id: Uuid,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

/// Finds user by UID.
// #[get("/user/{user_id}")]
pub async fn get_user<T: Clone + Database>(
    data: &web::Data<AppState<T>>,
    req: actix_web::HttpRequest,
) -> Result<User, Error> {
    println!(">>>> validate::get_user");

    let db = &data.db;
    let jwt = get_cookie_map(req)
        .get(JWT_COOKIE_KEY)
        .cloned()
        .unwrap_or_default();

    match dec_jwt(&JWT_SECRET.to_string(), &jwt) {
        Some(uid) => {
            println!(">>>> user id: {:?}", uid);
            let user_result = gateways::users::get_user_by_id(db, &uid);
            user_result
        }
        _ => Err(Error::from(MyError {
            name: "invalid token",
        })),
    }
}

fn get_cookie_map(req: actix_web::HttpRequest) -> HashMap<String, String> {
    match get_cookie_string(req) {
        Some(cookie_str) => {
            println!(">>>> cookie: {:?}", cookie_str);
            let cookies: Vec<&str> = cookie_str.split(";").collect();
            cookies
                .iter()
                .fold(HashMap::<String, String>::new(), |mut acc, cur| {
                    let entry: Vec<&str> = cur.split("=").collect();
                    acc.insert(entry[0].to_string(), entry[1].to_string());
                    acc
                })
        }
        None => HashMap::new(),
    }
}

fn get_cookie_string(req: actix_web::HttpRequest) -> Option<String> {
    let cookie_header = req.headers().get("cookie");
    if let Some(v) = cookie_header {
        let cookie_string = v.to_str().unwrap();
        return Some(String::from(cookie_string));
    }
    return None;
}
