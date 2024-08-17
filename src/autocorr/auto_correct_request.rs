use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AutoCorrectBatchRequest {
    pub from_folder: String,
    pub to_folder: String
}

#[derive(Deserialize, Debug)]
pub struct AutoCorrectSingleRequest {
    pub from_path: String,
    pub to_folder: String
}