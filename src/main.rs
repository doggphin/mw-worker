#![allow(dead_code)]
use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self};

mod handlers;
mod qc;
mod utils;
mod autocorr;
use utils::send_text::{self, WsStatus};
use handlers::jobs;

struct WorkerWs;
impl Actor for WorkerWs {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WorkerWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                print!("Received a message:\n{}\n", text);
                let _ = jobs::service_router(text.to_string(), ctx);
            }
            Ok(ws::Message::Close(close_reason)) => {
                println!("Received a close message!");
                if let Some(reason) = &close_reason {
                    println!("Closing socket because: {} ({})", reason.description.clone().unwrap_or("(no close reason provided)".to_string()), u16::from(reason.code))
                }
                ctx.close(close_reason);
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Opened a socket!");
        send_text::send("Connected to a worker!", Some(WsStatus::Success), ctx);
    }

    fn finished(&mut self, _ctx: &mut Self::Context) {
        println!("Closed a socket!");
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WorkerWs {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "127.0.0.1";
    let port = 7001;
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new().service(
            web::scope("/ws")
                .route("/", web::get().to(index))
        )
        .wrap(cors)
    });
    match server.bind((ip, port)) {
        Ok(v) => {
            println!("Hosting at {}:{}", ip, port);
            v.run().await
        }
        Err(e) => { 
            println!("{}", e.to_string());
            Err(e)   
        }
    }
}