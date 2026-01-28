use chrono::Utc;

use super::*;
use crate::prelude::*;

pub async fn get_tags(claims: Claims) -> Result<Vec<schemas::TagInfo>> {
    let tags = curd::list_resources::<models::Tag>().await?;

    debug!("Tags: {:?}", tags);

    let tags_info = tags
        .into_iter()
        .map(|tag| -> Result<schemas::TagInfo> {
            Ok(schemas::TagInfo {
                id: tag.id_key_str()?,
                name: tag.name,
                you_are_maintainer: tag.maintainer == claims.get_user_id(),
            })
        })
        .collect();

    tags_info
}

pub async fn get_tag(claims: Claims, tag_id: String) -> Result<schemas::TagInfo> {
    let tag = curd::get_resource::<models::Tag>(tag_id).await?;

    Ok(schemas::TagInfo {
        id: tag.id_key_str()?,
        name: tag.name,
        you_are_maintainer: tag.maintainer == claims.get_user_id(),
    })
}

pub async fn create_tag(tag: schemas::CreateTag) -> Result<schemas::TagInfo> {
    let tag = curd::create_resource::<models::Tag, schemas::CreateTag>(tag).await?;

    Ok(schemas::TagInfo {
        id: tag.id_key_str()?,
        name: tag.name,
        you_are_maintainer: true,
    })
}

pub async fn update_tag(
    claims: Claims,
    tag_id: String,
    tag: schemas::UpdateTag,
) -> Result<schemas::TagInfo> {
    let tag = curd::update_resource::<models::Tag, schemas::UpdateTag>(claims, tag_id, tag).await?;

    Ok(schemas::TagInfo {
        id: tag.id_key_str()?,
        name: tag.name,
        you_are_maintainer: true,
    })
}
