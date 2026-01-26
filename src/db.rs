use crate::prelude::Result;
use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db() -> Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("all").use_db("weapons").await?;

    Ok(())
}

pub async fn init_tables() -> Result<()> {
    let users_table_sql = r#"
        DEFINE TABLE users SCHEMAFULL;
        DEFINE FIELD username ON users TYPE string ASSERT $value != "";
        DEFINE FIELD email ON users TYPE option<string>;
        DEFINE FIELD password_hash ON users TYPE string;
        DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
        DEFINE INDEX user_username_idx ON users COLUMNS username UNIQUE;
    "#;

    let all_sql = vec![users_table_sql];

    for sql in all_sql {
        DB.query(sql).await?;
    }

    Ok(())
}
