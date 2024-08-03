#[derive(Debug)]
pub enum ServicesError {
    RequestParseError(String),
    InvalidJob(Option<String>),
    InvalidFinalCheck(String),
}
impl std::error::Error for ServicesError {}
impl std::fmt::Display for ServicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            ServicesError::RequestParseError(err) => write!(f, "could not parse base json: {err}"),
            ServicesError::InvalidJob(opt_msg) => match opt_msg {
                    Some(v) => write!(f, "Invalid job requested: {}", v),
                    None => write!(f, "Invalid job requested!")
                },
            ServicesError::InvalidFinalCheck(msg) => write!(f, "{}", msg)  
        }
    }
}