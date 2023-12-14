use itertools::Itertools;
use std::fs::read_to_string;

fn solve_a(seeds: Vec<usize>, layers: &[Vec<(usize, usize, usize)>]) -> usize {
    let locations = layers.iter().fold(seeds, |seeds, mappings| {
        seeds
            .into_iter()
            .map(|seed| {
                mappings
                    .iter()
                    .find(|&&(_, src, range)| (src..src + range).contains(&seed))
                    .map(|(dst, src, _)| dst + seed - src)
                    .unwrap_or(seed)
            })
            .collect()
    });
    *locations.iter().min().unwrap()
}

fn solve_b(seeds: Vec<usize>, layers: &[Vec<(usize, usize, usize)>]) -> usize {
    let seeds = seeds
        .into_iter()
        .tuples()
        .map(|(a, len)| (a, a + len))
        .collect::<Vec<_>>();
    let locations = layers.iter().fold(seeds, |seeds, mappings| {
        seeds
            .iter()
            .flat_map(|&(start, end)| {
                let mut mapped = Vec::new();
                let mut unmapped = vec![(start, end)];
                for &(dst, src, len) in mappings {
                    let mut m = Vec::new();
                    for (start, end) in unmapped {
                        let a = (start, end.min(src));
                        let b = (start.max(src), (src + len).min(end));
                        let c = ((src + len).max(start), end);
                        if a.0 < a.1 {
                            m.push(a);
                        }
                        if b.0 < b.1 {
                            mapped.push((b.0 - src + dst, b.1 - src + dst));
                        }
                        if c.0 < c.1 {
                            m.push(c);
                        }
                    }
                    unmapped = m;
                }
                mapped.extend(unmapped);
                mapped
            })
            .collect()
    });
    locations.iter().map(|r| r.0).min().unwrap()
}

fn parse(path: &str) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    let input = read_to_string(path).unwrap();
    let (seeds, rest) = input.as_str().split_once("\n\n").unwrap();
    let seeds = seeds
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let layers: Vec<Vec<_>> = rest
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .skip(1)
                .map(|l| {
                    l.split_whitespace()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize, usize)>()
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, layers)
}

#[test]
fn test_a() {
    let (seeds, layers) = parse("tests/05_demo.txt");
    let ans_a = solve_a(seeds, &layers);
    eprintln!("ans_a: {}", ans_a);
    assert_eq!(ans_a, 35);
}

#[test]
fn test_a_input() {
    let (seeds, layers) = parse("tests/05_input.txt");
    let ans_a = solve_a(seeds, &layers);
    eprintln!("ans_a: {}", ans_a);
    assert_eq!(ans_a, 486613012);
}

#[test]
fn test_b() {
    let (seeds, layers) = parse("tests/05_input.txt");
    let ans_b = solve_b(seeds, &layers);
    eprintln!("ans_b: {}", ans_b);
    assert_eq!(ans_b, 56931769);
}

fn main() {}
