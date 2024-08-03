use actix::Actor;
use crate::FilesWs;

pub fn send_message(message: &str, ctx: &mut<FilesWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"msg\": \"{message}\"}}"));
}

pub fn send_status(message: &str, ctx: &mut<FilesWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"status\": \"{message}\"}}"));
}