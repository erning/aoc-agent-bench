use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<bool>>,
    counters: Vec<i64>,
}

fn parse_machines(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        let bs = line.find('[').unwrap();
        let be = line.find(']').unwrap();
        let lights: Vec<bool> = line[bs+1..be].chars().map(|c| c == '#').collect();

        let mut buttons = Vec::new();
        let mut pos = be + 1;
        while let Some(ps) = line[pos..].find('(') {
            let abs = pos + ps;
            let pe = line[abs..].find(')').unwrap();
            let indices: Vec<usize> = line[abs+1..abs+pe].split(',')
                .map(|s| s.trim().parse().unwrap()).collect();
            let max_i = indices.iter().max().copied().unwrap_or(0);
            let mut btn = vec![false; max_i + 1];
            for &i in &indices { btn[i] = true; }
            buttons.push(btn);
            pos = abs + pe + 1;
        }

        let bs = line.find('{').unwrap();
        let be = line.find('}').unwrap();
        let counters: Vec<i64> = line[bs+1..be].split(',')
            .map(|s| s.trim().parse().unwrap()).collect();

        machines.push(Machine { lights, buttons, counters });
    }
    machines
}

// ============================================================
// Part 1: GF(2) linear system
// ============================================================

fn gauss_gf2(mat: &mut [Vec<bool>], rhs: &mut [bool]) -> (usize, Vec<usize>) {
    if mat.is_empty() { return (0, vec![]); }
    let cols = mat[0].len();
    let mut rank = 0;
    let mut col = 0;
    let mut pivots = vec![];

    while col < cols && rank < mat.len() {
        let pivot = (rank..mat.len()).find(|&r| mat[r][col]);
        if pivot.is_none() { col += 1; continue; }
        let pr = pivot.unwrap();
        mat.swap(rank, pr);
        rhs.swap(rank, pr);
        for r in 0..mat.len() {
            if r != rank && mat[r][col] {
                for c in col..cols { mat[r][c] ^= mat[rank][c]; }
                rhs[r] ^= rhs[rank];
            }
        }
        pivots.push(col);
        rank += 1;
        col += 1;
    }
    (rank, pivots)
}

fn solve_part1(machine: &Machine) -> u64 {
    let nl = machine.lights.len();
    let m = machine.buttons.iter().map(|b| b.len()).max().unwrap_or(0).max(nl);
    let n = machine.buttons.len();

    let mat: Vec<Vec<bool>> = (0..m).map(|i| {
        (0..n).map(|j| i < machine.buttons[j].len() && machine.buttons[j][i]).collect()
    }).collect();
    let rhs: Vec<bool> = (0..m).map(|i| if i < nl { machine.lights[i] } else { false }).collect();

    let keep: Vec<usize> = (0..m).filter(|&i| mat[i].iter().any(|&v| v) || rhs[i]).collect();
    let mat: Vec<Vec<bool>> = keep.iter().map(|&i| mat[i].clone()).collect();
    let rhs: Vec<bool> = keep.iter().map(|&i| rhs[i]).collect();

    if mat.is_empty() { return 0; }

    let mut em = mat.clone();
    let mut er = rhs.clone();
    let (rank, pivots) = gauss_gf2(&mut em, &mut er);

    for i in rank..em.len() { if er[i] { return u64::MAX; } }

    let mut free = vec![];
    let mut pi = 0;
    for c in 0..n {
        if pi < pivots.len() && pivots[pi] == c { pi += 1; } else { free.push(c); }
    }

    let rows = em.len().min(rank);
    let xp: Vec<bool> = (0..n).map(|c| {
        pivots.iter().position(|&p| p == c).map_or(false, |r| er[r])
    }).collect();

    let nullity = free.len();
    let nullspace: Vec<Vec<bool>> = free.iter().map(|&fc| {
        let mut ns = vec![false; n];
        ns[fc] = true;
        for r in 0..rows {
            if em[r][fc] { ns[pivots[r]] = true; }
        }
        ns
    }).collect();

    let mut best = u64::MAX;
    for mask in 0..(1u64 << nullity) {
        let mut x = xp.clone();
        let mut w = 0u64;
        for i in 0..nullity {
            if (mask >> i) & 1 == 1 {
                for j in 0..n {
                    if nullspace[i][j] { x[j] ^= true; }
                }
            }
        }
        for j in 0..n { if x[j] { w += 1; } }
        if w < best { best = w; }
    }
    best
}

// ============================================================
// Part 2: Integer linear system
// ============================================================

fn gcd(a: i128, b: i128) -> i128 { if b == 0 { a } else { gcd(b, a % b) } }

#[derive(Clone, Debug)]
struct Rat { num: i128, den: i128 }

impl Rat {
    fn new(n: i128, d: i128) -> Self {
        let mut r = Rat { num: n, den: d };
        r.reduce();
        r
    }
    fn int(n: i128) -> Self { Rat { num: n, den: 1 } }
    fn reduce(&mut self) {
        if self.den < 0 { self.num = -self.num; self.den = -self.den; }
        let g = gcd(self.num.abs(), self.den.abs());
        if g > 1 { self.num /= g; self.den /= g; }
    }
    fn zero() -> Self { Rat { num: 0, den: 1 } }
    fn is_zero(&self) -> bool { self.num == 0 }
    fn to_int(&self) -> Option<i128> { if self.den == 1 { Some(self.num) } else { None } }
}

impl std::ops::Add for &Rat {
    type Output = Rat;
    fn add(self, o: &Rat) -> Rat { Rat::new(self.num*o.den + o.num*self.den, self.den*o.den) }
}
impl std::ops::Sub for &Rat {
    type Output = Rat;
    fn sub(self, o: &Rat) -> Rat { Rat::new(self.num*o.den - o.num*self.den, self.den*o.den) }
}
impl std::ops::Mul for &Rat {
    type Output = Rat;
    fn mul(self, o: &Rat) -> Rat { Rat::new(self.num*o.num, self.den*o.den) }
}
impl std::ops::Div for &Rat {
    type Output = Rat;
    fn div(self, o: &Rat) -> Rat { Rat::new(self.num*o.den, self.den*o.num) }
}
impl std::ops::Neg for &Rat {
    type Output = Rat;
    fn neg(self) -> Rat { Rat::new(-self.num, self.den) }
}

/// Returns (rank, pivot_cols) and modifies mat/rhs to RREF.
fn gauss_rat(mat: &mut [Vec<Rat>], rhs: &mut [Rat]) -> (usize, Vec<usize>) {
    if mat.is_empty() { return (0, vec![]); }
    let cols = mat[0].len();
    let mut rank = 0;
    let mut col = 0;
    let mut pivots = vec![];

    while col < cols && rank < mat.len() {
        let pivot = (rank..mat.len()).find(|&r| !mat[r][col].is_zero());
        if pivot.is_none() { col += 1; continue; }
        let pr = pivot.unwrap();
        mat.swap(rank, pr);
        rhs.swap(rank, pr);

        let pv = mat[rank][col].clone();
        for c in col..cols { mat[rank][c] = &mat[rank][c] / &pv; }
        rhs[rank] = &rhs[rank] / &pv;

        for r in 0..mat.len() {
            if r != rank {
                let f = mat[r][col].clone();
                if !f.is_zero() {
                    for c in col..cols {
                        let s = &f * &mat[rank][c];
                        mat[r][c] = &mat[r][c] - &s;
                    }
                    let s = &f * &rhs[rank];
                    rhs[r] = &rhs[r] - &s;
                }
            }
        }
        pivots.push(col);
        rank += 1;
        col += 1;
    }
    (rank, pivots)
}

fn solve_part2(machine: &Machine) -> u64 {
    let nc = machine.counters.len();
    let n = machine.buttons.len();
    let m = machine.buttons.iter().map(|b| b.len()).max().unwrap_or(0).max(nc);

    let mat: Vec<Vec<Rat>> = (0..m).map(|i| {
        (0..n).map(|j| {
            if i < machine.buttons[j].len() && machine.buttons[j][i] { Rat::int(1) } else { Rat::zero() }
        }).collect()
    }).collect();
    let rhs_vec: Vec<Rat> = (0..m).map(|i| {
        Rat::int(if i < nc { machine.counters[i] as i128 } else { 0 })
    }).collect();

    // Remove zero rows with zero RHS
    let keep: Vec<usize> = (0..m).filter(|&i| {
        mat[i].iter().any(|v| !v.is_zero()) || !rhs_vec[i].is_zero()
    }).collect();
    let mat: Vec<Vec<Rat>> = keep.iter().map(|&i| mat[i].clone()).collect();
    let rhs_vec: Vec<Rat> = keep.iter().map(|&i| rhs_vec[i].clone()).collect();

    if mat.is_empty() { return 0; }

    let mut em = mat.clone();
    let mut er = rhs_vec.clone();
    let (rank, pivots) = gauss_rat(&mut em, &mut er);

    // Check inconsistent
    for i in rank..em.len() {
        if !er[i].is_zero() { return u64::MAX; }
    }

    let rows = em.len().min(rank);

    // Identify free columns
    let mut free_cols = vec![];
    {
        let mut pi = 0;
        for c in 0..n {
            if pi < pivots.len() && pivots[pi] == c { pi += 1; }
            else { free_cols.push(c); }
        }
    }
    let nullity = free_cols.len();

    if nullity == 0 {
        // Unique solution
        let mut total = 0u64;
        for &pc in &pivots {
            let x = &er[pivots.iter().position(|&p| p == pc).unwrap()];
            match x.to_int() {
                Some(v) if v >= 0 => total += v as u64,
                _ => return u64::MAX,
            }
        }
        return total;
    }

    // Scale to integer coefficients using per-row denominators
    // For each pivot row r (pivot at column pc):
    // x_pc + sum_{fc in free} em[r][fc] * x_fc = er[r]
    // => x_pc = er[r] - sum(em[r][fc] * x_fc)
    // Need x_pc >= 0 and integer for all pivots
    // Also x_fc >= 0 and integer (free vars)

    // Use safe bounds based on target values.
    // Each button press adds 1 to some counters, so the button press count
    // can't exceed the target value for any counter it affects.
    let max_target = machine.counters.iter().max().copied().unwrap_or(0) as i128;
    let max_free: Vec<i128> = free_cols.iter().map(|&fc| {
        let mut bound = max_target;
        for i in 0..nc {
            if i < machine.buttons[fc].len() && machine.buttons[fc][i] {
                bound = bound.min(machine.counters[i] as i128);
            }
        }
        // For buttons that affect no counters, use a small bound
        if bound > max_target { bound = max_target; }
        bound
    }).collect();

    // Enumeration with pruning
    let mut best = u64::MAX;
    let mut current_free = vec![0i128; nullity];

    search_free(
        0, nullity, &mut current_free, &max_free, &free_cols,
        &pivots, rows, &em, &er, 0, &mut best,
    );

    best
}

fn search_free(
    depth: usize,
    nullity: usize,
    current: &mut [i128],
    max_free: &[i128],
    free_cols: &[usize],
    pivots: &[usize],
    rows: usize,
    em: &[Vec<Rat>],
    er: &[Rat],
    current_sum: u64,
    best: &mut u64,
) {
    if current_sum >= *best { return; }

    if depth == nullity {
        // All free vars assigned, verify pivot variables
        let mut total = current_sum;
        for r in 0..rows {
            let mut val = er[r].clone();
            for fi in 0..nullity {
                let fc = free_cols[fi];
                let term = &em[r][fc];
                val = &val - &(term * &Rat::int(current[fi]));
            }
            match val.to_int() {
                Some(v) if v >= 0 => {
                    total += v as u64;
                    if total >= *best { return; }
                }
                _ => return,
            }
        }
        *best = total;
        return;
    }

    let bound = max_free[depth].min(50000);
    for v in 0..=bound {
        current[depth] = v;
        let new_sum = current_sum + v as u64;
        if new_sum >= *best { break; }

        // Pruning: check partial feasibility
        let mut feasible = true;
        for r in 0..rows {
            // With free vars up to `depth` assigned, can we still reach >= 0?
            let mut partial = er[r].clone();
            for fi in 0..=depth {
                let fc = free_cols[fi];
                let term = &em[r][fc];
                partial = &partial - &(term * &Rat::int(current[fi]));
            }
            // partial = rhs - sum_{assigned} coeff * x_fc
            // Remaining: - sum_{unassigned} coeff * x_fc
            // For worst case (largest subtraction), if coeff > 0, max subtraction = coeff * max_bound
            // If coeff < 0, increasing x_fc INCREASES partial, so min subtraction is 0 (when x_fc = 0)
            // We need: partial - max_possible_subtraction >= 0
            let mut worst = partial.clone();
            for fj in (depth + 1)..nullity {
                let fc = free_cols[fj];
                let coeff = &em[r][fc];
                if coeff.num > 0 {
                    worst = &worst - &(coeff * &Rat::int(max_free[fj]));
                }
            }
            // Also check if partial can actually become >= 0 with some assignment
            // Best case: if coeff < 0, max addition = |coeff| * max_bound
            // partial - sum_{coeff>0} coeff*0 + sum_{coeff<0} |coeff|*max_bound >= 0?
            let mut best_possible = partial.clone();
            for fj in (depth + 1)..nullity {
                let fc = free_cols[fj];
                let coeff = &em[r][fc];
                if coeff.num < 0 {
                    best_possible = &best_possible - &(coeff * &Rat::int(max_free[fj]));
                    // Note: coeff is negative, so -coeff * max = positive addition
                }
            }
            if worst.num < 0 || best_possible.num < 0 {
                // worst.num < 0 means even with all positive-coeff vars at max,
                // the pivot is negative. Infeasible.
                // best_possible.num < 0 means even with all negative-coeff vars
                // at max (which helps), the pivot is still negative. Also infeasible.
                // Actually we need the final value to be >= 0.
                // worst is the minimum possible value.
                // If worst < 0 AND best_possible >= 0, it might still be feasible.
                // If best_possible < 0, definitely infeasible.
                if best_possible.num < 0 {
                    feasible = false;
                    break;
                }
                // If worst < 0 but best_possible >= 0, we're not sure. Don't prune.
            }
            // Check integrality: can partial - sum(coeff * x_fc) be integer?
            // This requires solving congruences, which is complex for partial assignment.
            // Skip integrality pruning for now.
        }

        if feasible {
            search_free(
                depth + 1, nullity, current, max_free, free_cols,
                pivots, rows, em, er, new_sum, best,
            );
        }
    }
}

// ============================================================
// Main
// ============================================================

pub fn solve() {
    let input = fs::read_to_string("puzzles/2025-10-input.txt")
        .expect("Failed to read input");
    let machines = parse_machines(&input);

    let part1: u64 = machines.iter().map(|m| solve_part1(m)).sum();
    println!("Day 10 Part 1: {part1}");

    let part2: u64 = machines.iter().map(|m| solve_part2(m)).sum();
    println!("Day 10 Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machines = parse_machines(input);
        let p1: Vec<u64> = machines.iter().map(|m| solve_part1(m)).collect();
        assert_eq!(p1, vec![2, 3, 2]);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machines = parse_machines(input);
        let p2: Vec<u64> = machines.iter().map(|m| solve_part2(m)).collect();
        assert_eq!(p2, vec![10, 12, 11]);
    }
}
