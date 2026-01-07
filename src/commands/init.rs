use crate::blob::BlobStorage;
use std::fs;

pub(crate) fn init(storage: &BlobStorage) -> Result<(), InitError> {
    fs::create_dir(".git").map_err(|e| InitError::CreateDir {
        dir: ".git".to_string(),
        err: e,
    })?;
    storage
        .init_directory()
        .map_err(InitError::InitBlobStorage)?;
    fs::create_dir(".git/refs").map_err(|e| InitError::CreateDir {
        dir: ".git/refs".to_string(),
        err: e,
    })?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").map_err(|e| InitError::WriteFile {
        file: ".git/HEAD".to_string(),
        err: e,
    })?;
    println!("Initialized git directory");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum InitError {
    #[error("Failed to create directory {dir}: {err}")]
    CreateDir { dir: String, err: std::io::Error },
    #[error("Failed to write file {file}: {err}")]
    WriteFile { file: String, err: std::io::Error },
    #[error("Failed to initialize blob storage: {0}")]
    InitBlobStorage(std::io::Error),
}
