#[derive(Debug)]
pub enum FileNameParseError {
    NameShort(String),
    UnrecognizedMediaType(String),
    ExpectedGroupOrIndexNumber(String),
    NoIndexNumber,
    InvalidExtension(String),
    ExpectedEnd(String)
}
impl std::error::Error for FileNameParseError {}
impl std::fmt::Display for FileNameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FileNameParseError::NameShort(word) => write!(f, "name \"{word}\" was too short"),
            FileNameParseError::UnrecognizedMediaType(word) => write!(f, "unrecognized media type \"{word}\""),
            FileNameParseError::ExpectedGroupOrIndexNumber(word) => write!(f, "unrecognized text \"{word}\" where a group or index number should have been"),
            FileNameParseError::NoIndexNumber => write!(f, "no index number could be found"),
            FileNameParseError::InvalidExtension(word) => write!(f, "invalid extension \"{word}\""),
            FileNameParseError::ExpectedEnd(word) => write!(f, "unexpected additional field \"{word}\"")
        }
    }
}