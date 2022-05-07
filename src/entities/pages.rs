/// Enterprise business rules を記述する部分
/// 要するに，普遍性の高い規則を記述する．
/// markdown からの html の変換は，このアプリケーションに特有の話ではないので，
/// ここに記述する．
use actix_web::Error;
use either::*;
use pulldown_cmark::{html, CowStr, Event, LinkType, Options, Parser, Tag};
use regex::Regex;
// use urlencoding;
// use crate::util;
// use pulldown_cmark::{html, Options, Parser};
// use std::fs::File;
// use std::io::prelude::*;

/// convert markdown to html
pub fn html_of_markdown(markdown: &str) -> Result<String, Error> {
    // backslash をエスケープする．
    // pulldown-cmark は backslash を無視してしまうっぽい．
    // TODO: markdown の仕様を確認して backslash をどう扱うべきか再考する．
    let markdown = markdown.replace("\\", "\\\\");

    // コメントアウトを削除
    let re = Regex::new(r"(?m)^//.*$\n?").unwrap();
    let markdown = re.replace_all(&markdown, "");

    println!("markdown {}", markdown);

    // Set parser options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    // Parse the given markdown with the pulldown_cmark parser
    println!("parsing the given markdown with the pulldown_cmark parser");

    // let parser = Parser::new_ext(&markdown, options);
    // Set up parser. We can treat is as any other iterator. We replace Peter by John
    // and image by its alt text.
    let parser = Parser::new_ext(&markdown, Options::empty()).map(|event| match event {
        // Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
        // 画像などのファイルのパスをいじりたい
        Event::Start(Tag::Image(LinkType::Inline, url, title))
        | Event::End(Tag::Image(LinkType::Inline, url, title)) => {
            println!("url is {}", url);
            let re = Regex::new(r"(^https?://[^\s]*)|^/").unwrap();
            let mut url = url.to_string();
            // let mut url = url;
            if !re.is_match(&url) {
                url = String::from("/files/files/") + &url;
            }
            Event::Start(Tag::Image(LinkType::Inline, CowStr::from(url), title))
        }
        _ => event,
    });

    // 本文中の url をリンクに変換する．
    // すでにリンクになっている場合はそのままにする必要があるので，
    // リンクのネスト具合を link_level で管理する
    let mut link_level = 0;
    let parser = parser.flat_map(|event| match event {
        // Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
        Event::Start(Tag::Image(..)) | Event::Start(Tag::Link(..)) => {
            link_level += 1;
            Right(std::iter::once(event))
        }
        Event::End(Tag::Image(..)) | Event::End(Tag::Link(..)) => {
            link_level -= 1;
            Right(std::iter::once(event))
        }
        Event::Text(text) => {
            if link_level > 0 {
                return Right(std::iter::once(Event::Text(text)));
            }
            println!("text is {}", text);
            let re = Regex::new(r"^https?://[^\s]*").unwrap();
            let text_str = text.to_string();
            if re.is_match(&text_str) {
                println!("{}", text_str);
                return Left(
                    std::iter::once(Event::Start(Tag::Link(
                        LinkType::Inline,
                        CowStr::from(text_str.to_owned()),
                        CowStr::from(text_str.to_owned()),
                    )))
                    .chain(std::iter::once(Event::Text(CowStr::from(
                        text_str.to_owned(),
                    ))))
                    .chain(std::iter::once(Event::End(Tag::Link(
                        LinkType::Inline,
                        CowStr::from(text_str.to_owned()),
                        CowStr::from(text_str.to_owned()),
                    )))),
                );
            }
            Right(std::iter::once(Event::Text(text)))
        }
        _ => Right(std::iter::once(event)),
    });

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    println!("parsed: {}", html_buf);

    // decode the path to obtain the title
    Ok(html_buf)
}

// /// Embed the list of files in the given html contents
// pub fn embed_pages_list(contents: &str) -> Option<String> {
//     let pages_list = list_pages().expect("file list");
//     println!("pages list {:?}", pages_list);
//     let mut vec_pages_list = Vec::new();
//     for (decoded, path) in pages_list {
//         vec_pages_list.push(format!(
//             "<li><a href=\"pages?path={}\">{}</a></li>",
//             path, decoded
//         ));
//     }
//
//     // Load the file
//     let contents = contents.replace("{{ INDEX_UL }}", &vec_pages_list.join("\n"));
//     Some(contents)
// }
//
// /// get modified date from DirEntry
// pub fn get_modified(entry: &std::fs::DirEntry) -> Result<u64, std::io::Error> {
//     let path = entry.path();
//
//     let metadata = std::fs::metadata(&path)?;
//     let last_modified = metadata.modified()?.elapsed().expect("hoge").as_secs();
//     Ok(last_modified)
// }
//
// /// Get the list of files
// /// sorted by the modified date
// pub fn list_pages() -> Option<Vec<(String, String)>> {
//     let mut vec = Vec::new();
//     let paths = std::fs::read_dir("public/pages/").unwrap();
//     let mut vec_files = Vec::new();
//     for dir_entry in paths {
//         if let Ok(entry) = dir_entry {
//             vec_files.push(entry)
//         }
//     }
//     // sort by the modified date
//     vec_files.sort_by(|a, b| {
//         let a_modified = get_modified(&a).unwrap();
//         let b_modified = get_modified(&b).unwrap();
//         a_modified.partial_cmp(&b_modified).unwrap()
//     });
//
//     // for path in paths {
//     for path in vec_files {
//         let filename = path.path();
//         let filename = filename.file_name()?.to_str()?;
//
//         // decode the path to obtain the title
//         let decoded_filename = urlencoding::decode(&filename).expect("cannot decode");
//
//         // println!("Name: {}", filename);
//         vec.push((decoded_filename.to_string(), filename.to_string()));
//     }
//     Some(vec)
// }