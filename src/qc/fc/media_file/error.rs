#[derive(Debug)]
pub enum MediaFileParseError {
    NameShort(String),
    NotAFile(std::path::PathBuf),
    UnrecognizedMediaType(String),
    ExpectedGroupOrIndexNumber(String),
    NoIndexNumber,
    InvalidExtension(String),
    ExpectedEnd(String)
}
impl std::error::Error for MediaFileParseError {}
impl std::fmt::Display for MediaFileParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            MediaFileParseError::NameShort(word) => write!(f, "name \"{word}\" was too short"),
            MediaFileParseError::NotAFile(path) => write!(f, "path \"{}\" is not a file", path.to_string_lossy()),
            MediaFileParseError::UnrecognizedMediaType(word) => write!(f, "unrecognized media type \"{word}\""),
            MediaFileParseError::ExpectedGroupOrIndexNumber(word) => write!(f, "unrecognized text \"{word}\" where a group or index number should have been"),
            MediaFileParseError::NoIndexNumber => write!(f, "no index number could be found"),
            MediaFileParseError::InvalidExtension(word) => write!(f, "invalid extension \"{word}\""),
            MediaFileParseError::ExpectedEnd(word) => write!(f, "unexpected additional field \"{word}\"")
        }
    }
}