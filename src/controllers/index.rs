use crate::controllers::appstate::AppState;
use crate::controllers::authenticate::authenticate;
use crate::gateways;
use crate::gateways::db::Database;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;

/// simple handle
pub async fn index<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    authenticate(&data.db, auth)?;

    // Load the page
    let contents = gateways::pages::get_html(&data.db, "top")?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
