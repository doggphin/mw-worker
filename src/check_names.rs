use crate::FilesWs;
use actix::Actor;
use serde_json::Value;
use serde::Deserialize;
use glob::glob;

#[derive(Deserialize, Debug)]
struct Data {
    data: FinalCheckData
}

#[derive(Deserialize, Debug)]
struct FinalCheckData {
    first_name: String,
    last_name: String,
    group_number: u64, 
    #[serde(skip_serializing_if = "Option::is_none")]
    group_number_precision: Option<u64>,   // Default 2
    expecting_media: ExpectingMedia
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug)]
struct ExpectingMedia {
    slides: Option<PhotoGroupOptions>,
    prints: Option<PhotoGroupOptions>,
    negatives: Option<PhotoGroupOptions>
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug)]
struct PhotoGroupOptions {
    dpi: Option<u64>,               // Defaults (does not check)
    scanner: Option<u64>,           // Defaults 0
    hs: Option<u64>,                // Defaults 0
    oshs: Option<u64>,              // Defaults 0
    is_corrected: Option<bool>,     // Defaults (does not check)
    index_precision: Option<u64>    // Defaults 3
}

/* https://github.com/lovasoa/custom_error */
#[derive(Debug)]
pub enum FinalCheckError {
    InvalidRequestFormatting(String),
    InvalidRequestMissingFields(String),
    IncorrectFileNameClientName(String),
    InvalidFileNameMediaType(String, String),
    InvalidFileNameGroupNumber(String, String),
    InvalidFileNameTooLong(String),
    IncorrectFileNameGroupNumber(u64, u64, String),
    IncorrectFileNameGroupNumberPrecision(u64, u64, String),
}
impl std::error::Error for FinalCheckError {}
impl std::fmt::Display for FinalCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FinalCheckError::InvalidRequestFormatting(msg) => write!(f, "invalid request formatting: {}", msg),
            FinalCheckError::InvalidRequestMissingFields(msg) => write!(f, "missing field in request: {}", msg),
            FinalCheckError::IncorrectFileNameClientName(msg) => write!(f, "file with incorrect client name found: {}", msg),
            FinalCheckError::InvalidFileNameMediaType(media_type, file_name) => write!(f, "\"{media_type}\" did not match any known media types in file \"{file_name}\""),
            FinalCheckError::InvalidFileNameGroupNumber(group_number, file_name) => write!(f, "could not convert group number \"{group_number}\" to a number in file \"{file_name}\""),
            FinalCheckError::IncorrectFileNameGroupNumber(file_group_number, expected_group_number, file_name) => write!(f, "file \"{file_name}\" had group number {file_group_number} when it should have had group number {expected_group_number}"),
            FinalCheckError::IncorrectFileNameGroupNumberPrecision(file_group_number_precision, expected_group_number_precision, file_name) => write!(f, "file \"{file_name}\" had the correct group number, but had {file_group_number_precision} digits of precision instead of the expected {expected_group_number_precision}"),
            FinalCheckError::InvalidFileNameTooLong(file_name) => write!(f, "file \"{file_name}\" had too many words; should follow format <name>_<media>_<group#>_<index#><_optionalscanmethod>.<extension>")
        }
    }
}

pub fn final_check(dir: String, request_json: Value, ctx: &mut<FilesWs as Actor>::Context) -> std::result::Result<(), FinalCheckError> {
    let mut data: FinalCheckData = match serde_json::from_value::<Data>(request_json) {
        Ok(v) => v.data,
        Err(e) => { return Err(FinalCheckError::InvalidRequestMissingFields(e.to_string())); }
    };
    if data.expecting_media.slides.is_none() && data.expecting_media.negatives.is_none() && data.expecting_media.prints.is_none() {
        return Err(FinalCheckError::InvalidRequestMissingFields(String::from("expecting_media")));
    }
    for entry in glob(&*format!("{dir}\\*.jpg")).expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Err(e) = final_check_entry_name(path, &mut data) {
                    return Err(e);
                }
            }
            Err(e) => println!("{:?}", e)
        }
    }
    
    Ok(())
}

fn final_check_entry_name(path: std::path::PathBuf, data: &mut FinalCheckData) -> Result<(), FinalCheckError> {
    // <Last Name><First Name Initial>_<Media Type>_<Group #>_<Index #>_<Scan Type>
    let file_name = path.file_name().unwrap().to_str().unwrap();
    for (i, val) in file_name.split("_").into_iter().enumerate() {
        let mut media_type = "";
        match i {
            0 => {  // LancasterB
                if let Err(e) = check_client_name(file_name, val, data) {
                    return Err(e);
                }
            }
            1 => {  // Slides
                media_type = match check_media_name(file_name, val, data) {
                    Ok(_) => val,
                    Err(e) => return Err(e)
                }
            }
            2 => {  // 03
                if let Err(e) = check_group_number(file_name, val, data) {
                    return Err(e);
                }
            }
            3 => {
                // This can either be a group number OR a group number and the extension
                // Group number should also be able to be ommited if the project only has one group
                if val.contains(".") {
                    let values = Vec::from_iter(val.split(".").map(String::from));
                    if let Err(e) = check_index_number(file_name, &*values[0], media_type, data) {
                        return Err(e);
                    }
                    if let Err(e) = check_extension(file_name, &*values[1], media_type, data) {
                        return Err(e);
                    }
                    return Ok(());
                } else {
                    if let Err(e) = check_index_number(file_name, val, media_type, data) {
                        return Err(e);
                    }
                }
            }
            4 => {
                // This is a scan type+extension
                let values = Vec::from_iter(val.split(".").map(String::from));
                if let Err(e) = check_scan_type(file_name, &*values[0], media_type, data) {
                    return Err(e);
                }
                if let Err(e) = check_extension(file_name, &*values[1], media_type, data) {
                    return Err(e);
                }
                return Ok(());
            }
            _ => {
                return Err(FinalCheckError::InvalidFileNameTooLong(file_name.to_string()));
            }
        }
    }

    Ok(())
}


fn check_client_name(file_name: &str, client_name: &str, data: &FinalCheckData) -> Result<(), FinalCheckError> {
    let expected_first_name_initial = match data.first_name.chars().nth(0) {
        Some(v) => v,
        None => { return Err(FinalCheckError::InvalidRequestFormatting(format!("invalid first name specified: \"{}\"", data.first_name))); }
    };
    let expected_client_name = format!("{}{}", data.last_name, expected_first_name_initial);
    if client_name != expected_client_name {
        return Err(FinalCheckError::IncorrectFileNameClientName(format!("expected \"{}\", found \"{}\" in file \"{}\"", expected_client_name, client_name, file_name)));
    }

    Ok(())
}


fn check_media_name(file_name: &str, media_name: &str, data: &mut FinalCheckData) -> Result<(), FinalCheckError> {
    match media_name {
        "Slides" | "Prints" | "Negatives" => Ok(()),
        _ => return Err(FinalCheckError::InvalidFileNameMediaType(media_name.to_string(), file_name.to_string()))
    }
}


fn check_group_number(file_name: &str, group_number: &str, data: &FinalCheckData) -> Result<(), FinalCheckError> {
    let group_number_u64 = match group_number.parse::<u64>() {
        Ok(v) => v,
        _ => return Err(FinalCheckError::InvalidFileNameGroupNumber(group_number.to_string(), file_name.to_string()))
    };
    if group_number_u64 != data.group_number {
        return Err(FinalCheckError::IncorrectFileNameGroupNumber(group_number_u64, data.group_number, file_name.to_string()))
    }
    let group_number_digits: u64 = u64::try_from(group_number.len()).unwrap();
    if data.group_number_precision.is_some() && group_number_digits != data.group_number_precision.unwrap() {
        return Err(FinalCheckError::IncorrectFileNameGroupNumberPrecision(group_number_digits, data.group_number_precision.unwrap(), file_name.to_string()))
    }
    Ok(())
}


fn check_index_number(file_name: &str, index_number: &str, media_type: &str, data: &mut FinalCheckData) -> Result<(), FinalCheckError> {
    Ok(())
}


fn check_extension(file_name: &str, extension: &str, media_type: &str, data: &mut FinalCheckData) -> Result<(), FinalCheckError> {
    Ok(())
}


fn check_scan_type(file_name: &str, index_number: &str, media_type: &str, data: &mut FinalCheckData) -> Result<(), FinalCheckError> {
    Ok(())
}