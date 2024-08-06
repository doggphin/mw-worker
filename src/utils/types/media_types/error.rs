use super::photo_media_data::error::PhotoMediaDataError;

#[derive(Debug)]
pub enum MediaTypeError {
    CouldNotReadPath(std::path::PathBuf),
    PhotoMediaDataError(PhotoMediaDataError),
    UnrecognizedMediaType(String)
}
impl std::error::Error for MediaTypeError {}
impl std::fmt::Display for MediaTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MediaTypeError::CouldNotReadPath(path) => write!(f, "could not read {}", path.to_string_lossy()),
            MediaTypeError::PhotoMediaDataError(e) => write!(f, "error creating photo media data: {e}"),
            MediaTypeError::UnrecognizedMediaType(word) => write!(f, "unrecognized media type {word}")
        }
    }
}