#![allow(unused)]
pub struct Day7;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl crate::Day for Day7 {
    fn part_1(&self) -> String {
        let input = include_str!("../data/day_7.txt");
        Self::solve_part_1(input).to_string()
    }

    fn part_2(&self) -> String {
        let input = include_str!("../data/day_7.txt");
        Self::solve_part_2(input).to_string()
    }
}

impl Day7 {
    fn solve_part_1(input: &str) -> u64 {
        let grid = Day7::process_input(input);
        let start = (0..grid.len())
            .flat_map(|i| (0..grid.first().map_or(0, |row| row.len())).map(move |j| (i, j)))
            .find(|&(i, j)| grid[i][j] == b'S')
            .expect("couldn't find start position");

        let counter = Rc::new(RefCell::new(0));
        let beam = Beam::new(start, counter.clone());
        let mut curr_map = HashMap::from([(start, beam)]);

        while !curr_map.is_empty() {
            curr_map = curr_map
                .values()
                .filter_map(|b| b.increment(&grid))
                .flatten()
                .map(|b| (b.pos, b))
                .collect();
        }

        *counter.borrow()
    }

    fn solve_part_2(input: &str) -> u64 {
        0
    }

    fn process_input(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|l| l.bytes().collect()).collect()
    }
}

#[derive(Clone, Debug)]
struct Beam {
    pos: (usize, usize),
    counter: Rc<RefCell<u64>>,
}

impl Beam {
    fn new(pos: (usize, usize), counter: Rc<RefCell<u64>>) -> Beam {
        Beam { pos, counter }
    }

    fn increment(&self, grid: &[Vec<u8>]) -> Option<Vec<Beam>> {
        let (x, y) = self.pos;
        match grid.get(x + 1)?.get(y)? {
            b'.' => Some(vec![Beam::new((x + 1, y), self.counter.clone())]),
            b'^' => {
                *self.counter.borrow_mut() += 1;
                Some(vec![
                    Beam::new((x + 1, y.checked_sub(1)?), self.counter.clone()),
                    Beam::new(
                        (x + 1, (y + 1 < grid.first()?.len()).then_some(y + 1)?),
                        self.counter.clone(),
                    ),
                ])
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = include_str!("../data/day_7_test.txt");
        let expected = 21;
        let actual = Day7::solve_part_1(data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../data/day_7_test.txt");
        let expected = 40;
        let actual = Day7::solve_part_2(data);
        assert_eq!(expected, actual);
    }
}
