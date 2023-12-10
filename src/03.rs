use std::collections::{HashMap, HashSet};

use aoc2023::helper::read_to_lines;

fn solve(path: &str) -> u32 {
    let lines = read_to_lines(path);
    let map = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let check_adjust_sym = |i: usize, j: usize| -> bool {
        let dir = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        dir.iter().any(|(x, y)| {
            let x = i as i32 + x;
            let y = j as i32 + y;
            if x >= 0 && x < map.len() as i32 && y >= 0 && y < map[0].len() as i32 {
                let c = map[x as usize][y as usize];
                !c.is_alphanumeric() && c != '.'
            } else {
                false
            }
        })
    };

    let row = map.len();
    let col = map[0].len();
    let mut ans = 0;
    for i in 0..row {
        let mut j = 0;
        while j < col {
            let mut k = 0;
            let mut num = 0;
            if map[i][j].is_digit(10) {
                k = j;
                let mut has_adjust = check_adjust_sym(i, j);
                while k < col && map[i][k].is_digit(10) {
                    num = num * 10 + map[i][k].to_digit(10).unwrap();
                    has_adjust |= check_adjust_sym(i, k);
                    k += 1;
                }
                if has_adjust {
                    ans += num;
                }
            }
            j = k.max(j + 1);
        }
    }
    ans
}

fn solve_b(path: &str) -> u32 {
    let lines = read_to_lines(path);
    let map = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let check_adjust = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut adj = vec![];
        let dir = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        dir.iter().for_each(|(x, y)| {
            let x = i as i32 + x;
            let y = j as i32 + y;
            if (x >= 0 && x < map.len() as i32 && y >= 0 && y < map[0].len() as i32)
                && map[x as usize][y as usize] == '*'
            {
                adj.push((x as usize, y as usize));
            }
        });
        adj
    };

    let row = map.len();
    let col = map[0].len();
    let mut hash: HashMap<(usize, usize), HashSet<u32>> = HashMap::new();

    for i in 0..row {
        let mut j = 0;
        while j < col {
            let mut k = 0;
            let mut adj = HashSet::new();
            let mut num = 0;
            if map[i][j].is_digit(10) {
                k = j;
                while k < col && map[i][k].is_digit(10) {
                    num = num * 10 + map[i][k].to_digit(10).unwrap();
                    for (x, y) in check_adjust(i, k) {
                        adj.insert((x, y));
                    }
                    k += 1;
                }
                if adj.len() > 0 {
                    for (x, y) in adj {
                        let key = (x, y);
                        if let Some(s) = hash.get_mut(&key) {
                            s.insert(num);
                        } else {
                            let mut set = HashSet::new();
                            set.insert(num);
                            hash.insert(key, set);
                        }
                    }
                }
            }
            j = k.max(j + 1);
        }
    }
    let mut ans = 0;
    for (_, v) in hash {
        if v.len() == 2 {
            ans += v.iter().fold(1, |a, b| a * b);
        }
    }
    ans
}

#[test]
fn test_a() {
    let ans = solve("tests/03_demo.txt");
    assert_eq!(ans, 4361);

    let ans = solve("tests/03_input.txt");
    assert_eq!(ans, 531932);
}

#[test]
fn test_b() {
    let ans = solve_b("tests/03_demo.txt");
    assert_eq!(ans, 467835);

    let ans = solve_b("tests/03_input.txt");
    assert_eq!(ans, 73646890);
}
