use actix_web::http::header::TryIntoHeaderValue;
use serde::Deserialize;

use super::{error::FCError, media_file::MediaFile, FinalCheckRequest, PhotoGroupOptions};
use little_exif::{endian::Endian, metadata::Metadata};
use little_exif::exif_tag::ExifTag;

pub mod error;
use error::{ MediaGroupsError };


#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug)]
pub struct MediaGroups {
    pub slides: Option<PhotoGroupOptions>,
    pub prints: Option<PhotoGroupOptions>,
    pub negatives: Option<PhotoGroupOptions>
}
impl MediaGroups {
    pub fn counts_equal(&self, expected_media: MediaGroups) -> Result<(), MediaGroupsError> {
        fn equals_or_err(counted: u64, expected: u64, media_and_scan_type : &str) -> Result<(), MediaGroupsError> {
            return match counted == expected {
                true => Ok(()),
                false => Err(MediaGroupsError::IncorrectMediaAndScanTypeCount(media_and_scan_type.to_string(), counted, expected))
            }
        }

        if let Some(counted_slides) = &self.slides {
            let expected_slides = expected_media.slides.unwrap();
            equals_or_err(counted_slides.scanner, expected_slides.scanner, "scanner slides")?;
            equals_or_err(counted_slides.hs, expected_slides.hs, "handscan slides")?;
        }
        if let Some(counted_prints) = &self.prints {
            let expected_prints = expected_media.prints.unwrap();
            equals_or_err(counted_prints.scanner, expected_prints.scanner, "scanner prints")?;
            equals_or_err(counted_prints.hs, expected_prints.hs, "handscan prints")?;
            equals_or_err(counted_prints.oshs, expected_prints.oshs, "oversized prints")?;
        }
        if let Some(counted_negs) = &self.negatives {
            let expected_negs = expected_media.negatives.unwrap();
            equals_or_err(counted_negs.scanner, expected_negs.scanner, "scanner negatives")?;
            equals_or_err(counted_negs.hs, expected_negs.hs, "handscan negatives")?;
        }

        Ok(())
    }


    pub fn from_parsed_file_names(file_names: &Vec<MediaFile>) -> Result<MediaGroups, MediaGroupsError> {
        let ret = MediaGroups { slides: None, prints: None, negatives: None };
        Ok(ret)
    }


    pub fn check_file_metadata(&self, parsed_file_name: &MediaFile) -> Result<(), MetadataCheckError> {
        match parsed_file_name.media_type {
            MediaType::Slides | MediaType::Prints | MediaType::Negatives => {
                
            }
        }

        
        Ok(())
        //let resolution_unit = metadata.get_tag(&ExifTag::XResolution(0));
    }
}