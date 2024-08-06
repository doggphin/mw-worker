pub mod photo_media_data;
pub mod error;
use photo_media_data::PhotoMediaData;
use error::MediaTypeError;

#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Prints(PhotoMediaData),
    Slides(PhotoMediaData),
    Negatives(PhotoMediaData)
}
impl ToString for MediaType {
    fn to_string(&self) -> String {
        match self {
            MediaType::Prints(_) => "Prints".to_string(),
            MediaType::Slides(_) => "Slides".to_string(),
            MediaType::Negatives(_) => "Negatives".to_string(),
        }
    }
}
impl MediaType {
    pub fn from_path(word: &str, path: &std::path::PathBuf) -> Result<MediaType, MediaTypeError> {
        match word {
            "Prints" => Ok(MediaType::Prints(PhotoMediaData::from_path(path).map_err(|e| MediaTypeError::PhotoMediaDataError(e))?)),
            "Slides" => Ok(MediaType::Slides(PhotoMediaData::from_path(path).map_err(|e| MediaTypeError::PhotoMediaDataError(e))?)),
            "Negs" => Ok(MediaType::Negatives(PhotoMediaData::from_path(path).map_err(|e| MediaTypeError::PhotoMediaDataError(e))?)),
            _ => Err(MediaTypeError::UnrecognizedMediaType(word.to_string()))
        }
    }
}

/*
impl FromStr for MediaType {
    type Err = ();
    fn from_str(input: &str) -> Result<MediaType, Self::Err> {
        match input {
            "Prints" => Ok(MediaType::Prints(PhotoMediaData::new())),
            "Slides" => Ok(MediaType::Slides(PhotoMediaData::new())),
            "Negs" => Ok(MediaType::Negatives(PhotoMediaData::new())),
            _ => Err(())
        }
    }
} */
