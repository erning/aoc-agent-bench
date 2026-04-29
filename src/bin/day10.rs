use std::fs;

struct Machine {
    target_lights: Vec<bool>,
    buttons_lights: Vec<Vec<usize>>,
    target_joltage: Vec<i64>,
    buttons_joltage: Vec<Vec<usize>>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let bracket_start = line.find('[').unwrap();
        let bracket_end = line.find(']').unwrap();
        let lights_str = &line[bracket_start + 1..bracket_end];
        let target_lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

        let rest = &line[bracket_end + 1..];
        let mut buttons_lights = Vec::new();
        let mut buttons_joltage = Vec::new();
        let mut pos = 0;
        let bytes = rest.as_bytes();
        while pos < bytes.len() {
            if bytes[pos] == b'(' {
                let end = rest[pos..].find(')').unwrap() + pos;
                let inner = &rest[pos + 1..end];
                let indices: Vec<usize> = inner
                    .split(',')
                    .map(|s| s.trim().parse().unwrap())
                    .collect();
                buttons_lights.push(indices.clone());
                buttons_joltage.push(indices);
                pos = end + 1;
            } else {
                pos += 1;
            }
        }

        let brace_start = line.find('{').unwrap();
        let brace_end = line.find('}').unwrap();
        let joltage_str = &line[brace_start + 1..brace_end];
        let target_joltage: Vec<i64> = joltage_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        machines.push(Machine {
            target_lights,
            buttons_lights,
            target_joltage,
            buttons_joltage,
        });
    }
    machines
}

fn solve_gf2(a: &[Vec<bool>], b: &[bool]) -> Option<usize> {
    let n = a.len();
    if n == 0 {
        return Some(0);
    }
    let m = a[0].len();
    if m == 0 {
        return if b.iter().all(|&x| !x) { Some(0) } else { None };
    }

    let mut mat: Vec<Vec<bool>> = Vec::new();
    for i in 0..n {
        let mut row = a[i].clone();
        row.push(b[i]);
        mat.push(row);
    }

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();
    for col in 0..m {
        let mut found = None;
        for row in pivot_row..n {
            if mat[row][col] {
                found = Some(row);
                break;
            }
        }
        if let Some(row) = found {
            mat.swap(pivot_row, row);
            for i in 0..n {
                if i != pivot_row && mat[i][col] {
                    for j in 0..=m {
                        mat[i][j] ^= mat[pivot_row][j];
                    }
                }
            }
            pivot_cols.push(col);
            pivot_row += 1;
        }
    }

    for row in pivot_row..n {
        if mat[row][m] {
            return None;
        }
    }

    let mut free_cols = Vec::new();
    for col in 0..m {
        if !pivot_cols.contains(&col) {
            free_cols.push(col);
        }
    }

    let mut particular = vec![false; m];
    for (i, &col) in pivot_cols.iter().enumerate() {
        particular[col] = mat[i][m];
    }

    let mut null_basis: Vec<Vec<bool>> = Vec::new();
    for &free_col in &free_cols {
        let mut basis = vec![false; m];
        basis[free_col] = true;
        for (i, &pivot_col) in pivot_cols.iter().enumerate() {
            basis[pivot_col] = mat[i][free_col];
        }
        null_basis.push(basis);
    }

    let num_free = free_cols.len();
    let mut best = usize::MAX;

    for mask in 0..(1usize << num_free) {
        let mut solution = particular.clone();
        for j in 0..num_free {
            if (mask >> j) & 1 == 1 {
                for k in 0..m {
                    solution[k] ^= null_basis[j][k];
                }
            }
        }
        let weight = solution.iter().filter(|&&x| x).count();
        best = best.min(weight);
    }

    Some(best)
}

// Two-phase simplex solver
// Returns (solution, objective_value) or None if infeasible
fn simplex_solve(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<(Vec<f64>, f64)> {
    let m = a.len(); // constraints
    let n = a[0].len(); // variables

    if m == 0 {
        return Some((vec![0.0; n], 0.0));
    }

    // Phase I: find initial BFS
    // Variables: x[0..n] (original), s[n..n+m] (artificial)
    let total_vars = n + m;
    let mut tableau = vec![vec![0.0; total_vars + 1]; m + 1];

    // Fill constraint matrix [A | I | b]
    for i in 0..m {
        for j in 0..n {
            tableau[i][j] = a[i][j];
        }
        tableau[i][n + i] = 1.0;
        tableau[i][total_vars] = b[i].max(0.0);
    }

    // If any b[i] < 0, negate that row
    for i in 0..m {
        if b[i] < 0.0 {
            for j in 0..=total_vars {
                tableau[i][j] = -tableau[i][j];
            }
        }
    }

    // Phase I objective: minimize sum of artificials
    // reduced_cost[j] = c_j - c_B^T * a_col_j
    // c_j = 0 for original vars, 1 for artificial vars
    // c_B = [1, 1, ..., 1] (all artificials basic)
    for j in 0..n {
        tableau[m][j] = -a.iter().map(|row| row[j]).sum::<f64>();
    }
    for j in n..total_vars {
        tableau[m][j] = 0.0;
    }
    tableau[m][total_vars] = -b.iter().map(|v| v.max(0.0)).sum::<f64>();

    let mut basis: Vec<usize> = (n..n + m).collect();

    // Phase I simplex iterations
    for _ in 0..10000 {
        // Find entering variable (most negative reduced cost)
        let mut enter = None;
        let mut min_rc = -1e-9;
        for j in 0..total_vars {
            if tableau[m][j] < min_rc {
                min_rc = tableau[m][j];
                enter = Some(j);
            }
        }
        let enter = match enter {
            Some(e) => e,
            None => break,
        };

        // Minimum ratio test
        let mut leave = None;
        let mut min_ratio = f64::INFINITY;
        for i in 0..m {
            if tableau[i][enter] > 1e-9 {
                let ratio = tableau[i][total_vars] / tableau[i][enter];
                if ratio < min_ratio {
                    min_ratio = ratio;
                    leave = Some(i);
                }
            }
        }
        let leave = match leave {
            Some(l) => l,
            None => return None,
        };

        // Pivot
        let pivot_val = tableau[leave][enter];
        for j in 0..=total_vars {
            tableau[leave][j] /= pivot_val;
        }
        for i in 0..=m {
            if i != leave {
                let factor = tableau[i][enter];
                if factor.abs() > 1e-12 {
                    for j in 0..=total_vars {
                        tableau[i][j] -= factor * tableau[leave][j];
                    }
                }
            }
        }
        basis[leave] = enter;
    }

    // Check feasibility
    for i in 0..m {
        if basis[i] >= n && tableau[i][total_vars] > 1e-6 {
            return None;
        }
    }

    // Phase II: minimize original objective
    // Recompute reduced costs
    for j in 0..=total_vars {
        let mut rc = if j < n { c[j] } else { 0.0 };
        for i in 0..m {
            if basis[i] < n {
                rc -= c[basis[i]] * tableau[i][j];
            }
        }
        tableau[m][j] = rc;
    }

    // Phase II iterations
    for _ in 0..10000 {
        let mut enter = None;
        let mut min_rc = -1e-9;
        for j in 0..n {
            if tableau[m][j] < min_rc {
                min_rc = tableau[m][j];
                enter = Some(j);
            }
        }
        let enter = match enter {
            Some(e) => e,
            None => break,
        };

        let mut leave = None;
        let mut min_ratio = f64::INFINITY;
        for i in 0..m {
            if tableau[i][enter] > 1e-9 {
                let ratio = tableau[i][total_vars] / tableau[i][enter];
                if ratio < min_ratio {
                    min_ratio = ratio;
                    leave = Some(i);
                }
            }
        }
        let leave = match leave {
            Some(l) => l,
            None => return None,
        };

        let pivot_val = tableau[leave][enter];
        for j in 0..=total_vars {
            tableau[leave][j] /= pivot_val;
        }
        for i in 0..=m {
            if i != leave {
                let factor = tableau[i][enter];
                if factor.abs() > 1e-12 {
                    for j in 0..=total_vars {
                        tableau[i][j] -= factor * tableau[leave][j];
                    }
                }
            }
        }
        basis[leave] = enter;
    }

    // Extract solution
    let mut solution = vec![0.0; n];
    for i in 0..m {
        if basis[i] < n {
            solution[basis[i]] = tableau[i][total_vars].max(0.0);
        }
    }

    let obj: f64 = c.iter().zip(solution.iter()).map(|(ci, xi)| ci * xi).sum();
    Some((solution, obj))
}

// ILP solver using branch and bound with LP relaxation
fn solve_ilp(a: &[Vec<usize>], target: &[i64]) -> Option<i64> {
    let num_counters = target.len();
    let num_buttons = a.len();

    if num_counters == 0 {
        return Some(0);
    }
    if target.iter().any(|&t| t < 0) {
        return None;
    }
    if num_buttons == 0 {
        return if target.iter().all(|&t| t == 0) { Some(0) } else { None };
    }

    // Build constraint matrix
    let a_f64: Vec<Vec<f64>> = (0..num_counters)
        .map(|i| {
            (0..num_buttons)
                .map(|j| if a[j].contains(&i) { 1.0 } else { 0.0 })
                .collect()
        })
        .collect();
    let b_f64: Vec<f64> = target.iter().map(|&v| v as f64).collect();
    let c: Vec<f64> = vec![1.0; num_buttons]; // minimize sum of presses

    // Solve LP relaxation
    let lp_result = simplex_solve(&a_f64, &b_f64, &c);

    match lp_result {
        Some((sol, obj)) => {
            // Check if solution is (close to) integer
            let is_integer = sol.iter().all(|&x| (x - x.round()).abs() < 1e-6);
            if is_integer {
                return Some(obj.round() as i64);
            }

            // Branch on most fractional variable
            let mut frac_var = 0;
            let mut max_frac = 0.0;
            for (i, &x) in sol.iter().enumerate() {
                let frac = (x - x.floor()).min(x.ceil() - x);
                if frac > max_frac {
                    max_frac = frac;
                    frac_var = i;
                }
            }

            let floor_val = sol[frac_var].floor() as i64;
            let ceil_val = sol[frac_var].ceil() as i64;

            // Try fixing x[frac_var] <= floor_val
            let mut best = None;

            // Branch 1: x[frac_var] <= floor_val
            // Add constraint: x[frac_var] <= floor_val → x[frac_var] + s = floor_val
            // We can handle this by modifying the upper bound
            // Actually, let's use a simpler approach: modify the problem

            // Branch 1: set x[frac_var] = floor_val, reduce target
            let mut new_target1 = target.to_vec();
            for &counter in &a[frac_var] {
                new_target1[counter] -= floor_val;
            }
            let mut new_a1 = a.to_vec();
            new_a1.remove(frac_var);
            if let Some(sub_sol) = solve_ilp(&new_a1, &new_target1) {
                best = Some(sub_sol + floor_val);
            }

            // Branch 2: set x[frac_var] >= ceil_val
            let mut new_target2 = target.to_vec();
            for &counter in &a[frac_var] {
                new_target2[counter] -= ceil_val;
            }
            let mut new_a2 = a.to_vec();
            new_a2.remove(frac_var);
            if let Some(sub_sol) = solve_ilp(&new_a2, &new_target2) {
                let candidate = sub_sol + ceil_val;
                best = match best {
                    Some(b) => Some(b.min(candidate)),
                    None => Some(candidate),
                };
            }

            best
        }
        None => None,
    }
}

fn main() {
    let input = fs::read_to_string("puzzles/2025-10-input.txt").expect("Failed to read input");
    let machines = parse_machines(&input);

    // Part 1
    let mut part1 = 0usize;
    for machine in &machines {
        let n = machine.target_lights.len();
        let m = machine.buttons_lights.len();
        if m == 0 {
            continue;
        }
        let mut a = vec![vec![false; m]; n];
        for (j, button) in machine.buttons_lights.iter().enumerate() {
            for &light in button {
                a[light][j] = true;
            }
        }
        if let Some(presses) = solve_gf2(&a, &machine.target_lights) {
            part1 += presses;
        }
    }

    // Part 2
    let mut part2 = 0i64;
    for machine in &machines {
        if let Some(presses) = solve_ilp(&machine.buttons_joltage, &machine.target_joltage) {
            part2 += presses;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string("puzzles/2025-10-example.txt").unwrap();
        let machines = parse_machines(&input);

        let mut part1 = 0usize;
        for machine in &machines {
            let n = machine.target_lights.len();
            let m = machine.buttons_lights.len();
            let mut a = vec![vec![false; m]; n];
            for (j, button) in machine.buttons_lights.iter().enumerate() {
                for &light in button {
                    a[light][j] = true;
                }
            }
            if let Some(presses) = solve_gf2(&a, &machine.target_lights) {
                part1 += presses;
            }
        }
        assert_eq!(part1, 7);

        let mut part2 = 0i64;
        for machine in &machines {
            let result = solve_ilp(&machine.buttons_joltage, &machine.target_joltage);
            if let Some(presses) = result {
                part2 += presses;
            }
        }
        assert_eq!(part2, 33);
    }
}
