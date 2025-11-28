//! Utilities and shared traits for Advent of Code 2025 solutions.
//!
//! Each day's puzzle lives in [`days`] and implements [`Solution`]. Tests
//! should exercise the solution functions directly so `cargo test` can run
//! everything quickly.

pub mod days;

/// Common interface implemented by each day's solution.
pub trait Solution {
    /// Computes the answer to part one.
    fn part_one(&self, input: &str) -> Result<String, String>;

    /// Computes the answer to part two.
    fn part_two(&self, input: &str) -> Result<String, String>;
}

/// Convenience helper to run both parts for a given day.
pub fn run_day<T: Solution>(solver: &T, input: &str) -> (Result<String, String>, Result<String, String>) {
    (solver.part_one(input), solver.part_two(input))
}
