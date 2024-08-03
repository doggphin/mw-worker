use actix::Actor;
use crate::FilesWs;

pub fn msg(message: &str, ctx: &mut<FilesWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"msg\": \"{message}\"}}"));
}

pub fn status(message: &str, ctx: &mut<FilesWs as Actor>::Context) -> () {
    ctx.text(format!("{{\"status\": \"{message}\"}}"));
}