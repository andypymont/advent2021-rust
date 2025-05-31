use std::num::ParseIntError;
use std::str::FromStr;

advent_of_code::solution!(3);

const DIGITS: usize = if cfg!(test) { 5 } else { 12 };
const MAX_VALUE: usize = 1 << DIGITS;

#[derive(Clone, Debug, PartialEq)]
struct NumberSet {
    flags: [bool; MAX_VALUE],
    length: usize,
}

impl NumberSet {
    const fn new() -> Self {
        Self {
            flags: [false; MAX_VALUE],
            length: 0,
        }
    }

    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..MAX_VALUE).filter(|x| self.flags[*x])
    }

    const fn insert(&mut self, value: usize) {
        let current = self.flags[value];
        if !current {
            self.length += 1;
        }
        self.flags[value] = true;
    }

    const fn remove(&mut self, value: usize) {
        let current = self.flags[value];
        if current {
            self.length -= 1;
        }
        self.flags[value] = false;
    }

    fn retain_only_matching(&mut self, digit: usize) {
        (0..MAX_VALUE)
            .filter(|value| value & digit == 0)
            .for_each(|value| self.remove(value));
    }

    fn remove_matching(&mut self, digit: usize) {
        (0..MAX_VALUE)
            .filter(|value| value & digit != 0)
            .for_each(|value| self.remove(value));
    }

    fn digit_counts(&self, digit: usize) -> (usize, usize) {
        (0..MAX_VALUE).fold((0, 0), |(zeroes, ones), value| {
            if self.flags[value] {
                if value & digit == 0 {
                    (zeroes + 1, ones)
                } else {
                    (zeroes, ones + 1)
                }
            } else {
                (zeroes, ones)
            }
        })
    }

    fn best_match(&self, common: bool) -> Option<usize> {
        let mut numbers = self.clone();
        for pos in (0..DIGITS).rev() {
            let digit = 1 << pos;
            let (zeroes, ones) = numbers.digit_counts(digit);

            if common {
                if ones >= zeroes {
                    numbers.retain_only_matching(digit);
                } else {
                    numbers.remove_matching(digit);
                }
            } else if ones < zeroes {
                numbers.retain_only_matching(digit);
            } else {
                numbers.remove_matching(digit);
            }

            if numbers.length == 1 {
                return numbers.iter().next();
            }
        }

        None
    }
}

impl FromStr for NumberSet {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut set = Self::new();

        for line in input.lines() {
            let value = usize::from_str_radix(line, 2)?;
            set.insert(value);
        }

        Ok(set)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    NumberSet::from_str(input).ok().map(|numbers| {
        let (gamma, epsilon) = (0..DIGITS).fold((0, 0), |(gamma, epsilon), pos| {
            let digit = 1 << pos;
            let (zeroes, ones) = numbers.digit_counts(digit);
            if ones >= zeroes {
                (gamma | digit, epsilon)
            } else {
                (gamma, epsilon | digit)
            }
        });

        gamma * epsilon
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    NumberSet::from_str(input).ok().map(|numbers| {
        let oxygen = numbers.best_match(true).unwrap_or(0);
        let carbon = numbers.best_match(false).unwrap_or(0);

        oxygen * carbon
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_number_set() -> NumberSet {
        let mut set = NumberSet::new();
        set.insert(4);
        set.insert(30);
        set.insert(22);
        set.insert(23);
        set.insert(21);
        set.insert(15);
        set.insert(7);
        set.insert(28);
        set.insert(16);
        set.insert(25);
        set.insert(2);
        set.insert(10);
        set
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            NumberSet::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_number_set()),
        );
    }

    #[test]
    fn test_digit_counts() {
        let set = example_number_set();
        assert_eq!(set.digit_counts(1), (7, 5));
        assert_eq!(set.digit_counts(2), (5, 7));
        assert_eq!(set.digit_counts(4), (4, 8));
        assert_eq!(set.digit_counts(8), (7, 5));
        assert_eq!(set.digit_counts(16), (5, 7));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_number_set_length() {
        let mut set = example_number_set();
        assert_eq!(set.length, 12);

        set.insert(1);
        assert_eq!(set.length, 13);
        set.insert(31);
        assert_eq!(set.length, 14);

        set.remove(7);
        assert_eq!(set.length, 13);
        set.remove(7);
        assert_eq!(set.length, 13);

        set.remove(3);
        assert_eq!(set.length, 13);
    }

    #[test]
    fn test_number_set_remove_matching() {
        let mut set = NumberSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);

        let mut removed = NumberSet::new();
        removed.insert(1);
        removed.insert(4);

        set.remove_matching(2);
        assert_eq!(set, removed);
    }

    #[test]
    fn test_number_set_retain_only_matching() {
        let mut set = NumberSet::new();
        set.insert(1);
        set.insert(9);
        set.insert(10);
        set.insert(11);
        set.insert(16);

        let mut retained = NumberSet::new();
        retained.insert(9);
        retained.insert(10);
        retained.insert(11);

        set.retain_only_matching(8);
        assert_eq!(set, retained);
    }

    #[test]
    fn test_find_best_match() {
        let set = example_number_set();
        assert_eq!(set.best_match(true), Some(23));
        assert_eq!(set.best_match(false), Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(230));
    }
}
