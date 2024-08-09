use glob::{GlobError, PatternError};

use crate::utils::types::{file_extension_type::FileExtensionType, media_types::MediaType};

use super::{media_file::{error::MediaFileParseError, MediaFile}, media_groups::error::MediaGroupsError};

#[derive(Debug)]
pub enum FCError {
    DeserializeError(serde_json::Error),
    InvalidRequest(String),
    InsufficientGroupNumberPrecision(u64, u64),
    GroupNumberPrecisionTooHigh(u64),
    InvalidDirectory(PatternError),
    NoFilesInDirectory(String),
    InvalidFile(GlobError),
    MediaGroupingError(MediaGroupsError),
    MediaFileParseError(std::path::PathBuf, MediaFileParseError),
    IncorrectMediaCount(MediaGroupsError),
    IncompatibleFileExtension(MediaType, FileExtensionType, MediaFile),
    OutOfPlaceMediaType(MediaType),
    IncorrectLastName(String, String, MediaFile),
    IncorrectFirstInitial(char, char, MediaFile),
    IncorrectDpi(u64, u64, MediaFile),
    NotCorrected(MediaFile),
    MissingGroupNumber(u64, MediaFile),
    IncorrectGroupNumber(u64, u64, MediaFile),
    MissingGroupChar(char, MediaFile),
    IncorrectGroupChar(char, char, MediaFile),
    IncorrectGroupNumberPrecision(u64, u64, MediaFile),
    RepeatedIndexNumber(u32, String, String),
    IncorrectIndexNumberPrecision(u64, u64, MediaFile),
    FolderSkippedIndexNumber(u32),
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
            FCError::NoFilesInDirectory(dir) => write!(f, "no files could be found in/at {dir}"),
            FCError::InvalidFile(err) => write!(f, "invalid file path: {err}"),
            FCError::MediaGroupingError(e) => write!(f, "error grouping media files: {e}"),
            FCError::MediaFileParseError(path, e) => write!(f, "error parsing {}: {e}", path.file_name().unwrap_or(std::ffi::OsStr::new("invalid file name")).to_string_lossy()),
            FCError::IncorrectMediaCount(e) => write!(f, "incorrect media count: {e}"),
            FCError::IncompatibleFileExtension(media_type, file_extension_type, media_file) => 
                write!(f, "file {} has a media type of {} but an incompatible file extension of {}", media_type.to_string(), file_extension_type.to_string(), media_file.raw_file_name),
            FCError::OutOfPlaceMediaType(media_type) => write!(f, "found a file with {}, but wasn't expecting any", media_type.to_string()),
            FCError::IncorrectLastName(expected, got, media_file) => write!(f, "file {} had last name {got} when it should be {expected}", media_file.raw_file_name),
            FCError::IncorrectFirstInitial(expected, got, media_file) => write!(f, "file {} had a first initial {got} when it should have been {expected}", media_file.raw_file_name),
            FCError::IncorrectDpi(expected, got, media_file) => write!(f, "file {} had dpi {got} when it should have been {expected}", media_file.raw_file_name),
            FCError::NotCorrected(media_file) => write!(f, "file {} has not been corrected", media_file.raw_file_name),
            FCError::MissingGroupNumber(expected, media_file) => write!(f, "file {} had no group number when it should have been {expected}", media_file.raw_file_name),
            FCError::IncorrectGroupNumber(expected, got, media_file) => write!(f, "file {} had group number {got} when it should have been {expected}", media_file.raw_file_name),
            FCError::MissingGroupChar(expected, media_file) => write!(f, "file {} had no group character when it should have been {expected}", media_file.raw_file_name),
            FCError::IncorrectGroupChar(expected, got, media_file) => write!(f, "file {} had group character {got} when it should have been {expected}", media_file.raw_file_name),
            FCError::IncorrectGroupNumberPrecision(expected, got, media_file) => write!(f, "file {} had a group number precision of {got} digits when it should have been {expected} digits", media_file.raw_file_name),
            FCError::RepeatedIndexNumber(index_number, file_name_1, file_name_2) => write!(f, "files {file_name_1} and {file_name_2} have the same index number {index_number}"),
            FCError::IncorrectIndexNumberPrecision(expected, got, media_file) => write!(f, "file {} had an index number precision of {got} digits when it should have been {expected}", media_file.raw_file_name),
            FCError::FolderSkippedIndexNumber(index_number) => write!(f, "folder skipped index number {index_number}")
        }
    }
}