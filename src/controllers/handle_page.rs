use crate::models::pages::Page;
// use std::io;

use crate::util;
// use actix_files;
// use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::{web, Error, HttpResponse};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::{Path, PathBuf};
use urlencoding;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPageObj {
    path: String,
    body: String,
}

/// Create and Update the file with POST method
pub async fn post(item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);

    let page = Page::create(&item.path, &item.body)?;
    Page::save(&page)?;

    // TODO: navigate to the new page created
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    path: String,
}

/// Delete the file with DELETE method
pub async fn delete(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);

    // delete the markdown file
    let path = util::get_path("public/edit", &item.path);
    std::fs::remove_file(&path)?;

    // delete the html file
    let path = util::get_path("public/pages", &item.path);
    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
pub async fn get_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_page ? {:?}", item);

    // Load the file
    let path = util::get_path("public/pages", &item.path);
    let contents = std::fs::read_to_string(&path)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// This handler uses json extractor with limit
/// GET the page for editing the page
pub async fn get_editor(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_edit_page ? {:?}", item);
    let path = util::get_path("public/edit", &item.path);
    let contents = util::read_with_default(&path.to_string_lossy(), "");

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the file for editing
    let editor = std::fs::read_to_string("public/layouts/edit.html")?;
    // Replace the contents
    let editor = editor
        .replace("TITLE", &title.into_owned())
        .replace("MARKDOWN", &contents);

    Ok(HttpResponse::Ok().content_type("text/html").body(editor))
}
