use super::*;
use crate::prelude::*;

#[get("/categories")]
pub async fn get_categories(_: Claims) -> Result<Json<Vec<schemas::CategoryInfo>>> {
    Ok(Json(services::get_categories().await?))
}
