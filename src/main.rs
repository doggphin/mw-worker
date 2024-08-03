use actix::{Actor, StreamHandler};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws::{self};

mod handlers;
use handlers::jobs;
mod qc;
mod utils;
use utils::send_text;

struct FilesWs;
impl Actor for FilesWs {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for FilesWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                print!("Received a message:\n{}\n", text);
                if let Err(e) = jobs::service_router(text.to_string(), ctx) {
                    println!("Closing socket because: {}", e.to_string());
                    ctx.close(Some(ws::CloseReason { code: ws::CloseCode::Error, description: Some(e.to_string()) } ))
                }
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Opened a socket!");
        send_text::msg("Connected to a worker!", ctx);
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(FilesWs {}, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip = "localhost";
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