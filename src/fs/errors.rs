use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsDatabaseError {
    #[error("Repository for collection {path} could not get created")]
    CollectionRespositoryError { path: PathBuf },

    #[error("Repository for collection {path} is missing")]
    CollectionRepoisitoryMissingError { path: PathBuf },

    #[error("Repository for collection {path} could not be downcast")]
    CollectionRepoisitoryDowncastError { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum FsRepositoryError {
    #[error("Failed to create directory: {path}")]
    DirectoryCreation { path: PathBuf },

    #[error("Failed to write file to: {path}")]
    FileCreation { path: PathBuf },

    #[error("Failed to delete file to: {path}")]
    FileDeletion { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum RecordHeaderError {
    #[error("Invalid magic: {magic}")]
    InvalidMagic { magic: u32 },

    #[error("Unsupported version: {version}")]
    UnsupportedVersion { version: u8 },

    #[error("Corruped Data: {offset}, {expected}, {actual}")]
    CorruptedData {
        offset: u64,
        expected: u32,
        actual: u32,
    },
}
