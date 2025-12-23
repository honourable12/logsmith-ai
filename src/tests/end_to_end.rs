use std::fs::File;
use std::io::Write;
use crossbeam_channel;
use std::thread;

use logsmith::{
    features::window::aggregate_windows,
    ingest::reader::read_file,
    pipeline::pipeline::spawn_workers,
};

#[test]
fn processes_log_file_end_to_end() {
    let mut file = File::create("test.log").unwrap();

    writeln!(
        file,
        r#"{"timestamp":"2025-11-08T10:00:00Z","level":"INFO","latency_ms":100}"#
    ).unwrap();

    writeln!(
        file,
        r#"{"timestamp":"2025-11-08T10:01:00Z","level":"INFO","latency_ms":200}"#
    ).unwrap();

    let (line_tx, line_rx) = crossbeam_channel::unbounded();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();

    let reader = thread::spawn(move || {
        read_file("test.log".into(), line_tx);
    });

    spawn_workers(2, line_rx, event_tx);

    let windows = aggregate_windows(event_rx, 300);

    reader.join().unwrap();

    assert_eq!(windows.len(), 1);
    assert_eq!(windows[0].request_count, 2);
    assert_eq!(windows[0].avg_latency, 150.0);
}
