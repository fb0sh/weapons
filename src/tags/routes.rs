use super::*;
use crate::prelude::*;

#[get("/tags")]
pub async fn get_tags(claims: Claims) -> Result<Json<Vec<schemas::TagInfo>>> {
    Ok(Json(services::get_tags(claims).await?))
}

#[get("/tags/{tag_id}")]
pub async fn get_tag(claims: Claims, tag_id: Path<String>) -> Result<Json<schemas::TagInfo>> {
    Ok(Json(services::get_tag(claims, tag_id.into_inner()).await?))
}

#[post("/tags")]
pub async fn create_tag(
    claims: Claims,
    tag: Json<schemas::CreateTag>,
) -> Result<Json<schemas::TagInfo>> {
    let mut tag = tag.into_inner();
    tag.maintainer = claims.get_user_id();

    Ok(Json(services::create_tag(tag).await?))
}

#[put("/tags/{tag_id}")]
pub async fn update_tag(
    claims: Claims,
    tag_id: Path<String>,
    tag: Json<schemas::UpdateTag>,
) -> Result<Json<schemas::TagInfo>> {
    Ok(Json(
        services::update_tag(claims, tag_id.into_inner(), tag.into_inner()).await?,
    ))
}

#[delete("/tags/{tag_id}")]
pub async fn delete_tag(claims: Claims, tag_id: Path<String>) -> Result<Json<models::Tag>> {
    Ok(Json(
        curd::delete_resource::<models::Tag>(claims, tag_id.into_inner()).await?,
    ))
}
