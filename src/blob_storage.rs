use crate::blob_object::{BlobObject, BlobObjectReadError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

const OBJECT_DIR: &str = ".git/objects";
const NAME_LENGTH: usize = 40;

#[derive(Default)]
pub(crate) struct BlobStorage {
    objects: HashMap<String, (BlobObject, bool)>,
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
            self.objects
                .insert(object_name.to_string(), (object, false));
        }

        self.objects
            .get(object_name)
            .ok_or(GetBlobObjectError::ObjectNotFound)
            .map(|(object, _)| object)
    }

    pub(crate) fn add_object(
        &mut self,
        path: &Path,
    ) -> Result<(String, &BlobObject), AddBlobObjectError> {
        if !path.exists() {
            return Err(AddBlobObjectError::FileNotFound);
        }

        let file = fs::File::open(path).map_err(AddBlobObjectError::OpenFile)?;
        let mut reader = io::BufReader::new(file);
        let mut object =
            BlobObject::create(&mut reader).map_err(AddBlobObjectError::CreateBlobObject)?;
        let hash = object
            .hash()
            .ok_or(AddBlobObjectError::ObjectHash)?
            .to_string();
        self.objects.insert(hash.to_string(), (object, true));

        self.objects
            .get(&hash)
            .ok_or(AddBlobObjectError::Unexpected)
            .map(|(object, _)| (hash.to_string(), object))
    }

    pub(crate) fn save(&self) -> Result<(), io::Error> {
        for (hash, (object, _)) in &self.objects {
            let path = get_path(hash);
            if let Some(directory_path) = path.parent()
                && !directory_path.exists()
            {
                fs::create_dir_all(directory_path)?;
            }
            let mut file = fs::File::create(path)?;
            object.write(&mut file)?;
        }
        Ok(())
    }
}

fn read_object(object: &str) -> Result<BlobObject, GetBlobObjectError> {
    let path = get_path(object);
    if !path.exists() {
        return Err(GetBlobObjectError::ObjectNotFound);
    }
    let file = fs::File::open(path)?;
    let mut reader = io::BufReader::new(file);
    Ok(BlobObject::read(&mut reader)?)
}

fn get_path(object: &str) -> PathBuf {
    PathBuf::new()
        .join(OBJECT_DIR)
        .join(&object[..2])
        .join(&object[2..])
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

#[derive(Debug, thiserror::Error)]
pub(crate) enum AddBlobObjectError {
    #[error("File not found")]
    FileNotFound,
    #[error("Failed to open file: {0}")]
    OpenFile(io::Error),
    #[error("Failed to create blob object: {0}")]
    CreateBlobObject(io::Error),
    #[error("Failed to calculate object hash")]
    ObjectHash,
    #[error("Unexpected error")]
    Unexpected,
}
