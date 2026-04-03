use std::collections::HashMap;
use std::fs;

fn main() {
    println!("=== Day 6: Trash Compactor ===\n");
    solve_day6();
    
    println!("\n=== Day 10: Factory ===\n");
    solve_day10();
}

// ========== Day 6 ==========

fn solve_day6() {
    let input = fs::read_to_string("puzzles/2025-06-input.txt").expect("Failed to read day 6 input");
    let lines: Vec<&str> = input.lines().collect();
    
    let part1 = solve_day6_part1(&lines);
    println!("Day 6 Part 1: {}", part1);
    
    let part2 = solve_day6_part2(&lines);
    println!("Day 6 Part 2: {}", part2);
}

fn solve_day6_part1(lines: &[&str]) -> i64 {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    
    let mut separators = vec![];
    for col in 0..num_cols {
        let all_space = (0..num_rows).all(|row| grid[row][col] == ' ');
        if all_space {
            separators.push(col);
        }
    }
    
    let mut groups = vec![];
    let mut start = 0;
    for &sep in &separators {
        if start < sep {
            groups.push((start, sep));
        }
        start = sep + 1;
    }
    if start < num_cols {
        groups.push((start, num_cols));
    }
    
    let mut total = 0i64;
    
    for (g_start, g_end) in groups {
        let mut op = ' ';
        for row in 0..num_rows {
            if grid[row][g_start..g_end].iter().any(|&c| c == '*' || c == '+') {
                op = grid[row][g_start..g_end].iter().find(|&&c| c == '*' || c == '+').copied().unwrap();
            }
        }
        
        let mut numbers = vec![];
        for row in 0..num_rows - 1 {
            let s: String = grid[row][g_start..g_end].iter().collect();
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                if let Ok(n) = trimmed.parse::<i64>() {
                    numbers.push(n);
                }
            }
        }
        
        let mut result = numbers[0];
        for &n in &numbers[1..] {
            match op {
                '*' => result *= n,
                '+' => result += n,
                _ => {}
            }
        }
        total += result;
    }
    
    total
}

fn solve_day6_part2(lines: &[&str]) -> i64 {
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    
    let mut separators = vec![];
    for col in 0..num_cols {
        let all_space = (0..num_rows).all(|row| grid[row][col] == ' ');
        if all_space {
            separators.push(col);
        }
    }
    
    let mut groups = vec![];
    let mut start = 0;
    for &sep in &separators {
        if start < sep {
            groups.push((start, sep));
        }
        start = sep + 1;
    }
    if start < num_cols {
        groups.push((start, num_cols));
    }
    
    let mut total = 0i64;
    
    for (g_start, g_end) in groups {
        let mut op = ' ';
        for row in 0..num_rows {
            if grid[row][g_start..g_end].iter().any(|&c| c == '*' || c == '+') {
                op = grid[row][g_start..g_end].iter().find(|&&c| c == '*' || c == '+').copied().unwrap();
            }
        }
        
        let mut numbers = vec![];
        for col in (g_start..g_end).rev() {
            let mut digits = vec![];
            for row in 0..num_rows - 1 {
                let c = grid[row][col];
                if c.is_ascii_digit() {
                    digits.push(c);
                }
            }
            if !digits.is_empty() {
                let s: String = digits.iter().collect();
                if let Ok(n) = s.parse::<i64>() {
                    numbers.push(n);
                }
            }
        }
        
        let mut result = numbers[0];
        for &n in &numbers[1..] {
            match op {
                '*' => result *= n,
                '+' => result += n,
                _ => {}
            }
        }
        total += result;
    }
    
    total
}

// ========== Day 10 ==========

#[derive(Debug, Clone)]
struct Machine {
    indicator: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i64>,
}

fn parse_day10_input(input: &str) -> Vec<Machine> {
    let mut machines = vec![];
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let indicator_start = line.find('[').unwrap();
        let indicator_end = line.find(']').unwrap();
        let indicator_str = &line[indicator_start + 1..indicator_end];
        let indicator: Vec<bool> = indicator_str.chars().map(|c| c == '#').collect();
        
        let mut buttons = vec![];
        let mut i = indicator_end + 1;
        while let Some(start) = line[i..].find('(') {
            let start = i + start;
            let end = line[start..].find(')').unwrap() + start;
            let btn_str = &line[start + 1..end];
            let btn: Vec<usize> = if btn_str.is_empty() {
                vec![]
            } else {
                btn_str.split(',').map(|s| s.parse().unwrap()).collect()
            };
            buttons.push(btn);
            i = end + 1;
        }
        
        let jolt_start = line.find('{').unwrap();
        let jolt_end = line.find('}').unwrap();
        let jolt_str = &line[jolt_start + 1..jolt_end];
        let joltages: Vec<i64> = jolt_str.split(',').map(|s| s.parse().unwrap()).collect();
        
        machines.push(Machine { indicator, buttons, joltages });
    }
    
    machines
}

fn solve_day10() {
    let input = fs::read_to_string("puzzles/2025-10-input.txt").expect("Failed to read day 10 input");
    let machines = parse_day10_input(&input);
    
    let part1: usize = machines.iter().map(|m| solve_day10_part1(m)).sum();
    println!("Day 10 Part 1: {}", part1);
    
    let mut part2 = 0i64;
    for m in &machines {
        let result = solve_day10_part2(m);
        part2 += result;
    }
    println!("Day 10 Part 2: {}", part2);
}

fn solve_day10_part1(machine: &Machine) -> usize {
    let n = machine.buttons.len();
    
    let mut target = 0u64;
    for (i, &on) in machine.indicator.iter().enumerate() {
        if on {
            target |= 1 << i;
        }
    }
    
    let button_masks: Vec<u64> = machine.buttons.iter().map(|btn| {
        let mut mask = 0u64;
        for &i in btn {
            mask |= 1 << i;
        }
        mask
    }).collect();
    
    if n <= 22 {
        let n1 = n / 2;
        let n2 = n - n1;
        
        let mut map1: HashMap<u64, usize> = HashMap::new();
        for mask in 0..(1usize << n1) {
            let mut xor = 0u64;
            let mut count = 0;
            for i in 0..n1 {
                if (mask >> i) & 1 == 1 {
                    xor ^= button_masks[i];
                    count += 1;
                }
            }
            let entry = map1.entry(xor).or_insert(usize::MAX);
            *entry = (*entry).min(count);
        }
        
        let mut min_presses = usize::MAX;
        for mask in 0..(1usize << n2) {
            let mut xor = 0u64;
            let mut count = 0;
            for i in 0..n2 {
                if (mask >> i) & 1 == 1 {
                    xor ^= button_masks[n1 + i];
                    count += 1;
                }
            }
            let needed = target ^ xor;
            if let Some(&c1) = map1.get(&needed) {
                min_presses = min_presses.min(c1 + count);
            }
        }
        
        min_presses
    } else {
        use std::collections::VecDeque;
        let mut dist: HashMap<u64, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        dist.insert(0, 0);
        queue.push_back(0u64);
        
        while let Some(state) = queue.pop_front() {
            let d = dist[&state];
            if state == target {
                return d;
            }
            for &btn in &button_masks {
                let next = state ^ btn;
                if !dist.contains_key(&next) {
                    dist.insert(next, d + 1);
                    queue.push_back(next);
                }
            }
        }
        
        usize::MAX
    }
}

fn solve_day10_part2(machine: &Machine) -> i64 {
    let m = machine.joltages.len();
    let n = machine.buttons.len();
    let target = &machine.joltages;
    
    // Build constraint matrix A (m constraints x n variables)
    // where A[i][j] = 1 if button j affects counter i
    let a: Vec<Vec<f64>> = (0..m).map(|i| {
        (0..n).map(|j| {
            if machine.buttons[j].contains(&i) { 1.0 } else { 0.0 }
        }).collect()
    }).collect();
    
    let b: Vec<f64> = target.iter().map(|&x| x as f64).collect();
    let c: Vec<f64> = vec![1.0; n];  // Minimize sum of x
    
    // Solve LP relaxation using simplex
    if let Some(lp_sol) = simplex(&a, &b, &c) {
        // Check if LP solution is integer
        let is_integer = lp_sol.iter().all(|&x| (x - x.round()).abs() < 1e-5);
        
        if is_integer {
            return lp_sol.iter().map(|&x| x.round() as i64).sum();
        }
        
        // Use LP solution as starting point for local search
        let base: Vec<i64> = lp_sol.iter().map(|&x| x.floor() as i64).collect();
        let mut best = i64::MAX;
        
        // Search neighborhood
        search_neighborhood(&a, target, &base, 0, &mut vec![0i64; n], &mut best);
        
        if best < i64::MAX {
            return best;
        }
    }
    
    // Fall back to greedy
    greedy_solve(machine)
}

fn simplex(a: &[Vec<f64>], b: &[f64], c: &[f64]) -> Option<Vec<f64>> {
    // Two-phase simplex method
    let m = a.len();
    let n = a[0].len();
    
    // Phase 1: Find feasible solution
    // Tableau: m rows for constraints, 1 row for objective
    // Columns: n original + m artificial + 1 RHS
    
    let mut tableau = vec![vec![0.0; n + m + 1]; m + 1];
    
    // Fill constraint rows
    for i in 0..m {
        for j in 0..n {
            tableau[i][j] = a[i][j];
        }
        tableau[i][n + i] = 1.0;  // Artificial variable
        tableau[i][n + m] = b[i];  // RHS
    }
    
    // Phase 1 objective: minimize sum of artificial variables
    for i in 0..m {
        tableau[m][n + i] = 1.0;
    }
    
    // Pivot to eliminate artificial variables from objective
    for i in 0..m {
        for j in 0..=n + m {
            tableau[m][j] -= tableau[i][j];
        }
    }
    
    let eps = 1e-9;
    
    // Simplex iterations
    loop {
        // Find entering variable
        let mut entering = None;
        let mut max_coef = eps;
        for j in 0..n + m {
            if tableau[m][j] > max_coef {
                max_coef = tableau[m][j];
                entering = Some(j);
            }
        }
        
        if entering.is_none() { break; }
        let entering = entering.unwrap();
        
        // Find leaving variable
        let mut leaving = None;
        let mut min_ratio = f64::INFINITY;
        for i in 0..m {
            if tableau[i][entering] > eps {
                let ratio = tableau[i][n + m] / tableau[i][entering];
                if ratio < min_ratio {
                    min_ratio = ratio;
                    leaving = Some(i);
                }
            }
        }
        
        if leaving.is_none() { return None; }  // Unbounded
        let leaving = leaving.unwrap();
        
        // Pivot
        let pivot = tableau[leaving][entering];
        for j in 0..=n + m {
            tableau[leaving][j] /= pivot;
        }
        
        for i in 0..=m {
            if i != leaving {
                let factor = tableau[i][entering];
                for j in 0..=n + m {
                    tableau[i][j] -= factor * tableau[leaving][j];
                }
            }
        }
    }
    
    // Check feasibility
    if tableau[m][n + m].abs() > eps {
        return None;  // Infeasible
    }
    
    // Extract solution
    let mut solution = vec![0.0; n];
    for j in 0..n {
        // Check if variable j is basic
        let mut basic_row = None;
        for i in 0..m {
            if (tableau[i][j] - 1.0).abs() < eps {
                let mut is_basic = true;
                for k in 0..m {
                    if k != i && tableau[k][j].abs() > eps {
                        is_basic = false;
                        break;
                    }
                }
                if is_basic {
                    basic_row = Some(i);
                    break;
                }
            }
        }
        
        if let Some(row) = basic_row {
            solution[j] = tableau[row][n + m];
        }
    }
    
    Some(solution)
}

fn search_neighborhood(
    a: &[Vec<f64>],
    target: &[i64],
    base: &[i64],
    depth: usize,
    current: &mut [i64],
    best: &mut i64,
) {
    if depth == base.len() {
        // Check if valid
        for i in 0..target.len() {
            let sum: i64 = (0..current.len()).map(|j| current[j] * a[i][j] as i64).sum();
            if sum != target[i] {
                return;
            }
        }
        if current.iter().all(|&x| x >= 0) {
            let cost: i64 = current.iter().sum();
            *best = (*best).min(cost);
        }
        return;
    }
    
    // Try values around base
    for delta in -5..=10 {
        let val = base[depth] + delta;
        if val >= 0 && val < *best {
            current[depth] = val;
            search_neighborhood(a, target, base, depth + 1, current, best);
        }
    }
}

fn greedy_solve(machine: &Machine) -> i64 {
    let m = machine.joltages.len();
    let n = machine.buttons.len();
    let target = &machine.joltages;
    
    let a: Vec<Vec<i64>> = (0..m).map(|i| {
        (0..n).map(|j| {
            if machine.buttons[j].contains(&i) { 1 } else { 0 }
        }).collect()
    }).collect();
    
    let mut remaining = target.to_vec();
    let mut total = 0i64;
    
    loop {
        let mut best_j = None;
        let mut best_score = -1i64;
        
        for j in 0..n {
            let mut can_press = true;
            let mut score = 0i64;
            for i in 0..m {
                if a[i][j] > remaining[i] {
                    can_press = false;
                    break;
                }
                if remaining[i] > 0 {
                    score += a[i][j];
                }
            }
            if can_press && score > best_score {
                best_score = score;
                best_j = Some(j);
            }
        }
        
        if let Some(j) = best_j {
            for i in 0..m {
                remaining[i] -= a[i][j];
            }
            total += 1;
        } else {
            break;
        }
    }
    
    total
}
