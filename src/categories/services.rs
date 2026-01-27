use super::*;
use crate::prelude::*;

pub async fn get_categories() -> Result<Vec<schemas::CategoryInfo>> {
    let categories: Vec<models::Category> = DB.select(CATEGORIES).await?;

    Ok(categories
        .into_iter()
        .map(|category| schemas::CategoryInfo {
            id: category.id.to_string(),
            name: category.name,
        })
        .collect())
}
