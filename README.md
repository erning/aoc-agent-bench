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
  - `claude-opus-4.6`, `claude-mimo-v2.5-pro`, `kimi-k3`, and `pi-qwen3.8-27b` are special cases because plain `cargo run` is ambiguous or requires an input path; for those branches I used the explicit binary names and input files.
- Performance:
  - I ran `cargo build --release --quiet` first.
  - I then timed the release executable three times and report the warm mean of runs 2 and 3 to reduce first-run cache noise.
  - For `claude-opus-4.6`, `claude-mimo-v2.5-pro`, `kimi-k3`, and `pi-qwen3.8-27b`, the reported runtime is the sum of the two puzzle binaries.
- Delivery time:
  - I approximated branch completion time as `HEAD commit timestamp - branch creation timestamp`.
  - Branch creation time comes from local git reflog metadata, so it is approximate by nature.
  - For `pi-qwen3.8-27b`, completion time is measured from the first user prompt to the final answer in the saved Pi session because the solution was imported into this branch after the run.

## Correctness And Robustness

| Branch | Dev-mode status | Correctness summary | Correct parts | Verdict |
| --- | --- | --- | ---: | --- |
| `codex-gpt-5.4` | Stable | Correct on all four parts | 4/4 | Fully correct |
| `codex-gpt-5.5` | Stable | Correct on all four parts | 4/4 | Fully correct |
| `claude-deepseek-v4-pro` | Stable | Correct on all four parts | 4/4 | Fully correct |
| `pi-glm-5.2` | Stable | Correct on all four parts | 4/4 | Fully correct, fast but lightly verified |
| `kimi-k3` | Stable with explicit `--bin` and input path | Correct on all four parts | 4/4 | Fully correct, but slower and less polished |
| `pi-qwen3.8-27b` | Stable with explicit `--bin` and input path | Correct on all four parts | 4/4 | Fully correct, but slow and required one harness-recovery prompt |
| `claude-opus-4.6` | Stable with explicit `--bin`; plain `cargo run` is ambiguous | Correct except Day 10 part 2 | 3/4 | Very strong partial solution |
| `claude-k2p6-preview` | Stable | Correct except Day 10 part 2 | 3/4 | Very strong partial solution |
| `claude-mimo-v2.5-pro` | Stable with explicit `--bin`; plain `cargo run` is ambiguous | Correct except Day 10 part 2 | 3/4 | Strong partial solution |
| `kimi-k2p5` | Stable | Correct except Day 10 part 2 | 3/4 | Fast but not exact on Day 10 part 2 |
| `claude-qwen3.6-plus` | Panics in debug on Day 10 part 2 with integer overflow | Release build runs, but Day 10 part 2 is wrong | 3/4 | Release build runs, but final answer is wrong |
| `claude-glm-5.1` | Panics in debug on Day 10 part 2 with integer overflow | Compile path was fixed later, but Day 10 part 2 is wrong | 3/4 | Improved, but still incorrect on the hardest part |
| `claude-k2p5` | Stable | Wrong on Day 6 part 2 and Day 10 part 2 | 2/4 | Weakest overall result |

## Performance, Verification, And Delivery

All times below are in local `+08:00` time. Runtime is warm release runtime in milliseconds.

| Branch | Approx. completion | Commits | Warm runtime | Rust tests | Build warnings | Verification style |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `codex-gpt-5.5` | 6.7 min | 1 | 730.0 ms | 0 | 0 | Example assertions in `main` |
| `codex-gpt-5.4` | 9.3 min | 1 | 838.9 ms | 4 | 0 | Unit tests plus example assertions |
| `claude-opus-4.6` | 13.8 min | 1 | 10.1 ms | 0 | 0 | No tests, no runtime self-checks |
| `claude-k2p6-preview` | 29.1 min | 1 | 39.0 ms | 0 | 0 | No tests, no runtime self-checks |
| `kimi-k2p5` | 30.2 min | 1 | 7.2 ms | 0 | 1 | No tests, no runtime self-checks |
| `claude-mimo-v2.5-pro` | 34.3 min | 1 | 8.1 ms | 2 | 0 | Example-based unit tests |
| `claude-k2p5` | 38.0 min | 1 | 59.3 ms | 0 | 0 | No tests, no runtime self-checks |
| `pi-glm-5.2` | 62.4 min | 2 | 20.0 ms | 0 | 0 | Example assertions in `main` |
| `claude-deepseek-v4-pro` | 62.9 min | 1 | 53.0 ms | 3 | 0 | Example-based unit tests for both days |
| `claude-qwen3.6-plus` | 76.2 min | 1 | 48.4 ms | 0 | 2 | Example assertions in `main` |
| `pi-qwen3.8-27b` | 78.7 min | 1 | 3545.0 ms | 0 | 1 | Examples, internal feasibility checks, and independent Python spot checks |
| `claude-glm-5.1` | 79.9 min | 2 | 179.0 ms | 0 | 7 | Example assertions in `main` |
| `kimi-k3` | 106.2 min | 2 | 1265.0 ms | 2 | 0 | Day 10 fuzz tests plus independent verifier binary |

Notes:

- `claude-glm-5.1` is the only branch with a follow-up commit after the initial solve. Its first "solve" commit landed earlier, and the second commit fixed the `include_str!` paths.
- `pi-glm-5.2` also has a follow-up commit after the initial solve. The second commit evicted degenerate artificial variables before phase 2, after which the development and release runs both produced the correct answers.
- Before this evaluation, `codex-gpt-5.4`, `codex-gpt-5.5`, and `claude-deepseek-v4-pro` were the only branches that stayed exact end-to-end.
- `pi-glm-5.2` is now the fourth branch that stayed exact end-to-end, and it has the fastest runtime among the fully correct branches at about `20.0 ms` warm.
- `codex-gpt-5.5` delivered faster and runs slightly faster than `codex-gpt-5.4`, but it has weaker automated verification because it has no Rust tests.
- `kimi-k2p5` and `claude-opus-4.6` are the fastest runtime implementations, but both miss the Day 10 part 2 optimum.
- `claude-k2p6-preview` uses exact rational arithmetic for Day 10 part 2, which is more robust than floating-point, but its bounded search from the LP ceiling still misses the true integer optimum by a small margin (`21699` vs `21696`).
- `claude-mimo-v2.5-pro` is the only 3/4 branch that ships Rust unit tests, and at about `8.1 ms` warm it is nearly as fast as `kimi-k2p5`, but its Day 10 part 2 branch-and-bound prunes too aggressively and returns `21012`.
- `claude-deepseek-v4-pro` was the third 4/4 branch, and its runtime was previously the only fully correct result well under `100 ms`. Its Day 10 part 2 uses exact rational Gaussian elimination with bounded enumeration over the nullspace, guarded by a feasibility check on each partial assignment.
- `kimi-k3` was the fourth 4/4 branch. It uses exact rational Gaussian elimination with branch-and-bound and adds two randomized fuzz tests for Day 10 part 2. The current implementation is correct, but its two-binary command-line layout is less ergonomic, the test build emits four unused-code warnings, and its warm runtime is about `1265.0 ms`.
- `pi-qwen3.8-27b` is the sixth branch to reach 4/4. It uses exact rational LP relaxation with integer branch-and-bound, but its `3545.0 ms` warm runtime is the slowest among the fully correct branches. The run used Pi 0.84.2, Ollama 0.32.13, Qwen3.8-27B Q4_K_M on an RTX 5090, a configured 65,536-token context window (not the model's maximum), and medium thinking.

### `pi-glm-5.2`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Strengths:

- Produced all four correct final answers in both development and release mode.
- Fastest runtime among the fully correct branches at about `20.0 ms` warm.
- Uses exact rational arithmetic for the two-phase simplex relaxation and integer branch-and-bound for Day 10 part 2, avoiding floating-point rounding and heuristic-only optimization.
- The follow-up fix removes degenerate artificial variables from the simplex basis before phase 2, making the current implementation stable on the real input.
- Single-binary `cargo run` is unambiguous and the release build has no warnings.

Weaknesses:

- Ships no Rust tests and relies on example assertions in `main` for runtime verification.
- The simplex and branch-and-bound implementation is substantial but has no independent cross-check, randomized test, or proof-oriented verification around its exactness.
- The current result required a follow-up fix after the initial solve, so the final two-commit state is the relevant delivery point; its approximate completion time is `62.4 min`.

Assessment:

- A fully correct, compact, and very fast implementation. It is the best raw-performance reference among the exact branches, but its lack of automated tests makes it less trustworthy as a maintained reference than `codex-gpt-5.4` or `claude-deepseek-v4-pro`.

## Branch-By-Branch Assessment

### `codex-gpt-5.4`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Strengths:

- Produced all four correct final answers.
- Best verification discipline: 4 Rust tests plus example assertions in `main`.
- Cleanest structure: reusable logic in `src/lib.rs`, thin CLI in `src/main.rs`.
- Day 10 part 2 uses exact arithmetic with rational elimination and bounded search, so it avoids the floating-point and overflow traps that hurt several other branches.

Weaknesses:

- Slowest runtime by a wide margin at about `838.9 ms` warm.

Assessment:

- Best engineered branch overall.
- If maintainability and verification matter more than raw speed, this remains the branch to keep.

### `codex-gpt-5.5`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Strengths:

- Produced all four correct final answers.
- Fastest delivery time in the evaluated set at about `6.7 min`.
- Uses exact rational arithmetic for Day 10 part 2 and avoids the floating-point and overflow traps.
- Slightly faster than `codex-gpt-5.4` at about `730.0 ms` warm.
- Stable in development mode and builds with zero warnings.

Weaknesses:

- No Rust tests; verification is limited to example assertions in `main`.
- Single-file implementation is less cleanly packaged than `codex-gpt-5.4`.
- Runtime is still much slower than the fast incorrect branches.

Assessment:

- Strongest challenger to `codex-gpt-5.4` and the fastest fully correct branch.
- I would prefer it when delivery speed and a simple single-binary solution matter most, but I would still harden it with tests before using it as the reference solution.

### `claude-deepseek-v4-pro`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Cost: solving both puzzles on this branch cost `0.91 CNY` in DeepSeek-v4-Pro API spend.

Strengths:

- Produced all four correct final answers.
- Stable in development mode with a single `main.rs` that calls both `day06::solve` and `day10::solve`, so plain `cargo run` is unambiguous.
- Ships example-based Rust unit tests for both Day 6 and Day 10 (3 tests total, including separate Day 10 part 1 and part 2 cases) — the strongest test discipline among the 4/4 branches after `codex-gpt-5.4`.
- Day 10 part 2 uses exact rational Gaussian elimination with a bounded nullspace search, guarded by a feasibility check on each partial assignment. This is the only 4/4 implementation that combines exact arithmetic with a sub-`100 ms` warm runtime.
- Warm runtime around `53.0 ms` is roughly an order of magnitude faster than either `codex` branch.
- Zero build warnings and no external dependencies; `Cargo.toml` lists no entries under `[dependencies]`.

Weaknesses:

- Slowest delivery time of the 4/4 branches at about `62.9 min`, almost ten times longer than `codex-gpt-5.5`.
- Single-file modules; logic is not split into a reusable library, so the structure is less polished than `codex-gpt-5.4`.
- The bounded enumeration uses a hard cap (`50000`) on each free variable. The cap holds for this input but is brittle for harder instances and is not justified by the model.

Assessment:

- The best balance of correctness, runtime, and test discipline among the fully correct branches.
- I would treat it as the strongest candidate for a fast, exact reference solution, with the caveat that delivery time was high and the enumeration cap should be removed or tightened if the input changes.

### `kimi-k3`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Strengths:

- Produced all four correct final answers, including the Day 10 part 2 optimum after the follow-up pruning fix.
- Uses exact rational arithmetic and checks the reconstructed solution against every original counter equation.
- Ships two randomized fuzz tests for Day 10 part 2; both passed in the release test run.
- Includes an independent `verify_day10` binary that uses a separate memoized search implementation for cross-checking.
- The release build completes without warnings and has no external dependencies.

Weaknesses:

- `cargo run` is not ergonomic because the branch has multiple binaries, and both binaries require an explicit input path.
- The fuzz-test compilation emits four unused-code warnings because it includes the binary source directly.
- Warm runtime is about `1265.0 ms`, the slowest among the fully correct branches by a substantial margin.
- The branch took about `106.2 min` to reach its current two-commit state, the longest delivery time in the evaluated set.

Assessment:

- A correct and unusually well-checked implementation, but not a practical reference choice when runtime, command-line ergonomics, or delivery time matters.
- The exact arithmetic and randomized tests make it a credible correctness baseline; the main follow-ups would be extracting shared library code, removing test warnings, and improving the search performance.

### `pi-qwen3.8-27b`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21696`

Run configuration:

- Pi coding agent 0.84.2 with its built-in read, write, edit, bash, grep, find, and ls tools.
- Ollama 0.32.13 serving `qwen3.8:27b` Q4_K_M on an RTX 5090.
- Configured 65,536-token context window (not the model's maximum), medium thinking, no answer-bearing README in the working directory, and no network or parent-directory reads in the saved session.

Strengths:

- Produced all four correct answers in development and release mode without algorithmic hints.
- Uses exact rational arithmetic for the Day 10 part 2 LP relaxation and integer branch-and-bound rather than floating-point or heuristic optimization.
- Verifies every reconstructed Day 10 part 2 solution against the original counter equations.
- Independently cross-checks Day 10 part 1 and 38 tractable Day 10 part 2 machines with a Python verifier; both puzzle examples also match exactly.

Weaknesses:

- Took about `78.7 min` from the first prompt to the final answer and required extensive debugging after an initial 600-second timeout.
- Warm runtime is about `3545.0 ms`, the slowest fully correct implementation in this evaluation.
- Ships no Rust tests, produces one unused-variable warning, is not `cargo fmt --check` clean, and leaves per-machine timing output enabled on stderr.
- Pi/Ollama failed to continue automatically after context compaction with `no user query found in messages`; one user message was needed to request the already-computed final summary. This was a harness recovery, not an algorithm or answer hint.
- The agent claimed scratch verification artifacts were cleaned, but `lp_check.py`, `dbg.log`, and an orphaned temporary solver process remained in the original working directory. Those artifacts are excluded from this branch.

Assessment:

- A genuine 4/4 result from a locally hosted 27B Q4 model, and stronger on correctness than every partial branch, including `claude-opus-4.6`.
- The result is impressive for its model size and local deployment, but weak runtime, no Rust tests, a long debugging trajectory, and imperfect cleanup keep it behind the other fully correct branches overall.

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

### `claude-mimo-v2.5-pro`

Release output:

- 2025-06 part 1: `5977759036837`
- 2025-06 part 2: `9630000828442`
- 2025-10 part 1: `524`
- 2025-10 part 2: `21012` (incorrect; expected `21696`)

Strengths:

- Very fast warm runtime at about `8.1 ms` (sum of `day06` and `day10`).
- Correct on all of Day 6 and Day 10 part 1.
- Ships example-based Rust unit tests for both Day 6 and Day 10 — the only 3/4 branch with any Rust tests.
- Zero build warnings.
- Separate `day06` and `day10` binaries make the internal split clear.

Weaknesses:

- `cargo run` is not ergonomic because the branch has multiple binaries and no default run target.
- `src/main.rs` is still just `Hello, world!`, so the branch does not provide a polished top-level entry point.
- Day 10 part 2 uses a floating-point two-phase simplex (LP relaxation) plus a branch-and-bound that fixes each fractional variable to either `floor` or `ceil` of its LP value and then recurses on a reduced problem. This pruning discards integer optima that lie outside `{floor, ceil}` for the chosen branching variable, and it returns `21012` instead of `21696`.

Assessment:

- Strong partial solution that combines fast runtime, useful tests, and a multi-binary structure, but the Day 10 part 2 ILP solver loses optimality to its branching shortcut.

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
   - Fully correct.
   - Best verification and best engineering structure.

2. `claude-deepseek-v4-pro`
   - Fully correct, with exact arithmetic and the fastest runtime of any 4/4 branch.
   - Ships Rust unit tests for both days, but delivery was much slower than the `codex` branches.

3. `pi-glm-5.2`
   - Fully correct and the fastest runtime among all fully correct branches.
   - Exact arithmetic and a single-binary interface, but no Rust tests and a follow-up fix were needed.

4. `codex-gpt-5.5`
   - Fully correct and fastest to deliver.
   - Faster than `codex-gpt-5.4`, but weaker on tests and structure.

5. `kimi-k3`
   - Fully correct, with exact arithmetic and randomized Day 10 tests.
   - Much slower than the other fully correct branches and less ergonomic to run.

6. `pi-qwen3.8-27b`
   - Fully correct with exact rational branch-and-bound on a local 27B Q4 model.
   - Slowest fully correct runtime, no Rust tests, and one harness-recovery prompt after compaction.

7. `claude-opus-4.6`
   - Best partial solution by delivery time and raw speed.
   - Fast and close, but not exact on Day 10 part 2.

8. `claude-k2p6-preview`
   - Strong partial solution with exact rational arithmetic.
   - Very close on Day 10 part 2 (off by only 3), but still not optimal.

9. `claude-mimo-v2.5-pro`
   - Strong partial solution with the best test discipline among the 3/4 branches.
   - Fast runtime at about `8.1 ms`, but Day 10 part 2 loses the optimum to an over-aggressive branch-and-bound.

10. `kimi-k2p5`
   - Fastest runtime.
   - Stable, but heuristic on Day 10 part 2 and therefore incorrect.

11. `claude-qwen3.6-plus`
   - Better self-checking than most incorrect branches.
   - Debug/release mismatch makes it hard to trust.

12. `claude-glm-5.1`
   - Improved from the earlier state because it now compiles.
   - Still fails exactness and stability on Day 10 part 2.

13. `claude-k2p5`
   - Weakest correctness and weakest modeling choices.

## Bottom Line

If I were choosing one branch to keep as the reference solution, I would keep `codex-gpt-5.4` because it has the best tests and structure among the fully correct branches.

`claude-deepseek-v4-pro` is the strongest alternative on a runtime basis: it is the only 4/4 branch with a sub-`100 ms` warm runtime and it ships unit tests for both days, but its delivery time was much higher and its bounded enumeration relies on a hard `50000` cap that I would want removed before treating it as the canonical solution.

`kimi-k3` is also fully correct and has stronger randomized checking than most branches, but its `1265.0 ms` warm runtime, two-binary interface, and long delivery time make it better suited as a correctness cross-check than as the canonical implementation.

`pi-qwen3.8-27b` is a notable local-model result: it reaches 4/4 with a 27B Q4 model and exact arithmetic, but its `3545.0 ms` warm runtime, lack of Rust tests, long debugging process, and harness recovery make it the weakest of the fully correct branches.

`codex-gpt-5.5` is the best speed-to-correctness result: it delivered fastest and ran faster than `codex-gpt-5.4`, but it needs real tests and a cleaner module split before I would prefer it as the reference branch.

`pi-glm-5.2` is the best raw-performance exact branch: its custom rational simplex and integer branch-and-bound solve the current input in about `20.0 ms` warm. I would still add independent tests before using it as the canonical reference, especially because the current state depends on a follow-up fix for degenerate simplex artificials.

If I were choosing one incorrect branch to salvage, I would start from `claude-opus-4.6`: it is fast, compact, and already very close, but its Day 10 part 2 solver needs to be replaced with an exact integer method.

`claude-k2p6-preview` is another good salvage candidate: it already uses exact rational arithmetic, so it avoids the floating-point pitfalls of `claude-opus-4.6`, but its bounded search from the LP ceiling needs to be widened or replaced with a proper exact-integer solver to reach the true optimum.
