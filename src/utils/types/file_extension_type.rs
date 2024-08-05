use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum FileExtensionType {
    None,
    Jpg,
    Tiff,
}
impl FromStr for FileExtensionType {
    type Err = ();
    fn from_str(input: &str) -> Result<FileExtensionType, Self::Err> {
        match input {
            "jpg" | "jpeg" => Ok(FileExtensionType::Jpg),
            "tif" | "tiff" => Ok(FileExtensionType::Tiff),
            _ => Err(())
        }
    }
}