use actix::Actor;
use serde::Deserialize;
use serde_json::Value;
use crate::FilesWs;

use crate::files_utils::{send_message, send_status};
use crate::final_checker::final_check;

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
    InvalidFinalCheck(String),
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
            ServicesError::InvalidFinalCheck(msg) => write!(f, "{}", msg)
            
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
        "final_check" => {
            send_status("busy", ctx);
            send_message("Starting final check!", ctx);
            match final_check(job_request.dir, json.clone(), ctx) {
                Ok(_) => {
                    send_message("Final check successful!", ctx);
                    send_status("success", ctx);
                    Ok(())
                }
                Err(e) => {
                    send_message("Final check unsuccessful!", ctx);
                    send_status("failure", ctx);
                    Err(ServicesError::InvalidFinalCheck(e.to_string()))
                }
            }
        },
        "check_is_corrected" => Ok(()),
        _ => Err(ServicesError::InvalidJob(Some(String::from("Invalid job type specified in request!"))))
    }
}

fn parse_base_job(request_json: &Value) -> std::result::Result<ServiceRequest, serde_json::Error> {
    serde_json::from_value(request_json.clone())
}