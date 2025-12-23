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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingest::parser::LogEvent;
    use chrono::{TimeZone, Utc};
    use crossbeam_channel;

    #[test]
    fn aggregates_events_into_single_window() {
        let (tx, rx) = crossbeam_channel::unbounded();

        let events = vec! {
            LogEvent {
                timestamp: Utc.with_ymd_and_hms(2025, 11, 8, 10, 0, 0).unwrap(),
                level: "INFO".into(),
                latency_ms: 100,
            },
            LogEvent {
                timestamp: Utc.with_ymd_and_hms(2025, 11, 8, 10, 2, 0).unwrap(),
                level: "INFO".into(),
                                latency_ms: 200,
                            },
        };

                        for e in events {
                            tx.send(e).unwrap();
                        }
                        drop(tx);

                        let windows = aggregate_windows(rx, 300);

                        assert_eq!(windows.len(), 1);
                        assert_eq!(windows[0].request_count, 2);
                        assert_eq!(windows[0].avg_latency, 150.0);
                    }
                }
