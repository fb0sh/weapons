use super::*;
use crate::prelude::*;

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

    let token = create_jwt(user.id.to_string().as_str(), &user.role)?;

    let user_res = schemas::UserResponse {
        id: user.id.to_string(),
        username: user.username,
        created_at: user.created_at,
        updated_at: user.updated_at,
        email: user.email,
        token: token,
    };

    debug!("User logged in successfully: {:?}", user_res);

    Ok(user_res)
}
