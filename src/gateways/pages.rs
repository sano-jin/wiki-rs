/// DB とやりとりするためのコード
/// 今はファイルシステムをただ活用しているだけだけど，
/// ここを差し替えれば RDBM とかでも動くようにできる（ようにしようとしている）
///
use crate::gateways::attaches;
use crate::gateways::db::Database;
use crate::usecases::pages::Page;
use actix_web::Error;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
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

/// login ページを取得する
pub fn get_login_page() -> Result<String, Error> {
    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/login.html")?;
    Ok(default_page)
}

/// トップページの template を取得する
pub fn get_default_top_page() -> Result<String, Error> {
    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/top.html")?;
    Ok(default_page)
}

/// Side menu の template を取得する
pub fn get_default_menu_page() -> Result<String, Error> {
    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/menu.html")?;
    Ok(default_page)
}

/// 通常のページの template を取得する
pub fn get_default_page() -> Result<String, Error> {
    // Open the default file
    let default_page = std::fs::read_to_string("public/layouts/page.html")?;
    Ok(default_page)
}

/// Save the page to files
pub fn save(db: &impl Database, page: &Page) -> Result<(), Error> {
    let page_data = PageData {
        path: page.path.to_owned(),
        name: page.name.to_owned(),
        markdown: page.markdown.to_owned(),
        html: page.html.to_owned(),
        modified_rfc3339: page.modified.to_rfc3339(),
    };

    // Save the json to DB
    println!("insert into pages {:?}", page.path);
    db.insert("pages", &page.path, &page_data)?;

    Ok(())
}

/// Delete the page
pub fn delete(db: &impl Database, page_name: &str) -> Result<(), Error> {
    // TODO: return error if the filepath is TOP (the root page)

    // delete the data on DB
    let page_id = urlencoding::encode(&page_name);
    db.delete("pages", &page_id)?;

    Ok(())
}

/// Get the page
pub fn get_page(db: &impl Database, page_id: &str) -> Result<Page, Error> {
    // Load the data from DB
    // transform the data in DB to the Page

    // println!("page_id: {:?}", page_id);
    let page_data: PageData = db.get("pages", &page_id)?;

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
pub fn get_html(db: &impl Database, page_name: &str) -> Result<String, Error> {
    // Load the file
    println!("Getting html of page with page_name: {:?}", page_name);

    let page_id = urlencoding::encode(&page_name);
    let page = get_page(db, &page_id)?;

    let pages_list = list_pages(db).expect("file list");
    let attach_names = attaches::get_attach_names_in_page(&page_id).unwrap();

    // Load the menu
    let menu = get_page(db, "menu")?;

    let contents = page
        .render(
            &menu.markdown,
            pages_list.as_slice(),
            attach_names.as_slice(),
        )
        .expect("error");

    Ok(contents)
}

/// Get the list of files
/// sorted by the modified date
pub fn list_pages(db: &impl Database) -> Result<Vec<(String, String)>, Error> {
    let ids = db.get_ids("pages")?;
    let mut page_infos: Vec<_> = ids
        .iter()
        .map(|id| {
            let page: Page = get_page(db, id).unwrap();
            (page.modified, page.path)
        })
        .collect();

    // sort by the modified date
    page_infos.sort_by(|(t1, _), (t2, _)| t2.partial_cmp(t1).unwrap());

    // for path in paths {
    let mut vec = Vec::new();
    for (_, path) in page_infos {
        // decode the path to obtain the title
        let decoded_filename = urlencoding::decode(&path).expect("cannot decode");

        // println!("Name: {}", filename);
        vec.push((decoded_filename.to_string(), path.to_string()));
    }

    Ok(vec)
}

/// GET the page for editing the page
pub fn get_editor(db: &impl Database, page_name: &str) -> Result<String, Error> {
    let page_id = urlencoding::encode(&page_name);
    let contents = match get_page(db, &page_id) {
        Err(..) => String::from(""),
        Ok(page) => page.markdown,
    };
    // let contents = util::read_with_default(&path.to_string_lossy(), "");

    // decode the path to obtain the title
    // let title = urlencoding::decode(&path_str).expect("cannot decode");

    // Open the file for editing
    let editor = std::fs::read_to_string("public/layouts/edit.html")?;
    // Replace the contents
    let editor = editor
        .replace("{{ TITLE }}", &page_name)
        .replace("{{ MARKDOWN }}", &contents);

    Ok(editor)
}
