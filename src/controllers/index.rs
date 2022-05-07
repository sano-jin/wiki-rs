use crate::controllers::authenticate::User;
use crate::gateways::pages;
use crate::usecases::pages::Page;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

/// simple handle
pub async fn index(auth: BasicAuth, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    println!("auth: {:?}", auth);

    // let user_id = auth.user_id().to_string();
    // let password = auth.password().expect("password").to_string();
    // println!("user id: {}, password: {}", user_id, password);
    // if user_id != "uedalab" || password != "password" {
    //     println!("authentication failed");
    //     return Err(error::ErrorUnauthorized("authentication failed"));
    // }

    User::load().authenticate(auth)?;

    let pages_list = pages::list_pages().expect("file list");

    let contents = std::fs::read_to_string("public/index.html")?;
    let contents = Page::embed_pages_list(&contents, pages_list.as_slice()).expect("error");

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
