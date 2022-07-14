mod cursor;
mod engine;
mod fsstore;
mod log;
mod rle;
mod store;

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use cursor::Cursor;
use serde::Deserialize;

use crate::log::Log;
use engine::Engine;
use fsstore::FsStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let kilobyte = 1024;
    let store = FsStore::new().expect("create store failed");
    let mut engine = Engine::new(4 * kilobyte, store);

    crossbeam::scope(|s| {
        s.spawn(|_| {
            engine.run();
        });
    })
    .unwrap();

    let sender_data = web::Data::new(engine);
    HttpServer::new(move || {
        App::new()
            .app_data(sender_data.clone())
            .service(read)
            .service(insert)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}

#[post("/insert")]
async fn insert(
    log: web::Json<Log>,
    engine: web::Data<Engine<FsStore>>,
) -> Result<impl Responder, error::Error> {
    engine.insert(log.0);
    Ok(HttpResponse::Ok())
}

#[derive(Deserialize)]
struct Params {
    cursor: String,
}

#[get("/read")]
async fn read(params: web::Query<Params>) -> Result<impl Responder, error::Error> {
    let cursor_json =
        base64::decode(params.cursor.clone()).map_err(|e| error::ErrorBadRequest(e))?;
    let cursor: Cursor = serde_json::from_slice(cursor_json.as_slice())?;

    Ok(HttpResponse::Ok())
}
