use std::collections::{HashMap, HashSet};

use aoc2023::helper::read_to_lines;

fn solve_a(path: &str) -> u32 {
    let lines = read_to_lines(path);
    let mut ans = 0;
    lines.iter().for_each(|line| {
        let mut nums = line
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split("|")
            .into_iter();
        let winners = nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim().parse::<u32>())
            .filter_map(|x| x.ok())
            .collect::<Vec<u32>>();
        let candidates = nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim().parse::<u32>())
            .filter_map(|x| x.ok())
            .collect::<HashSet<u32>>();

        let c = winners.iter().filter(|x| candidates.contains(x)).count() as u32;
        if c > 0 {
            ans += (2 as u32).pow(c - 1);
        }
    });
    ans
}

fn solve_b(path: &str) -> u32 {
    let lines = read_to_lines(path);
    let mut cards = vec![];
    let mut original_cards = HashMap::new();
    lines.iter().for_each(|line| {
        let mut elems = line.split(":");
        let card = elems
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        //eprintln!("card: {:?}", card);

        let mut nums = elems.last().unwrap().trim().split("|").into_iter();

        let winners = nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim().parse::<u32>())
            .filter_map(|x| x.ok())
            .collect::<Vec<u32>>();
        let candidates = nums
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.trim().parse::<u32>())
            .filter_map(|x| x.ok())
            .collect::<HashSet<u32>>();
        //eprintln!("winners: {:?} candidates: {:?}", winners, candidates);
        let count = winners
            .iter()
            .filter_map(|c| {
                if candidates.contains(c) {
                    Some(c)
                } else {
                    None
                }
            })
            .count();
        cards.push((card, count));
        original_cards.insert(card, count);
    });

    let mut index = 0;
    let mut zero = 0;
    while index < cards.len() {
        let (card, count) = cards[index];
        if count > 0 {
            let mut j = index;
            while j < cards.len() && cards[j].0 == card {
                j += 1;
            }
            let same_count = j - index;
            for _ in 1..=same_count {
                for k in 1..=count {
                    if let Some(&c) = original_cards.get(&(card + k as u32)) {
                        if c == 0 {
                            zero += 1;
                        } else {
                            cards.push((card + k as u32, c));
                        }
                    }
                }
            }
            cards.sort_by(|(a, _), (b, _)| a.cmp(b));
            index = j;
        } else {
            index += 1;
        }
    }
    cards.len() as u32 + zero
}

#[test]
fn test_a() {
    assert_eq!(solve_a("tests/04_demo.txt"), 13);
    assert_eq!(solve_a("tests/04_input.txt"), 21558);
}

#[test]
fn main() {
    let ans = solve_b("tests/04_demo.txt");
    eprintln!("ans: {}", ans);
    assert_eq!(ans, 30);

    let ans = solve_b("tests/04_input.txt");
    eprintln!("ans: {}", ans);
    assert_eq!(ans, 10425665);
}
