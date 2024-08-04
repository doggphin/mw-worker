#[derive(Debug)]
pub enum MediaGroupsError {
    IncorrectMediaAndScanTypeCount(String, u64, u64),
}
impl std::error::Error for MediaGroupsError {}
impl std::fmt::Display for MediaGroupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MediaGroupsError::IncorrectMediaAndScanTypeCount(scan_and_media_type, counted, expected) => write!(f, "expected {expected} {scan_and_media_type}, only counted {counted}"),
        }
    }
}


#[derive(Debug)]
pub enum MetadataCheckError {
    CouldNotReadPath(std::path::PathBuf, std::io::Error),
    WrongDpi(u64, u64)
}
impl std::error::Error for MetadataCheckError {}
impl std::fmt::Display for MetadataCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MetadataCheckError::CouldNotReadPath(path, err) => write!(f, "could not read {}: {err}", path.to_string_lossy()),
            MetadataCheckError::WrongDpi(expected, found) => write!(f, "expected {expected} dpi, photo had {found} dpi")     
        }
    }
}