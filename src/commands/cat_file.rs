use crate::blob::{BlobStorage, GetBlobObjectError};

pub(crate) fn cat_file(storage: &mut BlobStorage, object: &str) -> Result<(), CatFileError> {
    let object = storage.get_object(object)?;
    print!("{}", std::str::from_utf8(object.content())?);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CatFileError {
    #[error("Failed to get object: {0}")]
    FailedToGetObject(#[from] GetBlobObjectError),
    #[error("Failed to decode object content as UTF-8")]
    DecodeError(#[from] std::str::Utf8Error),
}
