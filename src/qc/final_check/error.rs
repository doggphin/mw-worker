use glob::{GlobError, PatternError};

use super::media_file::error::MediaFileParseError;

#[derive(Debug)]
pub enum FCError {
    DeserializeError(serde_json::Error),
    InsufficientGroupNumberPrecision(u64, u64),
    GroupNumberPrecisionTooHigh(u64),
    InvalidRequest(String),

    InvalidDirectory(PatternError),
    InvalidFile(GlobError),

    FileNameParsingError(std::path::PathBuf, MediaFileParseError),
    //IncorrectMetadata(MediaFile, MetadataCheckError),

    Todo
}
impl std::error::Error for FCError {}
impl std::fmt::Display for FCError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FCError::DeserializeError(err) => write!(f, "could not deserialize request: {err}"),
            FCError::InsufficientGroupNumberPrecision(number, precision) => write!(f, "requested a group number {number} with only {precision} digits of precision"),
            FCError::GroupNumberPrecisionTooHigh(precision) => write!(f, "group number precision of {precision} is higher than the maximum 6"),
            FCError::InvalidRequest(err) => write!(f, "invalid request: {err}"),
            FCError::InvalidDirectory(err) => write!(f, "invalid directory: {err}"),
            FCError::InvalidFile(err) => write!(f, "invalid file path: {err}"),
            FCError::FileNameParsingError(path, err, ) => write!(f, "error parsing \"{}\"'s file name: {err}", path.to_str().unwrap_or("invalid path")),
            //FCError::IncorrectMetadata(media_file, err) => write!(f, "incorrect metadata found while checking \"{}\": {err}", media_file.path.to_string_lossy()),
            FCError::Todo => write!(f, "todo")

        }
    }
}