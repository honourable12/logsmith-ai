use crate::ingest::parser::LogEvent;
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct WindowFeatures {
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub request_count: usize,
    pub avg_latency: f64,
}

pub fn aggregate_windows(
    rx: crossbeam_channel::Receiver<LogEvent>,
    window_size_secs: i64,
) -> Vec<WindowFeatures> {
    let mut windows = Vec::new();

    let mut current_start: Option<DateTime<Utc>> = None;
    let mut count = 0usize;
    let mut total_latency = 0u64;

    for event in rx.iter() {
        let window_start = match current_start {
            Some(start) => start,
            None => {
                current_start = Some(event.timestamp);
                event.timestamp
            }
        };

        if event.timestamp - window_start < Duration::seconds(window_size_secs) {
            count += 1;
            total_latency += event.latency_ms;
        } else {
            windows.push(WindowFeatures {
                window_start,
                window_end: window_start + Duration::seconds(window_size_secs),
                request_count: count,
                avg_latency: total_latency as f64 / count.max(1) as f64,
            });

            current_start = Some(event.timestamp);
            count = 1;
            total_latency = event.latency_ms;
        }
    }

    if let Some(start) = current_start {
        windows.push(WindowFeatures {
            window_start: start,
            window_end: start + Duration::seconds(window_size_secs),
            request_count: count,
            avg_latency: total_latency as f64 / count.max(1) as f64,
        });
    }

    windows
}
