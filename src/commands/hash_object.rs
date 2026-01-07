use crate::blob::{AddBlobObjectError, BlobStorage};
use std::path::Path;

pub(crate) fn hash_object(
    storage: &mut BlobStorage,
    path: &Path,
    is_write: bool,
) -> Result<(), HashObjectError> {
    let (hash, _) = storage.add_object(path)?;

    if is_write {
        storage.save()?;
    }

    println!("{}", hash);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum HashObjectError {
    #[error("Failed to add object: {0}")]
    AddObject(#[from] AddBlobObjectError),
    #[error("Failed to save objects: {0}")]
    Save(#[from] std::io::Error),
}
