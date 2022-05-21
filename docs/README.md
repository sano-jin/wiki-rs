<!-- TITLE -->
<div align="center">
  <a href="https://github.com/sano-jin/wiki-rs">
    <img src="./images/logo.png" alt="Logo" width="120" height="120" />
  </a>

  <h3 align="center">wiki.rs</h3>
  <h1 align="center">Documentation</h1>

  <p align="center">
    A simple wiki created with rust
    <br />
    <br />
    <a href="https://github.com/sano-jin/wiki-rs/issues">Report Bug</a>
    ·
    <a href="https://github.com/sano-jin/wiki-rs/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#run">Run</a></li>
      </ul>
    </li>
    <li><a href="#design">Design</a></li>
    <ul>
      <li><a href="#api">API</a></li>
      <ul>
        <li><a href="#frontend">Frontend</a></li>
        <li><a href="#backend">Backend</a></li>
      </ul>
      <li><a href="#architecture">Architecture</a></li>
      <ul>
        <li><a href="#client-side">Client side</a></li>
        <li><a href="#server-side">Server side</a></li>
        <li><a href="#database">Database</a></li>
      </ul>
    </ul>
    <li><a href="#roadmap">Roadmap</a></li>
  </ol>
</details>

# Getting started

## Prerequisties

Install Cargo

## Run

1. TLS/SSL の証明書を発行し，`cert.pem`, `key.pem` という名前にして，
   プロジェクトのルートディレクトリに置く．
   TLS/SSL を有効にしないなら不要．

   - See <https://github.com/sano-jin/rust-hands-on-wiki/tree/master/https-server>

2. サンプルの db が docs 以下にあるので，それを持ってくる．

   ```sh
   cp -r docs/db .
   ```

3. 初期ユーザを追加する．
   ユーザ名が `foo`，パスワードが `bar` のユーザを追加したい場合は，
   `db/users` directory に
   ファイル名が `foo` で，以下のような JSON が書かれたファイルを配置する．

   ```json
   { "name": "foo", "password": "bar" }
   ```

   つまり，

   ```sh
   mkdir db/users
   echo '{"name":"foo","password":"bar"}' > db/users/foo
   ```

   を実行すれば良い．

4. `.env.template` を参考に，`.env` を生成する

   - 追記：これは現時点では不要

5. cargo で backend を実行する

   ```sh
   cargo run
   ```

   TLS/SSL を有効にしないなら，

   ```sh
   cargo run unsecure
   ```

   を実行する

   - 補足：cargo watch を install して，`cargo watch -x run` とすると，
     ソースコードを変更して保存するごとに，コンパイルして実行し直してくれるので便利です．

6. and access <https://127.0.0.1:8443/> on your browser.
   もし TLS/SSL を有効にしてなかったら，access <http://0.0.0.0:8080/> on your browser.

# Design

## API

随時追記する必要がある．

### Front

- 普通にアクセスして見る．
- 今見ているページの markdown を編集して，それでページを更新する．
  - edit button
- 新しいページの markdown を編集して，それでページを更新する．
  - create button

### Backend

- GET `/page?path=<Path to the page>`
  - `<Path to the page>` にある html ページを返す
  - サーバ上のファイルから読み込む
- GET `/edit?path=<Path to the page>`
  - 編集用の markdown を返す
  - サーバ上のファイルから読み込む
- POST `/edit {path:"<Path to the page>", body: "<The updated markdown>"}`
  - markdown を投げ，それで `<Path to the page>` を更新する
  - そのページがもともと存在しない場合は新しく作る．
  - サーバ上のファイルに書き出しておく
- DELETE `/edit?path=<Path to the page>`
  - `<Path to the page>` を消去する
  - サーバ上のファイルは消去する

## Architecture

### client-side

```sh
public/
├── assets/ # some assets files
│   ├── dark.css
│   ├── dracula.css
│   └── main.css
├── js/ # frontend javascript
│   ├── edit.js
│   ├── page.js
│   ├── top.js
│   └── user.js
└── layouts/ # some template html files
    ├── edit.html
    ├── menu.html
    ├── page.html
    ├── top.html
    └── user.html
```

### server-side

```sh
src
├── controllers/ # client と通信する POST API
│   ├── appstate.rs
│   ├── authenticate.rs
│   ├── handle_attach.rs
│   ├── handle_page.rs
│   ├── handle_user.rs
│   ├── index.rs
│   └── mod.rs
├── db/ # データベースもどき
│   ├── db_memory.rs
│   └── mod.rs
├── entities/ # markdown to html converter などの補助関数とか
│   ├── mod.rs
│   └── pages.rs
├── gateways/ # データベースとやりとりするためのコード
│   ├── attaches.rs
│   ├── db.rs
│   ├── mod.rs
│   ├── pages.rs
│   └── users.rs
├── lib.rs
├── main.rs
├── routes.rs
├── usecases/ # ページのデータ構造の定義とか
│   ├── mod.rs
│   ├── pages.rs
│   └── users.rs
└── util.rs
```

clean architecture を参考にしようとしています（が，全然理解できないのでヘルプです）．

- Enterprise business rules :: entities
  - markdown から page に変換したりするコードを置いておく
- application business rules :: use cases, interactor
  - page の型宣言をしておく
- interface adapters :: controllers, presenters, gateways
  - ページの更新や取得などを行うコードを書いておく
- frameworks and drivers :: web, ui, external interfaces, devices, db

参考資料：

- https://qiita.com/nrslib/items/a5f902c4defc83bd46b8#%E7%9F%A2%E5%8D%B0%E3%81%AE%E6%96%B9%E5%90%91
- https://nrslib.com/clean-architecture-with-java/#outline__6_3
- https://gist.github.com/mpppk/609d592f25cab9312654b39f1b357c60
- https://nrslib.com/clean-architecture-with-java/#outline__6_3

### Database

現在ファイルシステムをそのまま活用し，json ファイルを読み書きしている．

```sh
db/
├── attach/ # 添付ファイル
├── pages/ # ページ
│   ├── menu # サイドメニュー
│   └── top # トップページ
└── users/ # ユーザ
```

# Roadmap

- [ ] clean architecture などをもとにアーキテクチャを再考する．
- [ ] issue に挙げたものの解決
- [ ] ユーザごとのプライベートなページ
- [ ] github, youtube などの外部サイトとの連携
  - ソースコードや動画の埋め込み表示
