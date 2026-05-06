use std::fs;

struct Grid {
    rows: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let rows: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                let mut chars: Vec<char> = l.chars().collect();
                chars.resize(width, ' ');
                chars
            })
            .collect();
        Grid {
            rows,
            height,
            width,
        }
    }

    fn is_separator_column(&self, col: usize) -> bool {
        (0..self.height).all(|row| self.rows[row][col] == ' ')
    }

    fn problem_blocks(&self) -> Vec<(usize, usize)> {
        let mut blocks = Vec::new();
        let mut start: Option<usize> = None;
        for col in 0..self.width {
            if self.is_separator_column(col) {
                if let Some(s) = start {
                    blocks.push((s, col));
                    start = None;
                }
            } else if start.is_none() {
                start = Some(col);
            }
        }
        if let Some(s) = start {
            blocks.push((s, self.width));
        }
        blocks
    }

    fn operator_for_block(&self, block_start: usize, block_end: usize) -> char {
        let op_row = self.height - 1;
        for col in block_start..block_end {
            let ch = self.rows[op_row][col];
            if ch != ' ' {
                return ch;
            }
        }
        panic!("No operator found in block [{block_start}, {block_end})");
    }

    fn part1_numbers(&self, block_start: usize, block_end: usize) -> Vec<u64> {
        let data_rows = self.height - 1; // exclude operator row
        let mut numbers = Vec::new();
        for row in 0..data_rows {
            let s: String = self.rows[row][block_start..block_end]
                .iter()
                .collect();
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                numbers.push(trimmed.parse::<u64>().unwrap());
            }
        }
        numbers
    }

    fn part2_numbers(&self, block_start: usize, block_end: usize) -> Vec<u64> {
        let data_rows = self.height - 1;
        let mut numbers = Vec::new();
        // Read columns right-to-left
        for col in (block_start..block_end).rev() {
            let mut s = String::new();
            for row in 0..data_rows {
                let ch = self.rows[row][col];
                if ch != ' ' {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                numbers.push(s.parse::<u64>().unwrap());
            }
        }
        numbers
    }
}

fn compute_result(numbers: &[u64], operator: char) -> u64 {
    match operator {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => panic!("Unknown operator: {operator}"),
    }
}

pub fn solve() {
    let input = fs::read_to_string("puzzles/2025-06-input.txt")
        .expect("Failed to read input file");
    let grid = Grid::parse(&input);
    let blocks = grid.problem_blocks();

    // Part 1: left-to-right
    let part1: u64 = blocks
        .iter()
        .map(|&(start, end)| {
            let op = grid.operator_for_block(start, end);
            let nums = grid.part1_numbers(start, end);
            compute_result(&nums, op)
        })
        .sum();

    // Part 2: right-to-left column reading
    let part2: u64 = blocks
        .iter()
        .rev() // process problems right-to-left
        .map(|&(start, end)| {
            let op = grid.operator_for_block(start, end);
            let nums = grid.part2_numbers(start, end);
            compute_result(&nums, op)
        })
        .sum();

    println!("Day 6 Part 1: {part1}");
    println!("Day 6 Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let grid = Grid::parse(input);
        let blocks = grid.problem_blocks();
        assert_eq!(blocks.len(), 4);

        // Part 1
        let part1: u64 = blocks
            .iter()
            .map(|&(s, e)| {
                let op = grid.operator_for_block(s, e);
                let nums = grid.part1_numbers(s, e);
                compute_result(&nums, op)
            })
            .sum();
        assert_eq!(part1, 4277556);

        // Part 2
        let part2: u64 = blocks
            .iter()
            .rev()
            .map(|&(s, e)| {
                let op = grid.operator_for_block(s, e);
                let nums = grid.part2_numbers(s, e);
                compute_result(&nums, op)
            })
            .sum();
        assert_eq!(part2, 3263827);
    }
}
