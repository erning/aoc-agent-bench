use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("usage: day06 <input-file>");
    let content = fs::read_to_string(&path).expect("failed to read input file");

    // Keep trailing spaces; they matter for column layout.
    let mut lines: Vec<String> = content
        .lines()
        .map(|l| l.trim_end_matches('\r').to_string())
        .collect();
    assert!(!lines.is_empty(), "empty input");

    let width = lines.iter().map(|l| l.len()).max().unwrap();
    for l in lines.iter_mut() {
        while l.len() < width {
            l.push(' ');
        }
    }
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let num_rows = rows - 1; // last row holds the operators

    let is_separator = |c: usize| (0..rows).all(|r| grid[r][c] == ' ');

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    let mut c = 0;
    while c < width {
        if is_separator(c) {
            c += 1;
            continue;
        }
        let start = c;
        while c < width && !is_separator(c) {
            c += 1;
        }
        let end = c; // problem occupies columns [start, end)

        let op = (start..end)
            .map(|cc| grid[rows - 1][cc])
            .find(|&ch| ch == '+' || ch == '*')
            .expect("problem without operator");

        // Part 1: each row is one number (alignment within the block is ignored).
        let mut nums1 = Vec::new();
        for r in 0..num_rows {
            let s: String = grid[r][start..end].iter().collect();
            let s = s.trim();
            if !s.is_empty() {
                nums1.push(s.parse::<u64>().expect("invalid number"));
            }
        }

        // Part 2: each column is one number, most significant digit on top.
        let mut nums2 = Vec::new();
        for cc in start..end {
            let s: String = (0..num_rows)
                .map(|r| grid[r][cc])
                .filter(|ch| ch.is_ascii_digit())
                .collect();
            if !s.is_empty() {
                nums2.push(s.parse::<u64>().expect("invalid number"));
            }
        }

        let reduce = |nums: &[u64]| -> u64 {
            match op {
                '+' => nums.iter().sum(),
                '*' => nums.iter().product(),
                _ => unreachable!(),
            }
        };
        part1 += reduce(&nums1);
        part2 += reduce(&nums2);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
