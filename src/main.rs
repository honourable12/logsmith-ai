use crossbeam_channel;
use std::thread;

mod features;
mod ingest;
mod output;
mod pipeline;

use output::csv::write_csv;
use output::parquet::write_parquet;

fn main() {
    env_logger::init();
    log::info!("starting logsmith pipeline");

    let (line_tx, line_rx) = crossbeam_channel::unbounded();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();

    let path = "sample.log".to_string();

    let reader = thread::spawn({
        let tx = line_tx.clone();
        move || ingest::reader::read_file(path, tx)
    });

    pipeline::pipeline::spawn_workers(4, line_rx, event_tx.clone());

    drop(line_tx); // signal no more lines
    drop(event_tx); // signal no more events

    let features = features::extract::aggregate(event_rx.clone());

    reader.join().unwrap();

    println!(
        "requests={}, avg_latency={}",
        features.request_count, features.avg_latency
    );

    let windows = features::window::aggregate_windows(event_rx, 300);

    write_csv("features.csv", &windows).expect("failed to write CSV");
    write_parquet("features.parquet", &windows).expect("failed to write Parquet");
}
