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