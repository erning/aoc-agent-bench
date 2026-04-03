use std::fs;

fn main() {
    // Day 6 solutions
    let day6_input = fs::read_to_string("puzzles/2025-06-input.txt").expect("Failed to read day 6 input");
    let day6_example = fs::read_to_string("puzzles/2025-06-example.txt").expect("Failed to read day 6 example");

    println!("=== Day 6: Trash Compactor ===");
    println!("Example Part 1: {}", day6_part1(&day6_example));
    println!("Example Part 2: {}", day6_part2(&day6_example));
    println!("Part 1: {}", day6_part1(&day6_input));
    println!("Part 2: {}", day6_part2(&day6_input));

    // Day 10 solutions
    let day10_input = fs::read_to_string("puzzles/2025-10-input.txt").expect("Failed to read day 10 input");
    let day10_example = fs::read_to_string("puzzles/2025-10-example.txt").expect("Failed to read day 10 example");

    println!("\n=== Day 10: Factory ===");
    println!("Example Part 1: {}", day10_part1(&day10_example));
    println!("Example Part 2: {}", day10_part2(&day10_example));
    println!("Part 1: {}", day10_part1(&day10_input));
    println!("Part 2: {}", day10_part2(&day10_input));
}

// Day 6 Part 1: Parse problems top-to-bottom
// Each problem is a vertical column group with the operator at the bottom left
fn day6_part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        return 0;
    }

    let num_lines = &lines[..lines.len() - 1];
    let op_line = lines[lines.len() - 1];

    // Find operators with their positions
    let mut ops_with_pos: Vec<(usize, char)> = op_line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '*')
        .collect();
    ops_with_pos.sort_by_key(|(pos, _)| *pos);

    let mut grand_total = 0i64;

    // Process problems left to right
    for (_idx, &(op_pos, op)) in ops_with_pos.iter().enumerate() {
        // Determine column range for this problem
        // The range is a fixed 4-column group starting at the operator position
        let start_col = op_pos;
        let end_col = op_pos + 3;

        // For each row, find the number that overlaps with this column range
        let mut numbers = Vec::new();
        for line in num_lines {
            if let Some(num) = extract_number_in_range(line, start_col, end_col) {
                numbers.push(num);
            }
        }

        if !numbers.is_empty() {
            let result = numbers.iter().skip(1).fold(numbers[0], |acc, &n| {
                if op == '+' { acc + n } else { acc * n }
            });
            grand_total += result;
        }
    }

    grand_total
}

fn extract_number_in_range(line: &str, start_col: usize, end_col: usize) -> Option<i64> {
    let chars: Vec<char> = line.chars().collect();
    if start_col >= chars.len() {
        return None;
    }

    // Find all numbers in this line and check if any overlaps with [start_col, end_col]
    let mut i = 0;
    while i < chars.len() {
        // Skip non-digits
        while i < chars.len() && !chars[i].is_ascii_digit() {
            i += 1;
        }
        if i >= chars.len() {
            break;
        }

        // Found start of a number
        let num_start = i;
        let mut num_end = i;
        while num_end + 1 < chars.len() && chars[num_end + 1].is_ascii_digit() {
            num_end += 1;
        }

        // Check if this number overlaps with the range [start_col, end_col]
        // Overlap: number ends at or after start_col, and starts at or before end_col
        if num_end >= start_col && num_start <= end_col {
            return chars[num_start..=num_end].iter().collect::<String>().parse().ok();
        }

        i = num_end + 1;
    }
    None
}

// Day 6 Part 2: Parse problems right-to-left, columns bottom-to-top
// For each column in the problem (right-to-left), read digits bottom-to-top to form a number
fn day6_part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 2 {
        return 0;
    }

    let num_lines = &lines[..lines.len() - 1];
    let op_line = lines[lines.len() - 1];

    // Find operators with their positions
    let mut ops_with_pos: Vec<(usize, char)> = op_line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '+' || *c == '*')
        .collect();
    ops_with_pos.sort_by_key(|(pos, _)| *pos);

    let mut grand_total = 0i64;

    // Process problems from right to left
    for (_idx, &(op_pos, op)) in ops_with_pos.iter().enumerate().rev() {
        // Determine column range for this problem
        // The range is a fixed 4-column group starting at the operator position
        let start_col = op_pos;
        let end_col = op_pos + 3;

        // For each column in the problem's range (right-to-left), read bottom-to-top
        let mut numbers = Vec::new();

        for col in (start_col..=end_col).rev() {
            // Read digits at this column from TOP to BOTTOM (most to least significant)
            let digits: String = num_lines
                .iter()
                .filter_map(|line| {
                    let chars: Vec<char> = line.chars().collect();
                    if col < chars.len() && chars[col].is_ascii_digit() {
                        Some(chars[col])
                    } else {
                        None
                    }
                })
                .collect();

            if !digits.is_empty() {
                if let Ok(num) = digits.parse::<i64>() {
                    numbers.push(num);
                }
            }
        }

        if !numbers.is_empty() {
            let result = numbers.iter().skip(1).fold(numbers[0], |acc, &n| {
                if op == '+' { acc + n } else { acc * n }
            });
            grand_total += result;
        }
    }

    grand_total
}

// Day 10 Part 1: Minimum button presses to achieve light configuration
// This is a system of linear equations over GF(2)
fn day10_part1(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let machine = parse_machine(line);
        total += solve_lights_min_presses(&machine);
    }

    total
}

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>, // true = on (#), false = off (.)
    buttons: Vec<Vec<usize>>, // each button lists which lights it toggles
    joltage: Vec<i64>,
}

fn parse_machine(line: &str) -> Machine {
    // Parse format: [.##.] (3) (1,3) ... {3,5,4,7}
    let mut target_lights = Vec::new();
    let mut buttons = Vec::new();
    let mut joltage = Vec::new();

    let mut in_brackets = false;
    let mut in_parens = false;
    let mut in_braces = false;
    let mut current_token = String::new();

    for c in line.chars() {
        match c {
            '[' => {
                in_brackets = true;
                current_token.clear();
            }
            ']' => {
                in_brackets = false;
                // Parse light pattern
                for ch in current_token.chars() {
                    target_lights.push(ch == '#');
                }
                current_token.clear();
            }
            '(' => {
                in_parens = true;
                current_token.clear();
            }
            ')' => {
                in_parens = false;
                // Parse button wiring
                let indices: Vec<usize> = current_token
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                buttons.push(indices);
                current_token.clear();
            }
            '{' => {
                in_braces = true;
                current_token.clear();
            }
            '}' => {
                in_braces = false;
                // Parse joltage
                let values: Vec<i64> = current_token
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                joltage = values;
                current_token.clear();
            }
            _ => {
                if in_brackets || in_parens || in_braces {
                    current_token.push(c);
                }
            }
        }
    }

    Machine {
        target_lights,
        buttons,
        joltage,
    }
}

fn solve_lights_min_presses(machine: &Machine) -> i64 {
    let n = machine.target_lights.len();
    let m = machine.buttons.len();

    if n == 0 {
        return 0;
    }

    // Build augmented matrix for GF(2) system
    let mut matrix: Vec<Vec<u8>> = vec![vec![0; m + 1]; n];

    for i in 0..n {
        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                matrix[i][j] = 1;
            }
        }
        matrix[i][m] = if machine.target_lights[i] { 1 } else { 0 };
    }

    // Gaussian elimination over GF(2)
    let mut row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..m {
        // Find pivot
        let mut pivot = None;
        for r in row..n {
            if matrix[r][col] == 1 {
                pivot = Some(r);
                break;
            }
        }

        if let Some(pivot_row) = pivot {
            matrix.swap(row, pivot_row);
            pivot_cols.push(col);

            for r in 0..n {
                if r != row && matrix[r][col] == 1 {
                    for c in col..=m {
                        matrix[r][c] ^= matrix[row][c];
                    }
                }
            }
            row += 1;
        }
    }

    // Check for inconsistent system
    for r in row..n {
        if matrix[r][m] == 1 {
            return -1;
        }
    }

    // Find minimum weight solution using branch and bound for free variables
    let rank = pivot_cols.len();
    let _num_free = m - rank;

    // Map column indices to free variable indices
    let mut is_pivot = vec![false; m];
    for &col in &pivot_cols {
        is_pivot[col] = true;
    }

    let free_cols: Vec<usize> = (0..m).filter(|&c| !is_pivot[c]).collect();

    let mut min_presses = i64::MAX;

    // Branch and bound with pruning
    fn search(
        free_idx: usize,
        free_cols: &[usize],
        pivot_cols: &[usize],
        presses: &mut [i64],
        matrix: &[Vec<u8>],
        current_sum: i64,
        min_presses: &mut i64,
    ) {
        if current_sum >= *min_presses {
            return;
        }

        if free_idx == free_cols.len() {
            // All free variables set, compute pivot variables
            for (i, &pivot_col) in pivot_cols.iter().enumerate() {
                let mut val = matrix[i][matrix[0].len() - 1];
                for c in (pivot_col + 1)..presses.len() {
                    if matrix[i][c] == 1 {
                        val ^= presses[c] as u8;
                    }
                }
                presses[pivot_col] = val as i64;
            }

            let total: i64 = presses.iter().sum();
            if total < *min_presses {
                *min_presses = total;
            }
            return;
        }

        // Try both values for this free variable
        for val in [0, 1] {
            presses[free_cols[free_idx]] = val;
            search(
                free_idx + 1,
                free_cols,
                pivot_cols,
                presses,
                matrix,
                current_sum + val,
                min_presses,
            );
        }
    }

    let mut presses = vec![0; m];
    search(
        0,
        &free_cols,
        &pivot_cols,
        &mut presses,
        &matrix,
        0,
        &mut min_presses,
    );

    if min_presses != i64::MAX {
        return min_presses;
    }

    -1
}

// Day 10 Part 2: Minimum button presses to achieve joltage configuration
// This is a linear programming / integer linear system problem
fn day10_part2(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let machine = parse_machine(line);
        total += solve_joltage_min_presses(&machine);
    }

    total
}

fn solve_joltage_min_presses(machine: &Machine) -> i64 {
    let n = machine.joltage.len();
    let m = machine.buttons.len();

    if n == 0 {
        return 0;
    }

    // Build constraint matrix
    let mut a: Vec<Vec<i64>> = vec![vec![0; m]; n];
    for i in 0..n {
        for (j, button) in machine.buttons.iter().enumerate() {
            if button.contains(&i) {
                a[i][j] = 1;
            }
        }
    }

    // Use BFS with pruning for small systems
    let max_target = *machine.joltage.iter().max().unwrap_or(&0);
    let max_presses: i64 = machine.joltage.iter().sum();

    if m <= 20 && max_target <= 20 {
        let mut min_presses = i64::MAX;

        // Recursive search with pruning
        fn search_joltage(
            idx: usize,
            m: usize,
            presses: &mut [i64],
            current: &mut [i64],
            current_sum: i64,
            target: &[i64],
            a: &[Vec<i64>],
            min_presses: &mut i64,
            max_presses: i64,
        ) {
            if current_sum >= *min_presses || current_sum > max_presses {
                return;
            }

            if idx == m {
                // Check if solution is valid
                for i in 0..target.len() {
                    if current[i] != target[i] {
                        return;
                    }
                }
                *min_presses = current_sum;
                return;
            }

            // Determine max presses for this button
            // It can't exceed the minimum remaining target among counters it affects
            let mut max_p = max_presses - current_sum;
            for i in 0..target.len() {
                if a[i][idx] == 1 {
                    let remaining = target[i] - current[i];
                    if remaining < max_p {
                        max_p = remaining;
                    }
                }
            }
            if max_p < 0 {
                max_p = 0;
            }

            for p in 0..=max_p {
                presses[idx] = p;
                let mut valid = true;
                for i in 0..target.len() {
                    if a[i][idx] == 1 {
                        current[i] += p;
                        if current[i] > target[i] {
                            valid = false;
                        }
                    }
                }

                if valid {
                    search_joltage(
                        idx + 1,
                        m,
                        presses,
                        current,
                        current_sum + p,
                        target,
                        a,
                        min_presses,
                        max_presses,
                    );
                }

                // Backtrack
                for i in 0..target.len() {
                    if a[i][idx] == 1 {
                        current[i] -= p;
                    }
                }
            }
        }

        let mut presses = vec![0; m];
        let mut current = vec![0; n];
        search_joltage(
            0,
            m,
            &mut presses,
            &mut current,
            0,
            &machine.joltage,
            &a,
            &mut min_presses,
            max_presses,
        );

        if min_presses != i64::MAX {
            return min_presses;
        }
    }

    // Fallback: use a simple heuristic (may not be optimal)
    // Greedy: repeatedly press the button that gets us closest to target
    // without exceeding, prioritizing buttons that affect multiple counters
    let mut presses = vec![0; m];
    let mut current = vec![0; n];
    let mut total_presses = 0;

    loop {
        let mut improved = false;
        for j in 0..m {
            // Check if pressing button j helps
            let mut can_press = true;
            let mut needed = i64::MAX;
            for i in 0..n {
                if a[i][j] == 1 {
                    if current[i] >= machine.joltage[i] {
                        can_press = false;
                        break;
                    }
                    let rem = machine.joltage[i] - current[i];
                    if rem < needed {
                        needed = rem;
                    }
                }
            }

            if can_press && needed > 0 {
                // Press this button
                presses[j] += 1;
                total_presses += 1;
                for i in 0..n {
                    if a[i][j] == 1 {
                        current[i] += 1;
                    }
                }
                improved = true;
                break;
            }
        }

        if !improved {
            break;
        }
    }

    // Check if we reached the target
    let mut valid = true;
    for i in 0..n {
        if current[i] != machine.joltage[i] {
            valid = false;
            break;
        }
    }

    if valid {
        total_presses
    } else {
        // Try a different approach - BFS with state hashing for small cases
        if n <= 10 && max_target <= 10 {
            use std::collections::{HashMap, VecDeque};

            let mut dist: HashMap<Vec<i64>, i64> = HashMap::new();
            let mut queue: VecDeque<Vec<i64>> = VecDeque::new();

            let start = vec![0; n];
            dist.insert(start.clone(), 0);
            queue.push_back(start);

            while let Some(state) = queue.pop_front() {
                let d = dist[&state];
                if state == machine.joltage {
                    return d;
                }

                // Try each button
                for j in 0..m {
                    let mut new_state = state.clone();
                    let mut valid = true;
                    for i in 0..n {
                        if a[i][j] == 1 {
                            new_state[i] += 1;
                            if new_state[i] > machine.joltage[i] {
                                valid = false;
                                break;
                            }
                        }
                    }

                    if valid && !dist.contains_key(&new_state) {
                        dist.insert(new_state.clone(), d + 1);
                        queue.push_back(new_state);
                    }
                }
            }
        }

        0 // Could not find solution
    }
}
