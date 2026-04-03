#[derive(Clone, Debug)]
struct Day10Machine {
    light_count: usize,
    light_target: u16,
    button_masks: Vec<u16>,
    joltage_target: Vec<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rational {
    num: i128,
    den: i128,
}

impl Rational {
    fn new(num: i128, den: i128) -> Self {
        assert!(den != 0, "zero denominator");

        if num == 0 {
            return Self { num: 0, den: 1 };
        }

        let mut num = num;
        let mut den = den;
        if den < 0 {
            num = -num;
            den = -den;
        }

        let divisor = gcd_i128(num.unsigned_abs(), den as u128) as i128;
        Self {
            num: num / divisor,
            den: den / divisor,
        }
    }

    fn zero() -> Self {
        Self { num: 0, den: 1 }
    }

    fn one() -> Self {
        Self { num: 1, den: 1 }
    }

    fn from_i64(value: i64) -> Self {
        Self {
            num: value as i128,
            den: 1,
        }
    }

    fn is_zero(self) -> bool {
        self.num == 0
    }

    fn is_integer(self) -> bool {
        self.den == 1
    }
}

impl std::ops::Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den + rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den)
    }
}

impl std::ops::Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.den * rhs.den)
    }
}

impl std::ops::Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.den, self.den * rhs.num)
    }
}

fn gcd_i128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

pub fn solve_day06(input: &str) -> (u128, u128) {
    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    assert!(lines.len() >= 2, "day 6 input must have at least two lines");

    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let spans = worksheet_spans(&lines, width);

    let part1 = spans
        .iter()
        .map(|&(start, end)| evaluate_rowwise(&lines, start, end))
        .sum();
    let part2 = spans
        .iter()
        .map(|&(start, end)| evaluate_columnwise(&lines, start, end))
        .sum();

    (part1, part2)
}

fn worksheet_spans(lines: &[&[u8]], width: usize) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut current_start = None;

    for column in 0..width {
        let is_separator = lines
            .iter()
            .all(|line| line.get(column).copied().unwrap_or(b' ') == b' ');

        if is_separator {
            if let Some(start) = current_start.take() {
                spans.push((start, column));
            }
        } else if current_start.is_none() {
            current_start = Some(column);
        }
    }

    if let Some(start) = current_start {
        spans.push((start, width));
    }

    spans
}

fn evaluate_rowwise(lines: &[&[u8]], start: usize, end: usize) -> u128 {
    let operator = operator_for_span(lines, start, end);
    let numbers = lines[..lines.len() - 1]
        .iter()
        .filter_map(|line| {
            let digits: String = (start..end)
                .filter_map(|column| {
                    let byte = line.get(column).copied().unwrap_or(b' ');
                    byte.is_ascii_digit().then_some(byte as char)
                })
                .collect();

            (!digits.is_empty()).then(|| digits.parse::<u128>().unwrap())
        })
        .collect::<Vec<_>>();

    evaluate_numbers(operator, &numbers)
}

fn evaluate_columnwise(lines: &[&[u8]], start: usize, end: usize) -> u128 {
    let operator = operator_for_span(lines, start, end);
    let mut numbers = Vec::new();

    for column in (start..end).rev() {
        let digits: String = lines[..lines.len() - 1]
            .iter()
            .filter_map(|line| {
                let byte = line.get(column).copied().unwrap_or(b' ');
                byte.is_ascii_digit().then_some(byte as char)
            })
            .collect();

        if !digits.is_empty() {
            numbers.push(digits.parse::<u128>().unwrap());
        }
    }

    evaluate_numbers(operator, &numbers)
}

fn operator_for_span(lines: &[&[u8]], start: usize, end: usize) -> char {
    (start..end)
        .filter_map(|column| {
            let byte = lines
                .last()
                .and_then(|line| line.get(column))
                .copied()
                .unwrap_or(b' ');
            (byte != b' ').then_some(byte as char)
        })
        .next()
        .expect("missing operator in day 6 span")
}

fn evaluate_numbers(operator: char, numbers: &[u128]) -> u128 {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("unexpected operator: {operator}"),
    }
}

pub fn solve_day10(input: &str) -> (u64, u64) {
    let machines = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_day10_machine)
        .collect::<Vec<_>>();

    let part1 = machines.iter().map(min_toggle_presses).sum();
    let part2 = machines.iter().map(min_joltage_presses).sum();

    (part1, part2)
}

fn parse_day10_machine(line: &str) -> Day10Machine {
    let diagram = extract_first_section(line, '[', ']').expect("missing indicator diagram");
    let button_sections = extract_all_sections(line, '(', ')');
    let joltage_section = extract_first_section(line, '{', '}').expect("missing joltage target");

    let light_count = diagram.len();
    let light_target = diagram
        .bytes()
        .enumerate()
        .fold(0u16, |mask, (index, byte)| {
            if byte == b'#' {
                mask | (1u16 << index)
            } else {
                mask
            }
        });

    let mut button_masks = button_sections
        .into_iter()
        .map(|button| parse_button_mask(button, light_count))
        .filter(|&mask| mask != 0)
        .collect::<Vec<_>>();
    button_masks.sort_unstable();
    button_masks.dedup();

    let joltage_target = parse_number_list(joltage_section)
        .into_iter()
        .map(|value| value as i64)
        .collect::<Vec<_>>();

    assert_eq!(
        light_count,
        joltage_target.len(),
        "light and joltage dimensions must match",
    );

    Day10Machine {
        light_count,
        light_target,
        button_masks,
        joltage_target,
    }
}

fn extract_first_section(text: &str, open: char, close: char) -> Option<&str> {
    let start = text.find(open)? + open.len_utf8();
    let rest = &text[start..];
    let end = rest.find(close)?;
    Some(&rest[..end])
}

fn extract_all_sections(text: &str, open: char, close: char) -> Vec<&str> {
    let mut sections = Vec::new();
    let mut cursor = text;

    while let Some(start) = cursor.find(open) {
        let after_open = &cursor[start + open.len_utf8()..];
        let end = after_open.find(close).expect("unterminated section");
        sections.push(&after_open[..end]);
        cursor = &after_open[end + close.len_utf8()..];
    }

    sections
}

fn parse_button_mask(section: &str, limit: usize) -> u16 {
    let mut mask = 0u16;

    for index in parse_number_list(section) {
        assert!(index < limit, "button index out of range");
        mask |= 1u16 << index;
    }

    mask
}

fn parse_number_list(section: &str) -> Vec<usize> {
    section
        .split(',')
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<usize>().expect("invalid numeric token"))
        .collect()
}

fn min_toggle_presses(machine: &Day10Machine) -> u64 {
    let subset_count = 1usize << machine.button_masks.len();
    let mut best = u32::MAX;

    for choice in 0..subset_count {
        let presses = choice.count_ones();
        if presses >= best {
            continue;
        }

        let mut state = 0u16;
        for (index, &button) in machine.button_masks.iter().enumerate() {
            if ((choice >> index) & 1) == 1 {
                state ^= button;
            }
        }

        if state == machine.light_target {
            best = presses;
        }
    }

    best as u64
}

fn min_joltage_presses(machine: &Day10Machine) -> u64 {
    let rank = matrix_rank(&machine.button_masks, machine.light_count);
    let free_count = machine.button_masks.len() - rank;
    let (basis_indices, free_indices) = choose_basis(machine, rank, free_count);
    let basis_masks = basis_indices
        .iter()
        .map(|&index| machine.button_masks[index])
        .collect::<Vec<_>>();
    let free_masks = free_indices
        .iter()
        .map(|&index| machine.button_masks[index])
        .collect::<Vec<_>>();

    let mut max_weight_from = vec![basis_max_weight(&basis_masks); free_masks.len() + 1];
    for index in (0..free_masks.len()).rev() {
        let weight = free_masks[index].count_ones() as i64;
        max_weight_from[index] = max_weight_from[index + 1].max(weight);
    }

    let mut residual = machine.joltage_target.clone();
    let mut best = None;
    search_free_assignments(
        &basis_masks,
        &free_masks,
        &mut residual,
        0,
        0,
        &max_weight_from,
        &mut best,
    );

    best.expect("no exact joltage solution found") as u64
}

fn choose_basis(
    machine: &Day10Machine,
    rank: usize,
    free_count: usize,
) -> (Vec<usize>, Vec<usize>) {
    let total = machine.button_masks.len();
    if free_count == 0 {
        return ((0..total).collect(), Vec::new());
    }

    let mut best = None;
    let mut current = Vec::new();
    enumerate_combinations(total, free_count, 0, &mut current, &mut |free_indices| {
        let basis_indices = complement_indices(total, free_indices);
        let basis_masks = basis_indices
            .iter()
            .map(|&index| machine.button_masks[index])
            .collect::<Vec<_>>();

        if matrix_rank(&basis_masks, machine.light_count) != rank {
            return;
        }

        let complexity = free_indices
            .iter()
            .map(|&index| {
                (free_upper_bound(machine.button_masks[index], &machine.joltage_target) + 1) as u128
            })
            .product::<u128>();
        let tie_breaker = free_indices
            .iter()
            .map(|&index| free_upper_bound(machine.button_masks[index], &machine.joltage_target))
            .sum::<i64>();

        let candidate = (
            complexity,
            tie_breaker,
            basis_indices.clone(),
            free_indices.to_vec(),
        );

        match &best {
            Some((best_complexity, best_tie, _, _))
                if (*best_complexity, *best_tie) <= (complexity, tie_breaker) => {}
            _ => best = Some(candidate),
        }
    });

    let (_, _, basis_indices, free_indices) = best.expect("failed to choose basis");
    (basis_indices, free_indices)
}

fn enumerate_combinations(
    total: usize,
    choose: usize,
    start: usize,
    current: &mut Vec<usize>,
    callback: &mut impl FnMut(&[usize]),
) {
    if current.len() == choose {
        callback(current);
        return;
    }

    let remaining_slots = choose - current.len();
    for index in start..=total - remaining_slots {
        current.push(index);
        enumerate_combinations(total, choose, index + 1, current, callback);
        current.pop();
    }
}

fn complement_indices(total: usize, excluded: &[usize]) -> Vec<usize> {
    let mut result = Vec::with_capacity(total - excluded.len());
    let mut excluded_index = 0;

    for index in 0..total {
        if excluded_index < excluded.len() && excluded[excluded_index] == index {
            excluded_index += 1;
        } else {
            result.push(index);
        }
    }

    result
}

fn free_upper_bound(mask: u16, target: &[i64]) -> i64 {
    target
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| (((mask >> index) & 1) == 1).then_some(value))
        .min()
        .expect("free button must affect at least one counter")
}

fn basis_max_weight(basis_masks: &[u16]) -> i64 {
    basis_masks
        .iter()
        .map(|mask| mask.count_ones() as i64)
        .max()
        .unwrap_or(0)
}

fn search_free_assignments(
    basis_masks: &[u16],
    free_masks: &[u16],
    residual: &mut [i64],
    free_index: usize,
    free_sum: i64,
    max_weight_from: &[i64],
    best: &mut Option<i64>,
) {
    let lower_bound =
        free_sum + optimistic_press_lower_bound(residual, max_weight_from[free_index]);
    if best.is_some_and(|current_best| lower_bound >= current_best) {
        return;
    }

    if free_index == free_masks.len() {
        if let Some(solution) = solve_exact_system(basis_masks, residual) {
            let total = free_sum + solution.iter().sum::<i64>();
            if best.is_none_or(|current_best| total < current_best) {
                *best = Some(total);
            }
        }
        return;
    }

    let mask = free_masks[free_index];
    let bound = free_upper_bound(mask, residual);

    for count in 0..=bound {
        if best.is_some_and(|current_best| free_sum + count >= current_best) {
            break;
        }

        apply_mask_count(residual, mask, -(count as i64));
        search_free_assignments(
            basis_masks,
            free_masks,
            residual,
            free_index + 1,
            free_sum + count,
            max_weight_from,
            best,
        );
        apply_mask_count(residual, mask, count as i64);
    }
}

fn optimistic_press_lower_bound(residual: &[i64], max_weight: i64) -> i64 {
    let max_coordinate = residual.iter().copied().max().unwrap_or(0);
    let total_residual: i64 = residual.iter().sum();

    if total_residual == 0 {
        return 0;
    }

    let coverage_bound = if max_weight == 0 {
        i64::MAX / 4
    } else {
        (total_residual + max_weight - 1) / max_weight
    };

    max_coordinate.max(coverage_bound)
}

fn apply_mask_count(residual: &mut [i64], mask: u16, delta: i64) {
    for (index, value) in residual.iter_mut().enumerate() {
        if ((mask >> index) & 1) == 1 {
            *value += delta;
        }
    }
}

fn solve_exact_system(masks: &[u16], target: &[i64]) -> Option<Vec<i64>> {
    if masks.is_empty() {
        return target.iter().all(|&value| value == 0).then(Vec::new);
    }

    let rows = target.len();
    let columns = masks.len();
    let mut matrix = vec![vec![Rational::zero(); columns + 1]; rows];

    for (row, rhs) in target.iter().enumerate() {
        for (column, &mask) in masks.iter().enumerate() {
            if ((mask >> row) & 1) == 1 {
                matrix[row][column] = Rational::one();
            }
        }
        matrix[row][columns] = Rational::from_i64(*rhs);
    }

    let mut pivot_row = 0;
    let mut pivot_columns = Vec::new();

    for column in 0..columns {
        let Some(found_row) = (pivot_row..rows).find(|&row| !matrix[row][column].is_zero()) else {
            continue;
        };

        matrix.swap(pivot_row, found_row);

        let pivot_value = matrix[pivot_row][column];
        for entry in &mut matrix[pivot_row][column..=columns] {
            *entry = *entry / pivot_value;
        }

        for row in 0..rows {
            if row == pivot_row || matrix[row][column].is_zero() {
                continue;
            }

            let factor = matrix[row][column];
            for current_column in column..=columns {
                matrix[row][current_column] =
                    matrix[row][current_column] - factor * matrix[pivot_row][current_column];
            }
        }

        pivot_columns.push(column);
        pivot_row += 1;
        if pivot_row == rows {
            break;
        }
    }

    for row in &matrix {
        let all_zero = row[..columns].iter().all(|entry| entry.is_zero());
        if all_zero && !row[columns].is_zero() {
            return None;
        }
    }

    if pivot_columns.len() != columns {
        return None;
    }

    let mut solution = vec![0i64; columns];
    for (row, &column) in pivot_columns.iter().enumerate() {
        let value = matrix[row][columns];
        if !value.is_integer() || value.num < 0 {
            return None;
        }
        solution[column] = value.num as i64;
    }

    for (row, &expected) in target.iter().enumerate() {
        let actual = masks
            .iter()
            .zip(solution.iter())
            .filter(|(mask, _)| (((**mask) >> row) & 1) == 1)
            .map(|(_, count)| *count)
            .sum::<i64>();
        if actual != expected {
            return None;
        }
    }

    Some(solution)
}

fn matrix_rank(masks: &[u16], rows: usize) -> usize {
    if masks.is_empty() {
        return 0;
    }

    let columns = masks.len();
    let mut matrix = vec![vec![Rational::zero(); columns]; rows];

    for row in 0..rows {
        for (column, &mask) in masks.iter().enumerate() {
            if ((mask >> row) & 1) == 1 {
                matrix[row][column] = Rational::one();
            }
        }
    }

    let mut pivot_row = 0;
    let mut rank = 0;

    for column in 0..columns {
        let Some(found_row) = (pivot_row..rows).find(|&row| !matrix[row][column].is_zero()) else {
            continue;
        };

        matrix.swap(pivot_row, found_row);

        let pivot_value = matrix[pivot_row][column];
        for entry in &mut matrix[pivot_row][column..] {
            *entry = *entry / pivot_value;
        }

        for row in 0..rows {
            if row == pivot_row || matrix[row][column].is_zero() {
                continue;
            }

            let factor = matrix[row][column];
            for current_column in column..columns {
                matrix[row][current_column] =
                    matrix[row][current_column] - factor * matrix[pivot_row][current_column];
            }
        }

        pivot_row += 1;
        rank += 1;
        if pivot_row == rows {
            break;
        }
    }

    rank
}

#[cfg(test)]
mod tests {
    use super::{Day10Machine, min_joltage_presses, solve_day06, solve_day10};

    #[test]
    fn day06_example_matches_prompt() {
        assert_eq!(
            solve_day06(include_str!("../puzzles/2025-06-example.txt")),
            (4_277_556, 3_263_827),
        );
    }

    #[test]
    fn day10_example_matches_prompt() {
        assert_eq!(
            solve_day10(include_str!("../puzzles/2025-10-example.txt")),
            (7, 33)
        );
    }

    #[test]
    fn joltage_solver_handles_free_variable_search() {
        let machine = Day10Machine {
            light_count: 2,
            light_target: 0,
            button_masks: vec![0b01, 0b10, 0b11],
            joltage_target: vec![1, 1],
        };

        assert_eq!(min_joltage_presses(&machine), 1);
    }

    #[test]
    fn joltage_solver_handles_row_deficient_system() {
        let machine = Day10Machine {
            light_count: 3,
            light_target: 0,
            button_masks: vec![0b011, 0b110],
            joltage_target: vec![1, 2, 1],
        };

        assert_eq!(min_joltage_presses(&machine), 2);
    }
}
