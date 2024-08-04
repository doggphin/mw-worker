#[derive(Debug)]
pub enum MediaGroupsError {
    
}
impl std::error::Error for MediaGroupsError {}
impl std::fmt::Display for MediaGroupsError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            //MediaGroupsError::InvalidRequest(err) => write!(f, "invalid request: {err}"),
            _ => write!(f, "unimplemented error")
        }
    }
}