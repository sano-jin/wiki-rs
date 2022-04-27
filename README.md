wiki.rs

A simple wiki created with Rust from scratch.

![Demo](/docs/wiki-rs-demo.png)

# API design

## Front

- 普通にアクセスして見る．
- 今見ているページの markdown を編集して，それでページを更新する．
  - edit button
- 新しいページの markdown を編集して，それでページを更新する．
  - create button

## Backend API

- GET `/page/xxxxxx`
  - html ページのレスポンスを返す
  - サーバ上のファイルから読み込む
- GET `/edit?path=<Path to the page">`
  - 編集用の markdown を返す
  - サーバ上のファイルから読み込む
- POST /edit
  - body: `{path:"Path to the page", body: "The updated markdown"}`
  - markdown を投げ，それで /xxxxxx.html を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- DELETE `/edit?path=<Path to the page>`
  - /xxxxxx.html を消去する
  - サーバ上のファイルは消去する

## 構成

# Development

## Prerequisties

Install Cargo

## Run

```sh
cargo run
```

and access <https://192.168.0.0.1:8443/> on your browser.
