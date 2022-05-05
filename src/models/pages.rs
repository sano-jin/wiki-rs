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

use regex::Regex;

#[derive(Debug)]
pub struct Page {
    pub path: String, // id
    pub name: String,
    pub markdown: String,
    pub html: String,
}

impl Page {
    /// convert markdown to html
    pub fn html_of_markdown(markdown: &str) -> Result<String, Error> {
        // backslash をエスケープする．
        // pulldown-cmark は backslash を無視してしまうっぽい．
        // TODO: markdown の仕様を確認して backslash をどう扱うべきか再考する．
        let markdown_escaped = markdown.replace("\\", "\\\\");

        // リンクの処理
        // リンクを <> でかこむ
        let re = Regex::new(r"([^<])(https?://[^\s\)]*)([^>])").unwrap();
        let markdown_escaped = re.replace_all(&markdown_escaped, "$1<$2>$3");

        // 括弧で囲まれていた場合（ユーザがちゃんとリンクとして書いていた場合）は取り除く
        let re = Regex::new(r"\[([^\]]*)\]\(\s*<(https?://[^\s\)]*)>\s*\)").unwrap();
        let markdown_escaped = re.replace_all(&markdown_escaped, "[$1]($2)");
        // ここまでリンクの処理

        // コメントアウトを削除
        let re = Regex::new(r"(?m)^//.*$\n?").unwrap();
        let markdown_escaped = re.replace_all(&markdown_escaped, "");

        println!("markdown escaped {}", markdown_escaped);

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
        let parser = Parser::new_ext(&markdown_escaped, options);
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);
        // println!("parsed: {}", html_buf);

        // decode the path to obtain the title
        Ok(html_buf)
    }

    /// Create or update the page
    /// handles the POST request
    pub fn create(path: &str, markdown: &str) -> Result<Page, Error> {
        let html_buf = Page::html_of_markdown(&markdown)?;

        // decode the path to obtain the title
        let name = urlencoding::decode(&path).expect("cannot decode");

        // Open the default file
        let default_page = std::fs::read_to_string("public/layouts/page.html")?;
        // Replace the title, path, contents
        let html = default_page
            .replace("{{ TITLE }}", &name)
            .replace("{{ PATH }}", &path)
            .replace("{{ BODY }}", &html_buf);

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
        let contents = contents.replace("{{ INDEX_UL }}", &vec_pages_list.join("\n"));
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

    /// This handler uses json extractor with limit
    /// GET the page for editing the page
    pub fn get_editor(path_str: &str) -> Result<String, Error> {
        let path = util::get_path("public/edit", &path_str);
        let contents = util::read_with_default(&path.to_string_lossy(), "");

        // decode the path to obtain the title
        let title = urlencoding::decode(&path_str).expect("cannot decode");

        // Open the file for editing
        let editor = std::fs::read_to_string("public/layouts/edit.html")?;
        // Replace the contents
        let editor = editor
            .replace("{{ TITLE }}", &title.into_owned())
            .replace("{{ MARKDOWN }}", &contents);

        Ok(editor)
    }
}
