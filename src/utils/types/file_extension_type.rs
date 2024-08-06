use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum FileExtensionType {
    None,
    Jpeg,
    Tiff,
}
impl FromStr for FileExtensionType {
    type Err = ();
    fn from_str(input: &str) -> Result<FileExtensionType, Self::Err> {
        match input {
            "jpg" | "jpeg" => Ok(FileExtensionType::Jpeg),
            "tif" | "tiff" => Ok(FileExtensionType::Tiff),
            _ => Err(())
        }
    }
}
impl ToString for FileExtensionType {
    fn to_string(&self) -> String {
        match self {
            FileExtensionType::None => "".to_string(),
            FileExtensionType::Jpeg => "jpg".to_string(),
            FileExtensionType::Tiff => "tif".to_string()
        }
    }
}