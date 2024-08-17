pub mod error;
use error::SlidesAutocorrectError;

pub fn correct_image(file_path: std::path::PathBuf, to_folder: std::path::PathBuf) -> Result<(), SlidesAutocorrectError> {
    if !file_path.is_file() {
        
    }
    Ok(())
}