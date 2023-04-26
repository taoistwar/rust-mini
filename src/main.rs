use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use serde_json::Value;

fn main() {
    let file = File::open("/mnt/d/data/机型库.txt").unwrap();
    let reader = BufReader::new(file);

    let mut first = true;
    for line in reader.lines() {
        let line = line.unwrap();
        let line = match first {
            true => {
                // 去除 BOM
                first = false;
                String::from_utf8_lossy(&line.as_bytes()[3..]).to_string()
            }
            _ => line,
        };
        let json = serde_json::from_str(&line).unwrap_or(Value::Null);

        println!("{:?}", json);
    }
}
