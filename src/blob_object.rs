use bytes::{Bytes, BytesMut};
use flate2::read::ZlibDecoder;
use std::io::Read;

pub(crate) struct BlobObject {
    content: Bytes,
}

impl BlobObject {
    pub(crate) fn read<T: Read>(reader: &mut T) -> Result<BlobObject, BlobObjectReadError> {
        let mut compressed_content = BytesMut::new();
        reader
            .read(&mut compressed_content)
            .map_err(BlobObjectReadError::FailedToRead)?;
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
    #[error("Failed to read object: {0}")]
    FailedToRead(std::io::Error),
    #[error("Failed to decompress object: {0}")]
    FailedToDecompress(std::io::Error),
}
