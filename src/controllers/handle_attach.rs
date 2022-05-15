/// これは POST API を actix-web で扱うことが前提なコードになっているので，
/// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::authenticate::authenticate;
use crate::gateways;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
// use crate::usecases::pages::Page;
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
// use std::fs::File;
// use std::io::Result;
// use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    path: String,
}

pub async fn post_attach(
    auth: BasicAuth,
    item: web::Query<QueryPath>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    authenticate(auth)?;

    println!("post {:?}", item);

    while let Ok(Some(mut field)) = payload.try_next().await {
        // let content_type = field.content_disposition().unwrap();
        let content_type = field.content_disposition();
        let filename = content_type.get_filename().unwrap();

        // let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let path = format!("{}/{}", item.path, filename);
        println!(">>>> saving attached file to path {:?}", path);

        // body
        let mut bytes_array = Vec::new();

        // バイナリをチャンクに分けてwhileループ
        // TODO: わざわざチャンクに分ける必要はあるのか
        // and 分けるなら buffering するべき
        while let Some(chunk) = field.next().await {
            // let data = chunk;
            let data = chunk.unwrap();

            // ファイルへの書き込み
            bytes_array.append(&mut data.to_vec());
        }

        gateways::attaches::save(&path, &bytes_array)?;
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
    println!("delete ? {:?}", item);
    authenticate(auth)?;

    // delete the page
    // let path = format!("{}/{}", item.path, item.file);
    // println!("delete_attach at {:?}", path);

    gateways::attaches::delete(&item.path, &item.file)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get_attach(
    auth: BasicAuth,
    item: web::Query<AttachQueryPath>,
) -> Result<HttpResponse, Error> {
    println!("get_attach ? {:?}", item);
    authenticate(auth)?;

    let path = format!("{}/{}", item.path, item.file);
    println!("get_attach at {:?}", path);

    // Load the file
    let contents = gateways::attaches::get(&path)?;

    // TODO: file type を取得できるようにする
    let content_type = "image/png";

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type(content_type).body(contents))
}
