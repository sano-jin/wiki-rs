/// Database
/// DB 本体
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Database
pub trait Database {
    /// Insert data with the id to the table
    fn insert<'a, T: Serialize + Deserialize<'a>>(
        &self, // &mut self
        table_name: &str,
        id: &str,
        data: &T,
    ) -> Result<()>;

    /// Delete the data with the id from the table
    fn delete(&self, table_name: &str, id: &str) -> Result<()>;

    /// Get the data with the id from the table
    fn get<T: Serialize + DeserializeOwned>(&self, table_name: &str, id: &str) -> Result<T>;

    /// Get the list of file ids
    fn get_ids(&self, table_name: &str) -> Result<Vec<String>>;

    /// Get the all data in the table
    fn get_all<T: Serialize + DeserializeOwned>(&self, table_name: &str) -> Result<Vec<T>>;
}
