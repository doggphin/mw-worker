use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct PhotoGroupOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<u64>,
    #[serde(default = "default_0")]
    pub scanner: u64,
    #[serde(default = "default_0")]
    pub hs: u64,
    #[serde(default = "default_0")]
    pub oshs: u64,
    #[serde(default = "default_false")]
    pub is_corrected: bool,
    #[serde(default = "default_3")]
    pub index_precision: u64
}
fn default_0() -> u64 { 0 }
fn default_false() -> bool { false }
fn default_3() -> u64 { 3 }
impl PhotoGroupOptions {
    pub fn new() -> PhotoGroupOptions {
        PhotoGroupOptions{dpi: None, scanner: 0, hs: 0, oshs: 0, is_corrected: false, index_precision: 0 }
    }
}