use super::*;
use crate::prelude::*;

#[get("/categories")]
pub async fn get_categories() -> Result<Json<Vec<schemas::CategoryInfo>>> {
    // services::categories::get_categories().await
    unimplemented!()
}
