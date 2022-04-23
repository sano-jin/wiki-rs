# wiki.rs

## How to create a own wiki from scrarch

1.  update rust

    ```sh
    rustup update
    ```

2.  create a new project

    ```sh
    cargo new wiki-rs
    cd wiki.rs
    ```

3.  See <https://github.com/actix/examples/tree/master/https-tls/openssl>
    copy `Cargo.toml` and `src/main.rs` and
    follow the instructions on README.md

    1.  use local CA

        ```sh
        mkcert -install
        ```

    2.  generate own cert/private key

        ```sh
        mkcert -install
        ```

        rename the `127.0.0.1-key.pem` file with `key.pem` and
        the `127.0.0.1.pem` file with `cert.pem`.

hoge
