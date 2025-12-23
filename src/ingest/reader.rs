use std::fs::File;
use std::io::{BufRead, BufReader};
use crossbeam::channel::Sender;

pub fn read_file(path: String, tx: Sender<String>) {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        if tx.send(line).is_err() {
            break;
        }
    }
}
