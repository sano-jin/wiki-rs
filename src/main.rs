use std::io;

use actix_files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use urlencoding;
// Newly added pulldown_cmark
use pulldown_cmark::{html, Options, Parser};

/// Get the new path <root_dir>/<encoded filename>
fn get_path(root_dir: &str, filename: &str) -> PathBuf {
    let encoded = urlencoding::encode(&filename); // encode the filename
    Path::new(&root_dir).join(Path::new(&encoded.into_owned()))
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}

/// Create and Update the file with POST method
async fn post(item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    println!("post {:?}", item);

    // Update the file with the given contents
    let path: PathBuf = get_path("public/edit", &item.path);
    println!("writing to the file {:?}", path);
    let mut file = File::create(&path)?;
    file.write_all(item.body.as_bytes())?;

    // Set parser options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    // Parse the given markdown with the pulldown_cmark parser
    println!("parsing the given markdown with the pulldown_cmark parser");
    let parser = Parser::new(&item.body);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed: {}", html_buf);

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/page.html")?;
    // Replace the title, path, contents
    let page = default_page
        .replace("TITLE", &title.into_owned())
        .replace("PATH", &item.path)
        .replace("BODY", &html_buf);

    // Update the file with the given contents
    let path: PathBuf = get_path("public/pages", &item.path);
    println!("writing to the file {:?}", path);
    let mut file = File::create(&path)?;
    file.write_all(page.as_bytes())?;

    // TODO: navigate to the new page created
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryPath {
    path: String,
}

/// Delete the file with DELETE method
async fn delete(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete ? {:?}", item);

    // delete the markdown file
    let path: PathBuf = get_path("public/edit", &item.path);
    std::fs::remove_file(&path)?;

    // delete the html file
    let path: PathBuf = get_path("public/pages", &item.path);
    std::fs::remove_file(&path)?;

    // TODO: navigate to the root page
    Ok(HttpResponse::Ok().json("deleted"))
}

/// GET the page
async fn get_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_page ? {:?}", item);

    // Load the file
    let path = get_path("public/pages", &item.path);
    let contents = std::fs::read_to_string(&path)?;

    // Return the response and display the html file on the browser
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// Read a file
/// If the file doesn not exists, then return the default string
fn read_with_default(path: &str, default: &str) -> String {
    let contents = std::fs::read_to_string(&path);
    match contents {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from(default),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    }
}

/// This handler uses json extractor with limit
/// GET the page for editing the page
async fn get_editor(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_edit_page ? {:?}", item);
    let path: PathBuf = get_path("public/edit", &item.path);
    let contents = read_with_default(&path.to_string_lossy(), "");

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

/// simple handle
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!"))
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
            // **Newly added here**
            .service(
                web::resource("/pages").route(web::get().to(get_page)), // GET the page
            )
            .service(
                web::resource("/edit")
                    .route(web::get().to(get_editor)) // GET the editor
                    .route(web::post().to(post)) // POST the new contents to update the file
                    .route(web::delete().to(delete)), // Delete the file
            )
            // GET /files/**/*.html and return the file /public/**/*.html
            .service(actix_files::Files::new("/assets", "public/assets").show_files_listing())
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
