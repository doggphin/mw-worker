use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScanType {
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
impl ToString for ScanType {
    fn to_string(&self) -> String {
        match self {
            ScanType::Default => "Default".to_string(),
            ScanType::HandScan => "Hand Scan".to_string(),
            ScanType::OversizedHandScan => "Oversized Hand Scan".to_string(),
        }
    }
}