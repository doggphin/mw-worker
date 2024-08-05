use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
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