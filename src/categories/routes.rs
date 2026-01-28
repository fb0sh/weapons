use super::*;
use crate::prelude::*;

#[get("/categories")]
pub async fn get_categories(claims: Claims) -> Result<Json<Vec<schemas::CategoryInfo>>> {
    Ok(Json(services::get_categories(claims).await?))
}

#[get("/categories/{category_id}")]
pub async fn get_category(
    claims: Claims,
    category_id: Path<String>,
) -> Result<Json<schemas::CategoryInfo>> {
    Ok(Json(
        services::get_category(claims, category_id.into_inner()).await?,
    ))
}

#[post("/categories")]
pub async fn create_category(
    claims: Claims,
    category: Json<schemas::CreateCategory>,
) -> Result<Json<schemas::CategoryInfo>> {
    let mut category = category.into_inner();
    category.maintainer = claims.get_user_id();

    Ok(Json(services::create_category(category).await?))
}

#[put("/categories/{category_id}")]
pub async fn update_category(
    claims: Claims,
    category_id: Path<String>,
    category: Json<schemas::UpdateCategory>,
) -> Result<Json<schemas::CategoryInfo>> {
    Ok(Json(
        services::update_category(claims, category_id.into_inner(), category.into_inner()).await?,
    ))
}

#[delete("/categories/{category_id}")]
pub async fn delete_category(
    claims: Claims,
    category_id: Path<String>,
) -> Result<Json<models::Category>> {
    Ok(Json(
        curd::delete_resource::<models::Category>(claims, category_id.into_inner()).await?,
    ))
}
