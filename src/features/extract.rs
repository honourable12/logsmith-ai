use std::sync::mpsc;

use crate::ingest::parser::LogEvent;

pub struct Features {
    pub request_count: usize;
    pub avg_latency: f64;
}

pub fn aggregate(rx: mpsc::Receiver<LogEvent>) -> Features {
    let mut count = 0usize;
    let mut total_latency: u64 = 0u64;
    
    for event in rx.iter() {
        count += 1;
        total_latency += event.latency_ms;
    }
    
    Features {
        request_count: count,
        avg_latency: total_latency as f64 / count.max(1) as f64,
    }
}