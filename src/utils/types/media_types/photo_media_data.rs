pub mod error;
use error::MetadataCheckError;

#[derive(Debug, Clone)]
pub struct PhotoMediaData {
    dpi: u32,
    is_corrected: bool,
    scan_type: ScanType,
}
impl PhotoMediaData {
    pub fn new() -> PhotoMediaData {
        PhotoMediaData { dpi: 0, is_corrected: false, scan_type: ScanType::Default }
    }

    pub fn set_data_from_path(path: &PathBuf) -> Result((), PhotoMediaDataError) {
        let mut metadata = Metadata::new_from_path(&path).unwrap();
        let dpi = metadata.get_tag_by_hex(0x011a).unwrap().value_as_u8_vec(&Endian::Little);
        let first_u32 = u32::from_le_bytes(dpi[0..4].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
        let second_u32 = u32::from_le_bytes(dpi[4..8].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
        let dpi = first_u32 / second_u32;
        Ok(())
    }
}