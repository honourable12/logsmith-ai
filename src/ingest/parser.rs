use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LogEvent {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub latency_ms: u64,
}
