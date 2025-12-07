pub struct Day4;

impl crate::Day for Day4 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_4.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_4.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day4 {
    fn solve_part_1(input: &str) -> u64 {
        let data = Day4::process_input(input);
        let (rows, cols) = (data.len(), data.first().map(|row| row.len()).unwrap_or(0));
        (0..rows)
            .flat_map(|r| {
                (0..cols)
                    .map(move |c| (r, c))
                    .filter(|&(r, c)| data[r][c] == b'@')
            })
            .filter(|&pos| {
                (0..)
                    .scan(Some(Dir::North), |dir, _| {
                        let res = dir.map(|d| d.get_coordinates(pos, (rows, cols)));
                        *dir = (*dir)?.next_dir();
                        res
                    })
                    .flatten()
                    .filter(|&(r, c)| data[r][c] == b'@')
                    .count()
                    .lt(&4)
            })
            .count() as u64
    }

    fn solve_part_2(input: &str) -> u64 {
        let mut data = Day4::process_input(input);
        let (rows, cols) = (data.len(), data.first().map(|row| row.len()).unwrap_or(0));

        let mut res = 0;
        let mut removals = vec![];

        loop {
            for (r, c) in (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c))) {
                if data[r][c] != b'@' {
                    continue;
                }

                let neighbors: Vec<_> = (0..)
                    .scan(Some(Dir::North), |dir, _| {
                        let res = dir.map(|d| d.get_coordinates((r, c), (rows, cols)));
                        *dir = (*dir)?.next_dir();
                        res
                    })
                    .flatten()
                    .filter(|&(r, c)| data[r][c] == b'@')
                    .collect();

                if neighbors.len() < 4 {
                    removals.push((r, c));
                    res += 1;
                }
            }

            if !removals.is_empty() {
                removals.iter().for_each(|&(r, c)| {
                    data[r][c] = b'.';
                });
                removals.clear();
            } else {
                break;
            }
        }

        res
    }

    fn process_input(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|l| l.bytes().collect()).collect()
    }
}

#[derive(Clone, Copy)]
enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Dir {
    fn get_coordinates(
        &self,
        (x, y): (usize, usize),
        (rows, cols): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Dir::North => Some((x.checked_sub(1)?, y)),
            Dir::NorthEast => Some((x.checked_sub(1)?, (y + 1 < cols).then_some(y + 1)?)),
            Dir::East => Some((x, (y + 1 < cols).then_some(y + 1)?)),
            Dir::SouthEast => Some((
                (x + 1 < rows).then_some(x + 1)?,
                (y + 1 < cols).then_some(y + 1)?,
            )),
            Dir::South => Some(((x + 1 < rows).then_some(x + 1)?, y)),
            Dir::SouthWest => Some(((x + 1 < rows).then_some(x + 1)?, y.checked_sub(1)?)),
            Dir::West => Some((x, y.checked_sub(1)?)),
            Dir::NorthWest => Some((x.checked_sub(1)?, y.checked_sub(1)?)),
        }
    }

    fn next_dir(&self) -> Option<Dir> {
        match self {
            Dir::North => Some(Dir::NorthEast),
            Dir::NorthEast => Some(Dir::East),
            Dir::East => Some(Dir::SouthEast),
            Dir::SouthEast => Some(Dir::South),
            Dir::South => Some(Dir::SouthWest),
            Dir::SouthWest => Some(Dir::West),
            Dir::West => Some(Dir::NorthWest),
            Dir::NorthWest => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_4_test.txt");
        let expected = 13;
        let actual = Day4::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_4_test.txt");
        let expected = 43;
        let actual = Day4::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
