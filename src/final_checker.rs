use crate::FilesWs;
use crate::file_name_parser::ParsedFileName;
use actix::Actor;
use serde_json::Value;
use serde::Deserialize;
use glob::glob;

#[derive(Deserialize, Debug)]
struct Data {
    data: FinalCheckRequest
}

#[derive(Deserialize, Debug)]
struct FinalCheckRequest {
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

#[derive(Debug)]
pub enum FinalCheckError {
    InvalidRequest(String),
    InvalidDirectory(String, String),
    InvalidFile(String, String),
    FileNameParsingError(String, String)
}
impl std::error::Error for FinalCheckError {}
impl std::fmt::Display for FinalCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            FinalCheckError::InvalidRequest(err) => write!(f, "invalid request: {err}"),
            FinalCheckError::InvalidDirectory(err, pattern) => write!(f, "invalid directory pattern \"{pattern}\": {err}"),
            FinalCheckError::InvalidFile(err, pattern) => write!(f, "invalid file in directory pattern \"{pattern}\": {err}"),
            FinalCheckError::FileNameParsingError(err, file_name) => write!(f, "error parsing file \"{file_name}\": {err}")
        }
    }
}

pub fn final_check(dir: String, request_json: Value, ctx: &mut<FilesWs as Actor>::Context) -> std::result::Result<(), FinalCheckError> {
    let final_check_request = match parse_final_check_request(request_json) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let pattern = format!("{dir}\\*");
    let files = match glob(&*pattern) {
        Ok(v) => v,
        Err(e) => return Err(FinalCheckError::InvalidDirectory(e.to_string(), pattern))
    };
    for entry in files {
        let entry = match entry {
            Ok(v) => v,
            Err(e) => return Err(FinalCheckError::InvalidFile(e.to_string(), pattern))
        };
        let file_name = entry.file_name().unwrap().to_str().unwrap();
        let parsed_file_name = match ParsedFileName::from(file_name) {
            Ok(v) => v,
            Err(e) => return Err(FinalCheckError::FileNameParsingError(e.to_string(), file_name.to_string()))
        };
        dbg!(parsed_file_name);
    }
    Ok(())
}

fn parse_final_check_request(request_json: Value) -> std::result::Result<FinalCheckRequest, FinalCheckError> {
    let data: FinalCheckRequest = match serde_json::from_value::<Data>(request_json) {
        Ok(v) => v.data,
        Err(e) => { return Err(FinalCheckError::InvalidRequest(e.to_string())); }
    };
    if data.expecting_media.slides.is_none() && data.expecting_media.negatives.is_none() && data.expecting_media.prints.is_none() {
        return Err(FinalCheckError::InvalidRequest(String::from("no properties were defined in expecting_media")));
    }
    Ok(data)
}