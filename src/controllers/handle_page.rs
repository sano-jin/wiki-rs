/// これは POST API を actix-web で扱うことが前提なコードになっているので，
/// 本来は，controller ではなく，もうひとつ上のレイヤ（framework）に来る気はしている．
/// gateways (DB) に依存しているのもよくない気がする．
use crate::controllers::authenticate::authenticate;
use crate::gateways;
use crate::gateways::db::Database;
use crate::usecases::pages::Page;
use actix_web::{web, Error, HttpResponse};
use actix_web_httpauth::extractors::basic::BasicAuth;
use serde::{Deserialize, Serialize};

// This struct represents state
#[derive(Clone)]
pub struct AppState<T: Clone + Database> {
    pub db: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPageObj {
    path: String,
    body: String,
}

/// Create and Update the file with POST method
pub async fn post<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Json<NewPageObj>,
) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);
    authenticate(auth)?;

    let path = urlencoding::encode(&item.path);

    // トップページか普通のページかで切り分け
    let default_page: String = if item.path == "top" {
        println!("is top page");
        gateways::pages::get_default_top_page()?
    } else if item.path == "menu" {
        println!("is menu page");
        gateways::pages::get_default_menu_page()?
    } else {
        println!("is normal page");
        gateways::pages::get_default_page()?
    };

    let page = Page::create(&default_page, &path, &item.body)?;
    gateways::pages::save(&data.db, &page)?;

    // TODO: navigate to the new page created
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    path: String,
}

/// Delete the file with DELETE method
pub async fn delete<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);
    authenticate(auth)?;

    // delete the page
    gateways::pages::delete(&data.db, &item.path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get_page<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    println!("get_page ? {:?}", item);
    authenticate(auth)?;

    // Load the page
    let contents = gateways::pages::get_html(&data.db, &item.path)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// GET the page for editing the page
pub async fn get_editor<T: Clone + Database>(
    data: web::Data<AppState<T>>,
    auth: BasicAuth,
    item: web::Query<QueryPath>,
) -> Result<HttpResponse, Error> {
    println!("get_edit_page ? {:?}", item);
    authenticate(auth)?;

    // get the editor html with the given file path
    let editor = gateways::pages::get_editor(&data.db, &item.path)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(editor))
}
