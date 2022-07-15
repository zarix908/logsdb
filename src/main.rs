mod cursor;
mod engine;
mod fsstore;
mod log;
mod memtable;
mod rle;
mod size;
mod store;

use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder};
use cursor::Cursor;
use serde::Deserialize;
use std::sync::Arc;

use crate::log::Log;
use engine::Engine;
use fsstore::FsStore;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let kilobyte = 1024;
    let store = FsStore::new().expect("create store failed");
    let engine = Arc::new(Engine::new(4 * kilobyte));

    let engine_clone = Arc::clone(&engine);
    let handle = std::thread::spawn(move || {
        engine_clone.run(store);
    });

    let sender_data = web::Data::new(Arc::clone(&engine));
    HttpServer::new(move || {
        App::new()
            .app_data(sender_data.clone())
            .service(read)
            .service(insert)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    engine.stop();
    handle.join().expect("engine running error");

    Ok(())
}

#[post("/insert")]
async fn insert(
    log: web::Json<Log>,
    engine: web::Data<Arc<Engine>>,
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
