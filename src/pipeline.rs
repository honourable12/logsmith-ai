use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::ingest::parser::LogEvent;

pub fn spawn_workers(
    n: usize,
    rx: Receiver<String>,
    tx: Sender<LogEvent>,
) {
    for _ in 0..n {
        let rx = rx.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            for line in rx.iter() {
                if let Ok(event) = serde_json::from_str::<LogEvent>(&line) {
                    let _ = tx.send(event);
                }
            }
        });
    }
}
