#[derive(Debug)]
pub enum PhotoMediaDataError {
    CouldNotReadPath(std::path::PathBuf),
    DifferentXYDpi(u32, u32),
    NoDpiFound,
    BadlyFormattedExifTag(String),
}
impl std::error::Error for PhotoMediaDataError {}
impl std::fmt::Display for PhotoMediaDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            PhotoMediaDataError::CouldNotReadPath(path) => write!(f, "could not read {}", path.to_string_lossy()),
            PhotoMediaDataError::DifferentXYDpi(x, y) => write!(f, "different dpi values found along the X ({x}) and Y ({y}) dimensions"),
            PhotoMediaDataError::NoDpiFound => write!(f, "no dpi tag found"),
            PhotoMediaDataError::BadlyFormattedExifTag(tag_name) => write!(f, "badly formatted {tag_name} couldn't be read"),
        }
    }
}