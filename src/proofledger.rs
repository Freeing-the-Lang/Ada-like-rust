use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn proof_log(event: &str) {
    let ts = SystemTime::now();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("proofledger.txt")
        .unwrap();
    writeln!(file, "[{:?}] {}", ts, event).unwrap();
}
