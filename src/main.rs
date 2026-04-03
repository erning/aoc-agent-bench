use std::fs;

// ========== Day 6: Trash Compactor ==========

fn solve_day6(input: &str) -> (u128, u128) {
    let lines: Vec<&str> = input.lines().collect();
    let n_rows = lines.len() - 1;
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let rows: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut ch: Vec<char> = l.chars().collect();
            ch.resize(max_len, ' ');
            ch
        })
        .collect();

    let mut col_groups: Vec<(Vec<usize>, char)> = Vec::new();
    let mut current_cols: Vec<usize> = Vec::new();

    for c in 0..max_len {
        let is_sep = (0..=n_rows).all(|r| rows[r][c] == ' ');
        if is_sep {
            if !current_cols.is_empty() {
                let op_char = rows[n_rows][current_cols[0]];
                col_groups.push((current_cols.clone(), op_char));
                current_cols.clear();
            }
        } else {
            current_cols.push(c);
        }
    }
    if !current_cols.is_empty() {
        let op_char = rows[n_rows][current_cols[0]];
        col_groups.push((current_cols, op_char));
    }

    let part1: u128 = col_groups.iter().map(|(cols, op)| {
        let nums: Vec<u128> = (0..n_rows)
            .map(|r| {
                let s: String = cols.iter().map(|&c| rows[r][c]).collect();
                s.trim().parse().unwrap_or(0)
            })
            .collect();
        apply_op(&nums, *op)
    }).sum();

    let part2: u128 = col_groups.iter().map(|(cols, op)| {
        let numbers: Vec<u128> = cols.iter().rev().map(|&col| {
            let digits: String = (0..n_rows)
                .map(|r| rows[r][col])
                .filter(|c| c.is_ascii_digit())
                .collect();
            digits.parse().unwrap_or(0)
        }).collect();
        apply_op(&numbers, *op)
    }).sum();

    (part1, part2)
}

fn apply_op(nums: &[u128], op: char) -> u128 {
    match op {
        '+' => nums.iter().sum::<u128>(),
        '*' => nums.iter().product::<u128>(),
        _ => 0,
    }
}

// ========== Day 10: Factory ==========

/// Simplified fraction type using f64 with rounding (accurate for small integer problems)
type Frac = f64;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

fn parse_day10(input: &str) -> Vec<Machine> {
    input.lines().filter(|l| !l.trim().is_empty()).map(|line| {
        let mut lights = Vec::new();
        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() && chars[i] == ' ' { i += 1; }
        if i < chars.len() && chars[i] == '[' {
            i += 1;
            while i < chars.len() && chars[i] != ']' {
                if chars[i] == '#' { lights.push(true); }
                else if chars[i] == '.' { lights.push(false); }
                i += 1;
            }
            if i < chars.len() { i += 1; }
        }

        while i < chars.len() {
            while i < chars.len() && chars[i] == ' ' { i += 1; }
            if i >= chars.len() || chars[i] != '(' { break; }
            i += 1;
            let mut btn = Vec::new();
            while i < chars.len() && chars[i] != ')' {
                if chars[i].is_ascii_digit() {
                    let start = i;
                    while i < chars.len() && chars[i].is_ascii_digit() { i += 1; }
                    btn.push(line[start..i].parse().unwrap());
                } else { i += 1; }
            }
            if i < chars.len() { i += 1; }
            buttons.push(btn);
        }

        while i < chars.len() && chars[i] == ' ' { i += 1; }
        if i < chars.len() && chars[i] == '{' {
            i += 1;
            while i < chars.len() && chars[i] != '}' {
                if chars[i].is_ascii_digit() {
                    let start = i;
                    while i < chars.len() && chars[i].is_ascii_digit() { i += 1; }
                    joltage.push(line[start..i].parse().unwrap());
                } else { i += 1; }
            }
        }

        Machine { lights, buttons, joltage }
    }).collect()
}

// ========== Part 1: GF(2) ==========

fn solve_part1(machine: &Machine) -> u64 {
    let n = machine.lights.len();
    let m = machine.buttons.len();
    if n == 0 || m == 0 { return 0; }

    let mut mat: Vec<u64> = Vec::new();
    for row in 0..n {
        let mut r: u64 = 0;
        for (j, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&row) { r |= 1u64 << j; }
        }
        if machine.lights[row] { r |= 1u64 << m; }
        mat.push(r);
    }

    let mut pivot_row_for_col: Vec<Option<usize>> = vec![None; m];
    let mut row_idx = 0usize;
    for col in 0..m {
        let mut found = None;
        for r in row_idx..n {
            if (mat[r] >> col) & 1 == 1 { found = Some(r); break; }
        }
        let Some(pr) = found else { continue; };
        mat.swap(row_idx, pr);
        pivot_row_for_col[col] = Some(row_idx);
        for r in 0..n {
            if r != row_idx && (mat[r] >> col) & 1 == 1 { mat[r] ^= mat[row_idx]; }
        }
        row_idx += 1;
    }

    for r in row_idx..n {
        if (mat[r] >> m) & 1 == 1 { return u64::MAX; }
    }

    let pivot_cols: Vec<usize> = (0..m)
        .filter(|&c| pivot_row_for_col[c].is_some())
        .collect();
    let free_cols: Vec<usize> = (0..m)
        .filter(|&c| !pivot_cols.contains(&c))
        .collect();
    let nf = free_cols.len();

    if nf > 24 {
        return solve_p1_heuristic(&mat, m, &pivot_row_for_col, &free_cols);
    }

    let mut best = u64::MAX;
    for mask in 0..(1usize << nf) {
        let mut sol = vec![0u64; m];
        for (i, &fc) in free_cols.iter().enumerate() {
            sol[fc] = ((mask >> i) & 1) as u64;
        }
        for &col in &pivot_cols {
            let Some(pr) = pivot_row_for_col[col] else { continue; };
            let mut val = (mat[pr] >> m) & 1;
            for c in 0..m {
                if c != col && (mat[pr] >> c) & 1 == 1 { val ^= sol[c] & 1; }
            }
            sol[col] = val;
        }
        let weight = sol.iter().sum::<u64>();
        if weight < best { best = weight; }
    }
    best
}

fn solve_p1_heuristic(
    mat: &[u64], m: usize,
    pivot_row_for_col: &[Option<usize>], free_cols: &[usize],
) -> u64 {
    let pivot_cols: Vec<usize> = (0..m)
        .filter(|&c| pivot_row_for_col[c].is_some())
        .collect();
    let mut best = u64::MAX;

    for &fc in free_cols {
        let mut sol = vec![0u64; m];
        sol[fc] = 1;
        for &col in &pivot_cols {
            let Some(pr) = pivot_row_for_col[col] else { continue; };
            let mut val = (mat[pr] >> m) & 1;
            for c in 0..m {
                if c != col && (mat[pr] >> c) & 1 == 1 { val ^= sol[c] & 1; }
            }
            sol[col] = val;
        }
        best = best.min(sol.iter().sum::<u64>());
    }
    best
}

// ========== Part 2: ILP via RREF ==========

/// Result of Gaussian elimination on augmented matrix [A | b]
struct RREF {
    rank: usize,
    n_vars: usize,
    n_constraints: usize,
    rref_matrix: Vec<Vec<Frac>>, // augmented RREF, size n_constraints x (n_vars + 1)
    pivot_cols: Vec<usize>,       // which columns are pivots
}

/// Compute RREF of augmented matrix [A | b]
fn rref(a: &[Vec<u64>], b: &[u64]) -> RREF {
    let n = a.len(); // number of constraints
    let m = a[0].len(); // number of variables
    let mut mat: Vec<Vec<Frac>> = Vec::new();
    for row in 0..n {
        let mut r: Vec<Frac> = Vec::with_capacity(m + 1);
        for j in 0..m {
            r.push(a[row][j] as Frac);
        }
        r.push(b[row] as Frac);
        mat.push(r);
    }

    let mut pivot_cols = Vec::new();
    let mut row_idx = 0;

    for col in 0..m {
        // Find pivot row
        let mut found = None;
        for r in row_idx..n {
            if mat[r][col].abs() > 1e-9 {
                found = Some(r);
                break;
            }
        }
        let Some(pr) = found else { continue; };
        mat.swap(row_idx, pr);
        pivot_cols.push(col);

        // Scale pivot row
        let pivot = mat[row_idx][col];
        for c in 0..=m {
            mat[row_idx][c] /= pivot;
        }
        // Eliminate all other rows
        for r in 0..n {
            if r != row_idx && mat[r][col].abs() > 1e-9 {
                let f = mat[r][col];
                for c in 0..=m {
                    mat[r][c] -= f * mat[row_idx][c];
                }
            }
        }
        row_idx += 1;
    }

    RREF {
        rank: row_idx,
        n_vars: m,
        n_constraints: n,
        rref_matrix: mat,
        pivot_cols,
    }
}

/// Solve Part 2 ILP: min sum(x) s.t. Ax=b, x>=0, x integer
fn solve_ilp(machine: &Machine) -> u64 {
    let b = &machine.joltage;
    let m = machine.buttons.len();
    let n = b.len();
    if n == 0 || b.iter().all(|&x| x == 0) { return 0; }

    // Build coefficient matrix: a[i][j] = 1 if counter i in button j
    let a: Vec<Vec<u64>> = (0..n).map(|i| {
        machine.buttons.iter().map(|btn| {
            if btn.contains(&i) { 1 } else { 0 }
        }).collect()
    }).collect();

    let rref = rref(&a, b);
    let RREF { rank, n_vars, n_constraints, rref_matrix: mat, pivot_cols, .. } = rref;

    // Check consistency
    for r in rank..n_constraints {
        if mat[r][n_vars].abs() > 0.5 {
            return u64::MAX; // inconsistent
        }
    }

    // Identify free columns
    let pivot_set: Vec<bool> = (0..m).map(|c| pivot_cols.contains(&c)).collect();
    let free_cols: Vec<usize> = (0..m).filter(|&c| !pivot_set[c]).collect();
    let n_free = free_cols.len();

    // If rank < number of free variables used, there are free variables
    // Build the parametric solution:
    //   x[pivot_col[p]] = mat[p][m] - sum(mat[p][free_col[j]] * t[j] for j)
    //   x[free_col[j]] = t[j]
    // We need all x >= 0 and integer.

    // Build particular solution (all free vars = 0)
    let mut x0 = vec![0.0f64; m];
    for (p, &pc) in pivot_cols.iter().enumerate() {
        x0[pc] = mat[p][n_vars];
    }

    // Build nullspace basis: for each free variable j,
    // setting t[j] = 1 and all other free vars = 0:
    //   x[pivot_col[p]] changes by -mat[p][free_col[j]]
    //   x[free_col[j]] = 1
    // Null vector v_j:
    //   v_j[free_col[j]] = 1
    //   v_j[pivot_col[p]] = -mat[p][free_col[j]]
    //   other entries = 0
    let mut nullspace: Vec<Vec<Frac>> = Vec::new();
    for (j, &fc) in free_cols.iter().enumerate() {
        let mut v = vec![0.0f64; m];
        v[fc] = 1.0;
        for (p, &pc) in pivot_cols.iter().enumerate() {
            v[pc] = -mat[p][fc];
        }
        nullspace.push(v);
    }

    // Now we need to find t[0..n_free-1] (all integers) such that:
    //   x0 + sum(t[j] * nullspace[j]) >= 0
    // and all entries are integers (they should be since a and b are integers)
    // Minimize sum of all x entries.

    // For integer solutions: since the nullspace may have fractional entries,
    // we need to ensure the solution is integer-valued.
    // Actually, since A and b are integer, and we're solving over R,
    // the nullspace could have fractional entries. But we need x integer.

    // Check if the particular solution is integer
    let x0_integer = x0.iter().all(|&v| (v - v.round()).abs() < 0.1);
    // Check if all nullspace vectors are integer
    let nullspace_integer = nullspace.iter().all(|v| {
        v.iter().all(|&val| (val - val.round()).abs() < 0.1)
    });

    if n_free == 0 {
        // Determined system
        if x0_integer && x0.iter().all(|&v| v >= -0.1) {
            return x0.iter().map(|v| v.max(0.0).round() as u64).sum();
        }
        return u64::MAX;
    }

    // Enumerate over free variables
    // For each free var t_j, find the valid range from x >= 0 constraints
    //   x0[i] + sum_j t[j] * nullspace[j][i] >= 0
    // For each constraint i, this gives a linear inequality in t.

    // Since n_free <= 3 (from analysis), we can enumerate.
    // But t_j must be integer, so we iterate over integer t_j.

    // Round nullspace to integers for integer arithmetic
    let nullspace_round: Vec<Vec<isize>> = nullspace.iter().map(|v| {
        v.iter().map(|&val| val.round() as isize).collect()
    }).collect();
    let x0_round: Vec<isize> = x0.iter().map(|&v| v.round() as isize).collect();

    // Check if rounding is accurate enough
    let acc_ok = nullspace.iter().enumerate().all(|(j, v)| {
        v.iter().zip(nullspace_round[j].iter()).all(|(&exact, &rounded)| {
            (exact - rounded as f64).abs() < 0.1
        })
    }) && x0.iter().zip(x0_round.iter()).all(|(&exact, &rounded)| {
        (exact - rounded as f64).abs() < 0.1
    });

    if !acc_ok {
        // Use float arithmetic with tolerance
        return enumerate_float(&x0, &nullspace, m);
    }

    enumerate_integer(&x0_round, &nullspace_round, m)
}

fn enumerate_integer(x0: &[isize], nullspace: &[Vec<isize>], m: usize) -> u64 {
    let n_free = nullspace.len();

    // For each variable index i, and each free var j:
    // x0[i] + sum_j t[j] * nullspace[j][i] >= 0
    // This constrains t[j].

    // Find bounds for each t[j] individually
    let inf: isize = 1000;
    let mut bounds: Vec<(isize, isize)> = vec![(-inf, inf); n_free];

    for j in 0..n_free {
        for i in 0..m {
            let c = nullspace[j][i];
            if c != 0 {
                // x0[i] + t[j] * c >= 0  →  t[j] >= -x0[i]/c  (c>0)  or  t[j] <= -x0[i]/c  (c<0)
                if c > 0 {
                    // t[j] >= ceil(-x0[i] / c) = (-x0[i]).div_ceil(c)
                    let neg_x0 = -x0[i];
                    let lo = (neg_x0 + c - 1) / c; // ceil division for positive c
                    bounds[j].0 = bounds[j].0.max(lo);
                } else {
                    // t[j] <= floor(-x0[i] / c) where c < 0
                    let neg_x0 = -x0[i];
                    let hi = neg_x0.div_euclid(c);
                    bounds[j].1 = bounds[j].1.min(hi);
                }
            }
        }
    }

    // Cap bounds to reasonable values
    for j in 0..n_free {
        if bounds[j].0 < -500 { bounds[j].0 = -500; }
        if bounds[j].1 > 500 { bounds[j].1 = 500; }
    }

    let mut best = u64::MAX;

    enumerate_rec(0, m, n_free, x0, nullspace, &bounds, &mut vec![0isize; n_free], &mut best);
    best
}

fn enumerate_rec(
    idx: usize, m: usize, n_free: usize,
    x0: &[isize], nullspace: &[Vec<isize>],
    bounds: &[(isize, isize)],
    t: &mut Vec<isize>,
    best: &mut u64,
) {
    if idx == n_free {
        // Compute solution and check non-negativity
        let mut sum = 0isize;
        for i in 0..m {
            let val = x0[i] + (0..n_free).map(|j| t[j] * nullspace[j][i]).sum::<isize>();
            if val < 0 { return; }
            sum += val;
        }
        if (sum as u64) < *best {
            *best = sum as u64;
        }
        return;
    }

    for v in bounds[idx].0..=bounds[idx].1 {
        t[idx] = v;
        enumerate_rec(idx + 1, m, n_free, x0, nullspace, bounds, t, best);
    }
}

fn enumerate_float(x0: &[f64], nullspace: &[Vec<f64>], m: usize) -> u64 {
    let n_free = nullspace.len();
    let mut best = u64::MAX;

    // Try small integer values for each free variable
    let range = -50..=50;

    let mut t = vec![0isize; n_free];

    #[allow(clippy::too_many_arguments)]
    fn go(
        idx: usize, m: usize, n_free: usize,
        x0: &[f64], nullspace: &[Vec<f64>],
        range: &std::ops::RangeInclusive<isize>,
        t: &mut Vec<isize>,
        best: &mut u64,
    ) {
        if idx == n_free {
            let mut sum = 0.0f64;
            for i in 0..m {
                let val = x0[i] + (0..n_free).map(|j| (t[j] as f64) * nullspace[j][i]).sum::<f64>();
                if val < -0.1 { return; }
                sum += val.abs();
            }
            let rounded = sum.round() as u64;
            if rounded < *best { *best = rounded; }
            return;
        }
        for v in range.clone() {
            t[idx] = v;
            go(idx + 1, m, n_free, x0, nullspace, range, t, best);
        }
    }

    go(0, m, n_free, x0, nullspace, &range, &mut t, &mut best);
    best
}

fn main() {
    // --- Day 6 ---
    let day6_input =
        fs::read_to_string("puzzles/2025-06-input.txt").expect("Day 6 input");
    let (d6p1, d6p2) = solve_day6(&day6_input);
    println!("Day 6 Part 1: {}", d6p1);
    println!("Day 6 Part 2: {}", d6p2);

    let day6_ex = fs::read_to_string("puzzles/2025-06-example.txt").unwrap();
    let (ex6p1, ex6p2) = solve_day6(&day6_ex);
    assert_eq!(ex6p1, 4277556);
    assert_eq!(ex6p2, 3263827);
    println!("  Day 6 example verified: Part1={ex6p1}, Part2={ex6p2}");

    // --- Day 10 ---
    let day10_input =
        fs::read_to_string("puzzles/2025-10-input.txt").expect("Day 10 input");
    let machines = parse_day10(&day10_input);

    let ans1: u64 = machines.iter().map(|m| solve_part1(m)).sum();
    println!("Day 10 Part 1: {}", ans1);

    let ex_machines = parse_day10(
        &fs::read_to_string("puzzles/2025-10-example.txt").unwrap());
    let ex1: u64 = ex_machines.iter().map(|m| solve_part1(m)).sum();
    assert_eq!(ex1, 7);
    println!("  Day 10 example Part 1: {ex1} (verified)");

    let ans2: u64 = machines.iter().map(|m| solve_ilp(m)).sum();
    println!("Day 10 Part 2: {}", ans2);

    let ex2: u64 = ex_machines.iter().map(|m| solve_ilp(m)).sum();
    assert_eq!(ex2, 33);
    println!("  Day 10 example Part 2: {ex2} (verified)");
}
