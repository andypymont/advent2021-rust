use std::str::FromStr;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq)]
struct InsertionRule(usize, usize, usize);

#[derive(Debug, PartialEq)]
struct Polymer {
    counts: [usize; 26],
    pairs: [usize; 26 * 26],
    rules: Vec<InsertionRule>,
}

impl Polymer {
    fn expand(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut counts = self.counts;
            let mut pairs = self.pairs;

            for rule in &self.rules {
                let pair = (rule.0 * 26) + rule.1;
                let count = self.pairs[pair];

                pairs[pair] -= count;
                pairs[(rule.0 * 26) + rule.2] += count;
                pairs[(rule.2 * 26) + rule.1] += count;
                counts[rule.2] += count;
            }

            self.counts = counts;
            self.pairs = pairs;
        }
    }

    fn check_sum(&self) -> Option<usize> {
        let mut counts = self.counts.iter().filter(|c| c > &&0);

        let mut min = counts.next()?;
        let mut max = min;

        for count in counts {
            min = min.min(count);
            max = max.max(count);
        }

        Some(max - min)
    }
}

#[derive(Debug, PartialEq)]
struct ParsePolymerError;

fn parse_element(character: char) -> Result<usize, ParsePolymerError> {
    let lower = character.to_ascii_lowercase();
    let digit = lower
        .to_digit(36)
        .map(|d| d - 10)
        .ok_or(ParsePolymerError)?;
    digit.try_into().map_err(|_| ParsePolymerError)
}

impl FromStr for InsertionRule {
    type Err = ParsePolymerError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (input, output) = line.split_once(" -> ").ok_or(ParsePolymerError)?;

        let mut input = input.chars();
        let left = input.next().map_or(Err(ParsePolymerError), parse_element)?;
        let right = input.next().map_or(Err(ParsePolymerError), parse_element)?;

        let output = output
            .chars()
            .next()
            .map_or(Err(ParsePolymerError), parse_element)?;

        Ok(Self(left, right, output))
    }
}

impl FromStr for Polymer {
    type Err = ParsePolymerError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (template_str, rules_str) = input.split_once("\n\n").ok_or(ParsePolymerError)?;

        let mut counts = [0; 26];
        let mut pairs = [0; 26 * 26];
        let mut prev = None;
        for ch in template_str.chars() {
            let element = parse_element(ch)?;
            counts[element] += 1;

            if let Some(prev) = prev {
                pairs[(prev * 26) + element] += 1;
            }

            prev = Some(element);
        }

        let mut rules = Vec::new();
        for line in rules_str.lines() {
            let rule = InsertionRule::from_str(line)?;
            rules.push(rule);
        }

        Ok(Self {
            counts,
            pairs,
            rules,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Polymer::from_str(input).ok().and_then(|mut polymer| {
        polymer.expand(10);
        polymer.check_sum()
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Polymer::from_str(input).ok().and_then(|mut polymer| {
        polymer.expand(40);
        polymer.check_sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const B: usize = 1;
    const C: usize = 2;
    const H: usize = 7;
    const N: usize = 13;

    fn example_rules() -> Vec<InsertionRule> {
        vec![
            InsertionRule(C, H, B),
            InsertionRule(H, H, N),
            InsertionRule(C, B, H),
            InsertionRule(N, H, C),
            InsertionRule(H, B, C),
            InsertionRule(H, C, B),
            InsertionRule(H, N, C),
            InsertionRule(N, N, C),
            InsertionRule(B, H, H),
            InsertionRule(N, C, B),
            InsertionRule(N, B, B),
            InsertionRule(B, N, B),
            InsertionRule(B, B, N),
            InsertionRule(B, C, B),
            InsertionRule(C, C, N),
            InsertionRule(C, N, C),
        ]
    }

    fn example_polymer() -> Polymer {
        let mut counts = [0; 26];
        counts[N] = 2;
        counts[C] = 1;
        counts[B] = 1;

        let mut pairs = [0; 26 * 26];
        pairs[(N * 26) + N] = 1;
        pairs[(N * 26) + C] = 1;
        pairs[(C * 26) + B] = 1;

        Polymer {
            counts,
            pairs,
            rules: example_rules(),
        }
    }

    fn example_polymer_after_four_steps() -> Polymer {
        let mut counts = [0; 26];
        counts[N] = 11;
        counts[B] = 23;
        counts[C] = 10;
        counts[H] = 5;

        let mut pairs = [0; 26 * 26];
        pairs[(B * 26) + B] = 9;
        pairs[(B * 26) + C] = 4;
        pairs[(B * 26) + H] = 3;
        pairs[(B * 26) + N] = 6;
        pairs[(C * 26) + B] = 5;
        pairs[(C * 26) + C] = 2;
        pairs[(C * 26) + N] = 3;
        pairs[(H * 26) + C] = 3;
        pairs[(H * 26) + H] = 1;
        pairs[(H * 26) + N] = 1;
        pairs[(N * 26) + B] = 9;
        pairs[(N * 26) + C] = 1;
        pairs[(N * 26) + H] = 1;

        Polymer {
            counts,
            pairs,
            rules: example_rules(),
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
        polymer.expand(4);
        assert_eq!(polymer, example_polymer_after_four_steps());
    }

    #[test]
    fn test_check_sum() {
        assert_eq!(example_polymer().check_sum(), Some(1));
        assert_eq!(example_polymer_after_four_steps().check_sum(), Some(18));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2_188_189_693_529));
    }
}
