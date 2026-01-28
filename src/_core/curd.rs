use super::db::{DB, SurrealModel};
use super::error::{Error, Result};
use super::jwt::Claims;
use surrealdb::RecordId;
use tracing::debug;

#[async_trait::async_trait]
pub trait OwnableResource: Sized + serde::de::DeserializeOwned + serde::Serialize {
    /// 获取资源拥有者
    fn owner_id(&self) -> Option<&RecordId>;
}

pub async fn list_resources<R>() -> Result<Vec<R>>
where
    R: OwnableResource + SurrealModel + serde::Serialize + std::fmt::Debug,
{
    debug!(
        "Listing resources: ===================={}====================",
        R::table()
    );

    let items: Vec<R> = DB.select(R::table()).await?;
    debug!("Items: {:?}", items);
    debug!(
        "END: Listing resources: ===================={}====================",
        R::table()
    );
    Ok(items)
}

pub async fn get_resource<R>(resource_id: String) -> Result<R>
where
    R: OwnableResource + SurrealModel + serde::Serialize + std::fmt::Debug,
{
    debug!(
        "Getting resource: ===================={}====================",
        R::table()
    );
    debug!("Resource ID: {}", resource_id);
    let record_id = RecordId::from_table_key(R::table(), resource_id);
    let item: Option<R> = DB.select(record_id).await?;

    debug!("Item: {:?}", item);

    let item = item.ok_or_else(|| Error::ResourceNotFound("Not found".into()))?;
    debug!(
        "END: Getting resource: ===================={}====================",
        R::table()
    );
    Ok(item)
}

pub async fn create_resource<R, C>(content: C) -> Result<R>
where
    R: OwnableResource
        + SurrealModel
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    C: serde::Serialize + Send + 'static,
{
    debug!(
        "Creating resource: ===================={}====================",
        R::table()
    );

    let item: Option<R> = DB.create(R::table()).content(content).await?;

    debug!("Created resource: {:?}", item);

    let item = item.ok_or_else(|| Error::CreateResourceError("Create fail".into()))?;

    debug!(
        "END: Creating resource: ===================={}====================",
        R::table()
    );
    Ok(item)
}

pub async fn update_resource<R, U>(claims: Claims, resource_id: String, update: U) -> Result<R>
where
    R: OwnableResource
        + SurrealModel
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
    U: serde::Serialize + Send + 'static,
{
    debug!(
        "Updating resource: ===================={}====================",
        R::table()
    );
    debug!("Resource ID: {}", resource_id);

    // 先查询记录
    let record_id = RecordId::from_table_key(R::table(), resource_id);
    let existing: Option<R> = DB.select(&record_id).await?;
    let existing = existing.ok_or_else(|| Error::ResourceNotFound("Not found".into()))?;

    // 权限检查
    let is_owner = existing.owner_id() == claims.get_user_id().as_ref();
    if !is_owner && !claims.is_manager().is_ok() {
        return Err(Error::PermissionError);
    }

    let updated: Option<R> = DB.update(record_id).merge(update).await?;

    debug!("Updated resource: {:?}", updated);

    let updated = updated.ok_or_else(|| Error::UpdateResourceError("Update failed".into()))?;

    debug!(
        "END: Updating resource: ===================={}====================",
        R::table()
    );
    Ok(updated)
}

pub async fn delete_resource<R>(claims: Claims, resource_id: String) -> Result<R>
where
    R: OwnableResource
        + SurrealModel
        + serde::Serialize
        + serde::de::DeserializeOwned
        + std::fmt::Debug,
{
    debug!(
        "Deleting resource: ===================={}====================",
        R::table()
    );
    debug!("Resource ID: {}", resource_id);
    // 先查询记录
    let record_id = RecordId::from_table_key(R::table(), resource_id);
    let existing: Option<R> = DB.select(&record_id).await?;
    let existing = existing.ok_or_else(|| Error::ResourceNotFound("Not found".into()))?;

    // 权限检查
    let is_owner = existing.owner_id() == claims.get_user_id().as_ref();
    let is_manager = claims.is_manager().is_ok();

    if !is_owner && !is_manager {
        return Err(Error::PermissionError);
    }

    let deleted: Option<R> = DB.delete(record_id).await?;

    debug!("Deleted resource: {:?}", deleted);

    let deleted = deleted.ok_or_else(|| Error::DeleteResourceError("Delete failed".into()))?;

    debug!(
        "END: Deleting resource: ===================={}====================",
        R::table()
    );
    Ok(deleted)
}
