use std::str::FromStr;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
struct Display {
    patterns: [u8; 10],
    output: [u8; 4],
}

impl Display {
    fn unique_outputs(&self) -> usize {
        self.output
            .iter()
            .filter(|out| matches!(out.count_ones(), 2 | 3 | 4 | 7))
            .count()
    }

    fn read_output(&self) -> usize {
        let mut decoded = [0; 10];
        let mut wires = [0; 7];

        // First, identify the unique-length patterns: 1, 4, 7, and 8; along the way, count
        // how many times each wire value appears
        let mut wire_counts = [0; 7];
        for pattern in self.patterns {
            for (bit, count) in wire_counts.iter_mut().enumerate() {
                let value = 1 << bit;
                if pattern & value == value {
                    *count += 1;
                }
            }

            match pattern.count_ones() {
                2 => decoded[1] = pattern,
                3 => decoded[7] = pattern,
                4 => decoded[4] = pattern,
                7 => decoded[8] = pattern,
                _ => (),
            }
        }

        // Wire 'a' (0) is in 7 and not in 1:
        for bit in 0..7 {
            let value = 1 << bit;
            if decoded[7] & value == value && decoded[1] & value == 0 {
                wires[0] = value;
                break;
            }
        }

        // Four other wires can be identified by how often they appear
        for (bit, count) in wire_counts.iter().enumerate() {
            let value = 1 << bit;
            match count {
                4 => wires[4] = value,
                6 => wires[1] = value,
                8 => {
                    if value != wires[0] {
                        wires[2] = value;
                    }
                }
                9 => wires[5] = value,
                _ => (),
            }
        }

        // And by determing those, we can now identify four more patterns
        for pattern in self.patterns {
            let segments = pattern.count_ones();

            if segments == 5 && (pattern & wires[1] == 0) && (pattern & wires[4] == 0) {
                decoded[3] = pattern;
            }
            if segments == 5 && (pattern & wires[2] == 0) && (pattern & wires[4] == 0) {
                decoded[5] = pattern;
            }
            if segments == 6 && (pattern & wires[2] == 0) {
                decoded[6] = pattern;
            }
            if segments == 6 && (pattern & wires[4] == 0) {
                decoded[9] = pattern;
            }
        }

        // Now a process of elimination leaves 2 remaining patterns
        for pattern in self.patterns {
            let segments = pattern.count_ones();

            if segments == 5 && pattern != decoded[3] && pattern != decoded[5] {
                decoded[2] = pattern;
            }
            if segments == 6 && pattern != decoded[6] && pattern != decoded[9] {
                decoded[0] = pattern;
            }
        }

        // Use the decoded patterns to calculate the four output digits
        let mut output = [0; 4];
        for (value, pattern) in decoded.iter().enumerate() {
            for (ix, out) in output.iter_mut().enumerate() {
                if self.output[ix] == *pattern {
                    *out += value;
                }
            }
        }

        // Combine the output digits into a single number
        output.iter().fold(0, |total, digit| (total * 10) + digit)
    }
}

#[derive(Debug, PartialEq)]
struct DisplaySystem {
    displays: Vec<Display>,
}

impl DisplaySystem {
    fn total_unique_outputs(&self) -> usize {
        self.displays.iter().map(Display::unique_outputs).sum()
    }

    fn total_of_outputs(&self) -> usize {
        self.displays.iter().map(Display::read_output).sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseDisplaySystemError;

fn parse_wire_labels(text: &str) -> Result<u8, ParseDisplaySystemError> {
    text.chars()
        .map(|ch| match ch {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => 0,
        })
        .reduce(|a, b| a | b)
        .ok_or(ParseDisplaySystemError)
}

impl FromStr for Display {
    type Err = ParseDisplaySystemError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((patterns_str, output_str)) = line.trim().split_once(" | ") else {
            return Err(ParseDisplaySystemError);
        };

        let mut patterns = [0; 10];
        for (ix, pattern) in patterns_str.split_whitespace().enumerate() {
            let pattern = parse_wire_labels(pattern)?;
            patterns[ix] = pattern;
        }

        let mut output = [0; 4];
        for (ix, out) in output_str.split_whitespace().enumerate() {
            let out = parse_wire_labels(out)?;
            output[ix] = out;
        }

        Ok(Self { patterns, output })
    }
}

impl FromStr for DisplaySystem {
    type Err = ParseDisplaySystemError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut displays = Vec::new();
        for line in input.lines() {
            let display = line.parse()?;
            displays.push(display);
        }
        Ok(Self { displays })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    DisplaySystem::from_str(input)
        .ok()
        .map(|display| display.total_unique_outputs())
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    DisplaySystem::from_str(input)
        .ok()
        .map(|display| display.total_of_outputs())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_display_system() -> DisplaySystem {
        DisplaySystem {
            displays: vec![
                Display {
                    patterns: [18, 127, 126, 125, 86, 124, 123, 62, 47, 26],
                    output: [127, 62, 126, 86],
                },
                Display {
                    patterns: [123, 94, 70, 68, 127, 122, 111, 31, 126, 116],
                    output: [126, 70, 127, 68],
                },
                Display {
                    patterns: [123, 68, 31, 107, 111, 126, 79, 101, 70, 127],
                    output: [68, 68, 111, 70],
                },
                Display {
                    patterns: [126, 14, 63, 91, 39, 6, 61, 31, 125, 127],
                    output: [63, 31, 125, 6],
                },
                Display {
                    patterns: [127, 98, 96, 115, 59, 116, 87, 119, 95, 111],
                    output: [116, 127, 98, 115],
                },
                Display {
                    patterns: [115, 5, 119, 127, 125, 110, 23, 123, 103, 37],
                    output: [127, 23, 5, 127],
                },
                Display {
                    patterns: [110, 104, 127, 116, 123, 63, 62, 79, 126, 96],
                    output: [116, 62, 116, 127],
                },
                Display {
                    patterns: [126, 119, 118, 125, 79, 24, 58, 28, 127, 94],
                    output: [24, 119, 79, 118],
                },
                Display {
                    patterns: [123, 126, 92, 55, 70, 127, 68, 111, 122, 118],
                    output: [127, 70, 68, 70],
                },
                Display {
                    patterns: [103, 100, 127, 87, 96, 95, 113, 119, 47, 126],
                    output: [113, 103, 96, 87],
                },
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            DisplaySystem::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_display_system()),
        );
    }

    #[test]
    fn test_count_unique_outputs() {
        let system = example_display_system();

        assert_eq!(system.displays[0].unique_outputs(), 2);
        assert_eq!(system.displays[1].unique_outputs(), 3);
        assert_eq!(system.displays[2].unique_outputs(), 3);
        assert_eq!(system.displays[3].unique_outputs(), 1);
        assert_eq!(system.displays[4].unique_outputs(), 3);
        assert_eq!(system.displays[5].unique_outputs(), 4);
        assert_eq!(system.displays[6].unique_outputs(), 3);
        assert_eq!(system.displays[7].unique_outputs(), 1);
        assert_eq!(system.displays[8].unique_outputs(), 4);
        assert_eq!(system.displays[9].unique_outputs(), 2);

        assert_eq!(system.total_unique_outputs(), 26);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_calculate_output_total() {
        let system = example_display_system();

        assert_eq!(system.displays[0].read_output(), 8394);
        assert_eq!(system.displays[1].read_output(), 9781);
        assert_eq!(system.displays[2].read_output(), 1197);
        assert_eq!(system.displays[3].read_output(), 9361);
        assert_eq!(system.displays[4].read_output(), 4873);
        assert_eq!(system.displays[5].read_output(), 8418);
        assert_eq!(system.displays[6].read_output(), 4548);
        assert_eq!(system.displays[7].read_output(), 1625);
        assert_eq!(system.displays[8].read_output(), 8717);
        assert_eq!(system.displays[9].read_output(), 4315);

        assert_eq!(system.total_of_outputs(), 61_229);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(61_229));
    }
}
