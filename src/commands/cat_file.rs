use crate::commands::OBJECT_DIR;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;

pub(crate) fn cat_file(hash: &str) -> Result<(), CatFileError> {
    let dir_name = &hash[..2];
    let file_name = &hash[2..];

    let bytes = fs::read(format!("./{}/{}/{}", OBJECT_DIR, dir_name, file_name)).map_err(|e| {
        CatFileError::FailedToRead {
            object: hash.to_string(),
            source: e,
        }
    })?;

    let mut decoder = ZlibDecoder::new(&bytes[..]);
    let mut content = String::new();
    decoder
        .read_to_string(&mut content)
        .map_err(|e| CatFileError::FailedToUnzip {
            object: hash.to_string(),
            source: e,
        })?;
    let content_start = match content.find("\0") {
        Some(start) => start + 1,
        None => 0,
    };

    print!("{}", &content.as_str()[content_start..]);
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum CatFileError {
    #[error("Failed to read object {object}: {source}")]
    FailedToRead {
        object: String,
        source: std::io::Error,
    },
    #[error("Failed to unzip object {object}: {source}")]
    FailedToUnzip {
        object: String,
        source: std::io::Error,
    },
}
