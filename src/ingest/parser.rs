use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LogEvent {
    pub timestamp: DataTime<Utc>,
    pub level: String,
    pub latency_ms: u64,
}