pub mod error;
use std::path::PathBuf;
use error::PhotoMediaDataError;
use little_exif::{endian::Endian, metadata::Metadata};
use crate::utils::types::scan_type::ScanType;

#[derive(Debug, Clone)]
pub struct  PhotoMediaData {
    dpi: u32,
    is_corrected: bool,
    true_scan_type: ScanType,
}
impl PhotoMediaData {

    pub fn new() -> PhotoMediaData {
        PhotoMediaData { dpi: 0, is_corrected: false, true_scan_type: ScanType::Default }
    }

    pub fn from_path(path: &PathBuf) -> Result<PhotoMediaData, PhotoMediaDataError> {
        fn get_dpi(metadata: &Metadata, hex_code: u16) -> Result<u32, PhotoMediaDataError> {
            let dpi = metadata.get_tag_by_hex(hex_code).unwrap().value_as_u8_vec(&Endian::Little);
            let first_u32 = u32::from_le_bytes(dpi[0..4].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
            let second_u32 = u32::from_le_bytes(dpi[4..8].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
            return Ok(first_u32 / second_u32);
        }

        let metadata = Metadata::new_from_path(&path).unwrap();

        // Get DPI
        let horiz_dpi = get_dpi(&metadata, 0x011a)?;
        let vert_dpi = get_dpi(&metadata, 0x011b)?;
        if horiz_dpi != vert_dpi {
            return Err(PhotoMediaDataError::DifferentXYDpi(horiz_dpi, vert_dpi));
        }
        let dpi = horiz_dpi;

        let software = metadata.get_tag_by_hex(0x0131).unwrap().value_as_u8_vec(&Endian::Little);
        let software = &*String::from_utf8_lossy(&software).to_ascii_lowercase();
        let is_corrected = software.contains("photoshop");

        let hardware = metadata.get_tag_by_hex(0x0110).unwrap().value_as_u8_vec(&Endian::Little);
        let hardware = &*String::from_utf8_lossy(&hardware).to_ascii_lowercase();
        let mut true_scan_type = ScanType::Unknown;
        if(hardware.contains("powerslide")) {
            true_scan_type = ScanType::Default;
        }
        
        // Todo: Set is_corrected and scan_type correctly
        Ok(PhotoMediaData{dpi, is_corrected, true_scan_type })
    }
}