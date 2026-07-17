//! Day 6: Trash Compactor (cephalopod math worksheet).

/// Parse the worksheet, dropping trailing all-blank lines. Keeps every other
/// character (including spaces) exactly as in the file.
pub fn parse(text: &str) -> Vec<String> {
    let mut lines: Vec<String> = text
        .lines()
        .map(|l| l.strip_suffix('\r').unwrap_or(l).to_string())
        .collect();
    while lines.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        lines.pop();
    }
    lines
}

/// Locate each problem's column span from the operator (last) row.
fn spans(lines: &[String]) -> Vec<(usize, usize, char)> {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let oprow = &lines[lines.len() - 1];
    let mut ops: Vec<usize> = Vec::new();
    for (i, ch) in oprow.chars().enumerate() {
        if ch == '+' || ch == '*' {
            ops.push(i);
        }
    }
    let mut out = Vec::with_capacity(ops.len());
    for (k, &pos) in ops.iter().enumerate() {
        let end = if k + 1 < ops.len() { ops[k + 1] } else { width };
        out.push((pos, end, oprow.chars().nth(pos).unwrap()));
    }
    out
}

fn char_at(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap_or(' ')
}

fn combine(op: char, nums: &[u128]) -> u128 {
    match op {
        '+' => nums.iter().sum(),
        '*' => nums.iter().product(),
        _ => 0,
    }
}

/// Part 1: numbers are arranged horizontally, one per row in each problem.
pub fn part1(lines: &[String]) -> u128 {
    let numrows = &lines[..lines.len() - 1];
    let span_list = spans(lines);
    let mut total: u128 = 0;
    for (start, end, op) in span_list {
        let mut nums: Vec<u128> = Vec::new();
        for r in numrows {
            let seg: String = (start..end).map(|i| char_at(r, i)).collect();
            if let Some(t) = seg.split_whitespace().next() {
                if let Ok(v) = t.parse::<u128>() {
                    nums.push(v);
                }
            }
        }
        total += combine(op, &nums);
    }
    total
}

/// Part 2: numbers are written top-to-bottom in columns; read each character
/// column within a problem's span to form one number per column.
pub fn part2(lines: &[String]) -> u128 {
    let numrows = &lines[..lines.len() - 1];
    let span_list = spans(lines);
    let mut total: u128 = 0;
    for (start, end, op) in span_list {
        let mut nums: Vec<u128> = Vec::new();
        for col in start..end {
            let mut digits = String::new();
            for r in numrows {
                let ch = char_at(r, col);
                if ch.is_ascii_digit() {
                    digits.push(ch);
                }
            }
            if let Ok(v) = digits.parse::<u128>() {
                nums.push(v);
            }
        }
        total += combine(op, &nums);
    }
    total
}
