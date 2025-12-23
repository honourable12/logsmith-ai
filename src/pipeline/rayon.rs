use crate::ingest::parser::LogEvent;
use rayon::prelude::*;

pub fn parse_parallel(lines: Vec<String>) -> Vec<LogEvent> {
    lines
        .par_iter()
        .filter_map(|line| serde_json::from_str::<LogEvent>(line).ok())
        .collect()
}
