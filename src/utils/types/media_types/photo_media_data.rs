pub mod error;
use std::{panic, path::PathBuf};
use error::PhotoMediaDataError;
use little_exif::{endian::Endian, metadata::Metadata};
use crate::utils::types::scan_type::ScanType;

#[derive(Debug, Clone, Copy)]
pub struct  PhotoMediaData {
    pub dpi: u64,
    pub is_corrected: bool,
    pub true_scan_type: Option<ScanType>,
}
impl PhotoMediaData {   
    pub fn from_path(path: &PathBuf) -> Result<PhotoMediaData, PhotoMediaDataError> {
        fn get_dpi(metadata: &Metadata, hex_code: u16) -> Result<u32, PhotoMediaDataError> {
            let dpi = metadata.get_tag_by_hex(hex_code).ok_or(PhotoMediaDataError::NoDpiFound)?.value_as_u8_vec(&Endian::Little);
            let first_u32 = u32::from_le_bytes(dpi[0..4].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
            let second_u32 = u32::from_le_bytes(dpi[4..8].try_into().map_err(|_| PhotoMediaDataError::BadlyFormattedExifTag("dpi".to_string()))?);
            return Ok(first_u32 / second_u32);
        }

        let metadata = panic::catch_unwind(|| Metadata::new_from_path(&path))
            .map_err(|_| PhotoMediaDataError::CouldNotReadPath(path.clone()))?.map_err(|_| PhotoMediaDataError::CouldNotReadPath(path.clone()))?;

        // Get DPI
        let horiz_dpi = get_dpi(&metadata, 0x011a)?;
        let vert_dpi = get_dpi(&metadata, 0x011b)?;
        if horiz_dpi != vert_dpi {
            return Err(PhotoMediaDataError::DifferentXYDpi(horiz_dpi, vert_dpi));
        }
        let dpi = u64::from(horiz_dpi);

        // Get software if used
        let mut is_corrected = false;
        if let Some(tag) = metadata.get_tag_by_hex(0x0131) {
            let software = &*String::from_utf8_lossy(&tag.value_as_u8_vec(&Endian::Little)).to_ascii_lowercase();
            if software.contains("photoshop") {
                is_corrected = true;
            }
            println!("{is_corrected}");
        }

        // Get hardware used to capture
        let mut true_scan_type = None;
        if let Some(tag) = metadata.get_tag_by_hex(0x0110) {
            let hardware = &*String::from_utf8_lossy(&tag.value_as_u8_vec(&Endian::Little)).to_ascii_lowercase();
            if hardware.contains("powerslide") {
                true_scan_type = Some(ScanType::Default);
            }
        }
        
        // Todo: Set is_corrected and scan_type correctly
        Ok(PhotoMediaData{dpi, is_corrected, true_scan_type })
    }
}