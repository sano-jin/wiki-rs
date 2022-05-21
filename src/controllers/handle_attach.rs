use crate::controllers::appstate::AppState;
/// これは POST API を actix-web で扱うことが前提なコードになっているので，
/// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::validate;
use crate::gateways;
use crate::gateways::db::Database;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    path: String,
}

pub async fn post<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    req: HttpRequest,
    item: web::Query<QueryPath>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);
    let user = match validate::get_user(&data, req).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish());
        }
    };
    println!("user {:?}", user);

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
pub async fn delete<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    req: HttpRequest,
    item: web::Query<AttachQueryPath>,
) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);
    let user = match validate::get_user(&data, req).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish());
        }
    };
    println!("user {:?}", user);

    // delete the page
    // let path = format!("{}/{}", item.path, item.file);
    // println!("delete_attach at {:?}", path);

    gateways::attaches::delete(&item.path, &item.file)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    req: HttpRequest,
    item: web::Query<AttachQueryPath>,
) -> Result<HttpResponse, Error> {
    println!("get_attach ? {:?}", item);
    let user = match validate::get_user(&data, req).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish());
        }
    };
    println!("user {:?}", user);

    let path = format!("{}/{}", item.path, item.file);
    println!("get_attach at {:?}", path);

    // Load the file
    let contents = gateways::attaches::get(&path)?;

    // TODO: file type を取得できるようにする
    let content_type = "image/png";

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type(content_type).body(contents))
}
