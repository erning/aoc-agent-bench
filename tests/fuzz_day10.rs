//! Fuzz test: compare solve_part2 against an exact memoized brute-force
//! solver on random small machines. Run with: cargo test --release

#[path = "../src/bin/day10.rs"]
mod day10;

use day10::Machine;
use std::collections::HashMap;

/// Exact, obviously-complete solver: per-button DFS with memoization.
/// Returns None on infeasible; gives up (Err) past a node budget.
fn brute_force(m: &Machine) -> Result<Option<i128>, ()> {
    const INF: i128 = i128::MAX / 4;
    const NODE_CAP: u64 = 200_000;
    fn dfs(
        i: usize,
        rem: &mut Vec<i128>,
        buttons: &[Vec<usize>],
        memo: &mut HashMap<(usize, Vec<i128>), i128>,
        nodes: &mut u64,
    ) -> i128 {
        *nodes += 1;
        if *nodes > NODE_CAP {
            return INF;
        }
        if rem.iter().all(|&r| r == 0) {
            return 0;
        }
        if i == buttons.len() {
            return INF;
        }
        let key = (i, rem.clone());
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let cap = buttons[i].iter().map(|&c| rem[c]).min().unwrap();
        let mut best = INF;
        for k in 0..=cap {
            for &c in &buttons[i] {
                rem[c] -= k;
            }
            let sub = dfs(i + 1, rem, buttons, memo, nodes);
            if sub != INF {
                best = best.min(k + sub);
            }
            for &c in &buttons[i] {
                rem[c] += k;
            }
        }
        memo.insert(key, best);
        best
    }
    let mut memo = HashMap::new();
    let mut nodes = 0u64;
    let mut rem = m.joltage.clone();
    let v = dfs(0, &mut rem, &m.buttons, &mut memo, &mut nodes);
    if nodes > NODE_CAP {
        return Err(()); // inconclusive
    }
    Ok(if v == INF { None } else { Some(v) })
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn fuzz_round(seed: u64, cases: u64, max_counters: u64, max_buttons: u64, max_jolt: u64) {
    let mut rng = Rng(seed);
    for case in 0..cases {
        let nc = 1 + rng.below(max_counters) as usize;
        let nb = 1 + rng.below(max_buttons) as usize;
        let buttons: Vec<Vec<usize>> = (0..nb)
            .map(|_| {
                let mut v: Vec<usize> = (0..nc).filter(|_| rng.below(2) == 0).collect();
                if v.is_empty() {
                    v.push(rng.below(nc as u64) as usize);
                }
                v
            })
            .collect();
        let joltage: Vec<i128> = (0..nc).map(|_| rng.below(max_jolt) as i128).collect();
        let m = Machine {
            target_mask: 0,
            buttons,
            joltage,
        };
        let Ok(expected) = brute_force(&m) else {
            continue; // brute force over budget: skip, solver alone is fast
        };
        let got = day10::solve_part2(&m);
        assert_eq!(
            got, expected,
            "case {}: buttons {:?} joltage {:?}",
            case, m.buttons, m.joltage
        );
    }
}

#[test]
fn fuzz_part2_small() {
    fuzz_round(0x9e3779b97f4a7c15, 100_000, 5, 8, 12);
}

#[test]
fn fuzz_part2_larger() {
    fuzz_round(0xdeadbeefcafef00d, 10_000, 6, 10, 15);
}
