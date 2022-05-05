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

    /// Save the page to files
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

    /// Delete the page
    pub fn delete(filepath: &str) -> Result<(), Error> {
        // delete the markdown file
        let path = util::get_path("public/edit", &filepath);
        std::fs::remove_file(&path)?;

        // delete the html file
        let path = util::get_path("public/pages", &filepath);
        std::fs::remove_file(&path)?;

        Ok(())
    }

    /// Get the html contents
    pub fn get_html(filepath: &str) -> Result<String, Error> {
        // Load the file
        let path = util::get_path("public/pages", &filepath);
        let contents = std::fs::read_to_string(&path)?;
        let contents = Page::embed_pages_list(&contents).expect("error");
        Ok(contents)
    }

    /// Embed the list of files in the given html contents
    pub fn embed_pages_list(contents: &str) -> Option<String> {
        let pages_list = Page::list_pages().expect("file list");
        println!("pages list {:?}", pages_list);
        let mut vec_pages_list = Vec::new();
        for (decoded, path) in pages_list {
            vec_pages_list.push(format!(
                "<li><a href=\"pages?path={}\">{}</a></li>",
                path, decoded
            ));
        }

        // Load the file
        let contents = contents.replace("INDEX_UL", &vec_pages_list.join("\n"));
        Some(contents)
    }

    /// get modified date from DirEntry
    pub fn get_modified(entry: &std::fs::DirEntry) -> Result<u64, std::io::Error> {
        let path = entry.path();

        let metadata = std::fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed().expect("hoge").as_secs();
        Ok(last_modified)
    }

    /// Get the list of files
    pub fn list_pages() -> Option<Vec<(String, String)>> {
        let mut vec = Vec::new();
        let paths = std::fs::read_dir("public/pages/").unwrap();
        let mut vec_files = Vec::new();
        for dir_entry in paths {
            if let Ok(entry) = dir_entry {
                vec_files.push(entry)
            }
        }
        // sort by the modified date
        vec_files.sort_by(|a, b| {
            let a_modified = Page::get_modified(&a).unwrap();
            let b_modified = Page::get_modified(&b).unwrap();
            a_modified.partial_cmp(&b_modified).unwrap()
        });

        // for path in paths {
        for path in vec_files {
            let filename = path.path();
            let filename = filename.file_name()?.to_str()?;

            // decode the path to obtain the title
            let decoded_filename = urlencoding::decode(&filename).expect("cannot decode");

            println!("Name: {}", filename);
            vec.push((decoded_filename.to_string(), filename.to_string()));
        }
        Some(vec)
    }
}
