use crate::Solution;

/// Placeholder implementation for Day 1.
pub struct Day01;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Result<String, String> {
        let lines = input.lines().filter(|line| !line.trim().is_empty()).count();
        Ok(format!("Lines: {}", lines))
    }

    fn part_two(&self, input: &str) -> Result<String, String> {
        let sum: usize = input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().len())
            .sum();
        Ok(format!("Line length sum: {}", sum))
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    use crate::run_day;

    const SAMPLE: &str = "AoC\n2025";

    #[test]
    fn part_one_counts_lines() {
        let (part_one, _) = run_day(&Day01, SAMPLE);
        assert_eq!(part_one.unwrap(), "Lines: 2");
    }

    #[test]
    fn part_two_sums_line_lengths() {
        let (_, part_two) = run_day(&Day01, SAMPLE);
        assert_eq!(part_two.unwrap(), "Line length sum: 7");
    }
}
