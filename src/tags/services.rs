use chrono::Utc;

use super::*;
use crate::prelude::*;

pub async fn get_tags() -> Result<Vec<schemas::TagInfo>> {
    let tags: Vec<models::Tag> = DB.select(models::Tag::table()).await?;

    debug!("Tags: {:?}", tags);

    let tags_info = tags
        .into_iter()
        .map(|tag| -> Result<schemas::TagInfo> {
            Ok(schemas::TagInfo {
                id: tag.id_str()?,
                name: tag.name,
            })
        })
        .collect();

    tags_info
}

pub async fn create_tag(claims: Claims, tag: schemas::CreateTag) -> Result<schemas::TagInfo> {
    let tag: Option<models::Tag> = DB
        .create(models::Tag::table())
        .content(models::Tag {
            id: None,
            maintainer: Some(claims.sub.parse()?),
            name: tag.name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await?;

    let tag = tag.ok_or(Error::CreateResourceError("Create Tag faild".to_string()))?;

    Ok(schemas::TagInfo {
        id: tag.id_str()?,
        name: tag.name,
    })
}

pub async fn update_tag(tag_id: String, tag: schemas::UpdateTag) -> Result<schemas::TagInfo> {
    let tag: Option<models::Tag> = DB.update((models::Tag::table(), tag_id)).merge(tag).await?;

    let tag = tag.ok_or(Error::UpdateResourceError("Update Tag faild".to_string()))?;

    Ok(schemas::TagInfo {
        id: tag.id_str()?,
        name: tag.name,
    })
}

pub async fn delete_tag(tag_id: String) -> Result<()> {
    let deleted_tag: Option<models::Tag> = DB.delete((models::Tag::table(), tag_id)).await?;

    match deleted_tag {
        Some(tag) => {
            debug!("Tag deleted: {:?}", tag);
            Ok(())
        }
        None => Err(Error::ResourceNotFound(
            "deleted_tag not found; pls check id".to_string(),
        )),
    }
}
