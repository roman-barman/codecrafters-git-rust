use crate::blob_object::{BlobObject, BlobObjectReadError};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

const OBJECT_DIR: &str = ".git/objects";
const NAME_LENGTH: usize = 40;

#[derive(Default)]
pub(crate) struct BlobStorage {
    objects: HashMap<String, BlobObject>,
}

impl BlobStorage {
    pub(crate) fn init_directory(&self) -> io::Result<()> {
        if !fs::exists(OBJECT_DIR)? {
            fs::create_dir(OBJECT_DIR)?;
        }

        Ok(())
    }

    pub(crate) fn get_object(
        &mut self,
        object_name: &str,
    ) -> Result<&BlobObject, GetBlobObjectError> {
        if object_name.len() != NAME_LENGTH {
            return Err(GetBlobObjectError::InvalidObjectName);
        }

        if !self.objects.contains_key(object_name) {
            let object = read_object(object_name)?;
            self.objects.insert(object_name.to_string(), object);
        }

        self.objects
            .get(object_name)
            .ok_or(GetBlobObjectError::ObjectNotFound)
    }
}

fn read_object(object: &str) -> Result<BlobObject, GetBlobObjectError> {
    let path = PathBuf::new()
        .join(OBJECT_DIR)
        .join(&object[..2])
        .join(&object[2..]);
    if !path.exists() {
        return Err(GetBlobObjectError::ObjectNotFound);
    }
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    Ok(BlobObject::read(&mut reader)?)
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum GetBlobObjectError {
    #[error("Invalid object name")]
    InvalidObjectName,
    #[error("Object not found")]
    ObjectNotFound,
    #[error("Failed to open file: {0}")]
    OpenFile(#[from] io::Error),
    #[error("Failed to read object: {0}")]
    ReadObject(#[from] BlobObjectReadError),
}
