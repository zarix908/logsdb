mod engine;
mod fsstore;
mod log;
mod rle;
mod store;

use ::log::error;
use actix_web::{error, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::mpsc::{sync_channel, SyncSender, TrySendError};
use std::thread;

use crate::log::Log;
use engine::Engine;
use fsstore::FsStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = sync_channel::<Log>(100);

    let kilobyte = 1024;
    let handle = thread::spawn(move || {
        let store = FsStore::new().expect("create store failed");
        let mut engine = Engine::new(4 * kilobyte, store);

        for log in receiver {
            let r = engine.insert(log);
            if let Err(err) = r {
                error!("insert log failed: {}", err)
            }
        }
    });

    let sender_data = web::Data::new(sender);
    let r = HttpServer::new(move || App::new().app_data(sender_data.clone()).service(insert))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    if r.is_err() {
        return r;
    }

    handle.join().or(Ok(()))
}

#[post("/insert")]
async fn insert(
    log: web::Json<Log>,
    sender: web::Data<SyncSender<Log>>,
) -> Result<impl Responder, error::Error> {
    let err = sender.try_send(log.0);

    if let Result::Err(send_err) = err {
        return match send_err {
            TrySendError::Full(_) => {
                Err(error::ErrorInternalServerError("insertion queue is full"))
            }
            TrySendError::Disconnected(_) => Err(error::ErrorInternalServerError("internal error")),
        };
    }

    Ok(HttpResponse::Ok())
}
