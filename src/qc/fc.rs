use actix::Actor;
use serde_json::Value;
use serde::Deserialize;
use glob::{glob, Paths};
use crate::FilesWs;

mod file_names;
mod error;
mod media_groups;
mod photo_group_options;

use error::FCError;
use file_names::ParsedFileName;
use media_groups::MediaGroups;
use photo_group_options::PhotoGroupOptions;


#[derive(Deserialize, Debug)]
struct Data {
    data: FinalCheckRequest
}

#[derive(Deserialize, Debug)]
struct FinalCheckRequest {
    first_name: String,
    last_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_num: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_char: Option<char>,
    #[serde(default = "default_2")]
    group_num_precision: u64,

    media: MediaGroups
}
fn default_2() -> u64 { 2 }
impl FinalCheckRequest {
    pub fn is_satisfied_by_media(&self, counted_media: MediaGroups) -> Result<(), FCError> {
        Ok(())
    }
}


pub fn check(dir: String, request_json: Value, ctx: &mut<FilesWs as Actor>::Context) -> std::result::Result<(), FCError> {
    let final_check_request = parse_final_check_request(request_json)?;
    let pattern = format!("{dir}\\*");
    let files = glob(&*pattern).map_err(|e| FCError::InvalidDirectory(e.to_string(), pattern))?;
    let parsed_file_names = parse_file_names(files)?;
    let counted_media = MediaGroups::from_parsed_file_names(parsed_file_names).map_err(|_| FCError::Todo)?;
    final_check_request.is_satisfied_by_media(counted_media)?;

    // Check DPI and stuff after

    Ok(())
}


fn parse_final_check_request(request_json: Value) -> std::result::Result<FinalCheckRequest, FCError> {
    let data = serde_json::from_value::<Data>(request_json).map_err(|e| FCError::InvalidRequest(e.to_string()))?;
    let data = data.data;
    if data.media.slides.is_none() && data.media.negatives.is_none() && data.media.prints.is_none() {
        return Err(FCError::InvalidRequest(String::from("no properties were defined in expecting_media")));
    }

    Ok(data)
}


fn parse_file_names(paths : Paths) -> Result<Vec<ParsedFileName>, FCError> {
    let mut ret = Vec::new();
    
    for entry in paths {
        let entry = entry.map_err(|e| FCError::InvalidFile(e.to_string()))?;
        let file_name = entry.file_name().unwrap().to_str().unwrap();
        let parsed_file_name = ParsedFileName::from(file_name).map_err(|e| FCError::FileNameParsingError(e.to_string(), file_name.to_string()))?;
        ret.push(parsed_file_name);
    }

    Ok(ret)
}