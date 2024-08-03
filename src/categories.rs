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
    Error,
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
pub enum FileExtension {
    Error,
    Jpg,
    Tiff,
}
impl FromStr for FileExtension {
    type Err = ();
    fn from_str(input: &str) -> Result<FileExtension, Self::Err> {
        match input {
            "jpg" | "jpeg" => Ok(FileExtension::Jpg),
            "tif" | "tiff" => Ok(FileExtension::Tiff),
            _ => Err(())
        }
    }
}