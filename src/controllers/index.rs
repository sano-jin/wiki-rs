use crate::controllers::authenticate::{authenticate, load};
use crate::gateways;
// use crate::usecases::pages::Page;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

/// simple handle
pub async fn index(auth: BasicAuth, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    authenticate(load(), auth)?;

    // Load the page
    let contents = gateways::pages::get_html("top")?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
