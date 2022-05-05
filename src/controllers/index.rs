use crate::models::pages::Page;
use actix_web::{Error, HttpRequest, HttpResponse};

/// simple handle
pub async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let contents = std::fs::read_to_string("public/index.html")?;
    let contents = Page::embed_pages_list(&contents).expect("error");
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
