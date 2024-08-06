use super::{media_file::MediaFile, media_groups::MediaGroupValues};
pub struct MediaFolder {
    pub files : Vec<MediaFile>,
    pub group_options: MediaGroupValues
}