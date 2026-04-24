#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Fraction {
    num: i128,
    den: i128,
}

impl Fraction {
    fn new(num: i128, den: i128) -> Self {
        assert!(den != 0);
        if num == 0 {
            return Self { num: 0, den: 1 };
        }
        let mut num = num;
        let mut den = den;
        if den < 0 {
            num = -num;
            den = -den;
        }
        let g = gcd(num.abs(), den);
        Self {
            num: num / g,
            den: den / g,
        }
    }

    fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn is_integer(self) -> bool {
        self.num % self.den == 0
    }

    fn as_i128(self) -> i128 {
        assert!(self.is_integer());
        self.num / self.den
    }
}

impl std::ops::Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl std::ops::Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Fraction::new(self.num * rhs.den, self.den * rhs.num)
    }
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[derive(Debug)]
struct Machine {
    target_mask: u16,
    buttons: Vec<Vec<usize>>,
    requirements: Vec<i128>,
}

fn main() {
    let day06_example = include_str!("../puzzles/2025-06-example.txt");
    assert_eq!(day06_part1(day06_example), 4_277_556);
    assert_eq!(day06_part2(day06_example), 3_263_827);

    let day10_example = include_str!("../puzzles/2025-10-example.txt");
    let day10_example_machines = parse_day10(day10_example);
    assert_eq!(day10_part1(&day10_example_machines), 7);
    assert_eq!(day10_part2(&day10_example_machines), 33);

    let day06_input = include_str!("../puzzles/2025-06-input.txt");
    println!("2025-06 part 1: {}", day06_part1(day06_input));
    println!("2025-06 part 2: {}", day06_part2(day06_input));

    let day10_input = include_str!("../puzzles/2025-10-input.txt");
    let day10_machines = parse_day10(day10_input);
    println!("2025-10 part 1: {}", day10_part1(&day10_machines));
    println!("2025-10 part 2: {}", day10_part2(&day10_machines));
}

fn day06_part1(input: &str) -> i128 {
    let grid = padded_grid(input);
    let segments = non_empty_column_segments(&grid);
    segments
        .iter()
        .map(|&(start, end)| {
            let op = operator_in_segment(&grid, start, end);
            let nums: Vec<i128> = grid[..grid.len() - 1]
                .iter()
                .filter_map(|row| {
                    let text: String = row[start..end].iter().collect();
                    let text = text.trim();
                    (!text.is_empty()).then(|| text.parse::<i128>().unwrap())
                })
                .collect();
            apply_operation(op, &nums)
        })
        .sum()
}

fn day06_part2(input: &str) -> i128 {
    let grid = padded_grid(input);
    let segments = non_empty_column_segments(&grid);
    segments
        .iter()
        .map(|&(start, end)| {
            let op = operator_in_segment(&grid, start, end);
            let mut nums = Vec::new();
            for col in (start..end).rev() {
                let digits: String = grid[..grid.len() - 1]
                    .iter()
                    .map(|row| row[col])
                    .filter(|ch| ch.is_ascii_digit())
                    .collect();
                if !digits.is_empty() {
                    nums.push(digits.parse::<i128>().unwrap());
                }
            }
            apply_operation(op, &nums)
        })
        .sum()
}

fn padded_grid(input: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = rows.iter().map(Vec::len).max().unwrap_or(0);
    for row in &mut rows {
        row.resize(width, ' ');
    }
    rows
}

fn non_empty_column_segments(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let width = grid.first().map(Vec::len).unwrap_or(0);
    let mut segments = Vec::new();
    let mut col = 0;
    while col < width {
        while col < width && grid.iter().all(|row| row[col] == ' ') {
            col += 1;
        }
        let start = col;
        while col < width && grid.iter().any(|row| row[col] != ' ') {
            col += 1;
        }
        if start < col {
            segments.push((start, col));
        }
    }
    segments
}

fn operator_in_segment(grid: &[Vec<char>], start: usize, end: usize) -> char {
    grid.last()
        .unwrap()
        .get(start..end)
        .unwrap()
        .iter()
        .copied()
        .find(|ch| *ch == '+' || *ch == '*')
        .unwrap()
}

fn apply_operation(op: char, nums: &[i128]) -> i128 {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => unreachable!("unknown operator"),
    }
}

fn parse_day10(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_machine)
        .collect()
}

fn parse_machine(line: &str) -> Machine {
    let diagram_start = line.find('[').unwrap();
    let diagram_end = line.find(']').unwrap();
    let diagram = &line[diagram_start + 1..diagram_end];
    let target_mask = diagram.chars().enumerate().fold(0u16, |mask, (idx, ch)| {
        if ch == '#' {
            mask | (1u16 << idx)
        } else {
            mask
        }
    });

    let req_start = line.find('{').unwrap();
    let req_end = line.find('}').unwrap();
    let requirements = line[req_start + 1..req_end]
        .split(',')
        .map(|value| value.parse::<i128>().unwrap())
        .collect();

    let mut buttons = Vec::new();
    let mut rest = &line[diagram_end + 1..req_start];
    while let Some(start) = rest.find('(') {
        let after_start = &rest[start + 1..];
        let end = after_start.find(')').unwrap();
        let button = &after_start[..end];
        let indexes = if button.is_empty() {
            Vec::new()
        } else {
            button
                .split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect()
        };
        buttons.push(indexes);
        rest = &after_start[end + 1..];
    }

    Machine {
        target_mask,
        buttons,
        requirements,
    }
}

fn day10_part1(machines: &[Machine]) -> i128 {
    machines
        .iter()
        .map(|machine| {
            let button_masks: Vec<u16> = machine
                .buttons
                .iter()
                .map(|button| button.iter().fold(0u16, |mask, idx| mask | (1u16 << idx)))
                .collect();
            let mut best = i128::MAX;
            for chosen in 0usize..(1usize << button_masks.len()) {
                let mut mask = 0u16;
                let mut presses = 0i128;
                for (idx, button_mask) in button_masks.iter().enumerate() {
                    if (chosen & (1usize << idx)) != 0 {
                        mask ^= button_mask;
                        presses += 1;
                    }
                }
                if mask == machine.target_mask {
                    best = best.min(presses);
                }
            }
            best
        })
        .sum()
}

fn day10_part2(machines: &[Machine]) -> i128 {
    machines.iter().map(min_joltage_presses).sum()
}

fn min_joltage_presses(machine: &Machine) -> i128 {
    let n = machine.requirements.len();
    let m = machine.buttons.len();
    let mut matrix = vec![vec![Fraction::zero(); m + 1]; n];
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            matrix[counter_idx][button_idx] = Fraction::one();
        }
    }
    for (row, &requirement) in machine.requirements.iter().enumerate() {
        matrix[row][m] = Fraction::new(requirement, 1);
    }

    let pivots = rref(&mut matrix, m);
    for row in &matrix {
        if row[..m].iter().all(|value| value.is_zero()) && !row[m].is_zero() {
            panic!("machine has no solution: {:?}", machine);
        }
    }

    let mut pivot_for_col = vec![None; m];
    for &(row, col) in &pivots {
        pivot_for_col[col] = Some(row);
    }
    let free_cols: Vec<usize> = (0..m).filter(|&col| pivot_for_col[col].is_none()).collect();
    let upper_bounds: Vec<i128> = (0..m)
        .map(|button_idx| {
            machine.buttons[button_idx]
                .iter()
                .map(|&counter_idx| machine.requirements[counter_idx])
                .min()
                .unwrap_or(0)
        })
        .collect();

    let mut free_values = vec![0i128; free_cols.len()];
    let mut best = machine.requirements.iter().sum::<i128>() + 1;
    enumerate_free_values(
        0,
        &free_cols,
        &upper_bounds,
        &mut free_values,
        &matrix,
        &pivots,
        m,
        0,
        &mut best,
    );
    if best == machine.requirements.iter().sum::<i128>() + 1 {
        panic!(
            "machine has no non-negative integer solution: {:?}",
            machine
        );
    }
    best
}

fn rref(matrix: &mut [Vec<Fraction>], variable_cols: usize) -> Vec<(usize, usize)> {
    let row_count = matrix.len();
    let mut row = 0;
    let mut pivots = Vec::new();
    for col in 0..variable_cols {
        let Some(pivot_row) = (row..row_count).find(|&r| !matrix[r][col].is_zero()) else {
            continue;
        };
        matrix.swap(row, pivot_row);
        let pivot = matrix[row][col];
        for value in &mut matrix[row] {
            *value = *value / pivot;
        }
        let pivot_values = matrix[row].clone();
        for r in 0..row_count {
            if r == row || matrix[r][col].is_zero() {
                continue;
            }
            let factor = matrix[r][col];
            for (c, pivot_value) in pivot_values.iter().enumerate() {
                matrix[r][c] = matrix[r][c] - factor * *pivot_value;
            }
        }
        pivots.push((row, col));
        row += 1;
        if row == row_count {
            break;
        }
    }
    pivots
}

#[allow(clippy::too_many_arguments)]
fn enumerate_free_values(
    idx: usize,
    free_cols: &[usize],
    upper_bounds: &[i128],
    free_values: &mut [i128],
    matrix: &[Vec<Fraction>],
    pivots: &[(usize, usize)],
    variable_count: usize,
    free_sum: i128,
    best: &mut i128,
) {
    if free_sum >= *best {
        return;
    }
    if idx < free_cols.len() {
        let col = free_cols[idx];
        for value in 0..=upper_bounds[col].min(*best - free_sum - 1) {
            free_values[idx] = value;
            enumerate_free_values(
                idx + 1,
                free_cols,
                upper_bounds,
                free_values,
                matrix,
                pivots,
                variable_count,
                free_sum + value,
                best,
            );
        }
        return;
    }

    let mut values = vec![Fraction::zero(); variable_count];
    for (&col, &value) in free_cols.iter().zip(free_values.iter()) {
        values[col] = Fraction::new(value, 1);
    }
    for &(row, pivot_col) in pivots {
        let mut value = matrix[row][variable_count];
        for &free_col in free_cols {
            value = value - matrix[row][free_col] * values[free_col];
        }
        if !value.is_integer() || value.num < 0 {
            return;
        }
        values[pivot_col] = value;
    }

    let total = values.iter().map(|value| value.as_i128()).sum::<i128>();
    if total < *best {
        *best = total;
    }
}
