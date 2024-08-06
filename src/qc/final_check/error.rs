use glob::{GlobError, PatternError};

use super::{media_file::{error::MediaFileParseError, MediaFile}, media_groups::error::MediaGroupsError};

#[derive(Debug)]
pub enum FCError {
    DeserializeError(serde_json::Error),
    InsufficientGroupNumberPrecision(u64, u64),
    GroupNumberPrecisionTooHigh(u64),
    InvalidRequest(String),
    InvalidDirectory(PatternError),
    InvalidFile(GlobError),
    MediaGroupingError(MediaGroupsError),
    MediaFileParseError(std::path::PathBuf, MediaFileParseError),
    IncorrectMediaCount(MediaGroupsError),
    IncorrectMetadataError(MediaGroupsError)
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
            FCError::MediaGroupingError(e) => write!(f, "error grouping media files: {e}"),
            FCError::MediaFileParseError(path, e) => write!(f, "error parsing {}: {e}", path.to_string_lossy()),
            FCError::IncorrectMediaCount(e) => write!(f, "incorrect media count: {e}"),
            FCError::IncorrectMetadataError(e) => write!(f, "media file had incorrect metadata: {e}")
        }
    }
}