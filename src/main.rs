mod engine;
mod log;
mod writer;

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use crossbeam_queue::ArrayQueue;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time::Duration};

use engine::Engine;
use log::Log;
use writer::Writer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let queue = web::Data::new(ArrayQueue::<Log>::new(100));

    HttpServer::new(move || App::new().app_data(queue.clone()).service(insert))
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
async fn insert(log: web::Json<Log>, queue: web::Data<ArrayQueue<Log>>) -> impl Responder {
    queue.force_push(log.0);
    HttpResponse::Ok()
}
