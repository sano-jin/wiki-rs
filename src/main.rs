use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use actix_web_httpauth::extractors::basic::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
// use std::collections::HashMap;
// use std::sync::Mutex;
use dotenv::dotenv;
use wiki_rs::controllers::appstate::AppState;
use wiki_rs::db::db_memory::DatabaseImplOnMemory;
use wiki_rs::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    dotenv().ok();

    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let data = web::Data::new(AppState::new(DatabaseImplOnMemory {}));

    if args.len() > 1 && args[1] == "non-secure" {
        // enable in http (not https)
        println!("run http (not https) server");
        println!("Started http server: 0.0.0.0:8080");

        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default()) // enable logger
                .app_data(Config::default().realm("Restricted area")) // basic authemtication
                .app_data(data.clone())
                .configure(routes::routes::<DatabaseImplOnMemory>)
        })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    } else {
        println!("run https (http with ssl/tsl) server");
        println!("Started http server: 127.0.0.1:8443");

        // load ssl keys
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();

        HttpServer::new(move || {
            let cors = Cors::default()
                // .allowed_origin("127.0.0.1")
                .allowed_origin("https://127.0.0.1:8443")
                // .allowed_origin("192.168.10.105")
                .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600);

            App::new()
                .wrap(cors) // allow access from http://localhost
                .wrap(middleware::Logger::default()) // enable logger
                .app_data(Config::default().realm("Restricted area")) // basic authentication
                .app_data(data.clone())
                .configure(routes::routes::<DatabaseImplOnMemory>)
        })
        .bind_openssl("127.0.0.1:8443", builder)?
        .run()
        .await
    }
}
