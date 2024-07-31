use actix::Actor;
use crate::FilesWs;

pub fn send_message(message: String, ctx: &mut<FilesWs as Actor>::Context) -> () {
    let my_string = format!("{{\"msg\": \"{message}\"}}");
    ctx.text(my_string);
}