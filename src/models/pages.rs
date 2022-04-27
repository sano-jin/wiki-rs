// use std::io;

use crate::util;
// use actix_files;
// use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::Error;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use pulldown_cmark::{html, Options, Parser};
// use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
// use std::path::{Path, PathBuf};
use urlencoding;

#[derive(Debug)]
pub struct Page {
    pub path: String, // id
    pub name: String,
    pub markdown: String,
    pub html: String,
}

impl Page {
    /// Create or update the page
    /// handles the POST request
    pub fn create(path: &str, markdown: &str) -> Result<Page, Error> {
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
        let parser = Parser::new(&markdown);
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);
        println!("parsed: {}", html_buf);

        // decode the path to obtain the title
        let name = urlencoding::decode(&path).expect("cannot decode");

        // Open the default file
        let default_page = std::fs::read_to_string("public/layouts/page.html")?;
        // Replace the title, path, contents
        let html = default_page
            .replace("TITLE", &name)
            .replace("PATH", &path)
            .replace("BODY", &html_buf);

        Ok(Page {
            path: path.to_string(),
            name: name.to_string(),
            markdown: markdown.to_string(),
            html: html.to_string(),
        })
    }

    pub fn save(page: &Page) -> Result<(), Error> {
        // Update the file with the given contents
        let path = util::get_path("public/edit", &page.path);
        println!("writing to the file {:?}", path);
        let mut file = File::create(&path)?;
        file.write_all(page.markdown.as_bytes())?;

        // Update the file with the given contents
        let path = util::get_path("public/pages", &page.path);
        println!("writing to the file {:?}", path);
        let mut file = File::create(&path)?;
        file.write_all(page.html.as_bytes())?;

        Ok(())
    }

    pub fn delete(filepath: &str) -> Result<(), Error> {
        // delete the markdown file
        let path = util::get_path("public/edit", &filepath);
        std::fs::remove_file(&path)?;

        // delete the html file
        let path = util::get_path("public/pages", &filepath);
        std::fs::remove_file(&path)?;

        Ok(())
    }

    pub fn get_html(filepath: &str) -> Result<String, Error> {
        // Load the file
        let path = util::get_path("public/pages", &filepath);
        let contents = std::fs::read_to_string(&path)?;
        Ok(contents)
    }
}
