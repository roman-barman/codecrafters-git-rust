use bytes::{Bytes, BytesMut};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::Digest;
use std::io::{Read, Write};

pub(crate) struct BlobObject {
    content: Bytes,
    hash: Option<String>,
}

impl BlobObject {
    pub(crate) fn read<T: Read>(reader: &mut T) -> Result<BlobObject, BlobObjectReadError> {
        let mut compressed_content = vec![];
        reader
            .read_to_end(&mut compressed_content)
            .map_err(BlobObjectReadError::FailedToRead)?;
        let mut decoder = ZlibDecoder::new(&compressed_content[..]);
        let mut content = BytesMut::with_capacity(compressed_content.len());
        decoder
            .read(&mut content)
            .map_err(BlobObjectReadError::FailedToDecompress)?;

        Ok(BlobObject {
            content: content.freeze(),
            hash: None,
        })
    }

    pub(crate) fn create<T: Read>(reader: &mut T) -> Result<BlobObject, std::io::Error> {
        let mut content = vec![];
        let len = reader.read_to_end(&mut content)?;

        let mut result = BytesMut::from(format!("blob {}\0", len).as_bytes());
        result.extend_from_slice(&content[..len]);
        let hash = calculate_hash(&result);

        Ok(BlobObject {
            content: result.freeze(),
            hash: Some(hash),
        })
    }

    pub(crate) fn content(&self) -> &[u8] {
        let start_content = match self.content.iter().position(|b| *b != 0) {
            Some(pos) => pos + 1,
            None => return &self.content,
        };
        &self.content[start_content..]
    }

    pub(crate) fn hash(&mut self) -> Option<&str> {
        if self.hash.is_none() {
            self.hash = Some(calculate_hash(&self.content));
        }
        self.hash.as_deref()
    }

    pub(crate) fn write<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        encoder.write_all(&self.content)?;
        encoder.finish()?;
        Ok(())
    }
}

fn calculate_hash(content: &[u8]) -> String {
    let hash = sha1::Sha1::digest(content);
    hex::encode(hash)
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum BlobObjectReadError {
    #[error("Failed to read object: {0}")]
    FailedToRead(std::io::Error),
    #[error("Failed to decompress object: {0}")]
    FailedToDecompress(std::io::Error),
}
