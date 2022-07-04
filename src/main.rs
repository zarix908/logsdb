mod engine;
mod log;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time::Duration};

use engine::Engine;
use log::Log;

fn main() {
    let mut engine = Engine::new(120);

    for i in 0..4 {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        thread::sleep(Duration::from_millis(1000));
        let log = Log {
            timestamp: current_time,
            ip: [192, 168, 103, 200 + i],
            request: String::from("GET /api HTTP/1.1"),
        };

        engine.insert(log);
    }
}
