use std::fs::File;
use std::io;
use std::io::prelude::*;

use actix_files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use json::JsonValue;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

/// simple handle
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("request: {:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
}

#[derive(Debug, Serialize, Deserialize)]
struct ReqObj {
    path: String,
}

/// This handler uses json extractor with limit
/// GET the page for editing the page
async fn get_edit_page(item: web::Json<ReqObj>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("get_edit_page");
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = Path::new("public").join(Path::new(&item.path));
    println!("path: {:?}", path);

    // TODO: use BufReader (low priority)
    let contents = std::fs::read_to_string(&path);

    let contents = match contents {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from(""),
            // if the file does not exists (that is, user is trying to create a new page),
            // return default string (currently empty string)
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    println!("contents: {}", contents);

    Ok(HttpResponse::Ok().json(contents)) // <- send json response
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}

/// This handler uses json extractor with limit
async fn post_edited(item: web::Json<NewPageObj>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("post_edited");
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = Path::new("public").join(Path::new(&item.path));
    println!("path: {:?}", path);

    // TODO: use BufReader
    let mut file = File::create(&path)?;
    file.write_all(item.body.as_bytes())
        .expect("cannot write to a file");

    // TODO: navigate to the new page created
    Ok(HttpResponse::Ok().json("created")) // <- send json response
}

/// This handler uses json extractor with limit
async fn delete_page(item: web::Json<ReqObj>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("delete_page");
    println!("request: {:?}", req);
    println!("model: {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = Path::new("public").join(Path::new(&item.path));
    println!("path: {:?}", path);

    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted")) // <- send json response
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    println!("Started http server: 127.0.0.1:8443");

    // load TLS keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // with path parameters
            // Viewing the pages
            .service(actix_files::Files::new("/pages", "public").show_files_listing())
            // Editing
            .service(
                web::resource("/edit")
                    // GET the page for editing
                    .route(web::get().to(get_edit_page))
                    // POST the new markdown
                    .route(web::post().to(post_edited))
                    // DELETE the page
                    .route(web::delete().to(delete_page)),
            )
            // register simple handler, handle all methods
            .service(web::resource("/index.html").to(index))
            // with path parameters
            .service(web::resource("/").route(web::get().to(|| async {
                HttpResponse::Found()
                    .append_header(("LOCATION", "/index.html"))
                    .finish()
            })))
    })
    .bind_openssl("127.0.0.1:8443", builder)?
    .run()
    .await
}
