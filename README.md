# rust-cp

A Rust workspace for competitive programming. It keeps reusable algorithms in a normal library during development and turns a solution into one source file for submission.

## Workspace

- `crates/cp-library`: reusable data structures and algorithms.
- `crates/cargo-cp`: the `cargo cp` workflow tool.
- `solutions/src/bin`: one binary per problem.
- `fuzz`: cargo-fuzz targets for randomized testing.

The starter library contains:

- `Dsu`: disjoint-set union with path compression and union by size.
- `Fenwick<T>`: point updates and range sums.
- `SegmentTree<T, F>`: generic point updates and associative range queries.
- `minimum_spanning_forest`: Kruskal's algorithm, built on the library's `Dsu` module.
- `Cin` and `Cout`: buffered, typed token input and chainable output inspired by C++ streams.
- `Itertools`: a dependency-free, contest-focused adaptation of [`rust-itertools`](https://github.com/rust-itertools/itertools), including sorting, joining, uniqueness, counts, and Cartesian products.

## Run the Cargo subcommand

The workspace’s `.cargo/config.toml` defines `cp` as a Cargo alias, so no installation is needed when running commands from this workspace:

```sh
cargo cp --help
cargo cp bundle solutions/src/bin/range_sum.rs
cargo cp fuzz fuzz_target_1
```

To make the command available outside this workspace, install its executable:

```sh
cargo install --path crates/cargo-cp
```

## Add and run a solution

Create `solutions/src/bin/problem_name.rs` and import normal library paths:

```rust
use cp_library::{Cin, Cout, Dsu};

let mut cin = Cin::new();
let n: usize = cin.read();
let values: Vec<i64> = cin.read_vec(n);

let mut cout = Cout::new();
cout.print("values:").space().print_iter(values, " ").newline();
```

Iterator helpers are available by importing the extension trait. The implementation is based on `rust-itertools` 0.15.0 but intentionally keeps a smaller API so bundled submissions remain compact:

```rust
use cp_library::Itertools;

let answer = [3, 1, 2, 1]
    .into_iter()
    .unique()
    .sorted()
    .join(" ");
assert_eq!(answer, "1 2 3");

let grid = (0..2)
    .cartesian_product(['a', 'b'])
    .collect_vec();
assert_eq!(grid, [(0, 'a'), (0, 'b'), (1, 'a'), (1, 'b')]);
```

Run it with:

```sh
cargo run -p solutions --bin problem_name < input.txt
```

## Run a fuzz target

Install [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) and the nightly toolchain once:

```sh
rustup toolchain install nightly
cargo install cargo-fuzz
```

Then run a target from `fuzz/fuzz_targets` through the nightly compiler:

```sh
cargo cp fuzz fuzz_target_1
```

This wraps `cargo +nightly fuzz run <target>` and forwards the fuzzer's input and output directly to the terminal.

## Bundle a submission

Bundle to the default sibling path, `solutions/src/bin/range_sum_bundled.rs`:

```sh
cargo cp bundle solutions/src/bin/range_sum.rs
```

Override the output path with `-o` or `--output`:

```sh
cargo cp bundle solutions/src/bin/range_sum.rs -o submission.rs
rustc --edition=2024 submission.rs
```

`bundle` uses `syn` to:

1. Parse the solution as Rust syntax.
2. Recursively replace out-of-line modules (`mod graph;`) with their parsed contents. Both `graph.rs` and `graph/mod.rs` layouts, plus `#[path = "..."]`, are supported.
3. Run `cargo metadata` and identify workspace library crates referenced by the solution.
4. Resolve root re-exports and retain only the transitive closure of modules referenced by the solution, including dependencies between `cp-library` modules.
5. Inline the retained modules and rewrite their `crate::...` paths for the new nesting.
6. Pretty-print one submission-ready Rust file.

Only referenced path-based workspace libraries and reachable modules are embedded. Grouped imports, root re-exports, direct paths, dependencies inside library modules, and paths in macro arguments are considered. Whole-crate aliases and glob imports conservatively retain the complete library. If a solution imports a crates.io or Git dependency declared in its `Cargo.toml`, bundling fails with an explicit error; submission code must use the standard library or workspace library. Macro-generated module declarations and `$crate` references cannot be structurally rewritten by `syn` and should be avoided in submission code.

## Ideas for the next `cargo cp` subcommands

- `cargo cp new <name>`: scaffold a solution with fast input/output and optional contest metadata.
- `cargo cp test <name>`: run local `.in`/`.out` sample pairs with useful diffs and time limits.
- `cargo cp fetch <url>`: download samples from supported judges.
- `cargo cp submit <name>`: bundle and submit through an opt-in judge adapter.
- `cargo cp stress <fast> <slow> <generator>`: differential randomized testing.
- `cargo cp expand <item>`: print only one library algorithm for debugging or notebook export.

Keeping these in `cargo-cp` gives the repository an xtask-like automation layer while retaining Cargo's familiar command syntax.
