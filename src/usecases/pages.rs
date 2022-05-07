/// ページのデータ構造の変換とかを書くための部分
/// データベースとのやりとりとかは書かない
use crate::entities::pages;
use actix_web::Error;
use chrono::{DateTime, Utc};
use urlencoding;

// #[derive(Debug, Serialize, Deserialize)]
#[derive(Debug)]
pub struct Page {
    pub path: String, // id
    pub name: String,
    pub markdown: String,
    pub html: String,
    pub modified: DateTime<Utc>,
    // let mut headings: Vec<(String, usize, String)> = Vec::new();
}

impl Page {
    /// Create or update the page
    /// handles the POST request
    /// Create a page from the default page template, path and markdown
    pub fn create(default_page: &str, path: &str, markdown: &str) -> Result<Page, Error> {
        // heading にリンクが追加されていなかったら uuid を活用して id をふる
        let (markdown, toc) = pages::add_heading_ids(&markdown);
        println!("heading map: {:?}", toc);

        // markdown を html に変換する
        let html_buf = pages::html_of_markdown(&path, &markdown)?;

        // decode the path to obtain the title
        let name = urlencoding::decode(&path).expect("cannot decode");

        // // Open the default file
        // let default_page = std::fs::read_to_string("public/layouts/page.html")?;

        // Replace the title, path, contents
        let html = default_page
            .replace("{{ TITLE }}", &name)
            .replace("{{ PATH }}", &path)
            .replace("{{ BODY }}", &html_buf);

        let modified_datetime: DateTime<Utc> = Utc::now();

        Ok(Page {
            path: path.to_string(),
            name: name.to_string(),
            markdown: markdown.to_string(),
            html: html.to_string(),
            modified: modified_datetime,
        })
    }

    /// Embed the list of files in the given html contents
    pub fn render(&self, pages_list: &[(String, String)]) -> Option<String> {
        // let pages_list = Page::list_pages().expect("file list");

        // Load the file
        let contents = &self.html;
        let contents = Page::embed_pages_list(&contents, &pages_list)?;
        Some(contents)
    }

    /// Embed the list of files in the given html contents
    pub fn embed_pages_list(contents: &str, pages_list: &[(String, String)]) -> Option<String> {
        // let pages_list = Page::list_pages().expect("file list");

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
}
