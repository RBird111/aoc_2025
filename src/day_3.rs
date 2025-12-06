pub struct Day3;

impl crate::Day for Day3 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_3.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_3.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day3 {
    fn solve_part_1(input: &str) -> u64 {
        let data = Day3::process_input(input);
        data.into_iter()
            .filter_map(|arr| {
                let (l, mut r) = arr[..arr.len() - 1].iter().fold((0, 0), |(l, r), &n| {
                    if n > l {
                        (n, 0)
                    } else if n > r {
                        (l, n)
                    } else {
                        (l, r)
                    }
                });
                r = r.max(*arr.last()?);
                Some(l * 10 + r)
            })
            .sum::<u64>()
    }

    fn solve_part_2(input: &str) -> u64 {
        let data = Day3::process_input(input);
        data.into_iter()
            .map(|arr| {
                let n = arr.len();
                (1..13)
                    .rev()
                    .map(|rem| n - rem + 1)
                    .scan(0, |pos, end| {
                        let best = arr[*pos..end].iter().max()?;
                        *pos = arr
                            .iter()
                            .enumerate()
                            .position(|(i, b)| *pos <= i && b == best)
                            .unwrap()
                            + 1;
                        Some(*best)
                    })
                    .zip((0..12).rev())
                    .map(|(b, i)| b * 10u64.pow(i))
                    .sum::<u64>()
            })
            .sum::<u64>()
    }

    fn process_input(input: &str) -> Vec<Vec<u64>> {
        input
            .lines()
            .map(|l| l.bytes().map(|b| (b - b'0') as u64).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_3_test.txt");
        let expected = 357;
        let actual = Day3::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_3_test.txt");
        let expected = 3121910778619;
        let actual = Day3::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
