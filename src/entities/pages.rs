/// Enterprise business rules を記述する部分
/// 要するに，普遍性の高い規則を記述する．
/// markdown からの html の変換は，このアプリケーションに特有の話ではないので，
/// ここに記述する．
use actix_web::Error;
use either::*;
use pulldown_cmark::{html, CowStr, Event, LinkType, Options, Parser, Tag};
use regex::{Captures, Regex};
// use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Copy, Clone)]
enum State {
    IsNormal,  // normal text
    IsInCode1, // is in singly quoted inline code block
    IsInCode2, // is in doubly quoted inline code block
    IsInCode3, // is in triply quoted inline code block
    IsInCode4, // is in quadruply quoted inline code block
}

/// 一行読んで，読み終わった後の状態を返す
fn next_state_of(state: State, line: &str) -> State {
    if line.starts_with("    ") {
        // This is in a code block
        return state;
    }
    let mut state = state;
    let mut quotes_num = 0; // 連続して出現するバッククオートの数
    for c in line.chars() {
        if c == '`' {
            // println!("quoting...");
            quotes_num += 1;
            if state == State::IsInCode1 && quotes_num == 1
                || state == State::IsInCode2 && quotes_num == 2
                || state == State::IsInCode3 && quotes_num == 3
                || state == State::IsInCode4 && quotes_num == 4
            {
                state = State::IsNormal;
                quotes_num = 0;
            } else if state == State::IsNormal && quotes_num == 4 {
                state = State::IsInCode4;
                quotes_num = 0;
            } else if quotes_num > 4 {
                panic!("unreachable");
            }
        } else {
            if quotes_num == 1 {
                if state == State::IsNormal {
                    state = State::IsInCode1;
                } else if state == State::IsInCode1 {
                    state = State::IsNormal;
                }
            } else if quotes_num == 2 {
                if state == State::IsNormal {
                    state = State::IsInCode2;
                } else if state == State::IsInCode2 {
                    state = State::IsNormal;
                }
            } else if quotes_num == 3 {
                if state == State::IsNormal {
                    state = State::IsInCode3;
                } else if state == State::IsInCode3 {
                    state = State::IsNormal;
                }
            } else if quotes_num == 4 {
                if state == State::IsNormal {
                    state = State::IsInCode4;
                } else if state == State::IsInCode4 {
                    state = State::IsNormal;
                }
            } else if quotes_num > 4 {
                panic!("unreachable");
            }
            quotes_num = 0;
        }
    }
    state
}

/// 数式の中の backslash を escape する
fn escape_backslash_in_math(markdown: &str) -> String {
    // println!("> escaping markdown: {}", markdown);

    // escape in inline math mode
    let re = Regex::new(r"(?s)\\\((([^\\]*(\\[^\)])*)*)\\\)").unwrap();
    let markdown = re.replace_all(markdown, |caps: &Captures| {
        let escaped = &caps[1].replace("\\", "\\\\");
        // println!("escaped backslash in inline math mode: {}", escaped);
        format!("\\\\({}\\\\)", escaped)
    });
    let markdown = markdown.to_string();

    // escape in block math mode
    let re = Regex::new(r"(?s)\\\[(([^\\]*(\\[^\)])*)*)\\\]").unwrap();
    let markdown = re.replace_all(&markdown, |caps: &Captures| {
        let escaped = &caps[1].replace("\\", "\\\\");
        println!("escaped backslash in inline math mode: {}", escaped);
        format!("\\\\[{}\\\\]", escaped)
    });
    let markdown = markdown.to_string();

    // println!("> escaped markdown: {}", markdown);
    markdown.to_string()
}

// /// The elements of toc
// /// 目次の要素
// #[derive(Debug, PartialEq, Copy, Clone)]
// enum TocElem {
//     IsNormal,  // normal text
//     IsInCode1, // is in singly quoted inline code block
//     IsInCode2, // is in doubly quoted inline code block
//     IsInCode3, // is in triply quoted inline code block
//     IsInCode4, // is in quadruply quoted inline code block
// }

/// 添付ファイル名のリストからその markdown を生成する
pub fn markdown_of_attach_names(page_path: &str, attach_names: &Vec<(String, String)>) -> String {
    let attach_names: Vec<String> = attach_names
        .iter()
        .map(|(title, path)| format!("- [{}](/attach?path={}&file={})", title, page_path, path))
        .collect();
    attach_names.join("\n")
}

/// TOC からその markdown を生成する
pub fn markdown_of_toc(toc: &Vec<(String, usize, String)>) -> String {
    let toc: Vec<String> = toc
        .iter()
        .map(|(id, level, title)| {
            let spaces = std::iter::repeat("   ").take(level - 0).collect::<String>();
            format!("{}- [{}](#{})", spaces, title, id)
        })
        .collect();
    toc.join("\n")
}

/// heading にリンクが追加されていなかったら uuid を活用して id をふる
/// `# heading title { #id }` のようにする
pub fn add_heading_ids(markdown: &str) -> (String, Vec<(String, usize, String)>) {
    println!("adding heading ids");
    let lines = markdown.split("\n"); // 改行で区切る

    let is_heading = Regex::new(r"^(#+)\s*(.*)\s*$").unwrap();
    let has_id = Regex::new(r"^#+\s*(.*)\s+\{\s*#(\S+)\s*\}\s*$").unwrap();

    // let mut headings: HashMap<String, String> = HashMap::new();
    // Heading のベクタ
    let mut headings: Vec<(String, usize, String)> = Vec::new();
    let mut state = State::IsNormal; // コードブロックの中にいるかどうかの状態

    // TODO: Vec<String> じゃなくて，Vec<&str> を返すようにしたい（効率化）
    let lines: Vec<String> = lines
        .map(|line| {
            // println!("line: \"{}\", state: {:?}", line, state);
            if line.starts_with("    ") {
                return line.to_string();
            }
            if state != State::IsNormal {
                state = next_state_of(state, &line);
                return line.to_string();
            }
            state = next_state_of(state, &line);
            match is_heading.captures(line) {
                Some(caps) => {
                    let level_match = caps.get(1).unwrap();
                    let level = level_match.end() - level_match.start();
                    match has_id.captures(line) {
                        Some(caps) => {
                            let heading_text = caps.get(1).unwrap().as_str();
                            let id = caps.get(2).unwrap().as_str();
                            headings.push((id.to_string(), level, heading_text.to_string()));
                            return line.to_string();
                        }
                        None => {
                            let heading_text = caps.get(2).unwrap().as_str();
                            let id = Uuid::new_v4().to_string();
                            let line = format!(
                                "{} {} {{ #{} }}",
                                level_match.as_str(),
                                heading_text.trim(),
                                &id
                            );
                            headings.push((id, level, heading_text.to_string()));
                            return line;
                        }
                    };
                }
                None => line.to_string(),
            }
        })
        .collect();

    (lines.join("\n"), headings)
}

/// convert markdown to html
pub fn html_of_markdown(path: &str, markdown: &str) -> Result<String, Error> {
    // backslash をエスケープする．
    let markdown = escape_backslash_in_math(&markdown);

    // コメントアウトを削除
    let re = Regex::new(r"(?m)^//.*$\n?").unwrap();
    let markdown = re.replace_all(&markdown, "");

    // println!("markdown {}", markdown);

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
    let parser = Parser::new_ext(&markdown, options);

    let parser = parser.map(|event| match event {
        // 画像などのファイルのパスをいじりたい
        Event::Start(Tag::Image(LinkType::Inline, url, title)) => {
            // | Event::End(Tag::Image(LinkType::Inline, url, title)) => {
            // println!("url is {}", url);
            let re = Regex::new(r"(^https?://[^\s]*)|^/").unwrap();
            let mut url = url.to_string();
            // let mut url = url;
            if !re.is_match(&url) {
                // ファイルパスだった場合．
                url = format!("/attach?path={}&file={}", path, url);
            }
            Event::Start(Tag::Image(LinkType::Inline, CowStr::from(url), title))
        }
        _ => event,
    });

    // 本文中の url をリンクに変換する．
    // すでにリンクになっている場合はそのままにする必要があるので，
    // リンクのネスト具合を link_level で管理する
    // Heading にリンクをつける
    let mut link_level = 0;
    let parser = parser.flat_map(|event| match event {
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
            // println!("text is {}", text);
            let re = Regex::new(r"^https?://[^\s]*").unwrap();
            let text_str = text.to_string();
            if re.is_match(&text_str) {
                // println!("{}", text_str);
                return Left(
                    std::iter::once(Event::Start(Tag::Link(
                        LinkType::Autolink,
                        CowStr::from(text_str.to_owned()),
                        CowStr::from(text_str.to_owned()),
                    )))
                    .chain(std::iter::once(Event::Text(CowStr::from(
                        text_str.to_owned(),
                    ))))
                    .chain(std::iter::once(Event::End(Tag::Link(
                        LinkType::Autolink,
                        CowStr::from(text_str.to_owned()),
                        CowStr::from(text_str.to_owned()),
                    )))),
                );
            }
            Right(std::iter::once(Event::Text(text)))
        }
        _ => Right(std::iter::once(event)),
    });

    // Heading にリンクをつける
    let parser = parser.flat_map(|event| match event {
        // Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
        Event::Start(Tag::Heading(_, Some(id), _)) => {
            let url_path = format!("/pages?path={}#{}", path, id);
            return Left(
                std::iter::once(event)
                    .chain(std::iter::once(Event::Start(Tag::Link(
                        LinkType::Inline,
                        CowStr::from(url_path.to_owned()),
                        CowStr::from(url_path.to_owned()),
                    ))))
                    .chain(std::iter::once(Event::End(Tag::Link(
                        LinkType::Inline,
                        CowStr::from(url_path.to_owned()),
                        CowStr::from(url_path.to_owned()),
                    )))),
            );
        }
        _ => Right(std::iter::once(event)),
    });

    let mut html_buf = String::new();
    html::push_html(&mut html_buf, parser);
    // println!("parsed: {}", html_buf);

    // decode the path to obtain the title
    Ok(html_buf)
}
