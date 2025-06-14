use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq)]
struct InsertionRule(char, char, char);

#[derive(Debug, PartialEq)]
struct Polymer {
    template: Vec<char>,
    rules: Vec<InsertionRule>,
}

impl Polymer {
    fn char_counts(&self) -> BTreeMap<char, usize> {
        let mut counts = BTreeMap::new();

        for ch in &self.template {
            counts
                .entry(*ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        counts
    }

    fn expand(&mut self) {
        let mut after = Vec::new();

        let mut chars = self.template.iter();

        let Some(mut left) = chars.next() else {
            return;
        };
        for right in chars {
            after.push(*left);
            if let Some(insert) = self.insertion_for_pair(*left, *right) {
                after.push(insert);
            }

            left = right;
        }
        after.push(*left);
        self.template = after;
    }

    fn insertion_for_pair(&self, left: char, right: char) -> Option<char> {
        self.rules
            .iter()
            .find(|rule| rule.0 == left && rule.1 == right)
            .map(|rule| rule.2)
    }

    fn least_and_most_common_counts(&self) -> (usize, usize) {
        let mut least = usize::MAX;
        let mut most = 0;

        let counts = self.char_counts();
        for count in counts.values() {
            least = least.min(*count);
            most = most.max(*count);
        }

        (least, most)
    }
}

#[derive(Debug, PartialEq)]
struct ParsePolymerError;

impl FromStr for InsertionRule {
    type Err = ParsePolymerError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (input, output) = line.split_once(" -> ").ok_or(ParsePolymerError)?;

        let mut input = input.chars();
        let left = input.next().ok_or(ParsePolymerError)?;
        let right = input.next().ok_or(ParsePolymerError)?;
        let output = output.chars().next().ok_or(ParsePolymerError)?;

        Ok(Self(left, right, output))
    }
}

impl FromStr for Polymer {
    type Err = ParsePolymerError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (template, rules_str) = input.split_once("\n\n").ok_or(ParsePolymerError)?;
        let template = template.chars().collect();

        let mut rules = Vec::new();
        for line in rules_str.lines() {
            let rule = InsertionRule::from_str(line)?;
            rules.push(rule);
        }

        Ok(Self { template, rules })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Polymer::from_str(input).ok().map(|mut polymer| {
        for _ in 0..10 {
            polymer.expand();
        }
        let (least, most) = polymer.least_and_most_common_counts();
        most - least
    })
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_polymer() -> Polymer {
        Polymer {
            template: vec!['N', 'N', 'C', 'B'],
            rules: vec![
                InsertionRule('C', 'H', 'B'),
                InsertionRule('H', 'H', 'N'),
                InsertionRule('C', 'B', 'H'),
                InsertionRule('N', 'H', 'C'),
                InsertionRule('H', 'B', 'C'),
                InsertionRule('H', 'C', 'B'),
                InsertionRule('H', 'N', 'C'),
                InsertionRule('N', 'N', 'C'),
                InsertionRule('B', 'H', 'H'),
                InsertionRule('N', 'C', 'B'),
                InsertionRule('N', 'B', 'B'),
                InsertionRule('B', 'N', 'B'),
                InsertionRule('B', 'B', 'N'),
                InsertionRule('B', 'C', 'B'),
                InsertionRule('C', 'C', 'N'),
                InsertionRule('C', 'N', 'C'),
            ],
        }
    }

    fn example_polymer_after_four_steps() -> Polymer {
        Polymer {
            template: vec![
                'N', 'B', 'B', 'N', 'B', 'N', 'B', 'B', 'C', 'C', 'N', 'B', 'C', 'N', 'C', 'C',
                'N', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B', 'B', 'N', 'B', 'B', 'N', 'B', 'B',
                'C', 'B', 'H', 'C', 'B', 'H', 'H', 'N', 'H', 'C', 'B', 'B', 'C', 'B', 'H', 'C',
                'B',
            ],
            rules: vec![
                InsertionRule('C', 'H', 'B'),
                InsertionRule('H', 'H', 'N'),
                InsertionRule('C', 'B', 'H'),
                InsertionRule('N', 'H', 'C'),
                InsertionRule('H', 'B', 'C'),
                InsertionRule('H', 'C', 'B'),
                InsertionRule('H', 'N', 'C'),
                InsertionRule('N', 'N', 'C'),
                InsertionRule('B', 'H', 'H'),
                InsertionRule('N', 'C', 'B'),
                InsertionRule('N', 'B', 'B'),
                InsertionRule('B', 'N', 'B'),
                InsertionRule('B', 'B', 'N'),
                InsertionRule('B', 'C', 'B'),
                InsertionRule('C', 'C', 'N'),
                InsertionRule('C', 'N', 'C'),
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Polymer::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_polymer()),
        );
    }

    #[test]
    fn test_expand() {
        let mut polymer = example_polymer();

        polymer.expand();
        assert_eq!(polymer.template, vec!['N', 'C', 'N', 'B', 'C', 'H', 'B']);

        polymer.expand();
        assert_eq!(
            polymer.template,
            vec!['N', 'B', 'C', 'C', 'N', 'B', 'B', 'B', 'C', 'B', 'H', 'C', 'B'],
        );
    }

    #[test]
    fn test_count_chars() {
        let mut counts = BTreeMap::new();
        counts.insert('N', 2);
        counts.insert('C', 1);
        counts.insert('B', 1);
        assert_eq!(example_polymer().char_counts(), counts);

        let mut counts = BTreeMap::new();
        counts.insert('N', 11);
        counts.insert('B', 23);
        counts.insert('C', 10);
        counts.insert('H', 5);
        assert_eq!(example_polymer_after_four_steps().char_counts(), counts);
    }

    #[test]
    fn test_least_and_most_common_counts() {
        assert_eq!(
            example_polymer_after_four_steps().least_and_most_common_counts(),
            (5, 23)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
