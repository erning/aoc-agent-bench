use std::fs;

fn main() {
    let day6_input = fs::read_to_string("puzzles/2025-06-input.txt").unwrap();
    let (d6p1, d6p2) = day6::solve(&day6_input);
    println!("Day 6 Part 1: {}", d6p1);
    println!("Day 6 Part 2: {}", d6p2);

    let day10_input = fs::read_to_string("puzzles/2025-10-input.txt").unwrap();
    let (d10p1, d10p2) = day10::solve(&day10_input);
    println!("Day 10 Part 1: {}", d10p1);
    println!("Day 10 Part 2: {}", d10p2);
}

mod day6 {
    pub fn solve(input: &str) -> (i64, i64) {
        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();
        let cols = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        let mut grid: Vec<Vec<char>> = vec![vec![' '; cols]; rows];
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                grid[r][c] = ch;
            }
        }

        let is_sep = |c: usize| -> bool {
            (0..rows).all(|r| grid[r][c].is_whitespace())
        };

        let mut separators = vec![];
        for c in 0..cols {
            if is_sep(c) {
                separators.push(c);
            }
        }

        let mut problems: Vec<(usize, usize)> = vec![];
        let mut start = 0usize;
        for &sep in &separators {
            if start < sep {
                problems.push((start, sep));
            }
            start = sep + 1;
        }
        if start < cols {
            problems.push((start, cols));
        }

        let ops: Vec<char> = problems
            .iter()
            .map(|&(s, e)| {
                let mut op = ' ';
                for r in 0..rows {
                    for c in s..e {
                        let ch = grid[r][c];
                        if ch == '+' || ch == '*' {
                            op = ch;
                        }
                    }
                }
                op
            })
            .collect();

        let mut part1 = 0i64;
        for (idx, &(s, e)) in problems.iter().enumerate() {
            let op = ops[idx];
            let mut nums = vec![];
            for r in 0..rows {
                let mut sbuf = String::new();
                for c in s..e {
                    let ch = grid[r][c];
                    if ch.is_ascii_digit() {
                        sbuf.push(ch);
                    }
                }
                if !sbuf.is_empty() {
                    nums.push(sbuf.parse::<i64>().unwrap());
                }
            }
            let ans = nums.into_iter().reduce(|a, b| if op == '+' { a + b } else { a * b }).unwrap_or(0);
            part1 += ans;
        }

        let mut part2 = 0i64;
        for (idx, &(s, e)) in problems.iter().enumerate() {
            let op = ops[idx];
            let mut nums = vec![];
            for c in (s..e).rev() {
                let mut sbuf = String::new();
                for r in 0..rows {
                    let ch = grid[r][c];
                    if ch.is_ascii_digit() {
                        sbuf.push(ch);
                    }
                }
                if !sbuf.is_empty() {
                    nums.push(sbuf.parse::<i64>().unwrap());
                }
            }
            let ans = nums.into_iter().reduce(|a, b| if op == '+' { a + b } else { a * b }).unwrap_or(0);
            part2 += ans;
        }

        (part1, part2)
    }
}

mod day10 {
    use num_rational::Rational64;

    #[derive(Debug, Clone)]
    struct Machine {
        pattern: Vec<bool>,
        buttons: Vec<Vec<usize>>,
        targets: Vec<i64>,
    }

    fn parse(input: &str) -> Vec<Machine> {
        let mut machines = vec![];
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let bracket_end = line.find(']').unwrap();
            let pattern_str = &line[1..bracket_end];
            let pattern: Vec<bool> = pattern_str.chars().map(|c| c == '#').collect();

            let rest = &line[bracket_end + 1..];
            let curly_start = rest.find('{').unwrap();
            let btns_part = &rest[..curly_start];
            let targets_part = &rest[curly_start..];

            let mut buttons = vec![];
            for btn in btns_part.split('(') {
                let btn = btn.trim();
                if btn.is_empty() {
                    continue;
                }
                let btn = btn.trim_end_matches(')');
                let mut idxs = vec![];
                for num in btn.split(',') {
                    let num = num.trim();
                    if !num.is_empty() {
                        idxs.push(num.parse::<usize>().unwrap());
                    }
                }
                buttons.push(idxs);
            }

            let targets_str = &targets_part[1..targets_part.len() - 1];
            let mut targets = vec![];
            for num in targets_str.split(',') {
                let num = num.trim();
                if !num.is_empty() {
                    targets.push(num.parse::<i64>().unwrap());
                }
            }

            machines.push(Machine {
                pattern,
                buttons,
                targets,
            });
        }
        machines
    }

    pub fn solve(input: &str) -> (usize, usize) {
        let machines = parse(input);
        let mut part1 = 0usize;
        let mut part2 = 0usize;

        for m in &machines {
            part1 += solve_part1(m);
            part2 += solve_part2(m);
        }

        (part1, part2)
    }

    fn solve_part1(m: &Machine) -> usize {
        let n = m.buttons.len();
        let d = m.pattern.len();
        if n == 0 {
            return 0;
        }

        let mut aug = vec![vec![0u8; n + 1]; d];
        for (i, btn) in m.buttons.iter().enumerate() {
            for &light in btn {
                if light < d {
                    aug[light][i] = 1;
                }
            }
        }
        for i in 0..d {
            aug[i][n] = if m.pattern[i] { 1 } else { 0 };
        }

        let mut pivot_col = vec![None; d];
        let mut row = 0usize;
        for col in 0..n {
            let mut sel = None;
            for r in row..d {
                if aug[r][col] == 1 {
                    sel = Some(r);
                    break;
                }
            }
            if let Some(sel_r) = sel {
                aug.swap(row, sel_r);
                pivot_col[row] = Some(col);
                for r in 0..d {
                    if r != row && aug[r][col] == 1 {
                        for c in 0..=n {
                            aug[r][c] ^= aug[row][c];
                        }
                    }
                }
                row += 1;
            }
        }

        for r in row..d {
            let all_zero = (0..n).all(|c| aug[r][c] == 0);
            if all_zero && aug[r][n] == 1 {
                return usize::MAX;
            }
        }

        let mut free_vars = vec![];
        let mut is_pivot = vec![false; n];
        for r in 0..row {
            if let Some(pc) = pivot_col[r] {
                is_pivot[pc] = true;
            }
        }
        for i in 0..n {
            if !is_pivot[i] {
                free_vars.push(i);
            }
        }

        let k = free_vars.len();
        let mut best = usize::MAX;
        for mask in 0usize..(1usize << k) {
            let mut x = vec![0u8; n];
            for (j, &fv) in free_vars.iter().enumerate() {
                x[fv] = ((mask >> j) & 1) as u8;
            }
            for r in (0..row).rev() {
                if let Some(pc) = pivot_col[r] {
                    let mut val = aug[r][n];
                    for c in (pc + 1)..n {
                        if aug[r][c] == 1 {
                            val ^= x[c];
                        }
                    }
                    x[pc] = val;
                }
            }
            let presses = x.iter().map(|&v| v as usize).sum();
            best = best.min(presses);
        }
        best
    }

    fn solve_part2(m: &Machine) -> usize {
        let n = m.buttons.len();
        let d = m.targets.len();
        if n == 0 {
            return 0;
        }

        let mut a = vec![vec![0i64; n]; d];
        for (j, btn) in m.buttons.iter().enumerate() {
            for &light in btn {
                if light < d {
                    a[light][j] = 1;
                }
            }
        }
        let b = m.targets.clone();

        let (r, a_rows, b_rows) = get_independent_rows(&a, &b);

        if r == 0 {
            return 0;
        }

        let mut best_lp: Option<Rational64> = None;
        for basis in combinations(n, r) {
            let mut mat = vec![vec![Rational64::new(0, 1); r]; r];
            for i in 0..r {
                for j in 0..r {
                    mat[i][j] = Rational64::new(a_rows[i][basis[j]], 1);
                }
            }
            let rhs: Vec<Rational64> = b_rows.iter().map(|&v| Rational64::new(v, 1)).collect();
            if let Some(sol) = solve_linear_exact(&mat, &rhs) {
                if sol.iter().all(|v| *v >= Rational64::new(0, 1)) {
                    let s = sol.iter().copied().fold(Rational64::new(0, 1), |a, b| a + b);
                    if best_lp.is_none() || s < best_lp.unwrap() {
                        best_lp = Some(s);
                    }
                }
            }
        }

        let lp_val = best_lp.expect("No feasible LP solution found");
        let numer = *lp_val.numer();
        let denom = *lp_val.denom();
        let s_start = (numer + denom - 1) / denom;

        for s in s_start..=s_start + 200 {
            let mut a_ext = a_rows.clone();
            let mut b_ext = b_rows.clone();
            a_ext.push(vec![1i64; n]);
            b_ext.push(s);
            let (r2, a2, b2) = get_independent_rows(&a_ext, &b_ext);

            for basis in combinations(n, r2) {
                let mut mat = vec![vec![Rational64::new(0, 1); r2]; r2];
                for i in 0..r2 {
                    for j in 0..r2 {
                        mat[i][j] = Rational64::new(a2[i][basis[j]], 1);
                    }
                }
                let rhs: Vec<Rational64> = b2.iter().map(|&v| Rational64::new(v, 1)).collect();
                if let Some(sol) = solve_linear_exact(&mat, &rhs) {
                    if sol.iter().all(|v| *v >= Rational64::new(0, 1) && *v.denom() == 1) {
                        return s as usize;
                    }
                }
            }
        }

        panic!("Could not find integer solution for machine");
    }

    fn get_independent_rows(a: &[Vec<i64>], b: &[i64]) -> (usize, Vec<Vec<i64>>, Vec<i64>) {
        let m = a.len();
        let n = if m > 0 { a[0].len() } else { 0 };

        let mut mat: Vec<Vec<Rational64>> = a
            .iter()
            .map(|row| row.iter().map(|&v| Rational64::new(v, 1)).collect())
            .collect();
        let mut rhs: Vec<Rational64> = b.iter().map(|&v| Rational64::new(v, 1)).collect();
        let mut indices: Vec<usize> = (0..m).collect();

        let mut row = 0usize;
        for col in 0..n {
            let mut sel = None;
            for r in row..m {
                if mat[r][col] != Rational64::new(0, 1) {
                    sel = Some(r);
                    break;
                }
            }
            if let Some(sel_r) = sel {
                mat.swap(row, sel_r);
                rhs.swap(row, sel_r);
                indices.swap(row, sel_r);
                let piv = mat[row][col];
                for c in col..n {
                    mat[row][c] = mat[row][c] / piv;
                }
                rhs[row] = rhs[row] / piv;
                for r in (row + 1)..m {
                    if mat[r][col] != Rational64::new(0, 1) {
                        let factor = mat[r][col];
                        for c in col..n {
                            mat[r][c] = mat[r][c] - factor * mat[row][c];
                        }
                        rhs[r] = rhs[r] - factor * rhs[row];
                    }
                }
                row += 1;
            }
        }

        for r in row..m {
            let all_zero = mat[r].iter().all(|v| *v == Rational64::new(0, 1));
            if all_zero && rhs[r] != Rational64::new(0, 1) {
                panic!("Inconsistent system");
            }
        }

        let a_rows: Vec<Vec<i64>> = indices[..row].iter().map(|&idx| a[idx].clone()).collect();
        let b_rows: Vec<i64> = indices[..row].iter().map(|&idx| b[idx]).collect();

        (row, a_rows, b_rows)
    }

    fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
        if k == 0 {
            return vec![vec![]];
        }
        if k > n {
            return vec![];
        }
        let mut res = vec![];
        let mut cur = vec![0; k];
        fn backtrack(start: usize, pos: usize, n: usize, k: usize, cur: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
            if pos == k {
                res.push(cur.clone());
                return;
            }
            for i in start..=n - (k - pos) {
                cur[pos] = i;
                backtrack(i + 1, pos + 1, n, k, cur, res);
            }
        }
        backtrack(0, 0, n, k, &mut cur, &mut res);
        res
    }

    fn solve_linear_exact(a: &[Vec<Rational64>], b: &[Rational64]) -> Option<Vec<Rational64>> {
        let n = a.len();
        if n == 0 {
            return Some(vec![]);
        }
        let mut m = vec![vec![Rational64::new(0, 1); n + 1]; n];
        for i in 0..n {
            for j in 0..n {
                m[i][j] = a[i][j];
            }
            m[i][n] = b[i];
        }

        for col in 0..n {
            let mut pivot = None;
            for row in col..n {
                if m[row][col] != Rational64::new(0, 1) {
                    pivot = Some(row);
                    break;
                }
            }
            let p = pivot?;
            m.swap(col, p);
            let piv = m[col][col];
            for j in col..=n {
                m[col][j] = m[col][j] / piv;
            }
            for row in 0..n {
                if row != col && m[row][col] != Rational64::new(0, 1) {
                    let factor = m[row][col];
                    for j in col..=n {
                        m[row][j] = m[row][j] - factor * m[col][j];
                    }
                }
            }
        }

        let mut sol = vec![Rational64::new(0, 1); n];
        for i in 0..n {
            sol[i] = m[i][n];
        }
        Some(sol)
    }
}
