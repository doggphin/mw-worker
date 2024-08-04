use serde::Deserialize;
use super::{error::FCError, file_names::ParsedFileName, FinalCheckRequest, PhotoGroupOptions};
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;

pub mod error;
use error::{ MediaGroupsError, MetadataCheckError };


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


    pub fn from_parsed_file_names(file_names: &Vec<ParsedFileName>) -> Result<MediaGroups, MediaGroupsError> {
        let mut ret = MediaGroups { slides: None, prints: None, negatives: None };
        Ok(ret)
    }

    pub fn check_file_metadata(&self, parsed_file_name: &ParsedFileName) -> Result<(), MetadataCheckError> {
        let metadata = Metadata::new_from_path(&parsed_file_name.path).map_err(|e| MetadataCheckError::CouldNotReadPath(parsed_file_name.path.clone(), e))?;
        let tag = metadata.get_tag_by_hex(0xbc82);
        dbg!(tag);
        Ok(())
        //let resolution_unit = metadata.get_tag(&ExifTag::XResolution(0));
    }
}