use super::*;
use crate::{prelude::*, users::models::User};

pub async fn register_user(reg_user: schemas::RegisterUser) -> Result<()> {
    debug!("Registering user: {:?}", reg_user);

    let schemas::RegisterUser {
        username,
        email,
        password,
    } = reg_user;

    let query = r#"
        CREATE users SET
            username      = $username,
            email         = $email,
            password_hash = crypto::argon2::generate($password),
            created_at    = time::now(),
            updated_at    = time::now();
    "#;

    trace!("Query: {}", query);

    let res = DB
        .query(query)
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", password))
        .await?;

    debug!("User registered: {:?}", res);

    Ok(())
}

pub async fn login_user(login_user: schemas::LoginUser) -> Result<schemas::UserResponse> {
    debug!("Logging in user: {:?}", login_user);

    let schemas::LoginUser { username, password } = login_user;

    let query = r#"
        SELECT * FROM users
        WHERE username = $username
        AND crypto::argon2::compare(password_hash, $password);
    "#;

    trace!("Query: {}", query);

    let user: Option<models::User> = DB
        .query(query)
        .bind(("username", username))
        .bind(("password", password))
        .await?
        .take(0)?;

    let user = user.ok_or(Error::AuthError)?;

    let token = create_jwt(user.id_str()?.as_str(), &user.role)?;

    let user_res = schemas::UserResponse {
        id: user.id_str()?,
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
        email: user.email,
        token: token,
    };

    debug!("User logged in successfully: {:?}", user_res);

    Ok(user_res)
}

pub async fn delete_user(user_id: &str) -> Result<()> {
    let deleted_user: Option<User> = DB.delete((User::table(), user_id)).await?;

    match deleted_user {
        Some(user) => {
            debug!("User deleted: {:?}", user);
            Ok(())
        }
        None => Err(Error::ResourceNotFound(
            "deleted_user not found; pls check id".to_string(),
        )),
    }
}
