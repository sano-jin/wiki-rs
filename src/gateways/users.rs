/// DB とやりとりするためのコード
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::gateways::db::Database;
use crate::usecases::users::User;
use actix_web::Error;
use serde::{Deserialize, Serialize};
use urlencoding;

/// データベースとやりとりするためのデータ構造
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String, // id
    pub password: String,
}

/// Save the user to files
pub fn save(db: &impl Database, user: &User) -> Result<(), Error> {
    let user_data = UserData {
        name: user.name.to_owned(),
        password: user.password.to_owned(),
    };

    db.insert("users", &user.id, &user_data)?;

    Ok(())
}

/// Delete the user
pub fn delete(db: &impl Database, user_name: &str) -> Result<(), Error> {
    // delete the user
    let user_id = urlencoding::encode(&user_name);
    db.delete("users", &user_id)?;

    Ok(())
}

/// Get the user
pub fn get_user(db: &impl Database, user_name: &str) -> Result<User, Error> {
    // Load the file
    let user_id = urlencoding::encode(&user_name);
    get_user_by_id(db, &user_id)
}

/// Get the user
pub fn get_user_by_name(db: &impl Database, user_name: &str) -> Result<User, Error> {
    get_user(db, &user_name)
}

/// Get the user
pub fn get_user_by_id(db: &impl Database, user_id: &str) -> Result<User, Error> {
    let user_data: UserData = db.get("users", &user_id)?;

    Ok(User {
        id: user_id.to_string(),
        name: user_data.name,
        password: user_data.password,
    })
}

/// Get the list of files
/// sorted by the modified date
pub fn get_users(db: &impl Database) -> Result<Vec<User>, Error> {
    let users: Vec<UserData> = db.get_all("users")?;
    let users: Vec<_> = users
        .iter()
        .map(|user_data| {
            let user_id = urlencoding::encode(&user_data.name);
            User {
                id: user_id.to_string(),
                name: user_data.name.to_string(),
                password: user_data.password.to_string(),
            }
        })
        .collect();
    Ok(users)
}

/// GET the page for editing the page
pub fn get_editor(db: &impl Database) -> Result<String, Error> {
    // let contents = util::read_with_default(&path.to_string_lossy(), "");

    // Load the user
    let users = get_users(db).unwrap();
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
