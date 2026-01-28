use chrono::Utc;

use super::*;
use crate::{
    categories::{models::Category, schemas::CategoryInfo},
    prelude::*,
};

pub async fn get_categories() -> Result<Vec<schemas::CategoryInfo>> {
    let categories: Vec<models::Category> = DB.select(Category::table()).await?;

    debug!("Categories: {:?}", categories);

    let categories_info = categories
        .into_iter()
        .map(|category| -> Result<schemas::CategoryInfo> {
            Ok(schemas::CategoryInfo {
                id: category.id_str()?,
                name: category.name,
            })
        })
        .collect();

    categories_info
}

pub async fn create_category(
    claims: Claims,
    category: schemas::CreateCategory,
) -> Result<CategoryInfo> {
    let category: Option<Category> = DB
        .create(Category::table())
        .content(Category {
            id: None,
            name: category.name,
            maintainer: Some(claims.sub.parse()?),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
        .await?;

    let category = category.ok_or(Error::CreateResourceError(
        "Create Category faild".to_string(),
    ))?;

    Ok(schemas::CategoryInfo {
        id: category.id_str()?,
        name: category.name,
    })
}
