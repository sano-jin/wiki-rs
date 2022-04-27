// use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
// use dotenv::dotenv;
// use std::env;
//
// pub fn establish_connection() -> MysqlConnection {
//     dotenv().ok();
//
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

use std::io;

use std::path::{Path, PathBuf};
use urlencoding;

/// Get the new path <root_dir>/<encoded filename>
pub fn get_path(root_dir: &str, filename: &str) -> PathBuf {
    let encoded = urlencoding::encode(&filename); // encode the filename
    Path::new(&root_dir).join(Path::new(&encoded.into_owned()))
}

/// Read a file
/// If the file doesn not exists, then return the default string
pub fn read_with_default(path: &str, default: &str) -> String {
    let contents = std::fs::read_to_string(&path);
    match contents {
        Ok(contents) => contents,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from(default),
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    }
}
