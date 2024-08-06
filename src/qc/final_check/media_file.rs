use std::{ffi::OsStr, path::PathBuf, str::FromStr};
use little_exif::metadata::Metadata;
use regex;

pub mod error;
use error::MediaFileParseError;

use crate::utils::types::{file_extension_type::FileExtensionType, media_types::{photo_media_data::PhotoMediaData, MediaType}, scan_type::ScanType};

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub path: std::path::PathBuf,
    pub raw_file_name: String,
    pub last_name: String,
    pub first_name_initial: char,
    pub media_type: MediaType,
    pub group_number: Option<u32>,
    pub group_number_precision: Option<usize>,
    pub group_character: Option<char>,
    pub index_number: u32,
    pub index_number_precision: usize,
    pub scan_type: ScanType,
    pub file_extension: FileExtensionType,
}

#[derive(Debug)]
enum SectionReadState {
    FormattedName,
    MediaType,
    GroupNumber,
    GroupCharacter,
    IndexNumber,
    ScanType,
    Extension,
    End
}
impl MediaFile {
    pub fn from_path(path: &std::path::PathBuf) -> Result<MediaFile, MediaFileParseError> {
        if !path.is_file() {
            return Err(MediaFileParseError::NotAFile(path.clone()))
        }
        let file_name = &*path.file_name().unwrap_or(OsStr::new("invalid file name")).to_string_lossy().into_owned();
        println!("{file_name}");
        let re = regex::Regex::new(r"[._]").unwrap();

        let mut last_name: &str = "";
        let mut first_name_initial: char = ' ';
        let mut group_number: Option<u32> = None;
        let mut group_number_precision: Option<usize> = None;
        let mut group_character: Option<char> = None;
        let mut index_number: u32 = 0;
        let mut index_number_precision: usize = 0;
        let mut file_extension: FileExtensionType = FileExtensionType::None;
        let mut media_type: Option<MediaType> = None;
        let mut scan_type: ScanType = ScanType::Default;

        let mut current_section = SectionReadState::FormattedName;
        for word in re.split(file_name) {
            loop {
                match current_section {
                    SectionReadState::FormattedName => {
                        (last_name, first_name_initial) = parse_client_name(word)?;
                        current_section = SectionReadState::MediaType;
                        break;
                    }
                    SectionReadState::MediaType => {
                        media_type = Some(MediaType::from_path(word, path).map_err(|e| MediaFileParseError::MediaTypeError(e))?);
                        current_section = SectionReadState::GroupNumber;
                        break;
                    }
                    // GroupNumber and GroupCharacter could probably be combined
                    SectionReadState::GroupNumber => {
                        group_number = Some(try_get_number(word).map_err(|_| MediaFileParseError::ExpectedGroupOrIndexNumber(word.to_string()))?);
                        group_number_precision = Some(word.len());
                        current_section = SectionReadState::GroupCharacter;
                        break;
                    }
                    SectionReadState::GroupCharacter => {
                        current_section = SectionReadState::IndexNumber;
                        match try_get_char(word) {
                            Ok(v) => {
                                group_character = Some(v);
                                break;
                            }
                            Err(_) => continue
                        };
                    }
                    SectionReadState::IndexNumber => {
                        match try_get_number(word) {
                            Ok(v) => {
                                index_number = v;
                                index_number_precision = word.len();
                            }
                            Err(_) => {
                                // If no number is found here, the number we read as the group number was actually supposed to be the index number
                                if group_character.is_some() {
                                    // Should not be possible to be a non-number here if a group character was read
                                    return Err(MediaFileParseError::NoIndexNumber)
                                }
                                index_number = group_number.unwrap();
                                index_number_precision = group_number_precision.unwrap();
                                group_number = None;
                                group_number_precision = None;
                            }
                        }
                        current_section = SectionReadState::ScanType;   
                        break;              
                    }
                    SectionReadState::ScanType => { 
                        scan_type = match ScanType::from_str(word) {
                            Ok(v) => v,
                            Err(_) => ScanType::Default
                        };
                        current_section = SectionReadState::Extension;
                        if scan_type != ScanType::Default {
                            // If scan type was successfuly read here, consume iterator
                            break;
                        }
                        // Try reading this word as an extension instead
                        continue;  
                    }
                    SectionReadState::Extension => {
                        file_extension = FileExtensionType::from_str(word).map_err(|_| MediaFileParseError::InvalidExtension(word.to_string()))?;
                        current_section = SectionReadState::End;
                        break;
                    }
                    SectionReadState::End => {
                        return Err(MediaFileParseError::ExpectedEnd(word.to_string()))
                    }
                }
            }
        }

        let last_name = last_name.to_string();
        let media_type = media_type.unwrap();
        let path = path.clone();
        let raw_file_name = file_name.to_string();
        let ret = MediaFile { path, raw_file_name, last_name, first_name_initial, media_type, group_number, group_number_precision,
            group_character, index_number, index_number_precision, scan_type, file_extension };
        Ok(ret)
    }
}

fn parse_client_name(word: &str) -> Result<(&str, char), MediaFileParseError> {
    let len = word.len();
    if len < 2 {
        return Err(MediaFileParseError::NameShort(word.to_string()))
    }
    let first_initial = word.chars().last().unwrap();
    let last_name = word.get(0..len-1).unwrap();

    Ok((last_name, first_initial))
}


fn try_get_number(word: &str) -> Result<u32, ()> {
    word.parse::<u32>().map_err(|_| ())
}

fn try_get_char(word: &str) -> Result<char, ()> {
    match word.len() {
        1 => Ok(word.chars().next().unwrap()),
        _ => Err(())
    }
}