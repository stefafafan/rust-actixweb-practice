# rust-actixweb-practice

Personal practice repository of Rust, actix-web, sqlx, etc.

## How to run this

Install sqlx-cli

```sh
$ cargo install --version=0.1.0-beta.1 sqlx-cli
```

export DATABASE_URL

```sh
export DATABASE_URL="mysql://root:password@localhost/practice"
```

Apply migration

```sh
sqlx migrate run
```

Run application

```sh
$ cargo run
```

Access browser.

- http://localhost:8088/
- http://localhost:8088/users/show
- http://localhost:8088/users/show/{\d+}
