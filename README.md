# Branch Evaluation

This document evaluates every branch other than `master` using the current branch state in the repo-local worktrees under `.worktrees/<branch>`.

Correct answers used for this evaluation:

- `2025-06` part 1: `5977759036837`
- `2025-06` part 2: `9630000828442`
- `2025-10` part 1: `524`
- `2025-10` part 2: `21696`

## Methodology

- Correctness and robustness:
  - Development mode was checked with `cargo run --quiet`.
  - `claude-opus-4.6` is a special case because plain `cargo run` is ambiguous; for that branch I used `cargo run --quiet --bin day06` and `cargo run --quiet --bin day10`.
- Performance:
  - I ran `cargo build --release --quiet` first.
  - I then timed the release executable three times and report the warm mean of runs 2 and 3 to reduce first-run cache noise.
  - For `claude-opus-4.6`, the reported runtime is the sum of `day06` and `day10`.
- Delivery time:
  - I approximated branch completion time as `HEAD commit timestamp - branch creation timestamp`.
  - Branch creation time comes from local git reflog metadata, so it is approximate by nature.

## Correctness And Robustness

| Branch | Dev-mode status | Correctness summary | Correct parts | Verdict |
| --- | --- | --- | ---: | --- |
| `codex-gpt-5.4` | Stable | Correct on all four parts | 4/4 | Fully correct |
| `claude-opus-4.6` | Stable with explicit `--bin`; plain `cargo run` is ambiguous | Correct except Day 10 part 2 | 3/4 | Very strong partial solution |
| `claude-k2p6-preview` | Stable | Correct except Day 10 part 2 | 3/4 | Very strong partial solution |
| `kimi-k2p5` | Stable | Correct except Day 10 part 2 | 3/4 | Fast but not exact on Day 10 part 2 |
| `claude-qwen3.6-plus` | Panics in debug on Day 10 part 2 with integer overflow | Release build runs, but Day 10 part 2 is wrong | 3/4 | Release build runs, but final answer is wrong |
| `claude-glm-5.1` | Panics in debug on Day 10 part 2 with integer overflow | Compile path was fixed later, but Day 10 part 2 is wrong | 3/4 | Improved, but still incorrect on the hardest part |
| `claude-k2p5` | Stable | Wrong on Day 6 part 2 and Day 10 part 2 | 2/4 | Weakest overall result |

## Performance, Verification, And Delivery

All times below are in local `+08:00` time. Runtime is warm release runtime in milliseconds.

| Branch | Approx. completion | Commits | Warm runtime | Rust tests | Build warnings | Verification style |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `codex-gpt-5.4` | 9.3 min | 1 | 838.9 ms | 4 | 0 | Unit tests plus example assertions |
| `claude-opus-4.6` | 13.8 min | 1 | 10.1 ms | 0 | 0 | No tests, no runtime self-checks |
| `claude-k2p6-preview` | 29.1 min | 1 | 39.0 ms | 0 | 0 | No tests, no runtime self-checks |
| `kimi-k2p5` | 30.2 min | 1 | 7.2 ms | 0 | 1 | No tests, no runtime self-checks |
| `claude-k2p5` | 38.0 min | 1 | 59.3 ms | 0 | 0 | No tests, no runtime self-checks |
| `claude-qwen3.6-plus` | 76.2 min | 1 | 48.4 ms | 0 | 2 | Example assertions in `main` |
| `claude-glm-5.1` | 79.9 min | 2 | 179.0 ms | 0 | 7 | Example assertions in `main` |

Notes:

- `claude-glm-5.1` is the only branch with a follow-up commit after the initial solve. Its first "solve" commit landed earlier, and the second commit fixed the `include_str!` paths.
- `codex-gpt-5.4` is by far the slowest runtime, but it is also the only branch that stayed exact end-to-end.
- `kimi-k2p5` and `claude-opus-4.6` are the fastest runtime implementations, but both miss the Day 10 part 2 optimum.
- `claude-k2p6-preview` uses exact rational arithmetic for Day 10 part 2, which is more robust than floating-point, but its bounded search from the LP ceiling still misses the true integer optimum by a small margin (`21699` vs `21696`).

## Branch-By-Branch Assessment

### `codex-gpt-5.4`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Strengths:

- The only branch that produced all four correct final answers.
- Best verification discipline: 4 Rust tests plus example assertions in `main`.
- Cleanest structure: reusable logic in `src/lib.rs`, thin CLI in `src/main.rs`.
- Day 10 part 2 uses exact arithmetic with rational elimination and bounded search, so it avoids the floating-point and overflow traps that hurt several other branches.

Weaknesses:

- Slowest runtime by a wide margin at about `838.9 ms` warm.

Assessment:

- Best overall branch by a clear margin.
- If correctness matters more than raw speed, this is the winner.

### `claude-opus-4.6`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21682` (incorrect; expected `21696`)

Strengths:

- Very fast runtime at about `10.1 ms`.
- Fast delivery at about `13.8 min`.
- Correct on all of Day 6 and Day 10 part 1.
- Separate `day06` and `day10` binaries make the internal split clear.

Weaknesses:

- `cargo run` is not ergonomic because the branch has multiple binaries and no default run target.
- `src/main.rs` is still just `Hello, world!`, so the branch does not provide a polished top-level entry point.
- Day 10 part 2 is solved with a floating-point Big-M simplex relaxation, which is not an exact integer solver and returns `21682` instead of `21696`.

Assessment:

- Best partial solution.
- Excellent speed, but not exact on the hardest part and not polished as a final cargo package.

### `claude-k2p6-preview`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21699` (incorrect; expected `21696`)

Strengths:

- Clean, stable development-mode run with no ambiguity.
- Correct on all of Day 6 and Day 10 part 1.
- Uses exact rational arithmetic (`num-rational`) for Day 10 part 2, avoiding the floating-point and overflow issues that hurt other branches.
- Zero build warnings and zero tests (clean build, though no verification).
- Warm runtime is respectable at about `39.0 ms`.

Weaknesses:

- No unit tests and no runtime example assertions.
- Day 10 part 2 solves the LP relaxation exactly over all basis choices, then searches for an integer solution starting from the ceiling of the LP optimum. This bounded search returns `21699`, missing the true optimum `21696` by a small margin.

Assessment:

- Strong partial solution with the most numerically robust approach among the incorrect branches, but the integer search logic needs to be extended or replaced with an exact solver.

### `kimi-k2p5`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `19157` (incorrect; expected `21696`)

Strengths:

- Fastest runtime overall at about `7.2 ms`.
- Stable in development mode.
- Correct on both Day 6 answers and Day 10 part 1.
- Delivery time was reasonably fast at about `30.2 min`.

Weaknesses:

- No unit tests and no runtime example assertions.
- Day 10 part 2 uses simplex plus local-search / greedy fallback logic, so it finds a feasible answer but not the optimum.
- Final Day 10 part 2 answer is `19157`, which is materially wrong.

Assessment:

- Very fast and reasonably pragmatic, but the final optimization logic is heuristic rather than exact.

### `claude-qwen3.6-plus`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `19248` (incorrect; expected `21696`)

Strengths:

- Good self-checking habit in source: example assertions are present in `main`.
- Correct on Day 6 and Day 10 part 1.
- Warm runtime is respectable at about `48.4 ms`.

Weaknesses:

- Development-mode run panics on Day 10 part 2 with integer overflow.
- Release mode does not panic, but it silently produces the wrong answer `19248`.
- Day 10 part 2 relies on floating-point RREF, rounded nullspace vectors, and bounded enumeration, which is not robust enough for exact integer optimization.

Assessment:

- Better verification than most incorrect branches, but still too fragile because debug and release disagree.

### `claude-glm-5.1`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `33028` (incorrect; expected `21696`)

Strengths:

- The branch was improved after the initial pass: the later commit fixed the compile path problem, so the current branch now builds successfully.
- Example assertions are present in `main`.
- Correct on Day 6 and Day 10 part 1.

Weaknesses:

- Development-mode run still panics on Day 10 part 2 due to overflow in the integer elimination step.
- Release mode wraps through the overflow and prints the wrong Day 10 part 2 answer `33028`.
- It has the highest warning count of all branches (`7`).
- It also took the longest time to reach its current HEAD state (`79.9 min`), partly because of the follow-up compile fix.

Assessment:

- Better than the original evaluation suggested because it now compiles, but still not trustworthy on the hardest part.

### `claude-k2p5`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `23668715679561` (incorrect; expected `9630000828442`)
- 2025-10 part 1: `524`
- 2025-10 part 2: `958` (incorrect; expected `21696`)

Strengths:

- Stable to run.
- Runtime is decent at about `59.3 ms`.

Weaknesses:

- Incorrect on Day 6 part 2 and Day 10 part 2.
- Day 6 parsing assumes a fixed 4-column problem width, which does not generalize to the real worksheet.
- Day 10 part 2 only does exhaustive search for small cases and then falls back to a greedy heuristic, which is not sufficient for the real input.
- No tests and no runtime assertions.

Assessment:

- Weakest branch both algorithmically and empirically.

## Overall Ranking

1. `codex-gpt-5.4`
   - Only fully correct branch.
   - Best verification and best engineering structure.

2. `claude-opus-4.6`
   - Best partial solution by delivery time and raw speed.
   - Fast and close, but not exact on Day 10 part 2.

3. `claude-k2p6-preview`
   - Strong partial solution with exact rational arithmetic.
   - Very close on Day 10 part 2 (off by only 3), but still not optimal.

4. `kimi-k2p5`
   - Fastest runtime.
   - Stable, but heuristic on Day 10 part 2 and therefore incorrect.

5. `claude-qwen3.6-plus`
   - Better self-checking than most incorrect branches.
   - Debug/release mismatch makes it hard to trust.

6. `claude-glm-5.1`
   - Improved from the earlier state because it now compiles.
   - Still fails exactness and stability on Day 10 part 2.

7. `claude-k2p5`
   - Weakest correctness and weakest modeling choices.

## Bottom Line

If I were choosing one branch to keep as the reference solution, I would keep `codex-gpt-5.4`.

If I were choosing one incorrect branch to salvage, I would start from `claude-opus-4.6`: it is fast, compact, and already very close, but its Day 10 part 2 solver needs to be replaced with an exact integer method.

`claude-k2p6-preview` is another good salvage candidate: it already uses exact rational arithmetic, so it avoids the floating-point pitfalls of `claude-opus-4.6`, but its bounded search from the LP ceiling needs to be widened or replaced with a proper exact-integer solver to reach the true optimum.
