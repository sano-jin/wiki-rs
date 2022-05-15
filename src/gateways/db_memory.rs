/// Database
// <T: Copy + Ord>
/// DB 本体
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::gateways::db::{Database, Result};
// use crate::util;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone)]
pub struct DatabaseImplOnMemory {
    // pool
}

impl Database for DatabaseImplOnMemory {
    /// Insert data with the id to the table
    fn insert<'a, T: Serialize + Deserialize<'a>>(
        &self,
        table_name: &str,
        id: &str,
        data: &T,
    ) -> Result<()> {
        // serialize the data structure to json
        let data_json_str = serde_json::to_string(&data)?;
        // print!("page_data_json: {:?}", page_data_json);

        // Update the file with the given contents
        // let path = util::get_path(&db_path, &id);
        let path = format!("db/{}/{}", &table_name, &id);
        println!("writing to the file {:?}", path);
        let mut file = File::create(&path)?;
        file.write_all(data_json_str.as_bytes())?;

        Ok(())
    }

    /// Delete the data with the id from the table
    fn delete(&self, table_name: &str, id: &str) -> Result<()> {
        // Update the file with the given contents
        // let path = util::get_path(&db_path, &id);
        let path = format!("db/{}/{}", &table_name, &id);
        println!("deleting the file {:?}", path);

        // delete the file
        std::fs::remove_file(&path)?;

        Ok(())
    }

    /// Get the data with the id from the table
    fn get<T: Serialize + DeserializeOwned>(&self, table_name: &str, id: &str) -> Result<T> {
        // Update the file with the given contents
        // let path = util::get_path(&db_path, &id);
        let path = format!("db/{}/{}", &table_name, &id);
        // println!("getting the file {:?}", path);

        // Getting the data
        let page_data_json = std::fs::read_to_string(&path)?;

        // transform the data in DB to the Deserialized Data
        let data = serde_json::from_str(&page_data_json)?;

        Ok(data)
    }

    /// Get the list of file ids
    /// (possibly sorted by the modified date ?)
    fn get_ids(&self, table_name: &str) -> Result<Vec<String>> {
        // Update the file with the given contents
        let path = format!("db/{}", &table_name);
        println!("listing the files in directory {:?}", path);

        let dir_entries = std::fs::read_dir(&path).unwrap();
        let mut vec = Vec::new();
        for dir_entry in dir_entries {
            if let Ok(entry) = dir_entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Ok(filepath) = entry.file_name().into_string() {
                            // let filepath = urlencoding::decode(&filepath).expect("cannot decode");
                            // println!("filepath: {}", filepath);
                            let ancestors =
                                Path::new(&filepath).file_name().unwrap().to_str().unwrap();
                            // println!("File name was {}", ancestors);
                            vec.push(ancestors.to_string());
                        }
                    }
                }
            }
        }

        Ok(vec)
    }

    /// Get the all data in the table
    fn get_all<T: Serialize + DeserializeOwned>(&self, table_name: &str) -> Result<Vec<T>> {
        let filepaths = Self::get_ids(&self, &table_name)?;

        let files: Vec<T> = filepaths
            .iter()
            .map(|filepath| Self::get(&self, table_name, &filepath).unwrap())
            .collect();

        Ok(files)
    }
}
