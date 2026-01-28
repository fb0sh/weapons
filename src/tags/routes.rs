use super::*;
use crate::prelude::*;

#[get("/tags")]
pub async fn get_tags() -> Result<Json<Vec<schemas::TagInfo>>> {
    Ok(Json(services::get_tags().await?))
}

#[post("/tags")]
pub async fn create_tag(
    claims: Claims,
    tag: Json<schemas::CreateTag>,
) -> Result<Json<schemas::TagInfo>> {
    Ok(Json(services::create_tag(claims, tag.into_inner()).await?))
}

#[put("/tags/<tag_id>")]
pub async fn update_tag(
    _: Claims,
    tag_id: String,
    tag: Json<schemas::UpdateTag>,
) -> Result<Json<schemas::TagInfo>> {
    Ok(Json(services::update_tag(tag_id, tag.into_inner()).await?))
}

#[delete("/tags/<tag_id>")]
pub async fn delete_tag(_: Claims, tag_id: String) -> Result<Json<()>> {
    Ok(Json(services::delete_tag(tag_id).await?))
}
