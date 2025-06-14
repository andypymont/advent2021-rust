use std::str::FromStr;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq)]
struct Polymer {
    template: Vec<usize>,
    rules: [Option<usize>; 26 * 26],
}

impl Polymer {
    fn expand(&mut self) {
        let mut after = Vec::new();

        let mut elements = self.template.iter();

        let Some(mut left) = elements.next() else {
            return;
        };

        for right in elements {
            after.push(*left);
            if let Some(insert) = self.rules.get((left * 26) + right).unwrap_or(&None) {
                after.push(*insert);
            }
            left = right;
        }
        after.push(*left);
        self.template = after;
    }

    fn least_and_most_common_counts(&self) -> (usize, usize) {
        let mut counts = [0; 26 * 26];
        for element in &self.template {
            counts[*element] += 1;
        }

        let mut least = usize::MAX;
        let mut most = 0;
        for count in counts {
            if count > 0 && count < least {
                least = count;
            }
            most = most.max(count);
        }

        (least, most)
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

impl FromStr for Polymer {
    type Err = ParsePolymerError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (template_str, rules_str) = input.split_once("\n\n").ok_or(ParsePolymerError)?;

        let mut template = Vec::new();
        for ch in template_str.chars() {
            let element = parse_element(ch)?;
            template.push(element);
        }

        let mut rules = [None; 26 * 26];
        for line in rules_str.lines() {
            let (input, output) = line.split_once(" -> ").ok_or(ParsePolymerError)?;

            let mut input = input.chars();

            let left = input.next().ok_or(ParsePolymerError)?;
            let left = parse_element(left)?;

            let right = input.next().ok_or(ParsePolymerError)?;
            let right = parse_element(right)?;

            let output = output.chars().next().ok_or(ParsePolymerError)?;
            let output = parse_element(output)?;

            rules[(left * 26) + right] = Some(output);
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
        let b = 1;
        let c = 2;
        let h = 7;
        let n = 13;

        let mut rules = [None; 26 * 26];
        rules[(c * 26) + h] = Some(b);
        rules[(h * 26) + h] = Some(n);
        rules[(c * 26) + b] = Some(h);
        rules[(n * 26) + h] = Some(c);
        rules[(h * 26) + b] = Some(c);
        rules[(h * 26) + c] = Some(b);
        rules[(h * 26) + n] = Some(c);
        rules[(n * 26) + n] = Some(c);
        rules[(b * 26) + h] = Some(h);
        rules[(n * 26) + c] = Some(b);
        rules[(n * 26) + b] = Some(b);
        rules[(b * 26) + n] = Some(b);
        rules[(b * 26) + b] = Some(n);
        rules[(b * 26) + c] = Some(b);
        rules[(c * 26) + c] = Some(n);
        rules[(c * 26) + n] = Some(c);

        Polymer {
            template: vec![n, n, c, b],
            rules,
        }
    }

    fn example_polymer_after_four_steps() -> Polymer {
        let b = 1;
        let c = 2;
        let h = 7;
        let n = 13;

        let mut rules = [None; 26 * 26];
        rules[(c * 26) + h] = Some(b);
        rules[(h * 26) + h] = Some(n);
        rules[(c * 26) + b] = Some(h);
        rules[(n * 26) + h] = Some(c);
        rules[(h * 26) + b] = Some(c);
        rules[(h * 26) + c] = Some(b);
        rules[(h * 26) + n] = Some(c);
        rules[(n * 26) + n] = Some(c);
        rules[(b * 26) + h] = Some(h);
        rules[(n * 26) + c] = Some(b);
        rules[(n * 26) + b] = Some(b);
        rules[(b * 26) + n] = Some(b);
        rules[(b * 26) + b] = Some(n);
        rules[(b * 26) + c] = Some(b);
        rules[(c * 26) + c] = Some(n);
        rules[(c * 26) + n] = Some(c);

        Polymer {
            template: vec![
                n, b, b, n, b, n, b, b, c, c, n, b, c, n, c, c, n, b, b, n, b, b, n, b, b, b, n, b,
                b, n, b, b, c, b, h, c, b, h, h, n, h, c, b, b, c, b, h, c, b,
            ],
            rules,
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
        let b = 1;
        let c = 2;
        let h = 7;
        let n = 13;

        let mut polymer = example_polymer();

        polymer.expand();
        assert_eq!(polymer.template, vec![n, c, n, b, c, h, b]);

        polymer.expand();
        assert_eq!(
            polymer.template,
            vec![n, b, c, c, n, b, b, b, c, b, h, c, b]
        );
    }

    #[test]
    fn test_least_and_most_common_counts() {
        assert_eq!(example_polymer().least_and_most_common_counts(), (1, 2),);
        assert_eq!(
            example_polymer_after_four_steps().least_and_most_common_counts(),
            (5, 23),
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
