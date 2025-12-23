use crate::ingest::parser::LogEvent;
use rayon::prelude::*;

pub fn parse_parallel(lines: Vec<String>) -> Vec<LogEvent> {
    lines
        .par_iter()
        .filter_map(|line| serde_json::from_str::<LogEvent>(line).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_json_lines_in_parallel() {
        let lines = vec![
            r#"{"timestamp":"2025-11-08T10:00:00Z","level":"INFO","latency_ms":120}"#.to_string(),
            r#"{"timestamp":"2025-11-08T10:01:00Z","level":"INFO","latency_ms":80}"#.to_string(),
        ];

        let events = parse_parallel(lines);

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].latency_ms + events[1].latency_ms, 200);
    }
}
