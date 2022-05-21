use crate::controllers::appstate::AppState;
use crate::gateways;
use crate::gateways::db::Database;
use crate::usecases::users::User;
use actix_web::cookie::SameSite;
use actix_web::{web, Error, HttpResponse};
use chrono::Duration;
// use chrono::{DateTime, Utc};
use jsonwebtoken::{encode, EncodingKey};
use serde::{Deserialize, Serialize};
use std::env;

/// TODO: secret key は env からとってくるようにする．
pub const JWT_SECRET: &str = "secret";
pub const JWT_COOKIE_KEY: &str = "jwt";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub uuid: String,
}

/// ユーザの情報の入力フォーム
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserForm {
    pub name: String,
    pub password: String,
}

fn enc_jwt(secret: &String, user: &User) -> String {
    let mut header = jsonwebtoken::Header::default();
    header.typ = Some(String::from("JWT"));
    header.alg = jsonwebtoken::Algorithm::HS256;
    let claim = Claims {
        exp: (chrono::Utc::now() + Duration::hours(8)).timestamp(),
        uuid: user.id.to_string(),
    };
    encode(&header, &claim, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn set_jwt(user: &User, location: &str) -> HttpResponse {
    // Set JWT
    let cookie =
        actix_web::cookie::Cookie::build(JWT_COOKIE_KEY, enc_jwt(&JWT_SECRET.to_string(), &user))
            .http_only(true)
            .secure(is_https())
            .same_site(SameSite::Strict)
            .finish();

    println!(">> setting cookie {:?}", cookie.to_string());

    let ret = HttpResponse::Found()
        .append_header(("Location", "/login"))
        .append_header(("Set-Cookie", cookie.to_string()))
        .finish();

    ret
}

/// Finds user by username and password.
// #[post("/login")]
pub async fn login<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    form: web::Json<UserForm>,
) -> Result<HttpResponse, Error> {
    // let conn = pool.get().expect("couldn't get db connection from pool");
    let db = &data.db;
    let user_name = &form.name;
    let user_password = &form.password;

    let user_opt: Option<User> = {
        if let Ok(user) = gateways::users::get_user_by_name(db, &user_name) {
            if user.validate(&user_password) {
                println!("authentication succeeded");
                Some(user)
            } else {
                None
            }
        } else {
            None
        }
    };

    match user_opt {
        Some(user) => {
            // Get and set uuid

            // Set JWT
            Ok(set_jwt(&user, "/"))
        }
        _ => Ok(HttpResponse::NonAuthoritativeInformation()
            .content_type("text/plain; charset=utf-8")
            .body("login faied.")),
    }

    // web::block(move || actions::find_user_by_name(&form.name, &form.password, &conn))
    //     .await
    //     .map(|user_opt| match user_opt {
    //         Some(user) => {
    //             let cookie = actix_web::cookie::Cookie::build(
    //                 JWT_COOKIE_KEY,
    //                 enc_jwt(&JWT_SECRET.to_string(), &user),
    //             )
    //             .http_only(true)
    //             .finish();

    //             let ret = Ok(HttpResponse::Ok()
    //                 .header("Set-Cookie", cookie.to_string())
    //                 .content_type("text/plain; charset=utf-8")
    //                 .body("login success."));
    //             ret
    //         }
    //         _ => Ok(HttpResponse::NonAuthoritativeInformation()
    //             .content_type("text/plain; charset=utf-8")
    //             .body("login faied.")),
    //     })
    //     .map_err(|e| {
    //         eprintln!("{}", e);
    //         HttpResponse::InternalServerError().finish()
    //     })?
}

/// simple handle
pub async fn index<T: Clone + Database>(// data: web::Data<AppState<T>>,
) -> Result<HttpResponse, Error> {
    // Load the page
    let contents = gateways::pages::get_login_page()?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// 特定のキーで環境変数から値を取得するための関数
///
/// # Arguments
/// * `key` - 環境変数から取り出したい値のキー
///
/// # Return value
/// * String - 環境変数の値を文字列として取得する
///
fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(value) => return value,
        Err(e) => println!("ENV: ERR {:?}", e),
    }
    return String::new();
}

/// 環境変数に設定された HTTPS の値が 1 か判定する
/// Cookie の属性に Secure を付与するか判定するのに使用する
///
/// # Return value
/// * bool - Secure 属性を付与するか判定するための真偽値
///
fn is_https() -> bool {
    return get_env("HTTPS") == "1";
}
