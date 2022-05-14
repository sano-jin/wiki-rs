/// DB とやりとりするためのコード
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::usecases::users::User;
use crate::util;
use actix_web::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use urlencoding;

/// データベースとやりとりするためのデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String, // id
    pub password: String,
}

/// Save the user to files
pub fn save(user: &User) -> Result<(), Error> {
    let user_data = UserData {
        name: user.name.to_owned(),
        password: user.password.to_owned(),
    };

    // serialize the data structure to json
    let user_data_json = serde_json::to_string(&user_data)?;
    // print!("user_data_json: {:?}", user_data_json);

    // Update the file with the given contents
    let path = util::get_path("db/users", &user.path);
    println!("writing to the file {:?}", path);
    let mut file = File::create(&path)?;
    file.write_all(user_data_json.as_bytes())?;

    Ok(())
}

/// Delete the user
pub fn delete(filepath: &str) -> Result<(), Error> {
    // TODO: return error if the filepath is TOP (the root user)

    // delete the file
    let path = util::get_path("db/users", &filepath);
    std::fs::remove_file(&path)?;

    Ok(())
}

/// Get the user
pub fn get_user(user_name: &str) -> Result<User, Error> {
    // Load the file
    let path = util::get_path("db/users", &user_name);
    let user_data_json = std::fs::read_to_string(&path)?;

    // transform the data in DB to the User
    let user_data: UserData = serde_json::from_str(&user_data_json)?;

    Ok(User {
        path: path.into_os_string().into_string().unwrap(),
        name: user_data.name,
        password: user_data.password,
    })
}

/// Get the list of files
/// sorted by the modified date
pub fn get_users() -> Option<Vec<User>> {
    let dir_entries = std::fs::read_dir("db/users").unwrap();
    let mut vec_users = Vec::new();
    for dir_entry in dir_entries {
        if let Ok(entry) = dir_entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Ok(filepath) = entry.file_name().into_string() {
                        let filepath = urlencoding::decode(&filepath).expect("cannot decode");
                        // println!("filepath: {}", filepath);
                        let user = get_user(&filepath).unwrap();
                        vec_users.push(user);
                    }
                }
            }
        }
    }

    println!("users: {:?}", vec_users);

    Some(vec_users)
}

/// GET the page for editing the page
pub fn get_editor() -> Result<String, Error> {
    // let contents = util::read_with_default(&path.to_string_lossy(), "");

    // Load the user
    let users = get_users().unwrap();
    let users: Vec<String> = users
        .iter()
        .map(|user| format!("<li>{}</li>", user.name))
        .collect();

    let users = users.join("\n");

    // let contents = gateways::users::get_html(&item.path)?;
    let contents = format!("<ul>{}</ul>", users);

    // Open the file for editing
    let editor = std::fs::read_to_string("public/layouts/user.html")?;
    // Replace the contents
    let editor = editor.replace("{{ USERS }}", &contents);

    Ok(editor)
}
