#[derive(Debug)]
pub enum PhotoMediaDataError {
    CouldNotReadPath(std::path::PathBuf, std::io::Error),
    IncorrectDpi(u64, u64),
    DifferentXYDpi(u32, u32),
    NoDpiFound(MediaFile),
    BadlyFormattedExifTag(String),

    Todo
}
impl std::error::Error for PhotoMediaDataError {}
impl std::fmt::Display for PhotoMediaDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MetadataCheckError::CouldNotReadPath(path, err) => write!(f, "could not read {}: {err}", path.to_string_lossy()),
            MetadataCheckError::IncorrectDpi(expected, found) => write!(f, "expected {expected} dpi, photo had {found} dpi"),
            MetadataCheckError::DifferentXYDpi(x, y) => write!(f, "different dpi values found along the X ({x}) and Y ({y}) dimensions"),
            MetadataCheckError::NoDpiFound(parsed_file_name) => write!(f, "no dpi found while checking {}", parsed_file_name.raw_file_name),
            MetadataCheckError::BadlyFormattedExifTag(tag_name) => write!(f, "badly formatted {tag_name} couldn't be read"),
            
            MetadataCheckError::Todo => write!(f, "unimplemented error!")
        }
    }
}