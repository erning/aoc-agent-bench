use std::fs;

fn solve(input: &str) -> (i64, i64) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let num_rows = grid.len();
    if num_rows == 0 {
        return (0, 0);
    }
    let num_cols = grid[0].len();

    // Find column groups: contiguous columns with at least one non-space character
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut current_group: Vec<usize> = Vec::new();

    for col in 0..num_cols {
        let has_content = grid[..num_rows - 1].iter().any(|row| col < row.len() && row[col] != ' ');
        if has_content {
            current_group.push(col);
        } else if !current_group.is_empty() {
            groups.push(current_group.clone());
            current_group.clear();
        }
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    // Parse numbers from a group (top to bottom)
    let parse_numbers = |group: &[usize], rows: &[Vec<char>]| -> Vec<i64> {
        let mut numbers = Vec::new();
        for row in rows {
            let s: String = group.iter().map(|&c| row[c]).collect();
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                if let Ok(n) = trimmed.parse::<i64>() {
                    numbers.push(n);
                }
            }
        }
        numbers
    };

    // Part 1: normal reading
    let mut part1 = 0i64;
    for group in &groups {
        let operator = grid[num_rows - 1][group[0]];
        let numbers = parse_numbers(group, &grid[..num_rows - 1]);
        let result = numbers[1..]
            .iter()
            .fold(numbers[0], |acc, &n| if operator == '+' { acc + n } else { acc * n });
        part1 += result;
    }

    // Part 2: right-to-left, columns reversed within each group, read top-to-bottom
    let mut part2 = 0i64;
    for group in groups.iter().rev() {
        let operator = grid[num_rows - 1][group[0]];
        let mut numbers = Vec::new();
        for &col in group.iter().rev() {
            let mut digits = Vec::new();
            for row in 0..num_rows - 1 {
                let ch = grid[row][col];
                if ch != ' ' {
                    digits.push(ch);
                }
            }
            if !digits.is_empty() {
                let n: i64 = digits.into_iter().collect::<String>().parse().unwrap();
                numbers.push(n);
            }
        }
        let result = numbers[1..]
            .iter()
            .fold(numbers[0], |acc, &n| if operator == '+' { acc + n } else { acc * n });
        part2 += result;
    }

    (part1, part2)
}

fn main() {
    let input = fs::read_to_string("puzzles/2025-06-input.txt").expect("Failed to read input");
    let (part1, part2) = solve(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("puzzles/2025-06-example.txt").unwrap();
        let (part1, part2) = solve(&input);
        assert_eq!(part1, 4277556);
        assert_eq!(part2, 3263827);
    }
}
