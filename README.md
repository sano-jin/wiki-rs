wiki.rs

A simple wiki created with Rust from scratch.

![Demo](/docs/wiki-rs-demo.png)

# API design

## Front

- 普通にアクセスして見る．
- 今見ているページの markdown を編集して，それでページを更新する．
  - edit button
- 新しいページの markdown を編集して，それでページを更新する．
  - create button

## Backend API

- GET `/page/xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む
- GET `/edit?path=<Path to the page">`
  - 編集用の markdown を返す
  - サーバ上のファイルから読み込む
- POST /edit
  - body: `{path:"Path to the page", body: "The updated markdown"}`
  - markdown を投げ，それで /xxxxxx.html を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- DELETE `/edit?path=<Path to the page>`
  - /xxxxxx.html を消去する
  - サーバ上のファイルは消去する

## 構成

# How to create a own wiki from scrarch

## Prerequisties

Install Cargo

## Implement Https server

In this section, we implement a simple https server that returns a constant string message.

TODO: Modify this to firstly implement a http server
(not implementing https from the start)

### Update rust

```sh
rustup update
```

### Create a new project

```sh
cargo new wiki-rs
cd wiki.rs
```

### Enable CA

See <https://github.com/actix/examples/tree/master/https-tls/openssl> and
follow the instructions on README.md to enable CA

1. use local CA

   ```sh
   mkcert -install
   ```

2. generate own cert/private key

   ```sh
   mkcert -install
   ```

   rename the `127.0.0.1-key.pem` file with `key.pem` and
   the `127.0.0.1.pem` file with `cert.pem`.

### Add dependency

See <https://github.com/actix/examples/tree/master/https-tls/openssl>
and add dependency

`Cargo.toml`

```toml
# Cargo.toml
[package]
name = "wiki-rs"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = { version = "4", features = ["openssl"]  } # Use actix-web to implement a backend server
env_logger = "0.9" # for logging
openssl = "0.10" # for TLS
```

### Implement with actix-web

Implement `src/main.rs`

```rust
// src/main.rs

use std::io;

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

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
```

### Run the backend and access

```sh
cargo run
```

and

```sh
curl https://127.0.0.1:8443/index.html
```

with the other terminal.

or access <https://127.0.0.1:8443/index.html> on browser.

You will get `Welcome!` if it runs fine.

## Add static files

In this section, we let the server to desplay the files in `/public` directory.
i.e. static server.

See <https://actix.rs/docs/static-files/>

### Add dependencies

Add

```toml
actix-files = "0.6.0"
```

in the dependency list in the `Cargo.toml`.

### Add routing to `/public` directory

Add

```rust
use actix_files as fs;
```

and add

```rust
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())

            // with path parameters
            // **Newly added here**
            // GET /pages/**/*.html and return the file /public/**/*.html
            .service(fs::Files::new("/pages", "public").show_files_listing())

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
```

### Add some test files and test

create and add some test files in `/public` directory

```sh
mkdir public
cd public
echo "This is a test" > index.html
echo "This is a test" > test.html
```

## Contents server

In this section, we will extend the static server to contents management server.
We will add post, delete method to make it CRUD.

### Add dependencies

Add some dependencies to handle json.

```toml
json = "0.12"
serde = { version = "1.0", features = ["derive"]  }
serde_json = "1.0"
```

### Creating a file and a directory

We need to create a file.
However, creating a file in a non-existing directory will cause `no such file or directory` error.
Thus, we need to firstly create a directory and create the file.

```rust
/// Create a directory and a file `root_dir/path` and write with `contents`
fn create_dir_and_write(root_dir: &str, path: &str, contents: &str) -> Result<(), Error> {
    // TODO: check the path is valid
    let path: PathBuf = Path::new(&root_dir).join(Path::new(&path));
    println!("path: {:?}", path);

    // TODO: use BufReader
    println!("updating the markdown file");

    // Writing to a file

    // If the parent directory does not exists, then we should create it first
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    // Write to a file
    let mut file = File::create(&path)?;
    file.write_all(&contents.as_bytes())
        .expect("cannot write to a file");

    return Ok(());
}
```

### Deleting a directory

```rust
/// Remove directory recursively if it is empty
/// TODO: Succeeding with error may not the smartest solution
fn remove_dir(path: &Path) {
    println!("removing dir: {:?}", path);

    match std::fs::remove_dir(&path) {
        Ok(()) => remove_dir(path.parent().unwrap()),
        Err(_) => return,
    }
}
```

### Handle POST and DELETE methods

```rust
use std::fs::File;
use std::io;
use std::io::prelude::*;

use actix_files;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
// use json::JsonValue;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use std::path::{Path, PathBuf};


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
```

## Markdown parsing and generating html

In this section, we parse the posted markdown and convert it to a html file.

We will be saving the markdown file in `/public/edit` directory
and html file in `/public/pages` directory.

### Add dependencies

Add dependency to `Cargo.toml`.
We will be using `pulldown_cmark` to convert markdown to html.

```toml
pulldown-cmark = { version = "0.9.1", default-features = false }
```

### Convert markdown to html

add the converter form markdown to html to the `post_edited` function.

```rust

    // Parse the given markdown with the pulldown_cmark parser
    println!("parsing the given markdown with the pulldown_cmark parser");
    let parser = Parser::new(&item.body);
    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed: {}", html_buf);

```

### Test

test with curl

## Client

Add JavaScript to jump to the editor and to update the edited page.

Using fetch API.

## Some improvements

Add a list of recent updated pages.

Store `recent_updates` the list of the title of the recent updated files.
