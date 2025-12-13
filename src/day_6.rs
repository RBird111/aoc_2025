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
        let (nums, ops) = Day6::process_input_part_1(input);
        Day6::solve(nums, ops)
    }

    fn solve_part_2(input: &str) -> u64 {
        let (nums, ops) = Day6::process_input_part_2(input);
        Day6::solve(nums, ops)
    }

    fn solve(nums: Vec<Vec<u64>>, ops: Vec<&str>) -> u64 {
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

    fn process_input_part_1(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
        let (top, bot) = input.rsplit_once("\n").expect("couldn't find a new line?");
        let ops = bot.split_whitespace().collect();
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
        (nums, ops)
    }

    fn process_input_part_2(input: &str) -> (Vec<Vec<u64>>, Vec<&str>) {
        let (top, bot) = input.rsplit_once("\n").expect("couldn't find a new line?");
        let ops = bot.split_whitespace().collect();
        let nums_t: Vec<Vec<u8>> = top.lines().map(|l| l.bytes().collect()).collect();
        let nums: Vec<Vec<u64>> = (0..nums_t[0].len())
            .map(|j| {
                (0..nums_t.len())
                    .map(|i| nums_t[i][j] as char)
                    .collect::<String>()
                    .trim()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
            .split("\n\n")
            .map(|s| {
                s.split("\n")
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect()
            })
            .collect();
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
        let expected = 3263827;
        let actual = Day6::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
