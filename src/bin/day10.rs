use std::env;
use std::fs;

// ---- exact rational arithmetic ----

#[derive(Clone, Copy, Debug)]
struct Frac {
    n: i128,
    d: i128, // always > 0, fraction kept reduced
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

impl Frac {
    fn new(n: i128, d: i128) -> Frac {
        assert!(d != 0);
        let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
        let g = gcd(n, d).max(1);
        Frac { n: n / g, d: d / g }
    }
    fn int(n: i128) -> Frac {
        Frac { n, d: 1 }
    }
    fn zero() -> Frac {
        Frac::int(0)
    }
    fn is_zero(&self) -> bool {
        self.n == 0
    }
    fn is_positive(&self) -> bool {
        self.n > 0
    }
    fn is_negative(&self) -> bool {
        self.n < 0
    }
    fn add(self, o: Frac) -> Frac {
        Frac::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    fn sub(self, o: Frac) -> Frac {
        Frac::new(self.n * o.d - o.n * self.d, self.d * o.d)
    }
    fn mul(self, o: Frac) -> Frac {
        Frac::new(self.n * o.n, self.d * o.d)
    }
    fn div(self, o: Frac) -> Frac {
        assert!(!o.is_zero());
        Frac::new(self.n * o.d, self.d * o.n)
    }
    fn ge(&self, o: &Frac) -> bool {
        self.n * o.d >= o.n * self.d
    }
    /// floor(self / o); requires o > 0. Rounds toward negative infinity.
    fn floor_div(&self, o: &Frac) -> i128 {
        assert!(o.is_positive());
        (self.n * o.d).div_euclid(self.d * o.n)
    }
    fn as_int(&self) -> Option<i128> {
        if self.n % self.d == 0 {
            Some(self.n / self.d)
        } else {
            None
        }
    }
}

// ---- parsing ----

pub struct Machine {
    pub target_mask: u64,
    pub buttons: Vec<Vec<usize>>,
    pub joltage: Vec<i128>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut target_mask = 0u64;
            let mut buttons = Vec::new();
            let mut joltage = Vec::new();
            for tok in line.split_whitespace() {
                match tok.chars().next().unwrap() {
                    '[' => {
                        for (i, ch) in tok[1..tok.len() - 1].chars().enumerate() {
                            if ch == '#' {
                                target_mask |= 1 << i;
                            }
                        }
                    }
                    '(' => {
                        let btn = tok[1..tok.len() - 1]
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect();
                        buttons.push(btn);
                    }
                    '{' => {
                        joltage = tok[1..tok.len() - 1]
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i128>().unwrap())
                            .collect();
                    }
                    _ => panic!("unexpected token: {}", tok),
                }
            }
            Machine {
                target_mask,
                buttons,
                joltage,
            }
        })
        .collect()
}

// ---- part 1: toggle lights, GF(2); fewest presses = lightest subset ----

fn solve_part1(m: &Machine) -> u64 {
    let n = m.buttons.len();
    let masks: Vec<u64> = m
        .buttons
        .iter()
        .map(|b| b.iter().fold(0u64, |acc, &c| acc | (1 << c)))
        .collect();
    let mut best = u64::MAX;
    for subset in 0u64..(1 << n) {
        let mut state = 0u64;
        for (i, &bm) in masks.iter().enumerate() {
            if subset & (1 << i) != 0 {
                state ^= bm;
            }
        }
        if state == m.target_mask {
            best = best.min(subset.count_ones() as u64);
        }
    }
    assert!(best != u64::MAX, "no solution for part 1");
    best
}

// ---- part 2: minimize sum of x subject to A x = b, x >= 0 integer ----
//
// RREF over the rationals expresses pivot variables through the free ones:
//   x_pivot[i] = rem[i] - sum_f a[i][f] * x_f ,  x_f >= 0
// total presses = base + sum_f coef[f] * x_f .
// Every variable is bounded by the smallest joltage of the counters it
// feeds, so the free variables are enumerated with pruning.

pub fn solve_part2(m: &Machine) -> Option<i128> {
    let n_rows = m.joltage.len();
    let n_cols = m.buttons.len();

    let mut mat = vec![vec![Frac::zero(); n_cols + 1]; n_rows];
    for (j, btn) in m.buttons.iter().enumerate() {
        for &c in btn {
            mat[c][j] = Frac::int(1);
        }
    }
    for (i, &t) in m.joltage.iter().enumerate() {
        mat[i][n_cols] = Frac::int(t);
    }

    // Gauss-Jordan to reduced row echelon form.
    let mut pivot_cols: Vec<usize> = Vec::new();
    let mut row = 0;
    for col in 0..n_cols {
        if row >= n_rows {
            break;
        }
        let Some(sel) = (row..n_rows).find(|&r| !mat[r][col].is_zero()) else {
            continue;
        };
        mat.swap(row, sel);
        let pv = mat[row][col];
        for c in 0..=n_cols {
            mat[row][c] = mat[row][c].div(pv);
        }
        for r in 0..n_rows {
            if r != row && !mat[r][col].is_zero() {
                let factor = mat[r][col];
                for c in 0..=n_cols {
                    mat[r][c] = mat[r][c].sub(factor.mul(mat[row][c]));
                }
            }
        }
        pivot_cols.push(col);
        row += 1;
    }
    for r in 0..n_rows {
        let inconsistent =
            (0..n_cols).all(|c| mat[r][c].is_zero()) && !mat[r][n_cols].is_zero();
        if inconsistent {
            return None; // machine has no solution
        }
    }

    let np = pivot_cols.len();
    let free: Vec<usize> = (0..n_cols).filter(|c| !pivot_cols.contains(c)).collect();
    let nf = free.len();

    let base: Frac = (0..np).fold(Frac::zero(), |acc, i| acc.add(mat[i][n_cols]));
    let coef: Vec<Frac> = free
        .iter()
        .map(|&f| Frac::int(1).sub((0..np).fold(Frac::zero(), |acc, i| acc.add(mat[i][f]))))
        .collect();

    // Hard per-variable bound: a button press is limited by the smallest
    // joltage among the counters it feeds.
    let static_hi: Vec<i128> = free
        .iter()
        .map(|&f| m.buttons[f].iter().map(|&c| m.joltage[c]).min().unwrap())
        .collect();

    // later_neg[k][i]: some free variable after position k has a negative
    // coefficient in pivot row i. While it exists, residual row i is allowed
    // to drop below zero (the later variable can lift it back), so row i
    // gives no upper bound at position k.
    let mut later_neg = vec![vec![false; np]; nf + 1];
    for k in (0..nf).rev() {
        for i in 0..np {
            later_neg[k][i] = later_neg[k + 1][i] || mat[i][free[k]].is_negative();
        }
    }

    // Suffix sums of the most optimistic (negative) cost contributions,
    // used for branch-and-bound pruning.
    let mut suffix_opt = vec![Frac::zero(); nf + 1];
    for k in (0..nf).rev() {
        suffix_opt[k] = suffix_opt[k + 1];
        if coef[k].is_negative() {
            suffix_opt[k] = suffix_opt[k].add(coef[k].mul(Frac::int(static_hi[k])));
        }
    }

    let mut best: Option<Frac> = None;
    let mut rem: Vec<Frac> = (0..np).map(|i| mat[i][n_cols]).collect();
    let mut xs = vec![0i128; nf];
    let mut best_xs = vec![0i128; nf];

    fn dfs(
        k: usize,
        cost: Frac,
        free: &Vec<usize>,
        mat: &Vec<Vec<Frac>>,
        np: usize,
        base: &Frac,
        coef: &Vec<Frac>,
        static_hi: &Vec<i128>,
        later_neg: &Vec<Vec<bool>>,
        suffix_opt: &Vec<Frac>,
        rem: &mut Vec<Frac>,
        xs: &mut Vec<i128>,
        best: &mut Option<Frac>,
        best_xs: &mut Vec<i128>,
    ) {
        if let Some(b) = best {
            let optimistic = base.add(cost).add(suffix_opt[k]);
            if optimistic.ge(&b) {
                return;
            }
        }
        if k == free.len() {
            for i in 0..np {
                match rem[i].as_int() {
                    Some(v) if v >= 0 => {}
                    _ => return, // pivot variable not a non-negative integer
                }
            }
            let total = base.add(cost);
            if best.map_or(true, |b| !total.ge(&b)) {
                *best = Some(total);
                best_xs.clone_from_slice(xs);
            }
            return;
        }
        let f = free[k];
        let mut hi = static_hi[k];
        for i in 0..np {
            let a = mat[i][f];
            if a.is_positive() && !later_neg[k + 1][i] {
                hi = hi.min(rem[i].floor_div(&a));
            }
        }
        if hi < 0 {
            return;
        }
        // Try values in the order that finds a good incumbent early.
        let descending = coef[k].is_negative();
        for x in 0..=hi {
            let x = if descending { hi - x } else { x };
            let xf = Frac::int(x);
            for i in 0..np {
                rem[i] = rem[i].sub(mat[i][f].mul(xf));
            }
            xs[k] = x;
            dfs(
                k + 1,
                cost.add(coef[k].mul(xf)),
                free, mat, np, base, coef, static_hi, later_neg, suffix_opt, rem, xs, best,
                best_xs,
            );
            for i in 0..np {
                rem[i] = rem[i].add(mat[i][f].mul(xf));
            }
            // Stop once even the most optimistic completion (including
            // negative-cost later variables) can no longer beat the
            // incumbent; with a non-negative coefficient the bound only
            // grows as x increases.
            if !coef[k].is_negative() {
                if let Some(b) = best {
                    let lb = base
                        .add(cost.add(coef[k].mul(xf)))
                        .add(suffix_opt[k + 1]);
                    if lb.ge(&b) {
                        break;
                    }
                }
            }
        }
    }

    dfs(
        0,
        Frac::zero(),
        &free,
        &mat,
        np,
        &base,
        &coef,
        &static_hi,
        &later_neg,
        &suffix_opt,
        &mut rem,
        &mut xs,
        &mut best,
        &mut best_xs,
    );

    let best = best?.as_int().expect("non-integer optimum");

    // Self-check: rebuild the winning press counts and verify they satisfy
    // the original equations exactly (rules out infeasible "solutions").
    let mut x = vec![0i128; n_cols];
    for (k, &f) in free.iter().enumerate() {
        x[f] = best_xs[k];
    }
    for i in 0..np {
        let mut v = mat[i][n_cols];
        for (k, &f) in free.iter().enumerate() {
            v = v.sub(mat[i][f].mul(Frac::int(best_xs[k])));
        }
        x[pivot_cols[i]] = v.as_int().expect("non-integer pivot value");
    }
    for c in 0..n_rows {
        let s: i128 = (0..n_cols)
            .filter(|&j| m.buttons[j].contains(&c))
            .map(|j| x[j])
            .sum();
        assert_eq!(s, m.joltage[c], "solution violates counter {}", c);
    }
    assert!(x.iter().all(|&v| v >= 0));
    assert_eq!(x.iter().sum::<i128>(), best, "solution cost mismatch");

    Some(best)
}

fn main() {
    let path = env::args().nth(1).expect("usage: day10 <input-file>");
    let content = fs::read_to_string(&path).expect("failed to read input file");
    let machines = parse(&content);

    let part1: u64 = machines.iter().map(solve_part1).sum();
    let part2: i128 = machines
        .iter()
        .map(|m| solve_part2(m).expect("machine has no solution"))
        .sum();

    if env::var("VERBOSE").is_ok() {
        for (i, m) in machines.iter().enumerate() {
            println!(
                "machine {}: {} {}",
                i,
                solve_part1(m),
                solve_part2(m).unwrap()
            );
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
