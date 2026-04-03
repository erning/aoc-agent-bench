use std::fs;

fn main() {
    let input = fs::read_to_string("puzzles/2025-10-input.txt").expect("Failed to read input");
    let mut total_p1: u64 = 0;
    let mut total_p2: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (target_lights, buttons, joltage) = parse_machine(line);
        total_p1 += solve_part1(&target_lights, &buttons);
        total_p2 += solve_part2(&buttons, &joltage);
    }

    println!("Part 1: {}", total_p1);
    println!("Part 2: {}", total_p2);
}

fn parse_machine(line: &str) -> (Vec<bool>, Vec<Vec<usize>>, Vec<u64>) {
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];
    let target_lights: Vec<bool> = diagram.chars().map(|c| c == '#').collect();

    let rest = &line[bracket_end + 1..];
    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut joltage: Vec<u64> = Vec::new();

    let mut i = 0;
    let chars: Vec<char> = rest.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            let end = rest[i..].find(')').unwrap() + i;
            let content = &rest[i + 1..end];
            let indices: Vec<usize> = content.split(',').map(|s| s.trim().parse().unwrap()).collect();
            buttons.push(indices);
            i = end + 1;
        } else if chars[i] == '{' {
            let end = rest[i..].find('}').unwrap() + i;
            let content = &rest[i + 1..end];
            joltage = content.split(',').map(|s| s.trim().parse().unwrap()).collect();
            i = end + 1;
        } else {
            i += 1;
        }
    }

    (target_lights, buttons, joltage)
}

fn solve_part1(target: &[bool], buttons: &[Vec<usize>]) -> u64 {
    let n_lights = target.len();
    let n_buttons = buttons.len();

    let mut cols: Vec<u64> = Vec::new();
    for button in buttons {
        let mut mask: u64 = 0;
        for &idx in button {
            if idx < n_lights {
                mask |= 1 << idx;
            }
        }
        cols.push(mask);
    }

    let mut target_mask: u64 = 0;
    for (i, &t) in target.iter().enumerate() {
        if t {
            target_mask |= 1 << i;
        }
    }

    // Gaussian elimination over GF(2)
    // Build augmented matrix as rows [A | target]
    let mut rows: Vec<u64> = Vec::new();
    for i in 0..n_lights {
        let mut row: u64 = 0;
        for (j, &col) in cols.iter().enumerate() {
            if col & (1 << i) != 0 {
                row |= 1 << j;
            }
        }
        if target_mask & (1 << i) != 0 {
            row |= 1 << n_buttons;
        }
        rows.push(row);
    }

    let mut pivot_col: Vec<Option<usize>> = vec![None; n_lights];
    let mut used_rows = 0;
    let mut free_vars: Vec<usize> = Vec::new();

    for col in 0..n_buttons {
        let mut pivot = None;
        for row in used_rows..n_lights {
            if rows[row] & (1 << col) != 0 {
                pivot = Some(row);
                break;
            }
        }
        if let Some(p) = pivot {
            rows.swap(used_rows, p);
            for row in 0..n_lights {
                if row != used_rows && rows[row] & (1 << col) != 0 {
                    rows[row] ^= rows[used_rows];
                }
            }
            pivot_col[used_rows] = Some(col);
            used_rows += 1;
        } else {
            free_vars.push(col);
        }
    }

    for row in used_rows..n_lights {
        if rows[row] & (1 << n_buttons) != 0 {
            return u64::MAX;
        }
    }

    let n_free = free_vars.len();
    let mut min_presses = u64::MAX;

    for free_mask in 0u64..(1 << n_free) {
        let mut x = vec![0u8; n_buttons];

        for (i, &var) in free_vars.iter().enumerate() {
            x[var] = ((free_mask >> i) & 1) as u8;
        }

        for row in 0..used_rows {
            let pivot = pivot_col[row].unwrap();
            let mut val = ((rows[row] >> n_buttons) & 1) as u8;
            for &var in &free_vars {
                if rows[row] & (1 << var) != 0 {
                    val ^= x[var];
                }
            }
            x[pivot] = val;
        }

        let presses: u64 = x.iter().map(|&v| v as u64).sum();
        if presses < min_presses {
            min_presses = presses;
        }
    }

    min_presses
}

fn solve_part2(buttons: &[Vec<usize>], joltage: &[u64]) -> u64 {
    let n_counters = joltage.len();
    let n_buttons = buttons.len();

    // Build matrix A: A[i][j] = 1 if button j affects counter i
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n_buttons]; n_counters];
    for (j, button) in buttons.iter().enumerate() {
        for &idx in button {
            if idx < n_counters {
                a[idx][j] = 1.0;
            }
        }
    }

    let b: Vec<f64> = joltage.iter().map(|&v| v as f64).collect();

    // Minimize c^T x subject to Ax = b, x >= 0
    // Using Big-M simplex method
    let m = n_counters;
    let n = n_buttons;
    let total_vars = n + m; // original + artificial
    let big_m: f64 = 1e7;

    // Tableau: m constraint rows + 1 objective row
    // Columns: total_vars variables + 1 RHS
    let mut tableau: Vec<Vec<f64>> = vec![vec![0.0; total_vars + 1]; m + 1];

    for i in 0..m {
        for j in 0..n {
            tableau[i][j] = a[i][j];
        }
        tableau[i][n + i] = 1.0;
        tableau[i][total_vars] = b[i];
    }

    // Objective row stores z_j - c_j values
    // Initially: z_j - c_j = c_B^T * B^{-1} * a_j - c_j
    // With B = I (artificial), c_B = [M,...,M]:
    // For original var j: M * sum_i(A[i][j]) - 1
    // For artificial var j: M - M = 0
    for j in 0..n {
        let col_sum: f64 = (0..m).map(|i| a[i][j]).sum();
        tableau[m][j] = big_m * col_sum - 1.0;
    }
    // Artificial variables: 0 (already initialized)

    // RHS of objective row: z_0 = c_B^T * b = M * sum(b)
    tableau[m][total_vars] = big_m * b.iter().sum::<f64>();

    let mut basis: Vec<usize> = (n..n + m).collect();

    // Simplex iterations (minimization: enter on most positive z_j - c_j)
    let max_iter = 50000;
    for _ in 0..max_iter {
        // Find entering variable: most positive z_j - c_j
        let mut enter_col = 0;
        let mut max_val = 1e-9;
        for j in 0..total_vars {
            if tableau[m][j] > max_val {
                max_val = tableau[m][j];
                enter_col = j;
            }
        }
        if max_val <= 1e-9 {
            break; // Optimal
        }

        // Minimum ratio test for leaving variable
        let mut leave_row = None;
        let mut min_ratio = f64::MAX;
        for i in 0..m {
            if tableau[i][enter_col] > 1e-10 {
                let ratio = tableau[i][total_vars] / tableau[i][enter_col];
                if ratio < min_ratio - 1e-12 {
                    min_ratio = ratio;
                    leave_row = Some(i);
                }
            }
        }

        let leave_row = leave_row.expect("LP unbounded");

        // Pivot
        let pivot = tableau[leave_row][enter_col];
        for j in 0..=total_vars {
            tableau[leave_row][j] /= pivot;
        }
        for i in 0..=m {
            if i != leave_row {
                let factor = tableau[i][enter_col];
                if factor.abs() > 1e-14 {
                    for j in 0..=total_vars {
                        tableau[i][j] -= factor * tableau[leave_row][j];
                    }
                }
            }
        }
        basis[leave_row] = enter_col;
    }

    // Extract solution
    let mut total: f64 = 0.0;
    for i in 0..m {
        if basis[i] < n {
            total += tableau[i][total_vars];
        }
    }

    total.round() as u64
}
