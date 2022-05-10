/// これは POST API を actix-web で扱うことが前提なコードになっているので，
// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::authenticate::authenticate;
use crate::gateways;
use crate::usecases::users::User;
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Deserialize, Serialize};
// use actix_multipart::Multipart;
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
// use futures::{StreamExt, TryStreamExt};
// use std::fs::File;
// use std::io::Result;
// use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserObj {
    name: String,
    password: String,
}

/// Create and Update the user with POST method
pub async fn post(auth: BasicAuth, item: web::Json<NewUserObj>) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);
    authenticate(auth)?;

    let user = User::create(&item.name, &item.password);
    gateways::users::save(&user)?;

    // TODO: navigate to the new user created
    let url = "/users";
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    name: String,
}

/// Delete the file with DELETE method
pub async fn delete(auth: BasicAuth, item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);
    authenticate(auth)?;

    // delete the user
    gateways::users::delete(&item.name)?;

    // TODO: navigate to the root user
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the user
pub async fn get_users(auth: BasicAuth) -> Result<HttpResponse, Error> {
    println!("get_users");
    authenticate(auth)?;

    let contents = gateways::users::get_editor()?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

// /// GET the user for editing the user
// pub async fn get_editor(
//     auth: BasicAuth,
//     item: web::Query<QueryPath>,
// ) -> Result<HttpResponse, Error> {
//     println!("get_edit_user ? {:?}", item);
//     authenticate(auth)?;
//
//     // get the editor html with the given file path
//     let editor = gateways::users::get_editor(&item.path)?;
//
//     Ok(HttpResponse::Ok().content_type("text/html").body(editor))
// }
