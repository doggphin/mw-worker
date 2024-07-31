use actix_web_actors::ws;
use actix::Actor;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use crate::FilesWs;

use crate::check_names::final_check;

pub struct JobRequest {
    job: String,
    dir: String
}

pub struct Response {
    msg: String
}

#[derive(Deserialize)]
struct ServiceRequest {
    job: String,
    dir: String
}

#[derive(Debug)]
pub enum ServicesError {
    CouldNotParseRequest(String),
    InvalidJob(Option<String>),
    InvalidCheckNamesRequest(String),
}
impl std::error::Error for ServicesError {}
impl std::fmt::Display for ServicesError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
    -> std::fmt::Result {
        match self {
            ServicesError::CouldNotParseRequest(msg) => write!(f, "{}", msg),
            ServicesError::InvalidJob(opt_msg) => match opt_msg {
                    Some(v) => write!(f, "Invalid job requested: {}", v),
                    None => write!(f, "Invalid job requested!")
                },
            ServicesError::InvalidCheckNamesRequest(msg) => write!(f, "{}", msg)
            
        }
    }
}

pub fn service_router(request: String, ctx: &mut<FilesWs as Actor>::Context) -> Result<(), ServicesError> {
    let json: Value = match serde_json::from_str(&request) {
        Ok(v) => v,
        Err(_) => { return Err(ServicesError::CouldNotParseRequest(String::from("Could not parse base json"))); }
    };
    let job_request = match parse_base_job(&json) {
        Ok(v) => v,
        Err(_) => { return Err(ServicesError::InvalidJob(None)); }
    };
    return match &*job_request.job {
        "final_check" => match final_check(job_request.dir, json.clone(), ctx) {
            Ok(_) => Ok(()),
            Err(e) => Err(ServicesError::InvalidCheckNamesRequest(e.to_string()))
        },
        "check_is_corrected" => Ok(()),
        _ => Err(ServicesError::InvalidJob(Some(String::from("Invalid job type specified in request!"))))
    }
}

fn parse_base_job(request_json: &Value) -> std::result::Result<ServiceRequest, serde_json::Error> {
    serde_json::from_value(request_json.clone())
}