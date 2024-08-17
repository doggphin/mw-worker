#[derive(Debug)]
pub enum AutocorrectError {
    PhotoError().
}
impl std::error::Error for AutocorrectError {}
impl std::fmt::Display for AutocorrectError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            AutocorrectError::PhotoError(e) => write!(f, "error correcting photo: {e}"),
        }
    }
}