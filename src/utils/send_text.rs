use actix::Actor;
use crate::WorkerWs;

pub fn msg(message: &str, ctx: &mut<WorkerWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"msg\": \"{message}\"}}"));
}

pub fn status(message: &str, ctx: &mut<WorkerWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"status\": \"{message}\"}}"));
}