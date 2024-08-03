use actix::Actor;
use serde_json::Value;
use serde::Deserialize;
use glob::{glob, Paths};
use crate::FilesWs;

mod file_names;
mod error;
use error::FCError;
use file_names::ParsedFileName;

#[derive(Deserialize, Debug)]
struct Data {
    data: FinalCheckRequest
}

#[derive(Deserialize, Debug)]
struct FinalCheckRequest {
    first_name: String,
    last_name: String,
    group_num: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_char: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_num_precision: Option<u64>,   // Default 2
    media_groups: MediaGroups
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug)]
struct MediaGroups {
    slides: Option<PhotoGroupOptions>,
    prints: Option<PhotoGroupOptions>,
    negatives: Option<PhotoGroupOptions>
}
impl MediaGroups {
    pub fn is_finished(&self, expecting_media: MediaGroups) -> Result<(), FCError> {
        fn counts_are_okay(expected: Option<u64>, counted: Option<u64>, media_and_scan_types: &str) -> Result<(), FCError> {
            if let Some(expected_value) = expected {
                if let Some(counted_value) = counted {
                    if expected_value != counted_value {
                        Err(FCError::ScanTypeRemaining(media_and_scan_types.to_string(), expected_value, counted_value))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(FCError::NoneOfFileTypeFound(media_and_scan_types.to_string()))
                }
            } else {
                Ok(())
            }
        }

        if let Some(expecting) = expecting_media.slides {
            let counted = match &self.slides {
                Some(v) => v,
                None => return Err(FCError::NoneOfFileTypeFound("slides".to_string()))
            };
            counts_are_okay(expecting.scanner, counted.scanner, "scanner slides")?;
            counts_are_okay(expecting.hs, counted.hs, "handscan slides")?;
        }
        if let Some(expecting) = expecting_media.prints {
            let counted = match &self.prints {
                Some(v) => v,
                None => return Err(FCError::NoneOfFileTypeFound("prints".to_string()))
            };
            counts_are_okay(expecting.scanner, counted.scanner, "scanner prints")?;
            counts_are_okay(expecting.hs, counted.hs, "handscan prints")?;
            counts_are_okay(expecting.oshs, counted.oshs, "oversized handscan prints")?;
        }
        if let Some(expecting) = expecting_media.negatives {
            let counted = match &self.negatives {
                Some(v) => v,
                None => return Err(FCError::NoneOfFileTypeFound("negatives".to_string()))
            };
            counts_are_okay(expecting.scanner, counted.scanner, "scanner negatives")?;
            counts_are_okay(expecting.hs, counted.hs, "handscan negatives")?;
        }

        Ok(())
    }
}

#[serde_with::skip_serializing_none]
#[derive(Deserialize, Debug)]
struct PhotoGroupOptions {
    dpi: Option<u64>,               // Defaults (does not check)
    scanner: Option<u64>,           // Defaults 0
    hs: Option<u64>,                // Defaults 0
    oshs: Option<u64>,              // Defaults 0
    is_corrected: Option<bool>,     // Defaults (does not check)
    index_precision: Option<u64>    // Defaults 3 - should this even be defined..?
}

pub fn check(dir: String, request_json: Value, ctx: &mut<FilesWs as Actor>::Context) -> std::result::Result<(), FCError> {
    let final_check_request = parse_final_check_request(request_json)?;
    let pattern = format!("{dir}\\*");
    let files = glob(&*pattern).map_err(|e| FCError::InvalidDirectory(e.to_string(), pattern))?;
    let parsed_file_names = parse_file_names(files)?;

    Ok(())
}

fn parse_final_check_request(request_json: Value) -> std::result::Result<FinalCheckRequest, FCError> {
    let data = serde_json::from_value::<Data>(request_json).map_err(|e| FCError::InvalidRequest(e.to_string()))?;
    let data = data.data;
    if data.media_groups.slides.is_none() && data.media_groups.negatives.is_none() && data.media_groups.prints.is_none() {
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