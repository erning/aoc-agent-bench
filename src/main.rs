use std::collections::{HashMap, VecDeque};

fn main() {
    // ===== Day 6: Trash Compactor =====
    day6();

    // ===== Day 10: Factory =====
    day10();
}

fn day6() {
    println!("=== Day 6: Trash Compactor ===");

    let input = include_str!("../puzzles/2025-06-input.txt");
    let example = include_str!("../puzzles/2025-06-example.txt");

    // Part 1 - test with example first
    let example_answer = solve_day6_part1(example);
    println!("Day 6 Part 1 (example): {}", example_answer);
    assert_eq!(example_answer, 4277556);

    let answer = solve_day6_part1(input);
    println!("Day 6 Part 1 (answer): {}", answer);

    // Part 2 - test with example first
    let example_answer2 = solve_day6_part2(example);
    println!("Day 6 Part 2 (example): {}", example_answer2);
    assert_eq!(example_answer2, 3263827);

    let answer2 = solve_day6_part2(input);
    println!("Day 6 Part 2 (answer): {}", answer2);
}

fn solve_day6_part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    // Parse the worksheet: numbers are in vertical columns separated by columns of only spaces
    // Last row has operators (* or +)
    let num_rows = lines.len() - 1;
    let operator_row = lines.last().unwrap();

    // Find column groups: each group is a "problem" (separated by all-space columns)
    let line_len = lines[0].len();
    // Use the operator row to find problems since it's the most reliable separator
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Find problem boundaries from the operator row
    let mut problems: Vec<(usize, usize, char)> = Vec::new(); // (start_col, end_col, operator)
    let mut i = 0;
    let op_chars: Vec<char> = operator_row.chars().collect();

    while i < op_chars.len() {
        if op_chars[i] == '*' || op_chars[i] == '+' {
            let op = op_chars[i];
            let start = i;
            // Find end of this problem (next space column or end)
            let mut end = i + 1;
            while end < op_chars.len() && op_chars[end] != ' ' {
                end += 1;
            }
            // But we need to find the actual column span. The operator row has single chars
            // separated by spaces. Each problem spans from start to end of that operator's column group.
            // Actually, let's find column groups by looking at ALL rows.

            // Find the full width of this problem column
            // The operator is at position i. The problem extends left/right where there are digits.
            // Let's look at the rows above to find the full width.
            let mut col_start = i;
            let mut col_end = i + 1;

            for row in &lines[..num_rows] {
                let row_chars: Vec<char> = row.chars().collect();
                // Extend left
                while col_start > 0 {
                    let mut has_digit = false;
                    for r in &lines[..num_rows] {
                        let rc: Vec<char> = r.chars().collect();
                        if col_start > 0 && col_start - 1 < rc.len() && rc[col_start - 1] != ' ' {
                            has_digit = true;
                            break;
                        }
                    }
                    if has_digit {
                        // Check this doesn't merge with previous problem
                        // Look if there's a space-only column between col_start-1 and previous problem
                        let mut is_space_col = true;
                        for r in &lines[..num_rows] {
                            let rc: Vec<char> = r.chars().collect();
                            if col_start < rc.len() && rc[col_start] != ' ' {
                                is_space_col = false;
                                break;
                            }
                        }
                        if is_space_col {
                            break;
                        }
                        col_start -= 1;
                    } else {
                        break;
                    }
                }
                // Extend right
                while col_end < max_len {
                    let mut has_digit = false;
                    for r in &lines[..num_rows] {
                        let rc: Vec<char> = r.chars().collect();
                        if col_end < rc.len() && rc[col_end] != ' ' {
                            has_digit = true;
                            break;
                        }
                    }
                    if has_digit {
                        col_end += 1;
                    } else {
                        break;
                    }
                }
            }

            problems.push((col_start, col_end, op));
            i = end;
        } else {
            i += 1;
        }
    }

    // Actually, let me take a different approach. Parse the operator row to get operators,
    // then parse the number rows to get column-separated groups.

    // Simpler approach: find operator positions, then for each operator, collect the
    // numbers in the column group above it.

    // Parse operators from last row
    let operators: Vec<(usize, char)> = operator_row
        .char_indices()
        .filter(|&(_, c)| c == '*' || c == '+')
        .collect();

    // For each operator position, find the column group boundaries
    // A column group boundary is a column where ALL number rows have a space (or are shorter)
    let mut space_columns: Vec<bool> = vec![true; max_len + 1];
    for col in 0..max_len {
        let mut is_space = true;
        for row in &lines[..num_rows] {
            let chars: Vec<char> = row.chars().collect();
            if col < chars.len() && chars[col] != ' ' {
                is_space = false;
                break;
            }
        }
        space_columns[col] = is_space;
    }

    // Find column groups (contiguous non-space columns)
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut in_group = false;
    let mut group_start = 0;
    for col in 0..=max_len {
        if col < max_len && !space_columns[col] {
            if !in_group {
                group_start = col;
                in_group = true;
            }
        } else {
            if in_group {
                groups.push((group_start, col));
                in_group = false;
            }
        }
    }

    // Match each operator to a group
    // Each operator falls within exactly one group
    let mut grand_total: u64 = 0;

    for &(op_col, op) in &operators {
        // Find which group this operator belongs to
        let group_idx = groups
            .iter()
            .position(|&(start, end)| op_col >= start && op_col < end)
            .unwrap();

        let (g_start, g_end) = groups[group_idx];

        // Collect numbers from this column group
        let mut numbers: Vec<u64> = Vec::new();
        for row in &lines[..num_rows] {
            let chars: Vec<char> = row.chars().collect();
            let s: String = (g_start..g_end)
                .filter_map(|c| chars.get(c).filter(|&&ch| ch != ' ').copied())
                .collect();
            if !s.is_empty() {
                numbers.push(s.parse::<u64>().unwrap());
            }
        }

        // Apply operation
        if numbers.is_empty() {
            continue;
        }
        let result: u64 = if op == '*' {
            numbers.iter().product()
        } else {
            numbers.iter().sum()
        };
        grand_total += result;
    }

    grand_total
}

fn solve_day6_part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return 0;
    }

    let num_rows = lines.len() - 1;
    let operator_row = lines.last().unwrap();
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Find space-only columns in number rows
    let mut space_columns: Vec<bool> = vec![true; max_len + 1];
    for col in 0..max_len {
        let mut is_space = true;
        for row in &lines[..num_rows] {
            let chars: Vec<char> = row.chars().collect();
            if col < chars.len() && chars[col] != ' ' {
                is_space = false;
                break;
            }
        }
        space_columns[col] = is_space;
    }

    // Find column groups
    let mut groups: Vec<(usize, usize)> = Vec::new();
    let mut in_group = false;
    let mut group_start = 0;
    for col in 0..=max_len {
        if col < max_len && !space_columns[col] {
            if !in_group {
                group_start = col;
                in_group = true;
            }
        } else {
            if in_group {
                groups.push((group_start, col));
                in_group = false;
            }
        }
    }

    // Parse operators from last row
    let operators: Vec<(usize, char)> = operator_row
        .char_indices()
        .filter(|&(_, c)| c == '*' || c == '+')
        .collect();

    // In Part 2, numbers are read right-to-left, each column is one digit
    // Most significant digit at top, least significant at bottom
    // Process problems right-to-left

    // Match each operator to a group
    let mut problems: Vec<(usize, usize, char)> = Vec::new();
    for &(op_col, op) in &operators {
        let group_idx = groups
            .iter()
            .position(|&(start, end)| op_col >= start && op_col < end)
            .unwrap();
        let (g_start, g_end) = groups[group_idx];
        problems.push((g_start, g_end, op));
    }

    // Sort problems right-to-left (by start column, descending)
    problems.sort_by(|a, b| b.0.cmp(&a.0));

    let mut grand_total: u64 = 0;

    for (g_start, g_end, op) in problems {
        // Each column within the group is a single digit
        // Read columns from LEFT to RIGHT within the group
        // Each column gives one number: digits from top (MSD) to bottom (LSD)
        let mut numbers: Vec<u64> = Vec::new();

        for col in g_start..g_end {
            let mut digits = String::new();
            for row in &lines[..num_rows] {
                let chars: Vec<char> = row.chars().collect();
                if col < chars.len() {
                    let ch = chars[col];
                    if ch != ' ' {
                        digits.push(ch);
                    }
                }
            }
            if !digits.is_empty() {
                numbers.push(digits.parse::<u64>().unwrap());
            }
        }

        // Apply operation
        if numbers.is_empty() {
            continue;
        }
        let result: u64 = if op == '*' {
            numbers.iter().product()
        } else {
            numbers.iter().sum()
        };
        grand_total += result;
    }

    grand_total
}

fn day10() {
    println!("\n=== Day 10: Factory ===");

    let input = include_str!("../puzzles/2025-10-input.txt");
    let example = include_str!("../puzzles/2025-10-example.txt");

    // Part 1 - test with example
    let example_answer = solve_day10_part1(example);
    println!("Day 10 Part 1 (example): {}", example_answer);
    assert_eq!(example_answer, 7);

    let answer = solve_day10_part1(input);
    println!("Day 10 Part 1 (answer): {}", answer);

    // Part 2 - test with example
    let example_answer2 = solve_day10_part2(example);
    println!("Day 10 Part 2 (example): {}", example_answer2);
    assert_eq!(example_answer2, 33);

    let answer2 = solve_day10_part2(input);
    println!("Day 10 Part 2 (answer): {}", answer2);
}

#[derive(Clone)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

fn parse_machine(line: &str) -> Machine {
    // Parse: [.#.#] (0,2) (1,3) {3,5,4,7}
    let line = line.trim();
    let bracket_end = line.find(']').unwrap();
    let light_str = &line[1..bracket_end];
    let target_lights: Vec<bool> = light_str.chars().map(|c| c == '#').collect();

    let rest = &line[bracket_end + 1..];

    // Parse parenthesized groups and curly braces
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut joltage: Vec<u64> = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = rest.chars().collect();

    while i < chars.len() {
        if chars[i] == '(' {
            let end = chars[i + 1..].iter().position(|&c| c == ')').unwrap();
            let content: String = chars[i + 1..i + 1 + end].iter().collect();
            let nums: Vec<usize> = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            buttons.push(nums);
            i += end + 2;
        } else if chars[i] == '{' {
            let end = chars[i + 1..].iter().position(|&c| c == '}').unwrap();
            let content: String = chars[i + 1..i + 1 + end].iter().collect();
            joltage = content
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            i += end + 2;
        } else {
            i += 1;
        }
    }

    Machine {
        target_lights,
        buttons,
        joltage,
    }
}

fn solve_day10_part1(input: &str) -> u64 {
    let machines: Vec<Machine> = input.lines().filter(|l| !l.trim().is_empty()).map(parse_machine).collect();

    let mut total_presses: u64 = 0;

    for machine in &machines {
        let n = machine.target_lights.len();
        let target: u64 = to_bitmask(&machine.target_lights);
        let button_masks: Vec<u64> = machine
            .buttons
            .iter()
            .map(|b| {
                let mut mask: Vec<bool> = vec![false; n];
                for &idx in b {
                    mask[idx] = true;
                }
                to_bitmask(&mask)
            })
            .collect();

        // BFS to find minimum button presses to reach target from 0
        let min_presses = bfs_lights(target, &button_masks);
        total_presses += min_presses as u64;
    }

    total_presses
}

fn to_bitmask(bools: &[bool]) -> u64 {
    let mut mask: u64 = 0;
    for (i, &b) in bools.iter().enumerate() {
        if b {
            mask |= 1u64 << i;
        }
    }
    mask
}

fn bfs_lights(target: u64, button_masks: &[u64]) -> u32 {
    if target == 0 {
        return 0;
    }

    let mut dist: HashMap<u64, u32> = HashMap::new();
    let mut queue: VecDeque<u64> = VecDeque::new();

    queue.push_back(0);
    dist.insert(0, 0);

    while let Some(state) = queue.pop_front() {
        let d = dist[&state];

        for &mask in button_masks {
            let next = state ^ mask;
            if next == target {
                return d + 1;
            }
            if !dist.contains_key(&next) {
                dist.insert(next, d + 1);
                queue.push_back(next);
            }
        }
    }

    u32::MAX // shouldn't happen for valid puzzles
}

fn solve_day10_part2(input: &str) -> u64 {
    let machines: Vec<Machine> = input.lines().filter(|l| !l.trim().is_empty()).map(parse_machine).collect();

    let mut total_presses: u64 = 0;

    for machine in &machines {
        let n = machine.joltage.len();
        let target: Vec<u64> = machine.joltage.clone();

        // Each button press adds 1 to the specified counters
        // Find minimum total button presses to reach target values
        // This is a linear programming / integer programming problem
        // For small dimensions, we can use BFS or enumeration

        // Since we need to minimize total presses and each press adds 1 to some counters,
        // we can model this as: find non-negative integers x_1, ..., x_m (presses per button)
        // such that A * x = target, minimizing sum(x_i)
        // where A[j][i] = 1 if button i affects counter j, else 0

        let min_presses = solve_joltage(&machine.buttons, &target, n);
        total_presses += min_presses;
    }

    total_presses
}

fn solve_joltage(buttons: &[Vec<usize>], target: &[u64], n_counters: usize) -> u64 {
    // We solve: minimize sum(x_i) s.t. A*x = target, x >= 0 (integers)
    // where A[j][i] = 1 if button i affects counter j, else 0.
    // n equations (counters), m unknowns (buttons). m >= n typically.
    //
    // Use Gaussian elimination to express basic variables in terms of free variables,
    // then enumerate free variables with branch-and-bound.

    let n = n_counters; // number of equations
    let m = buttons.len(); // number of variables

    // Build augmented matrix [A | b] using i64
    // A is n x m, b is target
    let mut mat: Vec<Vec<i64>> = Vec::with_capacity(n);
    for j in 0..n {
        let mut row = vec![0i64; m + 1];
        for (i, btn) in buttons.iter().enumerate() {
            if btn.contains(&j) {
                row[i] = 1;
            }
        }
        row[m] = target[j] as i64;
        mat.push(row);
    }

    // Gaussian elimination with partial pivoting
    let mut pivot_col: Vec<Option<usize>> = vec![None; n]; // pivot_col[row] = which column is pivot
    let mut pivot_row: Vec<Option<usize>> = vec![None; m]; // pivot_row[col] = which row has pivot
    let mut current_row = 0usize;

    for col in 0..m {
        // Find a row >= current_row with nonzero entry in this column
        let mut found = None;
        for r in current_row..n {
            if mat[r][col] != 0 {
                found = Some(r);
                break;
            }
        }
        if let Some(r) = found {
            // Swap rows
            mat.swap(current_row, r);
            // Eliminate this column from all other rows
            for r in 0..n {
                if r != current_row && mat[r][col] != 0 {
                    let factor = mat[r][col];
                    let pivot_val = mat[current_row][col];
                    // Row r = pivot_val * row_r - factor * row_current
                    for c in 0..=m {
                        mat[r][c] = pivot_val * mat[r][c] - factor * mat[current_row][c];
                    }
                }
            }
            pivot_col[current_row] = Some(col);
            pivot_row[col] = Some(current_row);
            current_row += 1;
        }
    }

    let rank = current_row;

    // Check consistency: if any row has all zeros in A but nonzero in b, no solution
    for r in rank..n {
        if mat[r][m] != 0 {
            return u64::MAX; // No solution (shouldn't happen for valid puzzles)
        }
    }

    // Identify basic and free variables
    let mut is_basic = vec![false; m];
    for r in 0..rank {
        if let Some(col) = pivot_col[r] {
            is_basic[col] = true;
        }
    }

    let free_vars: Vec<usize> = (0..m).filter(|&i| !is_basic[i]).collect();
    let basic_vars: Vec<usize> = (0..m).filter(|&i| is_basic[i]).collect();

    // Express basic variables as: x_b = (rhs - sum of free_var contributions) / pivot_val
    // After elimination, each basic row has one pivot column.
    // The row for basic variable col has: pivot_val * x_col + sum_over_free(free_coeff * x_free) = rhs
    // So: x_col = (rhs - sum(free_coeff * x_free)) / pivot_val

    // For each basic variable, store (pivot_val, rhs, coefficients for free vars)
    struct BasicExpr {
        pivot_col: usize,
        pivot_val: i64,
        rhs: i64,
        free_coeffs: Vec<(usize, i64)>, // (free_var_index_in_free_vars, coefficient)
    }

    let mut basic_exprs: Vec<BasicExpr> = Vec::new();
    for r in 0..rank {
        let pcol = pivot_col[r].unwrap();
        let pivot_val = mat[r][pcol];
        let rhs = mat[r][m];

        let mut free_coeffs = Vec::new();
        for (fi, &fvar) in free_vars.iter().enumerate() {
            if mat[r][fvar] != 0 {
                free_coeffs.push((fi, mat[r][fvar]));
            }
        }

        basic_exprs.push(BasicExpr {
            pivot_col: pcol,
            pivot_val,
            rhs,
            free_coeffs,
        });
    }

    let n_free = free_vars.len();

    // Objective: minimize sum of all x_i
    // x_free vars contribute directly.
    // x_basic vars contribute: sum over basic of x_basic
    // Substitute: sum(x_basic) = sum over basic of (rhs - sum(free_coeff * x_free)) / pivot_val
    // Total obj = sum(x_free) + sum(x_basic)
    // = sum(x_free) + sum_b((rhs_b - sum_f(free_coeff_bf * x_free_f)) / pivot_val_b)

    // For branch-and-bound, enumerate free variables.
    // For each free variable assignment, compute basic variables and check non-negativity.

    // Compute bounds for free variables:
    // For each free var, what range can it take?
    // Since all basic vars must be >= 0, and basic vars depend on free vars,
    // we get constraints on free vars.

    // Use branch and bound: enumerate free variables, compute basic vars, track best objective.

    // Upper bound: greedy solution (sum of targets, since each counter needs at most target[j] presses)
    let mut best: i64 = target.iter().map(|&t| t as i64).sum();

    // For a single free variable, we can compute the range analytically.
    // For multiple, we enumerate recursively.

    fn eval_basic(
        free_vals: &[i64],
        basic_exprs: &[BasicExpr],
    ) -> Option<Vec<i64>> {
        let mut basic_vals = Vec::with_capacity(basic_exprs.len());
        for expr in basic_exprs {
            let mut numerator = expr.rhs;
            for &(fi, coeff) in &expr.free_coeffs {
                numerator -= coeff * free_vals[fi];
            }
            if numerator % expr.pivot_val != 0 {
                return None; // Not integer
            }
            let val = numerator / expr.pivot_val;
            if val < 0 {
                return None; // Negative
            }
            basic_vals.push(val);
        }
        Some(basic_vals)
    }

    fn compute_obj(
        free_vals: &[i64],
        basic_exprs: &[BasicExpr],
        free_vars: &[usize],
        m: usize,
    ) -> Option<i64> {
        let basic_vals = eval_basic(free_vals, basic_exprs)?;
        let mut obj: i64 = 0;
        for &v in free_vals {
            if v < 0 {
                return None;
            }
            obj += v;
        }
        for &v in &basic_vals {
            obj += v;
        }
        Some(obj)
    }

    // Compute a lower bound on objective for partial assignment
    // When free variables 0..idx are assigned, compute a lower bound
    // by assuming remaining free vars are 0 (which gives minimum basic vars)
    fn lower_bound(
        free_vals: &[i64],
        idx: usize,
        basic_exprs: &[BasicExpr],
    ) -> i64 {
        // Set unassigned free vars to 0 and compute basic vars
        let mut full_free = free_vals.to_vec();
        while full_free.len() < idx {
            full_free.push(0);
        }
        // Pad remaining with 0
        let n_free_needed = if basic_exprs.is_empty() {
            0
        } else {
            basic_exprs
                .iter()
                .flat_map(|e| e.free_coeffs.iter().map(|&(fi, _)| fi))
                .max()
                .map(|v| v + 1)
                .unwrap_or(0)
        };
        while full_free.len() < n_free_needed {
            full_free.push(0);
        }

        let mut obj: i64 = full_free.iter().sum();
        for expr in basic_exprs {
            let mut numerator = expr.rhs;
            for &(fi, coeff) in &expr.free_coeffs {
                if fi < full_free.len() {
                    numerator -= coeff * full_free[fi];
                }
            }
            // Lower bound: if numerator / pivot_val could be negative, use 0
            let val = if numerator <= 0 { 0 } else { (numerator + expr.pivot_val.abs() - 1) / expr.pivot_val.abs() };
            // Actually just compute the basic var value and clamp to 0
            // For lower bound, basic vars contribute at least 0
            obj += val.max(0);
        }
        obj
    }

    fn search(
        free_vals: &mut Vec<i64>,
        idx: usize,
        n_free: usize,
        basic_exprs: &[BasicExpr],
        free_vars: &[usize],
        m: usize,
        best: &mut i64,
    ) {
        if idx == n_free {
            if let Some(obj) = compute_obj(free_vals, basic_exprs, free_vars, m) {
                if obj < *best {
                    *best = obj;
                }
            }
            return;
        }

        // Compute upper bound for this free variable
        // The free variable x can range such that all basic vars remain >= 0
        // For each basic var: (rhs - coeff * x) / pivot_val >= 0
        // This gives constraints on x depending on signs

        // Compute range for free_vals[idx]
        let mut lo: i64 = 0;
        let mut hi: i64 = i64::MAX / 2;

        // From existing constraints (basic vars must be >= 0)
        for expr in basic_exprs {
            for &(fi, coeff) in &expr.free_coeffs {
                if fi == idx {
                    // expr: pivot_val * x_basic + ... + coeff * x_free[idx] + ... = rhs
                    // With other free vars at their assigned or 0 values:
                    let mut numerator = expr.rhs;
                    for &(fi2, coeff2) in &expr.free_coeffs {
                        if fi2 < idx {
                            numerator -= coeff2 * free_vals[fi2];
                        } else if fi2 == idx {
                            // This is the variable we're bounding
                        }
                        // fi2 > idx: treat as 0 for now (relaxation)
                    }
                    // x_basic = (numerator - coeff * x) / pivot_val >= 0
                    // numerator - coeff * x must have same sign as pivot_val (or be 0)
                    // If pivot_val > 0: numerator - coeff * x >= 0 => coeff * x <= numerator
                    // If pivot_val < 0: numerator - coeff * x <= 0 => coeff * x >= numerator
                    let pv = expr.pivot_val;
                    if pv > 0 {
                        if coeff > 0 {
                            // x <= numerator / coeff
                            hi = hi.min(numerator / coeff);
                        } else if coeff < 0 {
                            // x >= numerator / coeff (since dividing by negative flips)
                            let bound = if numerator % coeff == 0 {
                                numerator / coeff
                            } else {
                                numerator / coeff + 1 // ceiling division for negative
                            };
                            lo = lo.max(bound);
                        }
                    } else if pv < 0 {
                        if coeff > 0 {
                            // numerator - coeff * x <= 0 => coeff * x >= numerator
                            let bound = if numerator % coeff == 0 {
                                numerator / coeff
                            } else {
                                numerator / coeff + 1
                            };
                            lo = lo.max(bound);
                        } else if coeff < 0 {
                            // numerator - coeff * x <= 0 => -coeff * x <= -numerator => coeff * x >= numerator
                            // Wait, coeff < 0, so: numerator - coeff * x <= 0
                            // numerator + |coeff| * x <= 0
                            // This is always satisfied when numerator <= 0 (x >= 0)
                            // When numerator > 0: |coeff| * x >= numerator... wait
                            // coeff < 0: numerator - coeff * x = numerator + |coeff| * x
                            // This is >= numerator >= 0 always (since rhs is non-negative and we subtract non-negative things)
                            // Hmm, this is getting complicated. Let me just bound conservatively.
                            // Actually: numerator + |coeff| * x, and we need this / pivot_val >= 0
                            // pivot_val < 0, so we need numerator + |coeff| * x <= 0
                            // |coeff| * x <= -numerator, which requires numerator < 0
                            // This may be impossible. Let's just skip this constraint.
                        }
                    }
                }
            }
        }

        lo = lo.max(0);
        if lo > hi {
            return; // No feasible solution
        }

        // Also bound by current best objective
        // obj = sum(free) + sum(basic) >= sum(free) >= sum of assigned free + x
        let current_free_sum: i64 = free_vals.iter().sum();
        if current_free_sum >= *best {
            return;
        }
        hi = hi.min(*best - current_free_sum);

        if lo > hi {
            return;
        }

        // Try values from lo to hi. But this could be huge.
        // Key optimization: for basic vars to be integers, x must satisfy divisibility constraints.
        // For each basic var depending on x: (numerator - coeff * x) % pivot_val == 0
        // This means coeff * x ≡ numerator (mod pivot_val)

        // Compute step: LCM of all relevant pivot_val / gcd(pivot_val, coeff)
        let mut step: i64 = 1;
        for expr in basic_exprs.iter() {
            for &(fi, coeff) in expr.free_coeffs.iter() {
                if fi == idx && coeff != 0 {
                    let g: i64 = gcd(coeff.abs() as i64, expr.pivot_val.abs() as i64);
                    let period = expr.pivot_val.abs() / g;
                    step = lcm(step, period);
                    if step > hi - lo + 1 {
                        // Step is too large, just try a few values
                        step = 1;
                        break;
                    }
                }
            }
        }

        // Try values from lo to hi with given step
        // Start from a value near the middle or near the greedy solution
        // For now, try from lo upward
        let mut x = lo;
        while x <= hi {
            free_vals.push(x);
            let lb = lower_bound(free_vals, idx + 1, basic_exprs);
            if lb < *best {
                search(free_vals, idx + 1, n_free, basic_exprs, free_vars, m, best);
            }
            free_vals.pop();
            x += step;
        }
    }

    let mut free_vals: Vec<i64> = Vec::new();
    search(
        &mut free_vals,
        0,
        n_free,
        &basic_exprs,
        &free_vars,
        m,
        &mut best,
    );

    best as u64
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}
