use bytes::{Bytes, BytesMut};
use flate2::read::ZlibDecoder;
use memmap2::Mmap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const NAME_LENGTH: usize = 40;

pub(crate) struct BlobObject {
    content: Bytes,
}

impl BlobObject {
    pub(crate) fn try_read(
        object_storage: &Path,
        object: &str,
    ) -> Result<BlobObject, BlobObjectReadError> {
        if object.len() != NAME_LENGTH {
            return Err(BlobObjectReadError::InvalidObjectName);
        }

        let dir_name = &object[..2];
        let file_name = &object[2..];
        let path = object_storage.join(dir_name).join(file_name);

        if !path.exists() {
            return Err(BlobObjectReadError::ObjectNotFound);
        }

        let file = File::open(path).map_err(BlobObjectReadError::FailedToRead)?;
        let mmap = unsafe { Mmap::map(&file) }.map_err(BlobObjectReadError::FailedToRead)?;
        let compressed_content = Bytes::from_owner(mmap);
        let mut decoder = ZlibDecoder::new(&compressed_content[..]);
        let mut content = BytesMut::with_capacity(compressed_content.len());
        decoder
            .read(&mut content)
            .map_err(BlobObjectReadError::FailedToDecompress)?;

        Ok(BlobObject {
            content: content.freeze(),
        })
    }

    pub(crate) fn content(&self) -> &[u8] {
        let start_content = match self.content.iter().position(|b| *b != 0) {
            Some(pos) => pos + 1,
            None => return &self.content,
        };
        &self.content[start_content..]
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BlobObjectReadError {
    #[error("Invalid object name")]
    InvalidObjectName,
    #[error("Object not found")]
    ObjectNotFound,
    #[error("Failed to read object: {0}")]
    FailedToRead(std::io::Error),
    #[error("Failed to decompress object: {0}")]
    FailedToDecompress(std::io::Error),
}
