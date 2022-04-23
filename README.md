wiki.rs

A simple wiki created with Rust from scratch.

# API design

## Front

- 普通にアクセスして見る．
- 今見ているページの markdown を編集して，それでページを更新する．
  - edit button
- 新しいページの markdown を編集して，それでページを更新する．
  - create button

## Backend API

- GET /page/xxxxxx
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む
- GET /edit
  - body: `{path:"Path to the page"}`
  - 編集用の markdown を返す
  - サーバ上のファイルから読み込む
- POST /edit
  - body: `{path:"Path to the page", body: "The updated markdown"}`
  - markdown を投げ，それで /xxxxxx.html を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- DELETE /edit
  - body: `{path:"Path to the page"}`
  - /xxxxxx.html を消去する
  - サーバ上のファイルは消去する

## 構成

# How to create a own wiki from scrarch

## Prerequisties

Install Cargo

## Implement Https server

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
