/// DB とやりとりするためのコード
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::util;
use actix_web::Error;
// use chrono::DateTime;
// use serde::{Deserialize, Serialize};
// use serde_json;
use std::fs::File;
use std::io::prelude::*;
// use urlencoding;

// /// データベースとやりとりするためのデータ構造
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PageData {
//     pub path: String, // id
//     pub name: String,
//     pub markdown: String,
//     pub html: String,
//     pub modified_rfc3339: String, // ISO8601 string for datetime
// }

/// handle attached files
/// Save the attach file
pub fn save(path: &str, buf: &Vec<u8>) -> Result<(), Error> {
    // Update the file with the given contents
    let path = util::get_path("public/db/attach", &path);
    println!("writing to the file {:?}", path);

    let mut file = File::create(&path)?;
    file.write_all(buf)?;

    Ok(())
}

/// Delete the attached file
pub fn delete(filepath: &str) -> Result<(), Error> {
    // delete the file
    let path = util::get_path("public/db/attach", &filepath);

    println!("removing the file {:?}", path);
    std::fs::remove_file(&path)?;

    Ok(())
}

/// Get the attached file
pub fn get(filepath: &str) -> Result<Vec<u8>, Error> {
    // Load the file
    let path = util::get_path("public/db/attach", &filepath);
    println!("path is {:?}", path);
    // let data = std::io::Read::read_to_end(&path)?;

    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;
    // println!("{:?}", buf);

    Ok(buf)
    //     // transform the data in DB to the Page
    //     let page_data: PageData = serde_json::from_str(&page_data_json)?;
    //     let modified = DateTime::parse_from_rfc3339(&page_data.modified_rfc3339).expect("joge");
    //     let modified = DateTime::from(modified);
    //
    //     Ok(Page {
    //         path: page_data.path,
    //         name: page_data.name,
    //         markdown: page_data.markdown,
    //         html: page_data.html,
    //         modified: modified,
    //     })
}

/// Get the list of file names and paths
/// sorted by the modified date
pub fn get_attach_names_in_page(page_path: &str) -> Option<Vec<(String, String)>> {
    let dir_entries = std::fs::read_dir("public/db/attach").unwrap();
    let mut vec_attaches = Vec::new();
    for dir_entry in dir_entries {
        if let Ok(entry) = dir_entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(filepath) = entry.file_name().into_string() {
                        let filename = urlencoding::decode(&filepath).expect("cannot decode");
                        let filename = filename.to_string();
                        if filename.starts_with(&page_path) {
                            vec_attaches.push((filename, filepath));
                        }
                    }
                }
            }
        }
    }

    println!("- attach names: {:?}", vec_attaches);

    Some(vec_attaches)
}

/// Get the list of file names and paths
/// sorted by the modified date
pub fn get_attach_names() -> Option<Vec<(String, String)>> {
    let dir_entries = std::fs::read_dir("public/db/attach").unwrap();
    let mut vec_attaches = Vec::new();
    for dir_entry in dir_entries {
        if let Ok(entry) = dir_entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(filepath) = entry.file_name().into_string() {
                        let filename = urlencoding::decode(&filepath).expect("cannot decode");
                        let filename = filename.to_string();
                        vec_attaches.push((filename, filepath));
                    }
                }
            }
        }
    }

    println!("- attach names: {:?}", vec_attaches);

    Some(vec_attaches)
}

/// Get the list of files
/// // sorted by the modified date
pub fn get_attaches() -> Option<Vec<Vec<u8>>> {
    let dir_entries = std::fs::read_dir("public/db/attach").unwrap();
    let mut vec_attaches = Vec::new();
    for dir_entry in dir_entries {
        if let Ok(entry) = dir_entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(filepath) = entry.file_name().into_string() {
                        let filepath = urlencoding::decode(&filepath).expect("cannot decode");
                        // println!("filepath: {}", filepath);
                        let attach = get(&filepath).unwrap();
                        vec_attaches.push(attach);
                    }
                }
            }
        }
    }

    println!("attach files: {:?}", vec_attaches);

    Some(vec_attaches)
}
