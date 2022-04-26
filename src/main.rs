use std::fs::File;
use std::io;
use std::io::prelude::*;

use actix_files;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};
extern crate pulldown_cmark;
use pulldown_cmark::{html, Options, Parser};

use urlencoding;

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

/// Get the page at the fiven path
/// Add recent updated file names list
fn get_at(path: PathBuf) -> Result<HttpResponse, Error> {
    println!("path: {:?}", path);

    // Load the file
    let contents = std::fs::read_to_string(&path)?;

    // Add recent updated file names list
    let index_ul = read_with_default("public/index_ul", "");

    println!("index_ul: {}", index_ul);
    let contents = contents.replace("INDEX_UL", &index_ul);

    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

/// The top page
async fn index() -> Result<HttpResponse, Error> {
    get_at(PathBuf::from("public/index.html"))
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryPath {
    path: String,
}

/// Get the new path <root>/<encoded path>
fn get_path(root: &str, path: &str) -> PathBuf {
    let encoded = urlencoding::encode(&path);
    Path::new(&root).join(Path::new(&encoded.into_owned()))
}

/// GET the page
async fn get_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_page ? {:?}", item);
    get_at(get_path("public/pages", &item.path))
}

/// This handler uses json extractor with limit
/// GET the page for editing the page
async fn get_edit_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("get_edit_page ? {:?}", item);

    // TODO: check the path is valid
    let path: PathBuf = get_path("public/edit", &item.path);
    println!("path: {:?}", path);

    // TODO: use BufReader (low priority)
    // if the file does not exists (that is, user is trying to create a new page),
    // return the default string (currently empty string)
    let contents = read_with_default(&path.to_string_lossy(), "");

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the file for editing
    let edit_page = std::fs::read_to_string("static/edit.html")?;
    let edit_page = edit_page
        .replace("TITLE", &title.into_owned())
        .replace("PATH", &item.path)
        .replace("MARKDOWN", &contents);

    Ok(HttpResponse::Ok().content_type("text/html").body(edit_page))
}

/// Update the file for the html list of the file names
fn update_index_ul(index: Vec<&str>) -> Result<(), Error> {
    // generate the html list of the files
    let index_ul: Vec<String> = index
        .into_iter()
        .map(|filename| {
            format!(
                "<li><a href=\"/pages?path={}\">{}</a></li>",
                urlencoding::encode(&filename),
                filename
            )
        })
        .collect();
    let index_ul_str = index_ul.join("\n");

    let mut file = File::create("public/index_ul")?;
    file.write_all(&index_ul_str.as_bytes())?;
    Ok(())
}

/// write `contents` to the file `root_dir/path`
fn update_file(root_dir: &str, filename: &str, contents: &str) -> Result<(), Error> {
    let path: PathBuf = get_path(&root_dir, &filename);
    println!("updating the file at {:?}", path);

    // Write to a file with the given contents
    let mut file = File::create(&path)?;
    file.write_all(&contents.as_bytes())?;

    // Update index
    let index_file = read_with_default("public/index", "");
    let mut index: Vec<&str> = index_file.lines().collect();

    index.retain(|value| *value != filename); // remove if the filename already exists on the index
    index.insert(0, &filename); // push front the filename as the name of the most recent updated file
    let index_str = index.join("\n");
    println!("new index: {:?}", index);

    // update the index file
    let mut file = File::create("public/index")?;
    file.write_all(&index_str.as_bytes())?;

    // generate the html list of the files
    update_index_ul(index)?;

    return Ok(());
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPageObj {
    path: String,
    body: String,
}

/// This handler uses json extractor with limit
/// Post the edited file to update the page
async fn post_edited(item: web::Json<NewPageObj>) -> Result<HttpResponse, Error> {
    println!("post_edited ? {:?}", item);

    // Update the markdown file with the given contents
    update_file("public/edit", &item.path, &item.body)?;

    // Set parser options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    // Parse the given markdown with the pulldown_cmark parser
    let parser = Parser::new_ext(&item.body, options);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed html: {:?}", html_buf);

    // decode the path to obtain the title
    let title = urlencoding::decode(&item.path).expect("cannot decode");

    // Open the default file
    let default_page = std::fs::read_to_string("static/page.html")?;
    let default_page = default_page
        .replace("TITLE", &title.into_owned())
        .replace("PATH", &item.path)
        .replace("BODY", &html_buf);

    update_file("public/pages", &item.path, &default_page)?;

    // TODO: navigate to the new page created
    let url = format!("/pages?path={}", &item.path);
    Ok(HttpResponse::Ok().json(url))
}

/// This handler uses json extractor with limit
async fn delete_page(item: web::Query<QueryPath>) -> Result<HttpResponse, Error> {
    println!("delete_page ? {:?}", item);

    // Remove the markdown file
    std::fs::remove_file(get_path("public/edit", &item.path))?;

    // Remove the html file
    std::fs::remove_file(get_path("public/pages", &item.path))?;

    // Update index
    let index_file = read_with_default("public/index", "");
    let mut index: Vec<&str> = index_file.lines().collect();

    index.retain(|value| *value != &item.path); // remove if the filename already exists on the index
    let index_str = index.join("\n");
    println!("new index: {:?}", index);

    // update the index file
    let mut file = File::create("public/index")?;
    file.write_all(&index_str.as_bytes())?;

    // generate the html list of the files
    update_index_ul(index)?;

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
