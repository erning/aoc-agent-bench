//! Two-phase primal simplex over exact rationals, plus an integer
//! programming branch-and-bound driver for `min 1^T x  s.t.  A x = b, x >= 0`.

use crate::rational::Rat;

/// min c.x s.t. a.x = b, x >= 0.
/// `a` has one row per constraint, each of length n (the structural variables).
/// Returns `(optimal_value, x)` with `x.len() == n`, or `None` if infeasible.
pub fn simplex_min(a: &[Vec<i64>], b: &[i64], c: &[i64]) -> Option<(Rat, Vec<Rat>)> {
    let rows = a.len();
    let n = c.len();
    let total = n + rows; // structural + artificial columns

    // Build the tableau with one artificial per row.
    let mut t: Vec<Vec<Rat>> = Vec::with_capacity(rows);
    let mut rhs: Vec<Rat> = Vec::with_capacity(rows);
    let mut basis: Vec<usize> = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut ai: Vec<i64> = a[i].clone();
        let mut bi = b[i];
        if bi < 0 {
            bi = -bi;
            for v in ai.iter_mut() {
                *v = -*v;
            }
        }
        let mut row: Vec<Rat> = Vec::with_capacity(total);
        for &v in ai.iter() {
            row.push(Rat::from_i(v));
        }
        for _ in 0..rows {
            row.push(Rat::zero());
        }
        row[n + i] = Rat::one(); // artificial for this row
        t.push(row);
        rhs.push(Rat::from_i(bi));
        basis.push(n + i);
    }

    // Phase 1: minimize sum of artificials (any column may enter).
    let cost1: Vec<Rat> = (0..total)
        .map(|j| if j >= n { Rat::one() } else { Rat::zero() })
        .collect();
    run_simplex(&mut t, &mut rhs, &mut basis, &cost1, rows, total, total);

    // If any artificial remains basic with a positive value -> infeasible.
    for i in 0..rows {
        if basis[i] >= n && !rhs[i].is_zero() {
            return None;
        }
    }

    // Phase 2: minimize the real cost; only structural columns may enter.
    let cost2: Vec<Rat> = (0..total)
        .map(|j| if j < n { Rat::from_i(c[j]) } else { Rat::zero() })
        .collect();
    run_simplex(&mut t, &mut rhs, &mut basis, &cost2, rows, total, n);

    let val = (0..rows)
        .map(|i| cost2[basis[i]].mul(rhs[i]))
        .fold(Rat::zero(), |acc, v| acc.add(v));

    let mut x = vec![Rat::zero(); n];
    for i in 0..rows {
        if basis[i] < n {
            x[basis[i]] = x[basis[i]].add(rhs[i]);
        }
    }
    Some((val, x))
}

fn run_simplex(
    t: &mut Vec<Vec<Rat>>,
    rhs: &mut Vec<Rat>,
    basis: &mut Vec<usize>,
    cost: &[Rat],
    rows: usize,
    total: usize,
    enter_end: usize,
) {
    for _ in 0..10000 {
        // Reduced costs: cost[j] - sum_i cost[basis[i]] * t[i][j].
        let cb: Vec<Rat> = (0..rows).map(|i| cost[basis[i]]).collect();
        let mut entering: Option<usize> = None;
        let mut best_rc = Rat::zero();
        for j in 0..enter_end {
            let mut zj = Rat::zero();
            for i in 0..rows {
                if !t[i][j].is_zero() {
                    zj = zj.add(cb[i].mul(t[i][j]));
                }
            }
            let rc = cost[j].sub(zj);
            if rc.is_negative() && (entering.is_none() || rc < best_rc) {
                best_rc = rc;
                entering = Some(j);
            }
        }
        match entering {
            None => return,
            Some(col) => {
                if !pivot(t, rhs, basis, col, rows, total) {
                    return; // unbounded
                }
            }
        }
    }
}

fn pivot(
    t: &mut Vec<Vec<Rat>>,
    rhs: &mut Vec<Rat>,
    basis: &mut Vec<usize>,
    col: usize,
    rows: usize,
    total: usize,
) -> bool {
    let mut leave: Option<usize> = None;
    let mut best_ratio: Option<Rat> = None;
    for i in 0..rows {
        if t[i][col].is_positive() {
            let r = rhs[i].div(t[i][col]);
            match best_ratio {
                None => {
                    best_ratio = Some(r);
                    leave = Some(i);
                }
                Some(br) => {
                    if r < br || (r == br && basis[i] < basis[leave.unwrap()]) {
                        best_ratio = Some(r);
                        leave = Some(i);
                    }
                }
            }
        }
    }
    let bi = match leave {
        None => return false, // unbounded
        Some(x) => x,
    };
    let piv = t[bi][col];
    for j in 0..total {
        t[bi][j] = t[bi][j].div(piv);
    }
    rhs[bi] = rhs[bi].div(piv);
    for i in 0..rows {
        if i != bi && !t[i][col].is_zero() {
            let f = t[i][col];
            for j in 0..total {
                t[i][j] = t[i][j].sub(f.mul(t[bi][j]));
            }
            rhs[i] = rhs[i].sub(f.mul(rhs[bi]));
        }
    }
    basis[bi] = col;
    true
}

/// Bound constraint used during branch-and-bound.
#[derive(Clone, Copy)]
enum Bnd {
    Le(i64),
    Ge(i64),
}

/// Solve `min sum x` s.t. `a.x = b`, `x >= 0` integer, where `a` is c x m of 0/1.
/// Returns the minimum, or `None` if infeasible.
pub fn ilp_min_sum(a: &[Vec<i64>], b: &[i64]) -> Option<i64> {
    let m = if a.is_empty() { 0 } else { a[0].len() };

    fn solve(
        aeq: &[Vec<i64>],
        beq: &[i64],
        m: usize,
        bounds: &Vec<(usize, Bnd)>,
        best: &mut Option<i64>,
    ) {
        // Build the LP with a slack column for each active bound constraint.
        let nslack = bounds.len();
        let n = m + nslack;
        let mut rows: Vec<Vec<i64>> = Vec::with_capacity(aeq.len() + nslack);
        let mut rhs: Vec<i64> = Vec::with_capacity(aeq.len() + nslack);
        for (r, v) in aeq.iter().zip(beq.iter()) {
            let mut row = r.clone();
            row.resize(n, 0);
            rows.push(row);
            rhs.push(*v);
        }
        for (si, (var, bnd)) in bounds.iter().enumerate() {
            let mut row = vec![0i64; n];
            match *bnd {
                Bnd::Le(val) => {
                    row[*var] = 1;
                    row[m + si] = 1; // x + s = val
                    rhs.push(val);
                }
                Bnd::Ge(val) => {
                    row[*var] = 1;
                    row[m + si] = -1; // x - s = val
                    rhs.push(val);
                }
            }
            rows.push(row);
        }
        let mut cost = vec![1i64; m];
        cost.resize(n, 0);

        let (val, x) = match simplex_min(&rows, &rhs, &cost) {
            None => return, // infeasible node
            Some(v) => v,
        };

        // Lower bound: at least ceil(LP optimum).
        let lb = val.ceil();
        if let Some(bv) = *best {
            if lb >= bv as i128 {
                return;
            }
        }

        let all_int = (0..m).all(|j| x[j].is_integer());
        if all_int {
            let iv = val.floor() as i64;
            match *best {
                None => *best = Some(iv),
                Some(bv) if iv < bv => *best = Some(iv),
                _ => {}
            }
            return;
        }

        // Branch on the most fractional variable.
        let mut frac_best = Rat::zero();
        let mut chosen: Option<usize> = None;
        for j in 0..m {
            let f = x[j].frac();
            let comp = Rat::from_i(1).sub(f);
            let fr = if f < comp { f } else { comp };
            if chosen.is_none() || fr > frac_best {
                frac_best = fr;
                chosen = Some(j);
            }
        }
        let j = chosen.unwrap();
        let floor_v = x[j].floor() as i64;

        let mut left = bounds.clone();
        left.push((j, Bnd::Le(floor_v)));
        solve(aeq, beq, m, &left, best);

        let mut right = bounds.clone();
        right.push((j, Bnd::Ge(floor_v + 1)));
        solve(aeq, beq, m, &right, best);
    }

    let mut best: Option<i64> = None;
    solve(a, b, m, &Vec::new(), &mut best);
    best
}
