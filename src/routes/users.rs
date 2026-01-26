use crate::prelude::*;
use crate::{models::users::*, schemas::users::*, services::users::*};

use actix_web::{get, post, web::Json};

#[post("/users/register")]
pub async fn register_user_handler(reg_user: Json<RegisterUser>) -> Result<&'static str> {
    debug!("Registering user: {:?}", reg_user);

    info!("User ({}) registering", reg_user.username);

    register_user(reg_user.into_inner()).await?;

    Ok("User registered successfully")
}

#[post("/users/login")]
pub async fn login_user_handler(log_user: Json<LoginUser>) -> Result<Json<UserResponse>> {
    debug!("Logging in user: {:?}", log_user);

    info!("User ({}) logging in", log_user.username);

    let user_res = login_user(log_user.into_inner()).await?;

    Ok(Json(user_res))
}
