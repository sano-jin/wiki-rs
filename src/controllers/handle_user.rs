/// これは POST API を actix-web で扱うことが前提なコードになっているので，
// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::appstate::AppState;
use crate::controllers::authenticate::authenticate;
use crate::gateways;
use crate::gateways::db::Database;
use crate::usecases::users::User;
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserObj {
    name: String,
    password: String,
}

/// Create and Update the user with POST method
pub async fn post<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Json<NewUserObj>,
) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);
    authenticate(&data.db, auth)?;

    let user = User::create(&item.name, &item.password);
    gateways::users::save(&data.db, &user)?;

    // TODO: navigate to the new user created
    let url = "/users";
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    name: String,
}

/// Delete the file with DELETE method
pub async fn delete<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);
    authenticate(&data.db, auth)?;

    // delete the user
    gateways::users::delete(&data.db, &item.name)?;

    // TODO: navigate to the root user
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the user
pub async fn get_users<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
) -> Result<HttpResponse, Error> {
    println!("get_users");
    authenticate(&data.db, auth)?;

    let contents = gateways::users::get_editor(&data.db)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}
