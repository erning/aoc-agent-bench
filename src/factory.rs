//! Day 10: Factory (indicator lights + joltage ILP).

pub struct Machine {
    pub diagram: Vec<u8>, // '#' or '.'
    pub buttons: Vec<Vec<usize>>,
    pub jolts: Vec<i64>,
}

pub fn parse(text: &str) -> Vec<Machine> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let lb = line.find('[').unwrap();
        let rb = line.find(']').unwrap();
        let diagram: Vec<u8> = line[lb + 1..rb].bytes().collect();

        let lb_curly = line.find('{').unwrap();
        let rb_curly = line.find('}').unwrap();
        let jolts: Vec<i64> = line[lb_curly + 1..rb_curly]
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        // Buttons live in the text between ']' and '{'.
        let mid = &line[rb + 1..lb_curly];
        let mut buttons = Vec::new();
        let bytes = mid.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'(' {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] != b')' {
                    j += 1;
                }
                let inner = &mid[i + 1..j];
                let idxs: Vec<usize> = inner
                    .split(',')
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim().parse::<usize>().unwrap())
                    .collect();
                buttons.push(idxs);
                i = j + 1;
            } else {
                i += 1;
            }
        }

        out.push(Machine {
            diagram,
            buttons,
            jolts,
        });
    }
    out
}

/// Part 1: minimum number of button presses to reach the target light pattern,
/// toggling in GF(2). Pressing a button twice cancels, so each button is used
/// 0 or 1 times -> minimum Hamming-weight solution of A x = b (mod 2).
pub fn part1(machines: &[Machine]) -> u64 {
    let mut total: u64 = 0;
    for m in machines {
        let target: u32 = m
            .diagram
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == b'#')
            .map(|(i, _)| 1u32 << i)
            .sum();

        let cols: Vec<u32> = m
            .buttons
            .iter()
            .map(|btn| btn.iter().map(|&i| 1u32 << i).sum())
            .collect();

        let nb = cols.len();
        let mut best: u64 = u64::MAX;
        for mask in 0u32..(1u32 << nb) {
            let mut state: u32 = 0;
            let mut mm = mask;
            while mm != 0 {
                let b = mm.trailing_zeros() as usize;
                state ^= cols[b];
                mm &= mm - 1;
            }
            if state == target {
                let pc = mask.count_ones() as u64;
                if pc < best {
                    best = pc;
                }
            }
        }
        total += best;
    }
    total
}

/// Part 2: minimum total presses so each counter reaches its joltage level.
/// min 1^T x  s.t.  A x = b, x >= 0 integer, with A[i][j] = 1 if button j
/// affects counter i.
pub fn part2(machines: &[Machine]) -> u64 {
    let mut total: u64 = 0;
    for m in machines {
        let c = m.jolts.len();
        let nb = m.buttons.len();
        let mut a: Vec<Vec<i64>> = vec![vec![0i64; nb]; c];
        for (j, btn) in m.buttons.iter().enumerate() {
            for &idx in btn {
                if idx < c {
                    a[idx][j] = 1;
                }
            }
        }
        let val = crate::simplex::ilp_min_sum(&a, &m.jolts).expect("no feasible configuration");
        total += val as u64;
    }
    total
}
