#![feature(let_chains)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(path: &str) -> u32 {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut sum = 0;

    for line in lines.iter() {
        // find the first digit number
        let chars: Vec<char> = line.chars().collect();
        if let Some(left) = chars.iter().find(|&c| c.is_digit(10)) &&
            let Some(right) = chars.iter().rfind(|&c| c.is_digit(10)) {
            sum += left.to_digit(10).unwrap() * 10 + right.to_digit(10).unwrap();
        }
    }

    println!("sum: {}", sum);
    sum
}

fn solve_2(path: &str) -> u32 {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut sum = 0;

    let numbers: Vec<String> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let find_digit = |cur: &str, numbers: &Vec<String>, op: u32| -> Option<u32> {
        let mut res = None;
        let c = if op == 1 {
            cur.chars().next()
        } else {
            cur.chars().last()
        };
        if let Some(f) = c && f.is_digit(10) {
            res = Some(f.to_digit(10).unwrap());
        } else {
            for (i, n) in numbers.iter().enumerate() {
                if (op == 1 && cur.starts_with(n)) || (op == 2 && cur.ends_with(n)) {
                    let v = i as u32 + 1;
                    res = Some(v);
                }
            }
        }
        res
    };
    for line in lines.iter() {
        let mut left = None;
        let mut right = None;
        for i in 0..line.len() {
            let cur = &line[i..];
            if let Some(l) = find_digit(cur, &numbers, 1) {
                left = Some(l);
                break;
            }
        }
        for i in (0..line.len()).rev() {
            let cur = &line[..i + 1];
            if let Some(r) = find_digit(cur, &numbers, 2) {
                right = Some(r);
                break;
            }
        }
        if let Some(left) = left &&
            let Some(right) = right {
            sum += left * 10 + right;
        }
    }

    sum
}

fn main() {
    eprintln!("res1: {}", solve("tests/01_inputs.txt"));
    eprintln!("res2: {}", solve_2("tests/01_inputs.txt"));
}

#[test]
fn test() {
    assert_eq!(solve("tests/01_demo.txt"), 142);
    assert_eq!(solve("tests/01_inputs.txt"), 56042);
    assert_eq!(solve_2("tests/01_demo2.txt"), 281);
    assert_eq!(solve_2("tests/01_inputs.txt"), 55358);
}
