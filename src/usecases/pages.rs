/// ページのデータ構造の変換とかを書くための部分
/// データベースとのやりとりとかは書かない
use crate::entities::pages;
use actix_web::Error;
use chrono::{DateTime, Local, Utc};
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
    pub fn create(
        default_page: &str,
        path: &str,
        markdown: &str,
        // attach_names: Vec<String>,
    ) -> Result<Page, Error> {
        // heading にリンクが追加されていなかったら uuid を活用して id をふる
        let (markdown, toc) = pages::add_heading_ids(&markdown);
        // println!("heading map: {:?}", toc);

        ////////////////////////////////////////////////////////////////////////////////
        // TOC
        // headings からマークダウンの toc を生成する
        let toc = pages::markdown_of_toc(&toc);
        // let toc = format!("- Table of contents\n{}", &toc);
        // html に変換する
        let toc = pages::html_of_markdown(&path, &toc)?;

        // markdown を html に変換する
        let html_buf = pages::html_of_markdown(&path, &markdown)?;

        // toc を html に埋め込む
        let html_buf = format!("<div class=\"menu collapse\">{}</div>\n{}", toc, html_buf);
        ////////////////////////////////////////////////////////////////////////////////

        // decode the path to obtain the title
        let name = urlencoding::decode(&path).expect("cannot decode");

        let modified_datetime: DateTime<Utc> = Utc::now();
        let local_updated_time: DateTime<Local> = DateTime::from(modified_datetime);

        // Replace the title, path, contents
        let html = default_page
            .replace("{{ TITLE }}", &name)
            .replace("{{ PATH }}", &path)
            .replace(
                "{{ UPDATED_DATE }}",
                &local_updated_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            )
            .replace("{{ BODY }}", &html_buf);

        Ok(Page {
            path: path.to_string(),
            name: name.to_string(),
            markdown: markdown.to_string(),
            html: html.to_string(),
            modified: modified_datetime,
        })
    }

    /// Embed the list of files in the given html contents
    pub fn render(
        &self,
        menu_markdown: &str,
        pages_list: &[(String, String)],
        attach_names: &[(String, String)],
    ) -> Option<String> {
        // let pages_list = Page::list_pages().expect("file list");

        // Load the file
        let contents = &self.html;
        let contents = Page::embed_pages_list(&contents, &menu_markdown, &pages_list)?;

        // Set the names of attached files
        let attach_names: Vec<String> = attach_names
            .iter()
            .map(|(name, _)| {
                format!(
                    "<li><span>{}</span><span class=\"button btn-delete-attach\">delete</span></li>",
                    name
                )
            })
            .collect();
        let attach_names = attach_names.join("\n");
        let attach_names = format!("<ul class=\"attach-names\">{}</ul>", attach_names);

        // println!(">>>> attach_names: {:?}", attach_names);
        let contents = contents.replace("{{ ATTACH_NAMES }}", &attach_names);

        Some(contents)
    }

    /// Embed the list of files in the given html contents
    pub fn embed_pages_list(
        contents: &str,
        menu_markdown: &str,
        pages_list: &[(String, String)],
    ) -> Option<String> {
        // let pages_list = Page::list_pages().expect("file list");

        // recently updated pages list
        // println!("pages list {:?}", pages_list);
        let mut vec_pages_list = Vec::new();
        for (decoded, path) in pages_list {
            vec_pages_list.push(format!(
                "<li><a href=\"pages?path={}\">{}</a></li>",
                path, decoded
            ));
        }
        let pages_list = vec_pages_list.join("\n");
        let pages_list = format!("<ul>{}</ul>", pages_list);

        let menu = pages::html_of_markdown("", &menu_markdown).unwrap();
        let menu = menu.replace("{{ INDEX_UL }}", &pages_list);

        // Load the file
        // Load the file
        let contents = contents.replace("{{ INDEX_UL }}", &pages_list);
        let contents = contents.replace("{{ SIDE_MENU }}", &menu);
        Some(contents)
    }
}
