#![allow(unused)]
use std::ops::Range;
pub struct Day6;

impl crate::Day for Day6 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_6.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_6.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day6 {
    fn solve_part_1(input: &str) -> u64 {
        let (nums, ops) = Self::process_input(input);
        let mut sum = 0;
        for (i, row) in nums.into_iter().enumerate() {
            let res = match ops[i] {
                "+" => row.into_iter().sum::<u64>(),
                "*" => row.into_iter().product::<u64>(),
                _ => unreachable!(),
            };
            sum += res;
        }
        sum
    }

    fn solve_part_2(input: &str) -> u64 {
        0
    }

    fn process_input(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
        let (top, bot) = input.rsplit_once("\n").expect("couldn't find a new line?");
        let nums_t: Vec<Vec<u64>> = top
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .filter_map(|n| n.parse::<u64>().ok())
                    .collect()
            })
            .collect();
        let nums = (0..nums_t[0].len())
            .map(|j| (0..nums_t.len()).map(|i| nums_t[i][j]).collect())
            .collect();
        let ops = bot.split_whitespace().collect();
        (nums, ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_6_test.txt");
        let expected = 4277556;
        let actual = Day6::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_6_test.txt");
        let expected = 14;
        let actual = Day6::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
