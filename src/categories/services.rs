use super::*;
use crate::prelude::*;

pub async fn get_categories(claims: Claims) -> Result<Vec<schemas::CategoryInfo>> {
    let categories = curd::list_resources::<models::Category>().await?;

    let categories_info = categories
        .into_iter()
        .map(|category| -> Result<schemas::CategoryInfo> {
            Ok(schemas::CategoryInfo {
                id: category.id_key_str()?,
                name: category.name,
                you_are_maintainer: category.maintainer == claims.get_user_id(),
            })
        })
        .collect::<Result<Vec<schemas::CategoryInfo>>>()?;

    Ok(categories_info)
}

pub async fn get_category(claims: Claims, category_id: String) -> Result<schemas::CategoryInfo> {
    debug!("category_id: {}", category_id);
    let category = curd::get_resource::<models::Category>(category_id).await?;

    Ok(schemas::CategoryInfo {
        id: category.id_key_str()?,
        name: category.name,
        you_are_maintainer: category.maintainer == claims.get_user_id(),
    })
}

pub async fn create_category(category: schemas::CreateCategory) -> Result<schemas::CategoryInfo> {
    let category =
        curd::create_resource::<models::Category, schemas::CreateCategory>(category).await?;

    Ok(schemas::CategoryInfo {
        id: category.id_key_str()?,
        name: category.name,
        you_are_maintainer: true,
    })
}

pub async fn update_category(
    claims: Claims,
    category_id: String,
    category: schemas::UpdateCategory,
) -> Result<schemas::CategoryInfo> {
    let category = curd::update_resource::<models::Category, schemas::UpdateCategory>(
        claims,
        category_id,
        category,
    )
    .await?;

    Ok(schemas::CategoryInfo {
        id: category.id_key_str()?,
        name: category.name,
        you_are_maintainer: true,
    })
}
