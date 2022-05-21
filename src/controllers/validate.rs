// Validate JWT

use actix_web::{web, Error, HttpResponse};
// use chrono::Duration;
// use jsonwebtoken::{decode, encode, EncodingKey, Validation};
// use serde::{Deserialize, Serialize};
use uuid::Uuid;

//

use crate::controllers::appstate::AppState;
use crate::controllers::login::{Claims, JWT_COOKIE_KEY, JWT_SECRET};
// use crate::controllers::authenticate::authenticate;
use crate::gateways;
use crate::gateways::db::Database;
use crate::usecases::users::User;
// use actix_web_httpauth::extractors::basic::BasicAuth;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

/// Finds user by UID.
// #[get("/user/{user_id}")]
pub async fn get_user<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    // form: web::Json<FormUser>,
    // pool: web::Data<DbPool>,
    req: actix_web::HttpRequest,
    // user_uid: web::Path<Uuid>,
    // item: web::Query<QueryPath>,
    // user_uid: web::Query<Uuid>,
    // user_uid: web::Query<Uuid>,
    user_uid: web::Query<UserId>,
) -> Result<HttpResponse, Error> {
    println!(">>>> validate::get_user");

    // let user_uid = user_uid.into_inner();
    let user_name = &user_uid.name;
    let user_uid = &user_uid.id;

    let db = &data.db;
    // let conn = pool.get().expect("couldn't get db connection from pool");

    let jwt = get_cookie_map(req)
        .get(JWT_COOKIE_KEY)
        .cloned()
        .unwrap_or_default();

    match dec_jwt(&JWT_SECRET.to_string(), &jwt) {
        Some(_) => {
            // let user_opt = db.find_user_by_id(user_uid, &conn);
            let logged_in_users = data.logged_in_users.lock().unwrap(); // <- get counter's MutexGuard

            println!("logged_in_users: {:?}", logged_in_users);

            let last_login_date_opt = logged_in_users.get(user_uid); // <- access logged_in_users inside MutexGuard
            let is_valid_time = if let Some(last_login_date) = last_login_date_opt {
                println!("last login date: {:?}", last_login_date);
                let duration = Duration::hours(24); // 24 hours
                let current: DateTime<Utc> = Utc::now();
                current - last_login_date.to_owned() < duration
            } else {
                false
            };
            if !is_valid_time {
                return Ok(HttpResponse::NonAuthoritativeInformation()
                    .content_type("text/plain; charset=utf-8")
                    .body("invalid time")); // too late
            }

            let user_opt: Option<User> = {
                println!("username: {}", user_name);
                if let Ok(user) = gateways::users::get_user_by_name(db, user_name) {
                    Some(user)
                } else {
                    None
                }
            };

            // .map_err(|e| HttpResponse::InternalServerError().finish())?;

            match user_opt {
                Some(user) => Ok(HttpResponse::Ok().json(user)),
                _ => Ok(HttpResponse::NonAuthoritativeInformation()
                    .content_type("text/plain; charset=utf-8")
                    .body("user not found.")),
            }
        }
        _ => Ok(HttpResponse::NonAuthoritativeInformation()
            .content_type("text/plain; charset=utf-8")
            .body("invalid token.")),
    }
}

fn get_cookie_map(req: actix_web::HttpRequest) -> HashMap<String, String> {
    match get_cookie_string(req) {
        Some(cookie_str) => {
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
