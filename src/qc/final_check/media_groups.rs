use serde::Deserialize;
use crate::utils::types::{media_types::MediaType, scan_type::ScanType};

use super::{media_file::MediaFile, PhotoGroupOptions};

pub mod error;
use error::MediaGroupsError;


#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug, Copy, Clone)]
pub struct MediaGroupValues {
    pub slides: Option<PhotoGroupOptions>,
    pub prints: Option<PhotoGroupOptions>,
    pub negatives: Option<PhotoGroupOptions>
}
impl MediaGroupValues {
    pub fn counts_equal(&self, expected_media: MediaGroupValues) -> Result<(), MediaGroupsError> {
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


    pub fn from_media_files(media_files: &Vec<MediaFile>) -> Result<MediaGroupValues, MediaGroupsError> {
        let mut slides = PhotoGroupOptions::new();
        let mut prints = PhotoGroupOptions::new();
        let mut negatives = PhotoGroupOptions::new();
        let mut include_slides = false;
        let mut include_prints = false;
        let mut include_negatives = false;

        for media_file in media_files.iter() {
            match media_file.media_type {
                MediaType::Slides(_) => {
                    include_slides = true;
                    match media_file.scan_type {
                        ScanType::Default => slides.scanner += 1,
                        ScanType::HandScan => slides.hs += 1,
                        _ => return Err(MediaGroupsError::InvalidScanTypeMediaGroupCombo(media_file.scan_type.clone(), media_file.media_type.clone(), media_file.raw_file_name.clone())),
                    }
                }
                MediaType::Prints(_) => {
                    include_prints = true;
                    match media_file.scan_type {
                        ScanType::Default => prints.scanner += 1,
                        ScanType::HandScan => prints.hs += 1,
                        ScanType::OversizedHandScan => prints.oshs += 1,
                    }
                }
                MediaType::Negatives(_) => {
                    include_negatives = true;
                    match media_file.scan_type {
                        ScanType::Default => negatives.scanner += 1,
                        ScanType::HandScan => negatives.hs += 1,
                        _ => return Err(MediaGroupsError::InvalidScanTypeMediaGroupCombo(media_file.scan_type.clone(), media_file.media_type.clone(), media_file.raw_file_name.clone())),
                    }
                }
            }
        }
        
        let slides = if include_slides { Some(slides) } else { None };
        let prints = if include_prints { Some(prints) } else { None };
        let negatives = if include_negatives { Some(negatives) } else { None };

        Ok(MediaGroupValues{slides, prints, negatives})
    }
}