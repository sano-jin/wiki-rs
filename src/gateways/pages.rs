/// DB とやりとりするためのコード
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::usecases::pages::Page;
use crate::util;
use actix_web::Error;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use urlencoding;

/// データベースとやりとりするためのデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct PageData {
    pub path: String, // id
    pub name: String,
    pub markdown: String,
    pub html: String,
    pub modified_rfc3339: String, // ISO8601 string for datetime
}

pub fn get_default_page() -> Result<String, Error> {
    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/page.html")?;
    Ok(default_page)
}

/// Save the page to files
pub fn save(page: &Page) -> Result<(), Error> {
    let page_data = PageData {
        path: page.path.to_owned(),
        name: page.name.to_owned(),
        markdown: page.markdown.to_owned(),
        html: page.html.to_owned(),
        modified_rfc3339: page.modified.to_rfc3339(),
    };

    // serialize the data structure to json
    let page_data_json = serde_json::to_string(&page_data)?;
    // print!("page_data_json: {:?}", page_data_json);

    // Update the file with the given contents
    let path = util::get_path("public/db", &page.path);
    println!("writing to the file {:?}", path);
    let mut file = File::create(&path)?;
    file.write_all(page_data_json.as_bytes())?;

    Ok(())
}

/// Delete the page
pub fn delete(filepath: &str) -> Result<(), Error> {
    // delete the file
    let path = util::get_path("public/db", &filepath);
    std::fs::remove_file(&path)?;

    Ok(())
}

/// Get the page
pub fn get_page(filepath: &str) -> Result<Page, Error> {
    // Load the file
    let path = util::get_path("public/db", &filepath);
    let page_data_json = std::fs::read_to_string(&path)?;

    // transform the data in DB to the Page
    let page_data: PageData = serde_json::from_str(&page_data_json)?;
    let modified = DateTime::parse_from_rfc3339(&page_data.modified_rfc3339).expect("joge");
    let modified = DateTime::from(modified);

    Ok(Page {
        path: page_data.path,
        name: page_data.name,
        markdown: page_data.markdown,
        html: page_data.html,
        modified: modified,
    })
}

/// Get the html contents
pub fn get_html(filepath: &str) -> Result<String, Error> {
    // Load the file
    let page = get_page(&filepath)?;

    let pages_list = list_pages().expect("file list");
    let contents = page.render(pages_list.as_slice()).expect("error");

    Ok(contents)
}

/// get modified date from DirEntry
pub fn get_modified(entry: &std::fs::DirEntry) -> Result<u64, std::io::Error> {
    let path = entry.path();

    let metadata = std::fs::metadata(&path)?;
    let last_modified = metadata.modified()?.elapsed().expect("hoge").as_secs();
    Ok(last_modified)
}

/// Get the list of files
/// sorted by the modified date
pub fn list_pages() -> Option<Vec<(String, String)>> {
    let mut vec = Vec::new();
    let paths = std::fs::read_dir("public/db/").unwrap();
    let mut vec_files = Vec::new();
    for dir_entry in paths {
        if let Ok(entry) = dir_entry {
            vec_files.push(entry)
        }
    }
    // sort by the modified date
    vec_files.sort_by(|a, b| {
        let a_modified = get_modified(&a).unwrap();
        let b_modified = get_modified(&b).unwrap();
        a_modified.partial_cmp(&b_modified).unwrap()
    });

    // for path in paths {
    for path in vec_files {
        let filename = path.path();
        let filename = filename.file_name()?.to_str()?;

        // decode the path to obtain the title
        let decoded_filename = urlencoding::decode(&filename).expect("cannot decode");

        // println!("Name: {}", filename);
        vec.push((decoded_filename.to_string(), filename.to_string()));
    }
    Some(vec)
}

/// GET the page for editing the page
pub fn get_editor(path_str: &str) -> Result<String, Error> {
    let contents = match get_page(&path_str) {
        Err(..) => String::from(""),
        Ok(page) => page.markdown,
    };
    // let contents = util::read_with_default(&path.to_string_lossy(), "");

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
