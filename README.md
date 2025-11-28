# Advent Of Code 2025

## Goals
- Do Advent Of Code in Rust this year ( opportunity to learn Rust)
- Write it in the form of unit tests, and have the unit tests run automatically via github CI actions
- Document difficulties, lessons learned
- Have fun ;-)

## Project layout
- `src/lib.rs` defines the shared [`Solution`](src/lib.rs) trait and helper utilities.
- Each day lives in `src/days/dayXX.rs` and exposes a type that implements `Solution`.
- Unit tests sit next to each day module so `cargo test` runs every puzzle quickly.

A placeholder Day 1 lives in [`src/days/day01.rs`](src/days/day01.rs) to demonstrate the pattern.

## Development
- Add a new day by creating `src/days/dayXX.rs`, exporting it from `src/days/mod.rs`, and implementing `Solution`.
- Run the full test suite locally with:
  ```bash
  cargo test
  ```
- Code style follows the default `rustfmt`; format with `cargo fmt` if you make changes.

## Continuous integration
GitHub Actions runs `cargo test` on every push and pull request via [`.github/workflows/ci.yml`](.github/workflows/ci.yml).
