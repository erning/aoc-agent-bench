//! Independent cross-check for day10: plain memoized DFS over buttons,
//! deliberately sharing no code with the RREF-based solver in day10.rs.
//! Usage: verify_day10 <input-file> [num_machines]

use std::env;
use std::fs;

struct Machine {
    target_mask: u64,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
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
                        buttons.push(
                            tok[1..tok.len() - 1]
                                .split(',')
                                .filter(|s| !s.is_empty())
                                .map(|s| s.parse::<usize>().unwrap())
                                .collect(),
                        );
                    }
                    '{' => {
                        joltage = tok[1..tok.len() - 1]
                            .split(',')
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
                    }
                    _ => unreachable!(),
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

fn part1(m: &Machine) -> u64 {
    let n = m.buttons.len();
    let masks: Vec<u64> = m
        .buttons
        .iter()
        .map(|b| b.iter().fold(0u64, |a, &c| a | (1 << c)))
        .collect();
    (0u64..(1 << n))
        .filter(|s| {
            masks
                .iter()
                .enumerate()
                .filter(|(i, _)| s & (1 << i) != 0)
                .fold(0u64, |st, (_, bm)| st ^ bm)
                == m.target_mask
        })
        .map(|s| s.count_ones() as u64)
        .min()
        .unwrap()
}

const NODE_CAP: u64 = 20_000_000;

fn part2(m: &Machine) -> Option<i64> {
    // Branch and bound over buttons. Lower bound on remaining presses:
    // each press increments any single counter by at most 1, so we need at
    // least max(rem) more presses; also a counter is unsatisfiable if no
    // remaining button covers it.
    fn dfs(
        i: usize,
        rem: &mut Vec<i64>,
        buttons: &[Vec<usize>],
        best: &mut i64,
        cost: i64,
        nodes: &mut u64,
    ) {
        *nodes += 1;
        if *nodes > NODE_CAP {
            return; // give up on this machine; treated as skipped
        }
        let max_rem = *rem.iter().max().unwrap();
        if max_rem == 0 {
            *best = (*best).min(cost);
            return;
        }
        if i == buttons.len() || cost + max_rem >= *best {
            return;
        }
        // Feasibility: every nonzero counter must be coverable by buttons i..
        if rem.iter().enumerate().any(|(c, &r)| {
            r > 0 && !buttons[i..].iter().any(|b| b.contains(&c))
        }) {
            return;
        }
        let cap = buttons[i].iter().map(|&c| rem[c]).min().unwrap();
        // Larger counts first: finds a good incumbent early.
        for k in (0..=cap).rev() {
            for &c in &buttons[i] {
                rem[c] -= k;
            }
            dfs(i + 1, rem, buttons, best, cost + k, nodes);
            for &c in &buttons[i] {
                rem[c] += k;
            }
            if cost >= *best || *nodes > NODE_CAP {
                break;
            }
        }
    }
    let mut best = i64::MAX;
    let mut nodes = 0u64;
    let mut rem = m.joltage.clone();
    dfs(0, &mut rem, &m.buttons, &mut best, 0, &mut nodes);
    if nodes > NODE_CAP {
        return None; // search exhausted its budget: inconclusive
    }
    assert!(best != i64::MAX, "no solution");
    Some(best)
}

fn main() {
    let path = env::args().nth(1).expect("usage: verify_day10 <input> [n]");
    let n: usize = env::args()
        .nth(2)
        .map(|s| s.parse().unwrap())
        .unwrap_or(usize::MAX);
    let machines = parse(&fs::read_to_string(&path).unwrap());
    let mut solved = 0u32;
    for (i, m) in machines.iter().take(n).enumerate() {
        match part2(m) {
            Some(p2) => {
                solved += 1;
                println!("machine {}: {} {}", i, part1(m), p2);
            }
            None => println!("machine {}: {} SKIP", i, part1(m)),
        }
    }
    eprintln!("part2 conclusively solved: {}/{}", solved, machines.len().min(n));
}
