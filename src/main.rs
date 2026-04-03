use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let day06_example = fs::read_to_string(root.join("puzzles/2025-06-example.txt"))?;
    let day06_example_result = aoc_model_test::solve_day06(&day06_example);
    assert_eq!(day06_example_result, (4_277_556, 3_263_827));

    let day06_input = fs::read_to_string(root.join("puzzles/2025-06-input.txt"))?;
    let day06_result = aoc_model_test::solve_day06(&day06_input);
    println!("2025 Day 6");
    println!("Part 1: {}", day06_result.0);
    println!("Part 2: {}", day06_result.1);

    let day10_example = fs::read_to_string(root.join("puzzles/2025-10-example.txt"))?;
    let day10_example_result = aoc_model_test::solve_day10(&day10_example);
    assert_eq!(day10_example_result, (7, 33));

    let day10_input = fs::read_to_string(root.join("puzzles/2025-10-input.txt"))?;
    let day10_result = aoc_model_test::solve_day10(&day10_input);
    println!("2025 Day 10");
    println!("Part 1: {}", day10_result.0);
    println!("Part 2: {}", day10_result.1);

    Ok(())
}
