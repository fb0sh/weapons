CREATE TABLE notes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    maintainer_id  INTEGER NOT NULL,   -- 维护者
    title           TEXT NOT NULL,
    content         TEXT NOT NULL,
    category        TEXT,          -- 分类
    tags            TEXT,          -- JSON
    content_type    TEXT,          -- note/article/poc/...
    format          TEXT,          -- markdown/text/html
    content_length  INTEGER,       -- 字数
    ref_url         TEXT,          -- 参考链接
    authors         TEXT,          -- 作者 存放JSON
    source          TEXT,          -- 来源
    status          TEXT,       -- 状态 draft/done/verified
    read_scope    TEXT,      -- public/private/restricted
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE users (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    username        TEXT NOT NULL UNIQUE,   -- 登录名
    email           TEXT UNIQUE,
    password_hash   TEXT NOT NULL,           -- hash 后的密码
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- surreal users table
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD username ON users TYPE string ASSERT $value != "";
DEFINE FIELD email ON users TYPE option<string>;
DEFINE FIELD password_hash ON users TYPE string;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
DEFINE INDEX user_username_idx ON users COLUMNS username UNIQUE;

-- surreal notes table
DEFINE TABLE notes SCHEMAFULL;
DEFINE FIELD maintainer ON notes TYPE record(users);
DEFINE FIELD title ON notes TYPE string;
DEFINE FIELD content ON notes TYPE string;
DEFINE FIELD category ON notes TYPE option<string>;
DEFINE FIELD tags ON notes TYPE array<string>;
DEFINE FIELD content_type ON notes TYPE option<string>;   -- note / article / poc
DEFINE FIELD format ON notes TYPE option<string>;         -- markdown / text / html
DEFINE FIELD content_length ON notes TYPE option<int>;
DEFINE FIELD ref_url ON notes TYPE option<string>;
DEFINE FIELD source ON notes TYPE option<string>;
DEFINE FIELD authors ON notes TYPE array<string>;
DEFINE FIELD status ON notes TYPE string
    ASSERT $value IN ["draft", "done", "verified"]
    DEFAULT "draft";
DEFINE FIELD read_scope ON notes TYPE string
    ASSERT $value IN ["public", "private", "restricted"]
    DEFAULT "private";
DEFINE FIELD created_at ON notes TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON notes TYPE datetime DEFAULT time::now();


DEFINE TABLE notes SCHEMAFULL
PERMISSIONS
    FOR select
        WHERE
            -- public：任何人（包括匿名）
            read_scope = "public"

            -- restricted：只要登录就能看
            OR (
                read_scope = "restricted"
                AND $auth.id != NONE
            )

            -- private：只有维护者本人
            OR (
                read_scope = "private"
                AND maintainer = $auth.id
            )

    FOR create
        WHERE maintainer = $auth.id

    FOR update, delete
        WHERE maintainer = $auth.id
;
