use actix::Actor;
use serde::Deserialize;
use serde_json::Value;
use crate::FilesWs;
use crate::utils::send_text;
use crate::qc::fc;

mod error;
use error::ServicesError;

#[derive(Deserialize)]
struct ServiceRequest {
    job: String,
    dir: String
}

pub fn service_router(request: String, ctx: &mut<FilesWs as Actor>::Context) -> Result<(), ServicesError> {
    let json: Value = match serde_json::from_str(&request) {
        Ok(v) => v,
        Err(e) => { return Err(ServicesError::RequestParseError(e.to_string())); }
    };
    let job_request = match parse_base_job(&json) {
        Ok(v) => v,
        Err(_) => { return Err(ServicesError::InvalidJob(None)); }
    };
    return match &*job_request.job {
        "final_check" => {
            send_text::status("busy", ctx);
            send_text::msg("Starting final check!", ctx);
            match fc::check(job_request.dir, json.clone(), ctx) {
                Ok(_) => {
                    send_text::msg("Final check successful!", ctx);
                    send_text::status("success", ctx);
                    Ok(())
                }
                Err(e) => {
                    send_text::msg("Final check unsuccessful!", ctx);
                    send_text::status("failure", ctx);
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