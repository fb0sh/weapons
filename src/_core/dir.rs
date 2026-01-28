use crate::_core::Error;

use super::error::Result;
use std::{path::PathBuf, sync::OnceLock};

static WEAPONS_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

pub async fn init_data_dir() -> Result<()> {
    let dir = std::env::var("WEAPONS_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"));

    WEAPONS_DATA_DIR
        .set(dir)
        .map_err(|_| Error::InternalServerError("DATA_DIR INIT FALID!".to_string()))?;

    std::fs::create_dir_all(WEAPONS_DATA_DIR.get().unwrap())
        .map_err(|e| Error::InternalServerError(format!("Failed to create data dir: {}", e)))?;

    Ok(())
}

pub fn data_dir() -> &'static PathBuf {
    WEAPONS_DATA_DIR
        .get()
        .expect("WEAPONS_DATA_DIR not initialized")
}
