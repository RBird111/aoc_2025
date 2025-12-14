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
        let beam = Beam::new(start, 1, counter.clone());
        let mut map = HashMap::from([(start, beam)]);

        while !map.is_empty() {
            map = map
                .values()
                .filter_map(|b| b.split(&grid))
                .flatten()
                .map(|b| (b.pos, b))
                .collect();
        }

        *counter.borrow()
    }

    fn solve_part_2(input: &str) -> u64 {
        let grid = Day7::process_input(input);
        let start = (0..grid.len())
            .flat_map(|i| (0..grid.first().map_or(0, |row| row.len())).map(move |j| (i, j)))
            .find(|&(i, j)| grid[i][j] == b'S')
            .expect("couldn't find start position");

        let counter = Rc::new(RefCell::new(1));
        let beam = Beam::new(start, 1, counter.clone());
        let mut map = HashMap::new();
        let mut map_c = map.clone();
        map.insert(start, beam);

        while !map.is_empty() {
            map.values()
                .filter_map(|b| b.split(&grid))
                .flatten()
                .map(|b| (b.pos, b))
                .for_each(|(pos, beam)| {
                    map_c
                        .entry(pos)
                        .and_modify(|b| b.increment_local(beam.counter_local))
                        .or_insert(beam);
                });
            std::mem::swap(&mut map, &mut map_c);
            map_c.clear();
        }

        *counter.borrow()
    }

    fn process_input(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|l| l.bytes().collect()).collect()
    }
}

#[derive(Clone, Debug)]
struct Beam {
    pos: (usize, usize),
    counter_global: Rc<RefCell<u64>>,
    counter_local: u64,
}

impl Beam {
    fn new(pos: (usize, usize), counter_local: u64, counter_global: Rc<RefCell<u64>>) -> Beam {
        Beam {
            pos,
            counter_global,
            counter_local,
        }
    }

    fn split(&self, grid: &[Vec<u8>]) -> Option<Vec<Beam>> {
        let (x, y) = self.pos;
        match grid.get(x + 1)?.get(y)? {
            b'.' => Some(vec![Beam::new(
                (x + 1, y),
                self.counter_local,
                self.counter_global.clone(),
            )]),
            b'^' => {
                *self.counter_global.borrow_mut() += self.counter_local;
                Some(vec![
                    Beam::new(
                        (x + 1, y.checked_sub(1)?),
                        self.counter_local,
                        self.counter_global.clone(),
                    ),
                    Beam::new(
                        (x + 1, (y + 1 < grid.first()?.len()).then_some(y + 1)?),
                        self.counter_local,
                        self.counter_global.clone(),
                    ),
                ])
            }
            _ => unreachable!(),
        }
    }

    fn increment_local(&mut self, amount: u64) {
        self.counter_local += amount;
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
