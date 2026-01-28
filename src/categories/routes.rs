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

#[put("/categories/<category_id>")]
pub async fn update_category(
    _: Claims,
    category_id: String,
    category: Json<schemas::UpdateCategory>,
) -> Result<Json<schemas::CategoryInfo>> {
    Ok(Json(
        services::update_category(category_id, category.into_inner()).await?,
    ))
}

#[delete("/categories/<category_id>")]
pub async fn delete_category(_: Claims, category_id: String) -> Result<Json<()>> {
    Ok(Json(services::delete_category(category_id).await?))
}
