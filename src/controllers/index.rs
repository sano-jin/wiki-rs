use crate::controllers::appstate::AppState;
use crate::controllers::validate;
use crate::gateways;
use crate::gateways::db::Database;
use actix_web::{web, Error, HttpRequest, HttpResponse};

/// simple handle
pub async fn index<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let user = match validate::get_user(&data, req).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish());
        }
    };
    println!("user {:?}", user);

    // Load the page
    let contents = gateways::pages::get_html(&data.db, "top", &user)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
