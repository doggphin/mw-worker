use actix::Actor;
use serde_json::Value;
use serde::Deserialize;
use glob::{glob, Paths};
use crate::{FilesWs};

pub mod media_file;
pub mod media_folder;
pub mod error;
pub mod media_groups;
pub mod photo_group_options;

use error::FCError;
use media_file::MediaFile;
use media_groups::MediaGroupOptions;
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
    group_num_precision: u64,   // Guaranteed 6 or less

    media: MediaGroupOptions
}
fn default_2() -> u64 { 2 }
impl FinalCheckRequest {

}


pub fn final_check(dir: String, request_json: Value, ctx: &mut<FilesWs as Actor>::Context) -> std::result::Result<(), FCError> {
    let final_check_req = parse_final_check_request(request_json)?;

    let pattern = build_directory_pattern(&dir, &final_check_req)?;
    
    let files = glob(&*pattern).map_err(|e| FCError::InvalidDirectory(e))?;
    let media_files = parse_media_files(files)?;
    let counted_media = MediaGroupOptions::from_media_files(&media_files).map_err(|e| FCError::MediaGroupingError(e))?;
    counted_media.counts_equal(final_check_req.media).map_err(|e| FCError::IncorrectMediaCount(e))?;

    final_check_req.media.check_against_media_files(&media_files).map_err(|e| FCError::IncorrectMetadataError(e))?;

    Ok(())
}


fn build_directory_pattern(dir: &String, final_check_request: &FinalCheckRequest) -> Result<String, FCError> {
    let mut ret = format!("{dir}\\");

    if let Some(num) = final_check_request.group_num.and_then(|num| Some(num.to_string())) {
        let precision_difference: usize = usize::try_from(final_check_request.group_num_precision).unwrap() - num.len();
        if precision_difference > 0 {
            let padding = str::repeat("0", precision_difference);
            ret.push_str(&*format!("{padding}{num}\\"));
        }
    }
    
    ret.push('*');
    Ok(ret)
}

fn parse_final_check_request(request_json: Value) -> std::result::Result<FinalCheckRequest, FCError> {
    let data = serde_json::from_value::<Data>(request_json).map_err(|e| FCError::DeserializeError(e))?;
    let data = data.data;
    if data.media.slides.is_none() && data.media.negatives.is_none() && data.media.prints.is_none() {
        return Err(FCError::InvalidRequest("no properties were defined in request for expecting_media".to_string()));
    }
    if let Some(group_num) = data.group_num {
        if u64::try_from(group_num.to_string().len()).unwrap() > data.group_num_precision {
            return Err(FCError::InsufficientGroupNumberPrecision(group_num, data.group_num_precision))
        }
    }
    if data.group_num_precision > 6 {
        return Err(FCError::GroupNumberPrecisionTooHigh(data.group_num_precision))
    }

    Ok(data)
}


fn parse_media_files(paths : Paths) -> Result<Vec<MediaFile>, FCError> {
    let mut ret = Vec::new();
    
    for entry in paths {
        let path = entry.map_err(|e| FCError::InvalidFile(e))?;
        let media_file = MediaFile::from_path(&path).map_err(|e| FCError::MediaFileParseError(path, e))?;
        ret.push(media_file);
    }

    Ok(ret)
}