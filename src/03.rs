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
            if map[i][j].is_digit(10) {
                k = j + 1;
                let mut num = map[i][j].to_digit(10).unwrap();
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

#[test]
fn test_a() {
    let ans = solve("tests/03_demo.txt");
    assert_eq!(ans, 4361);

    let ans = solve("tests/03_input.txt");
    assert_eq!(ans, 531932);
}
