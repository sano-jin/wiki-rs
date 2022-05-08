use crate::controllers::authenticate::User;
/// これは POST API を actix-web で扱うことが前提なコードになっているので，
/// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::gateways;
use crate::usecases::pages::Page;
use actix_multipart::Multipart;
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web::{http::header, web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures::{StreamExt, TryStreamExt};
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

    let default_page: String = gateways::pages::get_default_page()?;
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

/// API for attached files
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAttachObj {
    path: String, // path to the page
    file: String, // name of the file
    body: Vec<u8>,
}

/// Create and Update the file with POST method
pub async fn post_attach(
    auth: BasicAuth,
    item: web::Json<NewAttachObj>,
) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;

    println!("post {:?}", item);

    let path = format!("{}/{}", item.path, item.file);
    println!("delete_attach at {:?}", path);

    gateways::pages::save_attach(&path, &item.body)?;

    Ok(HttpResponse::Ok().json(path))
}

pub async fn post_attach2(
    auth: BasicAuth,
    item: web::Query<QueryPath>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;

    println!("post {:?}", item);

    while let Ok(Some(mut field)) = payload.try_next().await {
        // let content_type = field.content_disposition().unwrap();
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();

        // let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let path = format!("{}/{}", item.path, filename);

        // body
        let mut bytes_array = Vec::new();

        // バイナリをチャンクに分けてwhileループ
        while let Some(chunk) = field.next().await {
            // let data = chunk;
            let data = chunk.unwrap();

            // ファイルへの書き込み
            bytes_array.append(&mut data.to_vec());
        }

        gateways::pages::save_attach(&path, &bytes_array)?;
    }

    let request_uri = format!("/pages?path={}", &item.path);

    Ok(HttpResponse::Found()
        .append_header(("Location", request_uri))
        .finish())

    //     Ok(HttpResponse::TemporaryRedirect()
    //         .append_header((header::LOCATION, request_uri))
    //         .finish())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachQueryPath {
    path: String,
    file: String,
}

/// Delete the file with DELETE method
pub async fn delete_attach(
    auth: BasicAuth,
    item: web::Query<AttachQueryPath>,
) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;

    println!("delete ? {:?}", item);

    // delete the page
    let path = format!("{}/{}", item.path, item.file);
    println!("delete_attach at {:?}", path);

    gateways::pages::delete_attach(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get_attach(
    auth: BasicAuth,
    item: web::Query<AttachQueryPath>,
) -> Result<HttpResponse, Error> {
    User::load().authenticate(auth)?;
    println!("get_attach ? {:?}", item);

    let path = format!("{}/{}", item.path, item.file);
    println!("get_attach at {:?}", path);

    // Load the file
    let contents = gateways::pages::get_attach(&path)?;

    // TODO: file type を取得できるようにする
    let content_type = "image/png";

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type(content_type).body(contents))
}
