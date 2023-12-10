use aoc2023::helper::read_to_lines;
use std::collections::HashMap;

fn parse(line: &str) -> (u32, Vec<HashMap<&str, u32>>) {
    let mut parts = line.split(":");
    let game_info = parts.next().unwrap();
    let id = game_info.split(" ").last().unwrap().parse::<u32>().unwrap();
    let counts = parts
        .next()
        .unwrap()
        .split(";")
        .into_iter()
        .map(|part| {
            let hash: HashMap<_, _> = part
                .split(",")
                .into_iter()
                .map(|p| {
                    let mut i = p.trim().split(" ");
                    let count = i.next().unwrap().parse::<u32>().unwrap();
                    let color = i.next().unwrap();
                    (color, count)
                })
                .collect();
            hash
        })
        .collect();
    (id, counts)
}

fn solve(path: &str) -> u32 {
    let lines = read_to_lines(path);
    let limits: HashMap<&str, u32> = vec![("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect();

    lines
        .iter()
        .map(|line| {
            let (id, counts) = parse(line);
            if counts.iter().any(|counts| {
                counts
                    .iter()
                    .any(|(color, count)| count > limits.get(color).unwrap())
            }) {
                0
            } else {
                id
            }
        })
        .sum()
}

fn solve_b(path: &str) -> u32 {
    let lines = read_to_lines(path);
    lines
        .iter()
        .map(|line| {
            let (_id, counts) = parse(line);
            let get = |color: &str| -> &u32 {
                counts
                    .iter()
                    .map(|c| c.get(color).unwrap_or(&0))
                    .max()
                    .unwrap()
            };
            get("red") * get("green") * get("blue")
        })
        .sum()
}

fn main() {
    let path = "tests/02_demo.txt";
    let ans = solve(path);
    eprintln!("ans: {}", ans);

    let ans = solve("tests/02_input.txt");
    eprintln!("ans: {}", ans);

    let path = "tests/02_input.txt";
    let ans = solve_b(path);
    eprintln!("ans: {}", ans);
}

#[test]
fn test_02() {
    let ans = solve("tests/02_demo.txt");
    assert_eq!(ans, 8);
}

#[test]
fn test_02_input() {
    let ans = solve("tests/02_input.txt");
    assert_eq!(ans, 2085);
}

#[test]
fn test_02_b() {
    let ans = solve_b("tests/02_demo.txt");
    assert_eq!(ans, 2286);
}

#[test]
fn test_02_b_input() {
    let ans = solve_b("tests/02_input.txt");
    assert_eq!(ans, 79315);
}
