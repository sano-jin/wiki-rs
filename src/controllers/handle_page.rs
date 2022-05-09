/// これは POST API を actix-web で扱うことが前提なコードになっているので，
/// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::authenticate::User;
use crate::gateways;
use crate::usecases::pages::Page;
// use actix_multipart::Multipart;
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
// use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Result;
// use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPageObj {
    path: String,
    body: String,
}

/// Create and Update the file with POST method
pub async fn post(auth: BasicAuth, item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;

    println!("post {:?}", item);

    // トップページか普通のページかで切り分け
    let default_page: String = if item.path == "top" {
        println!("is top page");
        gateways::pages::get_default_top_page()?
    } else {
        println!("is normal page");
        gateways::pages::get_default_page()?
    };

    let page = Page::create(&default_page, &item.path, &item.body)?;
    gateways::pages::save(&page)?;

    // TODO: navigate to the new page created
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    path: String,
}

/// Delete the file with DELETE method
pub async fn delete(auth: BasicAuth, item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;

    println!("delete ? {:?}", item);

    // delete the page
    gateways::pages::delete(&item.path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get_page(auth: BasicAuth, item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;
    println!("get_page ? {:?}", item);

    // Load the page
    let contents = gateways::pages::get_html(&item.path)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// GET the page for editing the page
pub async fn get_editor(
    auth: BasicAuth,
    item: web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;
    println!("get_edit_page ? {:?}", item);

    // get the editor html with the given file path
    let editor = gateways::pages::get_editor(&item.path)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(editor))
}
