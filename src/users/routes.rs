use super::*;
use crate::prelude::*;

#[post("/users/register")]
pub async fn register_user_handler(reg_user: Json<schemas::RegisterUser>) -> Result<&'static str> {
    debug!("Registering user: {:?}", reg_user);

    info!("User ({}) registering", reg_user.username);

    services::register_user(reg_user.into_inner()).await?;

    Ok("User registered successfully")
}

#[post("/users/login")]
pub async fn login_user_handler(
    log_user: Json<schemas::LoginUser>,
) -> Result<Json<schemas::UserResponse>> {
    debug!("Logging in user: {:?}", log_user);

    info!("User ({}) logging in", log_user.username);

    let user_res = services::login_user(log_user.into_inner()).await?;

    Ok(Json(user_res))
}

#[delete("/users/{id}")]
pub async fn delete_user_handler(id: Path<String>, claims: Claims) -> Result<&'static str> {
    debug!("Deleting user: {:?}", id);

    claims.is_manager()?;

    info!("User ({}) deleting", claims.sub);

    services::delete_user(id.to_string().as_str()).await?;

    Ok("User deleted successfully")
}
