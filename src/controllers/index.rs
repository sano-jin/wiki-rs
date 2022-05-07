use crate::gateways::pages;
use crate::usecases::pages::Page;
use actix_web::{Error, HttpRequest, HttpResponse};

/// simple handle
pub async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let pages_list = pages::list_pages().expect("file list");

    let contents = std::fs::read_to_string("public/index.html")?;
    let contents = Page::embed_pages_list(&contents, pages_list.as_slice()).expect("error");

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
