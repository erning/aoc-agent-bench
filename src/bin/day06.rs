use std::fs;

fn main() {
    let input = fs::read_to_string("puzzles/2025-06-input.txt").expect("Failed to read input");
    let lines: Vec<&str> = input.lines().collect();
    assert!(!lines.is_empty());

    let num_rows = lines.len();
    let max_cols = lines.iter().map(|l| l.len()).max().unwrap();

    // Pad all lines to same width
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut row: Vec<char> = l.chars().collect();
            row.resize(max_cols, ' ');
            row
        })
        .collect();

    // Identify separator columns: all spaces across ALL rows
    let mut is_separator = vec![true; max_cols];
    for col in 0..max_cols {
        for row in 0..num_rows {
            if grid[row][col] != ' ' {
                is_separator[col] = false;
                break;
            }
        }
    }

    // Group consecutive non-separator columns into problems
    let mut problems: Vec<Vec<usize>> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for col in 0..max_cols {
        if is_separator[col] {
            if !current.is_empty() {
                problems.push(current.clone());
                current.clear();
            }
        } else {
            current.push(col);
        }
    }
    if !current.is_empty() {
        problems.push(current);
    }

    let operator_row = num_rows - 1;
    let number_rows = num_rows - 1;

    // Part 1: Read numbers from rows
    let mut total_p1: u64 = 0;
    for cols in &problems {
        // Find operator: first non-space char in operator row among these columns
        let op = cols
            .iter()
            .filter_map(|&c| {
                let ch = grid[operator_row][c];
                if ch == '+' || ch == '*' {
                    Some(ch)
                } else {
                    None
                }
            })
            .next()
            .expect("No operator found");

        // Extract numbers from each row
        let mut numbers: Vec<u64> = Vec::new();
        for row in 0..number_rows {
            let s: String = cols.iter().map(|&c| grid[row][c]).collect();
            let s = s.trim();
            if !s.is_empty() {
                numbers.push(s.parse().expect("Failed to parse number"));
            }
        }

        let result = match op {
            '+' => numbers.iter().sum::<u64>(),
            '*' => numbers.iter().product::<u64>(),
            _ => unreachable!(),
        };
        total_p1 += result;
    }
    println!("Part 1: {}", total_p1);

    // Part 2: Read columns right-to-left within each problem
    let mut total_p2: u64 = 0;
    for cols in &problems {
        let op = cols
            .iter()
            .filter_map(|&c| {
                let ch = grid[operator_row][c];
                if ch == '+' || ch == '*' {
                    Some(ch)
                } else {
                    None
                }
            })
            .next()
            .expect("No operator found");

        // Each column (read top-to-bottom) gives a number
        // Read columns right-to-left
        let mut numbers: Vec<u64> = Vec::new();
        for &col in cols.iter().rev() {
            let s: String = (0..number_rows)
                .map(|row| grid[row][col])
                .collect();
            let s: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            if !s.is_empty() {
                numbers.push(s.parse().expect("Failed to parse number"));
            }
        }

        let result = match op {
            '+' => numbers.iter().sum::<u64>(),
            '*' => numbers.iter().product::<u64>(),
            _ => unreachable!(),
        };
        total_p2 += result;
    }
    println!("Part 2: {}", total_p2);
}
