use std::ops::RangeInclusive;

pub struct Day2;

impl crate::Day for Day2 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_2.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_2.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day2 {
    fn solve_part_1(input: &str) -> u64 {
        let data = Day2::process_input(input);
        data.into_iter()
            .flatten()
            .filter(|n| {
                let mut a = n.to_string();
                let b = a.split_off(a.len() / 2);
                a == b
            })
            .sum()
    }

    fn solve_part_2(input: &str) -> u64 {
        let data = Day2::process_input(input);
        data.into_iter()
            .flatten()
            .filter(|n| {
                let s = n.to_string().into_bytes();
                (1..=s.len() / 2)
                    .map(|i| s.chunks(i))
                    .any(|c| c.collect::<Vec<_>>().windows(2).all(|w| w[0] == w[1]))
            })
            .sum()
    }

    fn process_input(input: &str) -> Vec<RangeInclusive<u64>> {
        input
            .lines()
            .flat_map(|l| {
                l.split_terminator(',').filter_map(|s| {
                    let (start, stop) = s.trim().split_once('-')?;
                    Some(start.parse::<u64>().ok()?..=stop.parse::<u64>().ok()?)
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_2_test.txt");
        let expected = 1227775554;
        let actual = Day2::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_2_test.txt");
        let expected = 4174379265;
        let actual = Day2::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
