#[derive(Debug)]
pub enum FCError {
    InvalidRequest(String),
    InvalidDirectory(String, String),
    InvalidFile(String),
    FileNameParsingError(String, String),

    ScanTypeRemaining(String, u64, u64),
    NoneOfFileTypeFound(String)
}
impl std::error::Error for FCError {}
impl std::fmt::Display for FCError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FCError::InvalidRequest(err) => write!(f, "invalid request: {err}"),
            FCError::InvalidDirectory(err, pattern) => write!(f, "invalid directory pattern \"{pattern}\": {err}"),
            FCError::InvalidFile(err) => write!(f, "invalid file path: {err}"),
            FCError::FileNameParsingError(err, file_name) => write!(f, "error parsing file \"{file_name}\": {err}"),
        
            FCError::ScanTypeRemaining(scan_type, expected, counted) => write!(f, "expected {expected} {scan_type}, only found {counted}"),
            FCError::NoneOfFileTypeFound(scan_type) => write!(f, "expected to find some {scan_type}, but found none")
        }
    }
}