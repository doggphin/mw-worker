use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ScanType {
    Error,
    Default,
    HandScan,
    OversizedHandScan
}
impl FromStr for ScanType {
    type Err = ();
    fn from_str(input: &str) -> Result<ScanType, Self::Err> {
        match input {
            "" => Ok(ScanType::Default),
            "HS" => Ok(ScanType::HandScan),
            "OSHS" => Ok(ScanType::OversizedHandScan),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum MediaType {
    Prints,
    Slides,
    Negatives
}
impl FromStr for MediaType {
    type Err = ();
    fn from_str(input: &str) -> Result<MediaType, Self::Err> {
        match input {
            "Prints" => Ok(MediaType::Prints),
            "Slides" => Ok(MediaType::Slides),
            "Negs" => Ok(MediaType::Negatives),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum FileExtensionType {
    Error,
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