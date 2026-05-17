# Repository Guidelines

## Project Structure & Module Organization
`src/main.rs` drives the CLI and `src/lib.rs` stitches the analyzer modules. Use the feature folders as intended: `analysis/` (graph + reachability), `parser/` (tree-sitter grammars), `discovery/` (walkdir + ignore rules), `config/` (YAML + CLI), `refactor/` (safe delete), and `report/` (terminal, JSON, SARIF). Integration fixtures live in `tests/fixtures/`, are called by `tests/integration/`, and benchmarks sit in `benches/`.

## Build, Test, and Development Commands
- `cargo fmt` – format all sources with the repo’s rustfmt defaults.
- `cargo clippy --all-targets --all-features -D warnings` – static analysis; keep this clean before pushing.
- `cargo test` – run unit + integration suites; add `-- --nocapture` when debugging analyzer output.
- `cargo run -- --help` – verify argument wiring locally; pass a sample project path to exercise end-to-end.
- `cargo build --release` – produce the optimized CLI used for measuring scan time.
- `cargo bench parsing_bench` – run Criterion benchmarks in `benches/parsing_bench.rs`.

## Coding Style & Naming Conventions
Stick to Rust 2021 defaults: four-space indentation, `snake_case` modules/functions, `PascalCase` types, `SCREAMING_SNAKE_CASE` constants. Use expression-based error handling (`Result` + `?`), keep modules reasonably small, and document public APIs with `///`. No custom `rustfmt.toml`, so always run `cargo fmt`; treat `clippy`’s warnings as errors and favor iterator adapters or early returns over ad-hoc loops.

## Testing Guidelines
Place quick unit tests next to the modules they exercise (`#[cfg(test)]`), and keep CLI or multi-language flows in `tests/integration/`. Mirror fixture names (e.g., `android_manifest_kept`) in test functions for fast grepping, and update both Kotlin and Java fixtures whenever you touch analyzer logic or deletion safety. Run `cargo test` routinely and `cargo test -- --ignored` before merging if you add slow filesystem scenarios.

## Commit & Pull Request Guidelines
Archived upstream history uses short, imperative subjects (`analyzer: tighten entrypoint scan`) with optional scopes; stay under 72 characters, include a brief body when context or test output matters, and link issues as `#123`. PR descriptions must list the problem, approach, sample CLI output (terminal or JSON), and the checks you ran (`fmt`, `clippy`, `test`, release build). Attach SARIF/JSON snippets whenever reporting or deletion behavior shifts.

## Configuration & Safety Tips
Keep `chazer.yaml` under version control and capture any new `entry_points`, `retain_patterns`, or annotation rules in the same change that relies on them. Changes to deletion flows must include `--delete --dry-run` transcripts plus confirmation that generated restore scripts remain idempotent.
