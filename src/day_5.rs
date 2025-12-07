use std::ops::Range;
pub struct Day5;

impl crate::Day for Day5 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_5.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_5.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day5 {
    fn solve_part_1(input: &str) -> u64 {
        let (ranges, ids) = Day5::process_input(input);
        ids.into_iter()
            .filter(|id| ranges.iter().any(|rng| rng.contains(id)))
            .count() as u64
    }

    fn solve_part_2(input: &str) -> u64 {
        let (mut ranges, _) = Day5::process_input(input);
        ranges.sort_by_key(|rng| rng.start);
        ranges
            .into_iter()
            .scan(0u64, |prev, rng| {
                let (mut start, stop) = (rng.start, rng.end);
                start = start.max(*prev);
                *prev = stop.max(*prev);
                Some((*prev).saturating_sub(start))
            })
            .sum::<u64>()
    }

    fn process_input(input: &str) -> (Vec<Range<u64>>, Vec<u64>) {
        let (top, bot) = input.split_once("\n\n").unwrap_or(("", ""));
        let ranges = top
            .lines()
            .filter_map(|rng| {
                let (start, stop) = rng.split_once('-')?;
                Some(start.parse::<u64>().ok()?..(stop.parse::<u64>().ok()? + 1))
            })
            .collect();
        let ids = bot
            .lines()
            .filter_map(|id| id.parse::<u64>().ok())
            .collect();
        (ranges, ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_5_test.txt");
        let expected = 3;
        let actual = Day5::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_5_test.txt");
        let expected = 14;
        let actual = Day5::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
