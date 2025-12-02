pub struct Day1;

impl crate::Day for Day1 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_1.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_1.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day1 {
    fn solve_part_1(input: &str) -> u32 {
        let data = Day1::process_input(input);
        data.into_iter()
            .map(|arr| arr.iter().sum())
            .scan(50, |pos, turns: i32| {
                *pos = (*pos + turns) % 100;
                Some((*pos == 0) as u32)
            })
            .sum()
    }

    fn solve_part_2(input: &str) -> u32 {
        let data = Day1::process_input(input);
        data.into_iter()
            .flatten()
            .scan(50, |pos, turns| {
                *pos = (*pos + turns) % 100;
                Some((*pos == 0) as u32)
            })
            .sum()
    }

    fn process_input(input: &str) -> Vec<Vec<i32>> {
        input
            .lines()
            .map(|l| {
                let (dir, offset) = l.split_at(1);
                let turns = offset
                    .bytes()
                    .fold(0, |acc, d| acc * 10 + (d - b'0') as usize);
                match dir {
                    "L" => vec![-1; turns],
                    "R" => vec![1; turns],
                    _ => unreachable!("expected value of dir '{dir}' to be L or R"),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_1_test.txt");
        let expected = 3;
        let actual = Day1::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_1_test.txt");
        let expected = 6;
        let actual = Day1::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
