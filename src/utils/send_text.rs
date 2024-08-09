use actix::Actor;
use crate::WorkerWs;

pub enum WsStatus {
    Success,
    Failure,
    Busy,
}
impl ToString for WsStatus {
    fn to_string(&self) -> String {
        match self {
            WsStatus::Success => "success".to_string(),
            WsStatus::Failure => "failure".to_string(),
            WsStatus::Busy => "busy".to_string()
        }
    }
}

pub fn send(msg: &str, status: Option<WsStatus>, ctx: &mut<WorkerWs as Actor>::Context) -> () {
    let status_msg = match status {
        Some(status) => status.to_string(),
        None => "".to_string()
    };
    ctx.text(format!("{{\"msg\":\"{}\", \"status\":\"{status_msg}\"}}", msg.replace('\"', "\\\"")))
}