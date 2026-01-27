use crate::prelude::Result;
use std::sync::LazyLock;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

pub async fn init_db() -> Result<()> {
    DB.connect("rocksdb:./data/surreal_rocks.db").await?;

    DB.use_ns("all").use_db("weapons").await?;

    Ok(())
}

pub async fn init_tables() -> Result<()> {
    let users_table_sql = r#"
        DEFINE TABLE users SCHEMAFULL;
        DEFINE FIELD username ON users TYPE string ASSERT $value != "";
        DEFINE FIELD role ON users TYPE string ASSERT $value in ["maintainer", "manager"] DEFAULT "maintainer";
        DEFINE FIELD email ON users TYPE option<string>;
        DEFINE FIELD password_hash ON users TYPE string;
        DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();

        DEFINE INDEX user_username_idx ON users COLUMNS username UNIQUE;
        DEFINE INDEX user_email_idx ON users COLUMNS email UNIQUE;

        -- manager 可以查询所有用户
        -- maintainer 只能查询自己的信息
        -- 创建权限：所有人都可以创建
        -- 更新权限：用户本人或manager
        -- 删除权限：用户本人或manager
    "#;

    let categories_table_sql = r#"
        DEFINE TABLE categories SCHEMAFULL;
        DEFINE FIELD name ON categories TYPE string ASSERT $value != "";
        DEFINE FIELD maintainer ON categories TYPE option<record<users>>;
        DEFINE FIELD created_at ON categories TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON categories TYPE datetime DEFAULT time::now();

        DEFINE INDEX category_name_idx ON categories COLUMNS name UNIQUE;

        -- 查询权限：所有人都可以拉取下拉列表
        -- 创建权限：任何登录用户可以创建
        -- 更新权限：维护者本人或manager
        -- 删除权限：维护者本人或manager
        INSERT INTO categories [
                { id: categories:dict, name: "字典", maintainer: NONE },
                { id: categories:note, name: "笔记", maintainer: NONE }
        ] ON DUPLICATE KEY UPDATE name = name;
    "#;

    let tags_table_sql = r#"
        DEFINE TABLE tags SCHEMAFULL;
        DEFINE FIELD name ON tags TYPE string ASSERT $value != "";
        DEFINE FIELD maintainer ON tags TYPE option<record<users>>;
        DEFINE FIELD created_at ON tags TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON tags TYPE datetime DEFAULT time::now();

        DEFINE INDEX tag_name_idx ON tags COLUMNS name UNIQUE;

        -- 查询权限：所有人都可以拉取下拉列表
        -- 创建权限：任何登录用户可以创建
        -- 更新权限：维护者本人或manager
        -- 删除权限：维护者本人或manager
        INSERT INTO tags [
            { id: tags:rust, name: "Rust", maintainer: NONE },
            { id: tags:python, name: "Python", maintainer: NONE },
            { id: tags:javascript, name: "JavaScript", maintainer: NONE },
            { id: tags:web, name: "Web", maintainer: NONE }
        ] ON DUPLICATE KEY UPDATE name = name;
    "#;

    let items_table_sql = r#"
        -- surreal items table
        DEFINE TABLE items SCHEMAFULL;
        DEFINE FIELD maintainer ON items TYPE option<record<users>>;
        DEFINE FIELD category ON items TYPE option<record<categories>>
            DEFAULT categories:note;
        DEFINE FIELD title ON items TYPE string;
        DEFINE FIELD content ON items TYPE string;
        DEFINE FIELD tags ON items TYPE array<record<tags>>;
        DEFINE FIELD format ON items TYPE option<string>
            ASSERT $value IN ["markdown", "text", "html"]
            DEFAULT "markdown";
        DEFINE FIELD content_length ON items TYPE option<int>;
        DEFINE FIELD ref_url ON items TYPE option<string>;
        DEFINE FIELD source ON items TYPE option<string>;
        DEFINE FIELD authors ON items TYPE array<string>;
        DEFINE FIELD status ON items TYPE string
            ASSERT $value IN ["draft", "done", "verified"]
            DEFAULT "draft";
        DEFINE FIELD read_scope ON items TYPE string
            ASSERT $value IN ["public", "private", "restricted"]
            DEFAULT "private";
        DEFINE FIELD created_at ON items TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON items TYPE datetime DEFAULT time::now();

        DEFINE INDEX items_maintainer_idx ON items COLUMNS maintainer;
        DEFINE INDEX items_category_idx ON items COLUMNS category;
        DEFINE INDEX items_tags_idx ON items COLUMNS tags;
        DEFINE INDEX items_status_idx ON items COLUMNS status;
        DEFINE INDEX items_read_scope_idx ON items COLUMNS read_scope;

        -- 查询权限：根据read_scope决定
        -- 创建权限：任何登录用户可以创建
        -- 更新权限：维护者本人或manager
        -- 删除权限：维护者本人或manager
        "#;

    let all_sql = vec![
        users_table_sql,
        categories_table_sql,
        tags_table_sql,
        items_table_sql,
    ];

    for sql in all_sql {
        DB.query(sql).await?;
    }

    Ok(())
}
