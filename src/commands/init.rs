use crate::commands::OBJECT_DIR;
use std::fs;

pub(crate) fn init() -> Result<(), InitError> {
    fs::create_dir(".git").map_err(|e| InitError::FailedToCreateDir {
        dir: ".git".to_string(),
        err: e,
    })?;
    fs::create_dir(OBJECT_DIR).map_err(|e| InitError::FailedToCreateDir {
        dir: OBJECT_DIR.to_string(),
        err: e,
    })?;
    fs::create_dir(".git/refs").map_err(|e| InitError::FailedToCreateDir {
        dir: ".git/refs".to_string(),
        err: e,
    })?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").map_err(|e| InitError::FailedToWriteFile {
        file: ".git/HEAD".to_string(),
        err: e,
    })?;
    println!("Initialized git directory");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum InitError {
    #[error("Failed to create directory {dir}: {err}")]
    FailedToCreateDir { dir: String, err: std::io::Error },
    #[error("Failed to write file {file}: {err}")]
    FailedToWriteFile { file: String, err: std::io::Error },
}
