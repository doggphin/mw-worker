use std::str::FromStr;
use super::scan_type::ScanType;

pub mod photo_media_data;
use photo_media_data::PhotoMediaData;

#[derive(Debug, Clone)]
pub enum MediaType {
    Prints(PhotoMediaData),
    Slides(PhotoMediaData),
    Negatives(PhotoMediaData)
}
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
}