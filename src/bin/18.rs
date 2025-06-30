use std::ops::Add;
use std::str::FromStr;

advent_of_code::solution!(18);

const SNAILFISH_SIZE: usize = 63;
const SNAILFISH_EXPLODE: usize = 15;
const SNAILFISH_TRAVERSE: [usize; SNAILFISH_SIZE] = [
    0, 1, 3, 7, 15, 31, 32, 16, 33, 34, 8, 17, 35, 36, 18, 37, 38, 4, 9, 19, 39, 40, 20, 41, 42,
    10, 21, 43, 44, 22, 45, 46, 2, 5, 11, 23, 47, 48, 24, 49, 50, 12, 25, 51, 52, 26, 53, 54, 6,
    13, 27, 55, 56, 28, 57, 58, 14, 29, 59, 60, 30, 61, 62,
];

#[derive(Clone, Copy, Debug, PartialEq)]
enum Node {
    Unused,
    Branch,
    Leaf(u32),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SnailfishNumber {
    nodes: [Node; SNAILFISH_SIZE],
}

impl SnailfishNumber {
    const fn new() -> Self {
        Self {
            nodes: [Node::Unused; SNAILFISH_SIZE],
        }
    }

    fn copy_branch(&mut self, source: &Self, from: usize, to: usize) {
        self.nodes[to] = source.nodes[from];
        if source.nodes[from] == Node::Branch {
            self.copy_branch(source, (from * 2) + 1, (to * 2) + 1);
            self.copy_branch(source, (from * 2) + 2, (to * 2) + 2);
        }
    }

    fn magnitude(&self, position: usize) -> u32 {
        match self.nodes[position] {
            Node::Unused => 0,
            Node::Leaf(value) => value,
            Node::Branch => {
                let left = self.magnitude((position * 2) + 1);
                let right = self.magnitude((position * 2) + 2);
                (left * 3) + (right * 2)
            }
        }
    }

    fn next_number_to_left(&self, from: usize) -> Option<(usize, u32)> {
        SNAILFISH_TRAVERSE
            .into_iter()
            .rev()
            .filter_map(|position| {
                if let Node::Leaf(value) = self.nodes[position] {
                    Some((position, value))
                } else {
                    None
                }
            })
            .skip_while(|(position, _value)| *position != from)
            .nth(1)
    }

    fn next_number_to_right(&self, from: usize) -> Option<(usize, u32)> {
        SNAILFISH_TRAVERSE
            .into_iter()
            .filter_map(|position| {
                if let Node::Leaf(value) = self.nodes[position] {
                    Some((position, value))
                } else {
                    None
                }
            })
            .skip_while(|(position, _value)| *position != from)
            .nth(1)
    }

    fn reduce(&mut self) {
        loop {
            if let Some(position) = (SNAILFISH_EXPLODE..SNAILFISH_SIZE)
                .find(|position| self.nodes[*position] == Node::Branch)
            {
                let left = (position * 2) + 1;
                let Node::Leaf(left_value) = self.nodes[left] else {
                    unreachable!();
                };
                let right = (position * 2) + 2;
                let Node::Leaf(right_value) = self.nodes[right] else {
                    unreachable!();
                };

                if let Some((next_left, current)) = self.next_number_to_left(left) {
                    let value = current + left_value;
                    self.nodes[next_left] = Node::Leaf(value);
                }
                if let Some((next_right, current)) = self.next_number_to_right(right) {
                    let value = current + right_value;
                    self.nodes[next_right] = Node::Leaf(value);
                }

                self.nodes[position] = Node::Leaf(0);
                self.nodes[left] = Node::Unused;
                self.nodes[right] = Node::Unused;

                continue;
            }

            if let Some(position) =
                SNAILFISH_TRAVERSE
                    .into_iter()
                    .find(|position| match self.nodes[*position] {
                        Node::Branch | Node::Unused => false,
                        Node::Leaf(value) => value >= 10,
                    })
            {
                let Node::Leaf(value) = self.nodes[position] else {
                    unreachable!();
                };
                let half = value / 2;
                self.nodes[position] = Node::Branch;
                self.nodes[(position * 2) + 1] = Node::Leaf(half);
                self.nodes[(position * 2) + 2] = Node::Leaf(value - half);

                continue;
            }

            break;
        }
    }
}

impl Add<Self> for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output = Self::new();
        output.nodes[0] = Node::Branch;
        output.copy_branch(&self, 0, 1);
        output.copy_branch(&rhs, 0, 2);
        output.reduce();
        output
    }
}

#[derive(Debug, PartialEq)]
struct ParseSnailfishNumberError;

impl FromStr for SnailfishNumber {
    type Err = ParseSnailfishNumberError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        let mut position = 0;
        let mut value = None;

        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                value = Some((value.unwrap_or(0) * 10) + digit);
                continue;
            }

            match ch {
                '[' => {
                    nodes[position] = Node::Branch;
                    position = (position * 2) + 1;
                }
                ']' => {
                    if let Some(leaf) = value {
                        nodes[position] = Node::Leaf(leaf);
                    }
                    value = None;
                    position = (position - 1) / 2;
                }
                ',' => {
                    if let Some(leaf) = value {
                        nodes[position] = Node::Leaf(leaf);
                    }
                    value = None;
                    position += 1;
                }
                _ => return Err(ParseSnailfishNumberError),
            }
        }

        Ok(Self { nodes })
    }
}

impl SnailfishNumber {
    fn read_from_input(input: &str) -> Result<Vec<Self>, ParseSnailfishNumberError> {
        let mut numbers = Vec::new();

        for line in input.lines() {
            let number = Self::from_str(line)?;
            numbers.push(number);
        }

        Ok(numbers)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let numbers = SnailfishNumber::read_from_input(input).ok()?;
    numbers
        .into_iter()
        .reduce(|acc, number| acc + number)
        .map(|number| number.magnitude(0))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let numbers = SnailfishNumber::read_from_input(input).ok()?;

    let mut best = None;
    for a in 0..numbers.len() {
        for b in 0..numbers.len() {
            if a == b {
                continue;
            }

            let added = numbers[a] + numbers[b];
            let magnitude = added.magnitude(0);
            best = best.map_or(Some(magnitude), |current| Some(magnitude.max(current)));
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_snailfish_numbers() -> Vec<SnailfishNumber> {
        let mut numbers = Vec::new();

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Branch;
        nodes[4] = Node::Branch;
        nodes[5] = Node::Branch;
        nodes[6] = Node::Branch;
        nodes[7] = Node::Leaf(0);
        nodes[8] = Node::Branch;
        nodes[9] = Node::Leaf(0);
        nodes[10] = Node::Leaf(0);
        nodes[11] = Node::Branch;
        nodes[12] = Node::Branch;
        nodes[13] = Node::Leaf(9);
        nodes[14] = Node::Leaf(5);
        nodes[17] = Node::Leaf(4);
        nodes[18] = Node::Leaf(5);
        nodes[23] = Node::Leaf(4);
        nodes[24] = Node::Leaf(5);
        nodes[25] = Node::Leaf(2);
        nodes[26] = Node::Leaf(6);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(7);
        nodes[2] = Node::Branch;
        nodes[5] = Node::Branch;
        nodes[6] = Node::Branch;
        nodes[11] = Node::Branch;
        nodes[12] = Node::Branch;
        nodes[13] = Node::Branch;
        nodes[14] = Node::Branch;
        nodes[23] = Node::Leaf(3);
        nodes[24] = Node::Leaf(7);
        nodes[25] = Node::Leaf(4);
        nodes[26] = Node::Leaf(3);
        nodes[27] = Node::Leaf(6);
        nodes[28] = Node::Leaf(3);
        nodes[29] = Node::Leaf(8);
        nodes[30] = Node::Leaf(8);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Leaf(2);
        nodes[4] = Node::Branch;
        nodes[5] = Node::Branch;
        nodes[6] = Node::Branch;
        nodes[9] = Node::Branch;
        nodes[10] = Node::Branch;
        nodes[11] = Node::Branch;
        nodes[12] = Node::Leaf(1);
        nodes[13] = Node::Leaf(7);
        nodes[14] = Node::Branch;
        nodes[19] = Node::Leaf(0);
        nodes[20] = Node::Leaf(8);
        nodes[21] = Node::Leaf(3);
        nodes[22] = Node::Leaf(4);
        nodes[23] = Node::Leaf(6);
        nodes[24] = Node::Leaf(7);
        nodes[29] = Node::Leaf(1);
        nodes[30] = Node::Leaf(6);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Branch;
        nodes[4] = Node::Branch;
        nodes[5] = Node::Branch;
        nodes[6] = Node::Branch;
        nodes[7] = Node::Branch;
        nodes[8] = Node::Leaf(7);
        nodes[9] = Node::Leaf(6);
        nodes[10] = Node::Branch;
        nodes[11] = Node::Branch;
        nodes[12] = Node::Branch;
        nodes[13] = Node::Branch;
        nodes[14] = Node::Branch;
        nodes[15] = Node::Leaf(2);
        nodes[16] = Node::Leaf(4);
        nodes[21] = Node::Leaf(0);
        nodes[22] = Node::Leaf(5);
        nodes[23] = Node::Leaf(6);
        nodes[24] = Node::Leaf(8);
        nodes[25] = Node::Leaf(2);
        nodes[26] = Node::Leaf(8);
        nodes[27] = Node::Leaf(2);
        nodes[28] = Node::Leaf(1);
        nodes[29] = Node::Leaf(4);
        nodes[30] = Node::Leaf(5);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(7);
        nodes[2] = Node::Branch;
        nodes[5] = Node::Leaf(5);
        nodes[6] = Node::Branch;
        nodes[13] = Node::Branch;
        nodes[14] = Node::Branch;
        nodes[27] = Node::Leaf(3);
        nodes[28] = Node::Leaf(8);
        nodes[29] = Node::Leaf(1);
        nodes[30] = Node::Leaf(4);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Leaf(2);
        nodes[4] = Node::Branch;
        nodes[5] = Node::Leaf(8);
        nodes[6] = Node::Branch;
        nodes[9] = Node::Leaf(2);
        nodes[10] = Node::Leaf(2);
        nodes[13] = Node::Leaf(8);
        nodes[14] = Node::Leaf(1);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(2);
        nodes[2] = Node::Leaf(9);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(1);
        nodes[2] = Node::Branch;
        nodes[5] = Node::Branch;
        nodes[6] = Node::Branch;
        nodes[11] = Node::Branch;
        nodes[12] = Node::Leaf(9);
        nodes[13] = Node::Branch;
        nodes[14] = Node::Branch;
        nodes[23] = Node::Leaf(9);
        nodes[24] = Node::Leaf(3);
        nodes[27] = Node::Leaf(9);
        nodes[28] = Node::Leaf(0);
        nodes[29] = Node::Leaf(0);
        nodes[30] = Node::Leaf(7);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Leaf(1);
        nodes[3] = Node::Branch;
        nodes[4] = Node::Leaf(7);
        nodes[7] = Node::Leaf(5);
        nodes[8] = Node::Branch;
        nodes[17] = Node::Leaf(7);
        nodes[18] = Node::Leaf(4);
        numbers.push(SnailfishNumber { nodes });

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Branch;
        nodes[4] = Node::Leaf(6);
        nodes[5] = Node::Leaf(8);
        nodes[6] = Node::Leaf(7);
        nodes[7] = Node::Branch;
        nodes[8] = Node::Leaf(2);
        nodes[15] = Node::Leaf(4);
        nodes[16] = Node::Leaf(2);
        numbers.push(SnailfishNumber { nodes });

        numbers
    }

    #[test]
    fn test_read_snailfish_numbers() {
        assert_eq!(
            SnailfishNumber::read_from_input(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_snailfish_numbers()),
        );
    }

    #[test]
    fn test_simple_addition() {
        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(1);
        nodes[2] = Node::Leaf(2);
        let left = SnailfishNumber { nodes };

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(3);
        nodes[2] = Node::Leaf(4);
        let right = SnailfishNumber { nodes };

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Leaf(1);
        nodes[4] = Node::Leaf(2);
        nodes[5] = Node::Leaf(3);
        nodes[6] = Node::Leaf(4);
        let expected = SnailfishNumber { nodes };

        assert_eq!(left + right, expected);
    }

    #[test]
    fn test_next_number_to_left() {
        let numbers = example_snailfish_numbers();
        assert_eq!(numbers[0].next_number_to_left(7), None);
        assert_eq!(numbers[0].next_number_to_left(17), Some((7, 0)));
        assert_eq!(numbers[1].next_number_to_left(27), Some((26, 3)));
    }

    #[test]
    fn test_next_number_to_right() {
        let numbers = example_snailfish_numbers();
        assert_eq!(numbers[2].next_number_to_right(19), Some((20, 8)));
        assert_eq!(numbers[2].next_number_to_right(30), None);
    }

    #[test]
    fn test_addition_with_reduction() {
        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Branch;
        nodes[4] = Node::Leaf(4);
        nodes[5] = Node::Leaf(7);
        nodes[6] = Node::Branch;
        nodes[7] = Node::Branch;
        nodes[8] = Node::Leaf(4);
        nodes[13] = Node::Branch;
        nodes[14] = Node::Leaf(9);
        nodes[15] = Node::Leaf(4);
        nodes[16] = Node::Leaf(3);
        nodes[27] = Node::Leaf(8);
        nodes[28] = Node::Leaf(4);
        let lhs = SnailfishNumber { nodes };

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Leaf(1);
        nodes[2] = Node::Leaf(1);
        let rhs = SnailfishNumber { nodes };

        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Branch;
        nodes[4] = Node::Branch;
        nodes[5] = Node::Leaf(8);
        nodes[6] = Node::Leaf(1);
        nodes[7] = Node::Branch;
        nodes[8] = Node::Leaf(4);
        nodes[9] = Node::Branch;
        nodes[10] = Node::Branch;
        nodes[15] = Node::Leaf(0);
        nodes[16] = Node::Leaf(7);
        nodes[19] = Node::Leaf(7);
        nodes[20] = Node::Leaf(8);
        nodes[21] = Node::Leaf(6);
        nodes[22] = Node::Leaf(0);
        let expected = SnailfishNumber { nodes };

        assert_eq!(lhs + rhs, expected);
    }

    #[test]
    fn test_magnitude() {
        let mut nodes = [Node::Unused; SNAILFISH_SIZE];
        nodes[0] = Node::Branch;
        nodes[1] = Node::Branch;
        nodes[2] = Node::Branch;
        nodes[3] = Node::Leaf(9);
        nodes[4] = Node::Leaf(1);
        nodes[5] = Node::Leaf(1);
        nodes[6] = Node::Leaf(9);
        let number = SnailfishNumber { nodes };
        assert_eq!(number.magnitude(1), 29);
        assert_eq!(number.magnitude(2), 21);
        assert_eq!(number.magnitude(0), 129);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3488));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3805));
    }
}
