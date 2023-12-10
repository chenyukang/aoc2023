use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_to_lines(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    reader.lines().map(|l| l.unwrap()).collect()
}
