-- surreal users table
DEFINE TABLE users SCHEMAFULL;
DEFINE FIELD username ON users TYPE string ASSERT $value != "";
DEFINE FIELD email ON users TYPE option<string>;
DEFINE FIELD password_hash ON users TYPE string;
DEFINE FIELD created_at ON users TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON users TYPE datetime DEFAULT time::now();
DEFINE INDEX user_username_idx ON users COLUMNS username UNIQUE;

-- categories table
DEFINE TABLE categories SCHEMAFULL;
DEFINE FIELD name ON categories TYPE string ASSERT $value != "";
DEFINE FIELD maintainer ON notes TYPE record(users);
DEFINE FIELD created_at ON categories TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON categories TYPE datetime DEFAULT time::now();
DEFINE INDEX category_name_idx ON categories COLUMNS name UNIQUE;

-- tags table
DEFINE TABLE tags SCHEMAFULL;
DEFINE FIELD name ON tags TYPE string ASSERT $value != "";
DEFINE FIELD maintainer ON notes TYPE record(users);
DEFINE FIELD created_at ON tags TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON tags TYPE datetime DEFAULT time::now();

-- surreal notes table
DEFINE TABLE notes SCHEMAFULL;
DEFINE FIELD maintainer ON notes TYPE record(users);
DEFINE FIELD category ON notes TYPE record(categories);
DEFINE FIELD title ON notes TYPE string;
DEFINE FIELD content ON notes TYPE string;
DEFINE FIELD tags ON notes TYPE array<record(tags)>;
DEFINE FIELD format ON notes TYPE option<string>
    ASSERT $value IN ["markdown", "text", "html"]
    DEFAULT "markdown";
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
