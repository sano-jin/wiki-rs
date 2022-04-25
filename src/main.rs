use std::fs::File;
use std::io;
use std::io::prelude::*;

use actix_files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use json::JsonValue;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
extern crate pulldown_cmark;
use pulldown_cmark::{html, Parser};

use urlencoding::encode;

/// simple handle
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("request: {:?}", req);

    // Open the default file
    let default_page =
        std::fs::read_to_string("public/index.html").expect("cannot open the index.html file");

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(default_page))
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryPath {
    path: String,
}

/// Get the new path <root>/<encoded path>
fn get_path(root: &str, path: &str) -> PathBuf {
    let encoded = encode(&path);
    println!("encoded : {}", encoded);

    Path::new(&root).join(Path::new(&encoded.into_owned()))
}

/// GET the page
async fn get_page(item: web::Query<QueryPath>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("get_page");
    println!("request: {:?}", req);
    println!("item: {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = get_path("public/pages", &item.path);
    println!("path: {:?}", path);

    // TODO: use BufReader (low priority)
    let contents = std::fs::read_to_string(&path)?;

    println!("contents: {}", contents);

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// This handler uses json extractor with limit
/// GET the page for editing the page
async fn get_edit_page(
    item: web::Query<QueryPath>,
    req: HttpRequest,
) -> Result<HttpResponse, Error> {
    println!("get_edit_page");
    println!("request: {:?}", req);
    println!("item: {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = get_path("public/edit", &item.path);
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

    // Open the file for editing

    let edit_page = std::fs::read_to_string("static/edit.html").expect("cannot open edit file");
    let edit_page = edit_page
        .replace("TITLE", &item.path)
        .replace("MARKDOWN", &contents);

    Ok(HttpResponse::Ok().content_type("text/html").body(edit_page))
}

/// write `contents` to the file `root_dir/path`
fn update_file(root_dir: &str, path: &str, contents: &str) -> Result<(), Error> {
    // TODO: check the path is valid
    let path: PathBuf = get_path(&root_dir, &path);
    println!("path: {:?}", path);

    // TODO: use BufReader
    println!("updating the file");

    // Write to a file
    let mut file = File::create(&path)?;
    file.write_all(&contents.as_bytes())
        .expect("cannot write to a file");

    return Ok(());
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
    println!("item: {:?}", item);

    update_file("public/edit", &item.path, &item.body)?;

    // Parse the given markdown with the pulldown_cmark parser
    println!("parsing the given markdown with the pulldown_cmark parser");
    let parser = Parser::new(&item.body);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed: {}", html_buf);

    // Open the default file
    let default_page =
        std::fs::read_to_string("static/page.html").expect("cannot open the default file");
    let default_page = default_page
        .replace("TITLE", &item.path)
        .replace("BODY", &html_buf);

    update_file("public/pages", &item.path, &default_page)?;

    println!("updated the page");

    // TODO: navigate to the new page created
    let request_uri = format!("https://127.0.0.1:8443/pages?path={}", &item.path);
    println!("Redirecting to {}", request_uri);

    // let redirecting_html = format!(
    //     "<head><meta http-equiv=\"Refresh\" content=\"0; {}\"></head>",
    //     request_uri
    // );

    Ok(HttpResponse::Ok().json(request_uri))
}

/// This handler uses json extractor with limit
async fn delete_page(item: web::Query<QueryPath>, req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("delete_page");
    println!("request: {:?}", req);
    println!("item: {:?}", item);

    // TODO: check the validity of the path

    // Remove the markdown file
    std::fs::remove_file(get_path("public/edit", &item.path))?;

    // Remove the html file
    std::fs::remove_file(get_path("public/pages", &item.path))?;

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
            .service(actix_files::Files::new("/assets", "public/assets").show_files_listing())
            // Editing
            .service(web::resource("/pages").route(web::get().to(get_page)))
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
