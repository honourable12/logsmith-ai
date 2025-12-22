use std::sync::mpsc;
use std::thread;

mod ingest;
mod pipeline;
mod features;

fn main() {
    let (line_tx, line_rx) = mpsc::channel();
    let (event_tx, event_rx) = mpsc::channel();
    
    let path = "sample.log".to_string();
    
    let reader = thread::spwan({
        let tx = line_tx.clone();
        move || ingest::reader::read_file(path, tx)
    });
    
    pipeline::spawn_workers(4, line_rx, event_tx);
    
    drop(line_tx); // signal no more lines
    drop(event_tx); // signal no more events
    
    let features =  features::extract::aggregate(event_rx);
    
    reader.join().unwrap();
    
    println!("requests={}, avg_latency={}",
        features.request_count,
        features.avg_latency
    );
}