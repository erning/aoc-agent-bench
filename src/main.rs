//! Advent of Code agent bench solver (Rust).
//!
//! Solves both puzzles, printing each part's answer for the example and the
//! real input.

mod rational;
mod simplex;
mod cephalopod;
mod factory;

use std::fs;

fn read_input(candidates: &[&str]) -> String {
    for path in candidates {
        if let Ok(text) = fs::read_to_string(path) {
            return text;
        }
    }
    let tried = candidates.join(", ");
    panic!("could not read input; tried: {tried}");
}

fn main() {
    let p06_candidates = [
        "puzzles/2025-06-input.txt",
        "./puzzles/2025-06-input.txt",
        "../puzzles/2025-06-input.txt",
    ];
    let p06_example = [
        "puzzles/2025-06-example.txt",
        "./puzzles/2025-06-example.txt",
        "../puzzles/2025-06-example.txt",
    ];
    let p10_candidates = [
        "puzzles/2025-10-input.txt",
        "./puzzles/2025-10-input.txt",
        "../puzzles/2025-10-input.txt",
    ];
    let p10_example = [
        "puzzles/2025-10-example.txt",
        "./puzzles/2025-10-example.txt",
        "../puzzles/2025-10-example.txt",
    ];

    println!("=== Day 6: Trash Compactor (cephalopod math) ===");
    {
        let ex = cephalopod::parse(&read_input(&p06_example));
        println!("  example  part 1 = {} (expected 4277556)", cephalopod::part1(&ex));
        println!("  example  part 2 = {} (expected 3263827)", cephalopod::part2(&ex));

        let lines = cephalopod::parse(&read_input(&p06_candidates));
        println!("  input    part 1 = {}", cephalopod::part1(&lines));
        println!("  input    part 2 = {}", cephalopod::part2(&lines));
    }

    println!();
    println!("=== Day 10: Factory ===");
    {
        let ex = factory::parse(&read_input(&p10_example));
        println!("  example  part 1 = {} (expected 7)", factory::part1(&ex));
        println!("  example  part 2 = {} (expected 33)", factory::part2(&ex));

        let machines = factory::parse(&read_input(&p10_candidates));
        println!("  input    part 1 = {}", factory::part1(&machines));
        println!("  input    part 2 = {}", factory::part2(&machines));
    }
}
