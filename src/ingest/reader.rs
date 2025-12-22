use std::fs::file;
use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::Sender;

pub fn read_lines(path: &str) -> io::Result<impl Iterator<Item = String>> {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    
    for line in reader.lines().flatten() {
        if tx.send(line).is_err() {
            break;
        }
    }
}