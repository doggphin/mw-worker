use crate::utils::types::{media_types::{photo_media_data::error::PhotoMediaDataError, MediaType}, scan_type::ScanType};

#[derive(Debug)]
pub enum MediaGroupsError {
    IncorrectMediaAndScanTypeCount(String, u64, u64),
    InvalidScanTypeMediaGroupCombo(ScanType, MediaType, String),
    IncorrectPhotoMetadata(PhotoMediaDataError),
    OutOfPlaceMediaType(MediaType)
}
impl std::error::Error for MediaGroupsError {}
impl std::fmt::Display for MediaGroupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MediaGroupsError::IncorrectMediaAndScanTypeCount(scan_and_media_type, counted, expected) => write!(f, "expected {expected} {scan_and_media_type}, counted {counted}"),
            MediaGroupsError::InvalidScanTypeMediaGroupCombo(scan_type, media_type, file_name) => write!(f, "{file_name} can't be of type {} and be scanned as a/an {}", media_type.to_string(), scan_type.to_string()),
            MediaGroupsError::IncorrectPhotoMetadata(e) => write!(f, "incorrect photo metadata: {e}"),
            MediaGroupsError::OutOfPlaceMediaType(media_type) => write!(f, "found a file with {}, but wasn't expecting any", media_type.to_string())
        }
    }
}