mod engine;
mod log;
mod writer;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, error};
use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time::Duration};

use engine::Engine;
use log::Log;
use writer::Writer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = sync_channel::<Log>(100);
    let senderData = web::Data::new(sender);

    HttpServer::new(move || App::new().app_data(senderData.clone()).service(insert))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

    // let writer = Writer::new().expect("create writer failed");
    // let mut engine = Engine::new(120, writer);

    // for i in 0..4 {
    //     let current_time = SystemTime::now()
    //         .duration_since(UNIX_EPOCH)
    //         .expect("Time went backwards")
    //         .as_millis();
    //     thread::sleep(Duration::from_millis(1000));
    //     let log = Log {
    //         timestamp: current_time,
    //         ip: [192, 168, 103, 200 + i],
    //         request: String::from("GET /api HTTP/1.1"),
    //     };

    //     engine.insert(log).expect("insert log failed");
    // }
}

#[post("/insert")]
async fn insert(log: web::Json<Log>, sender: web::Data<SyncSender<Log>>) -> Result<impl Responder, error::Error> {
    let err = sender.try_send(log.0).err();

    if let Some(send_err) = err {
        return match send_err {
            TrySendError::Full(_) => Err(error::ErrorInternalServerError("insertion queue is full")),
            TrySendError::Disconnected(_) => Err(error::ErrorInternalServerError("internal error"))
        };
    }

    Ok(HttpResponse::Ok())
}
