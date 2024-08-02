use std::str::FromStr;
use regex;
use crate::categories::{ScanType, FileExtension, MediaType};

struct ParsedFileName {
    last_name: String,
    first_name_initial: char,
    media_type: MediaType,
    group_number: Option<u32>,
    group_number_precision: Option<usize>,
    group_character: Option<char>,
    index_number: u32,
    index_number_precision: usize,
    scan_type: ScanType,
    file_extension: FileExtension,
}

enum SectionType {
    FormattedName,
    MediaType,
    GroupNumber,
    GroupCharacter,
    IndexNumber,
    ScanType,
    Extension,
    End
}

#[derive(Debug)]
pub enum FileNameParseError {
    Todo,
    NameShort(String, String),
    UnrecognizedMediaType(String, String),
    ExpectedNumber(String, String),
    NoIndexNumber(String),
    InvalidExtension(String, String),
    ExpectedEnd(String, String)
}
impl std::error::Error for FileNameParseError {}
impl std::fmt::Display for FileNameParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FileNameParseError::Todo => write!(f, "incomplete branch reached"),
            FileNameParseError::NameShort(word, file) => write!(f, "name \"{word}\" was too short in file {file}"),
            FileNameParseError::UnrecognizedMediaType(word, file) => write!(f, "unrecognized media type \"{word}\" in file {file}"),
            FileNameParseError::ExpectedNumber(word, file) => write!(f, "unrecognized text \"{word}\" where a number should have been in file {file}"),
            FileNameParseError::NoIndexNumber(file) => write!(f, "no index number could be found in file {file}"),
            FileNameParseError::InvalidExtension(word, file) => write!(f, "invalid extension \"{word}\" found in file {file}"),
            FileNameParseError::ExpectedEnd(word, file) => write!(f, "expected the file to end before reading \"{word}\" in file {file}")
        }
    }
}

fn parse_file_name(path: std::path::PathBuf) -> Result<ParsedFileName, FileNameParseError> {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let re = regex::Regex::new(r"_|.").unwrap();

    let mut last_name: &str = "";
    let mut first_name_initial: char = ' ';
    let mut media_type: MediaType = MediaType::Error;
    let mut group_number: Option<u32> = None;
    let mut group_number_precision: Option<usize> = None;
    let mut group_character: Option<char> = None;
    let mut index_number: u32 = 0;
    let mut index_number_precision: usize = 0;
    let mut scan_type: ScanType = ScanType::Error;
    let mut file_extension: FileExtension = FileExtension::Error;

    let mut current_section = SectionType::FormattedName;
    for word in re.split(file_name) {
        loop {
            match current_section {
                SectionType::FormattedName => {
                    (last_name, first_name_initial) = match parse_client_name(word, file_name.to_string()) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    };
                    current_section = SectionType::MediaType;
                    break;
                }
                SectionType::MediaType => {
                    media_type = match word_to_media_type(word, file_name.to_string()) {
                        Ok(v) => v,
                        Err(e) => return Err(e)
                    };
                    current_section = SectionType::GroupNumber;
                    break;
                }
                SectionType::GroupNumber => {
                    group_number = match try_get_number(word) {
                        Some(v) => Some(v),
                        None => return Err(FileNameParseError::ExpectedNumber(word.to_string(), file_name.to_string()))
                    };
                    group_number_precision = Some(word.len());
                    current_section = SectionType::GroupCharacter;
                    break;
                }
                SectionType::GroupCharacter => {
                    group_character = try_get_char(word);
                    current_section = SectionType::IndexNumber;
                    // Only go to next iteration if the Group character ended up existing
                    if group_character.is_some() {
                        break;
                    }
                }
                SectionType::IndexNumber => {
                    match try_get_number(word) {
                        Some(v) => {
                            index_number = v;
                            index_number_precision = word.len();
                        }
                        None => {
                            // If no number is found here, the number we read as the group number was actually supposed to be the index number
                            if group_character.is_some() {
                                // Should not be possible to be a non-number here if a group character was read
                                return Err(FileNameParseError::NoIndexNumber(file_name.to_string()))
                            }
                            index_number = group_number.unwrap();
                            index_number_precision = group_number_precision.unwrap();
                            group_number = None;
                            group_number_precision = None;
                        }
                    }
                    current_section = SectionType::ScanType;   
                    break;              
                }
                SectionType::ScanType => { 
                    scan_type = match ScanType::from_str(word) {
                        Ok(v) => v,
                        Err(_) => ScanType::Default
                    };
                    current_section = SectionType::Extension;
                    if scan_type != ScanType::Default {
                        // If scan type was read, consume iterator
                        break;
                    }
                }
                SectionType::Extension => {
                    file_extension = match FileExtension::from_str(word) {
                        Ok(v) => v,
                        Err(_) => return Err(FileNameParseError::InvalidExtension(word.to_string(), file_name.to_string()))
                    };
                    current_section = SectionType::End;
                    break;
                }
                SectionType::End => {
                    return Err(FileNameParseError::ExpectedEnd(word.to_string(), file_name.to_string()))
                }
            }
        }
    }

    let last_name = last_name.to_string();
    Ok(ParsedFileName { last_name, first_name_initial, media_type, group_number, group_number_precision, group_character, index_number, index_number_precision, scan_type, file_extension })
}

fn parse_client_name(word: &str, file_name: String) -> Result<(&str, char), FileNameParseError> {
    let len = word.len();
    if len < 2 {
        return Err(FileNameParseError::NameShort(word.to_string(), file_name))
    }
    let first_initial = word.get(len-1..len-1).unwrap().chars().next().unwrap();
    let last_name = word.get(0..len-2).unwrap();

    Ok((last_name, first_initial))
}

fn word_to_media_type(word: &str, file_name: String) -> Result<MediaType, FileNameParseError> {
    Ok(match word {
        "Slides" => MediaType::Slides,
        "Negs" => MediaType::Negatives,
        "Prints" => MediaType::Prints,
        _ => return Err(FileNameParseError::NameShort(word.to_string(), file_name))
    })
}

fn try_get_number(word: &str) -> Option<u32> {
    match word.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => None
    }
}

fn try_get_char(word: &str) -> Option<char> {
    match word.len() {
        1 => Some(word.chars().next().unwrap()),
        _ => None
    }
}