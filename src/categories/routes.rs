use super::*;
use crate::prelude::*;

#[get("/categories")]
pub async fn get_categories(_: Claims) -> Result<Json<Vec<schemas::CategoryInfo>>> {
    Ok(Json(services::get_categories().await?))
}

#[post("/categories")]
pub async fn create_category(
    claims: Claims,
    category: Json<schemas::CreateCategory>,
) -> Result<Json<schemas::CategoryInfo>> {
    Ok(Json(
        services::create_category(claims, category.into_inner()).await?,
    ))
}
