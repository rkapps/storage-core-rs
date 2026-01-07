use std::fmt::Display;
use std::path::{Path, PathBuf};

/// Creates a JSON filename from an ID
pub fn json_filename<T: Display>(id: T) -> String {
    format!("{}.json", id)
}

/// Builds a full path to a record file
pub fn build_json_file_path<T: Display>(base: &Path, id: T) -> PathBuf {
    base.join(json_filename(id))
}