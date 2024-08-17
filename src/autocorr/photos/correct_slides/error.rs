#[derive(Debug)]
pub enum SlidesAutocorrectError {
    PhotoshopError(String)
}
impl std::error::Error for SlidesAutocorrectError {}
impl std::fmt::Display for SlidesAutocorrectError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            SlidesAutocorrectError::PhotoshopError(e) => write!(f, "photoshop error: {e}"),
        }
    }
}