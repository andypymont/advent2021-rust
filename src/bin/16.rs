use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(16);

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal {
        version: usize,
        value: usize,
    },
    Operator {
        version: usize,
        operation: Operation,
        subpackets: Vec<Packet>,
    },
}

impl Packet {
    fn total_of_version_numbers(&self) -> usize {
        match self {
            Self::Literal { version, value: _ } => *version,
            Self::Operator {
                version,
                operation: _,
                subpackets,
            } => {
                let inner: usize = subpackets.iter().map(Self::total_of_version_numbers).sum();
                version + inner
            }
        }
    }

    fn get_value(&self) -> usize {
        match self {
            Self::Literal { version: _, value } => *value,
            Self::Operator {
                version: _,
                operation,
                subpackets,
            } => {
                let mut sp = subpackets.iter().map(Self::get_value);
                match operation {
                    Operation::Sum => sp.sum(),
                    Operation::Product => sp.product(),
                    Operation::Minimum => sp.min().unwrap_or(0),
                    Operation::Maximum => sp.max().unwrap_or(0),
                    Operation::GreaterThan => {
                        usize::from(sp.next().unwrap_or(0) > sp.next().unwrap_or(0))
                    }
                    Operation::LessThan => {
                        usize::from(sp.next().unwrap_or(0) < sp.next().unwrap_or(0))
                    }
                    Operation::EqualTo => {
                        usize::from(sp.next().unwrap_or(0) == sp.next().unwrap_or(0))
                    }
                }
            }
        }
    }
}

struct PacketParser {
    bits: VecDeque<bool>,
}

impl PacketParser {
    fn new(input: &str) -> Self {
        let mut bits = VecDeque::new();

        for ch in input.chars() {
            let value = ch.to_digit(16).and_then(|x| x.try_into().ok()).unwrap_or(0);
            bits.extend((0..4).rev().map(|pos| (value >> pos) & 1 == 1));
        }

        Self { bits }
    }

    fn read_bool(&mut self) -> Option<bool> {
        self.bits.pop_front()
    }

    fn read_value(&mut self, length: usize) -> Option<usize> {
        let mut value = 0;
        for _ in 0..length {
            let bit = self.bits.pop_front().map(usize::from)?;
            value = (value << 1) | bit;
        }
        Some(value)
    }

    fn read_literal_segment(&mut self) -> Option<(bool, usize)> {
        let isnt_last = self.read_bool()?;
        let value = self.read_value(4)?;
        Some((isnt_last, value))
    }

    fn read_packet(&mut self) -> Option<(Packet, usize)> {
        let start = self.bits.len();

        let version = self.read_value(3)?;
        let type_id = self.read_value(3)?;

        if type_id == 4 {
            let mut value = 0;

            while let Some((isnt_last, segment)) = self.read_literal_segment() {
                value = (value << 4) | segment;
                if !isnt_last {
                    break;
                }
            }

            let packet = Packet::Literal { version, value };
            let length = start - self.bits.len();
            return Some((packet, length));
        }

        let mut subpackets = Vec::new();

        let length_type_id = self.read_value(1)?;
        if length_type_id == 0 {
            let mut subpackets_length = self.read_value(15)?;
            while subpackets_length > 0 {
                let (subpacket, length) = self.read_packet()?;
                subpackets_length -= length;
                subpackets.push(subpacket);
            }
        } else {
            let subpacket_count = self.read_value(11)?;
            for _ in 0..subpacket_count {
                let (subpacket, _) = self.read_packet()?;
                subpackets.push(subpacket);
            }
        }

        let operation = match type_id {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            _ => Operation::EqualTo,
        };

        let packet = Packet::Operator {
            version,
            operation,
            subpackets,
        };
        let length = start - self.bits.len();
        Some((packet, length))
    }
}

#[derive(Debug, PartialEq)]
struct ParsePacketError;

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parser = PacketParser::new(input);
        parser.read_packet().map(|(p, _)| p).ok_or(ParsePacketError)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Packet::from_str(input)
        .ok()
        .as_ref()
        .map(Packet::total_of_version_numbers)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Packet::from_str(input).ok().as_ref().map(Packet::get_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_packet() -> Packet {
        Packet::Operator {
            version: 6,
            operation: Operation::Maximum,
            subpackets: vec![
                Packet::Literal {
                    version: 0,
                    value: 7,
                },
                Packet::Literal {
                    version: 5,
                    value: 8,
                },
                Packet::Literal {
                    version: 0,
                    value: 9,
                },
            ],
        }
    }

    #[test]
    fn test_packet_from_str() {
        assert_eq!(
            Packet::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_packet()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
