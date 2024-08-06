use std::collections::HashMap;

use serde::Deserialize;

use crate::{qc::final_check::{error::FCError, media_file::MediaFile, photo_group_options::PhotoGroupOptions}, utils::types::media_types::{photo_media_data::PhotoMediaData, MediaType}};

use super::{media_folder::MediaFolder, media_groups::MediaGroupValues};

#[derive(Deserialize, Debug)]
pub struct FinalCheckRequest {
    pub first_name: String,
    pub last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_num: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_char: Option<char>,
    #[serde(default = "default_2")]
    pub group_num_precision: u64,   // Guaranteed 6 or less
    #[serde(default = "default_3")]
    pub index_num_precision: u64,

    pub media_group_values: MediaGroupValues
}
fn default_2() -> u64 { 2 }
fn default_3() -> u64 { 3 }
impl FinalCheckRequest {
    pub fn verify_media_folder(&self, media_folder: MediaFolder) -> Result<(), FCError> {

        fn check_against_photo_group_options(media_file: &MediaFile, photo_group_options: &Option<PhotoGroupOptions>, photo_data: &PhotoMediaData) -> Result<(), FCError> {
            if let Some(group_data) = photo_group_options {
                return Ok(photo_data.check_against_group_options(&group_data).map_err(|e| FCError::IncorrectPhotoMetadata(e))?);
            } else {
                return Err(FCError::OutOfPlaceMediaType(media_file.media_type.clone()));
            }
        }

        let mut seen_index_numbers: HashMap<u32, String> = HashMap::new();
        for media_file in media_folder.files {
            match &media_file.media_type {
                MediaType::Prints(print_data) => check_against_photo_group_options(&media_file, &self.media_group_values.prints, &print_data)?,
                MediaType::Slides(slides_data) => check_against_photo_group_options(&media_file, &self.media_group_values.slides, &slides_data)?,
                MediaType::Negatives(negatives_data) => check_against_photo_group_options(&media_file, &self.media_group_values.negatives, &negatives_data)?,
            }

            if media_file.last_name != self.last_name {
                return Err(FCError::IncorrectLastName(self.last_name.clone(), media_file.last_name.clone(), media_file))
            }
            let expected_first_initial = self.first_name.chars().next().unwrap();
            if media_file.first_name_initial != expected_first_initial {
                return Err(FCError::IncorrectFirstInitial(expected_first_initial, media_file.first_name_initial, media_file))
            }
            if let Some(expected) = self.group_num {
                if let Some(got) = media_file.group_number {
                    let got = u64::from(got);
                    if got != expected {
                        return Err(FCError::IncorrectGroupNumber(expected, got, media_file))
                    }
                    // Group number precisions are guaranteed to be defined at this point
                    let media_file_group_precision = u64::try_from(media_file.group_number_precision.unwrap()).unwrap();
                    if self.group_num_precision != media_file_group_precision {
                        return Err(FCError::IncorrectGroupNumberPrecision(self.group_num_precision, media_file_group_precision, media_file))
                    }
                } else {
                    return Err(FCError::MissingGroupNumber(expected, media_file))
                }
            }
            if let Some(expected) = self.group_char {
                if let Some(got) = media_file.group_character {
                    if got != expected {
                        return Err(FCError::IncorrectGroupChar(expected, got, media_file))
                    }
                } else {
                    return Err(FCError::MissingGroupChar(expected, media_file))
                }
            }
            if let Some(repeated_file_name) = seen_index_numbers.insert(media_file.index_number, media_file.raw_file_name.clone()) {
                return Err(FCError::RepeatedIndexNumber(media_file.index_number, media_file.raw_file_name, repeated_file_name))
            }
        }
        let mut seen_index_numbers: Vec<u32> = seen_index_numbers.keys().cloned().collect();
        seen_index_numbers.sort();
        let mut expecting_value = 1;
        for index_number in seen_index_numbers {
            if index_number != expecting_value {
                return Err(FCError::FolderSkippedIndexNumber(expecting_value));
            }
            expecting_value += 1;
        }

        Ok(())
    }
}