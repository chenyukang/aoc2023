#![feature(let_chains)]
use aoc2023::helper::read_to_lines;

fn solve(path: &str) -> u32 {
    let lines = read_to_lines(path);
    lines
        .iter()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            if let Some(left) = chars.iter().find(|&c| c.is_digit(10)) &&
                let Some(right) = chars.iter().rfind(|&c| c.is_digit(10)) {
                left.to_digit(10).unwrap() * 10 + right.to_digit(10).unwrap()
                } else {
                    0
                }
        })
        .sum()
}

fn solve_2(path: &str) -> u32 {
    let lines = read_to_lines(path);
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
    lines
        .iter()
        .map(|line| {
            let left = (0..line.len())
                .find_map(|i| {
                    if let Some(l) = find_digit(&line[i..], &numbers, 1) {
                        Some(l)
                    } else {
                        None
                    }
                })
                .unwrap();

            let right = (0..line.len())
                .rev()
                .find_map(|i| {
                    if let Some(r) = find_digit(&line[..i + 1], &numbers, 2) {
                        Some(r)
                    } else {
                        None
                    }
                })
                .unwrap();
            left * 10 + right
        })
        .sum()
}

fn main() {
    eprintln!("res1: {}", solve("tests/01_input.txt"));
    eprintln!("res2: {}", solve_2("tests/01_input.txt"));
}

#[test]
fn test() {
    assert_eq!(solve("tests/01_demo.txt"), 142);
    assert_eq!(solve("tests/01_input.txt"), 56042);
    assert_eq!(solve_2("tests/01_demo2.txt"), 281);
    assert_eq!(solve_2("tests/01_input.txt"), 55358);
}
