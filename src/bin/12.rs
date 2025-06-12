use std::collections::BTreeMap;
use std::str::FromStr;

advent_of_code::solution!(12);

#[derive(Debug, PartialEq)]
struct CaveSystem {
    connections: [usize; usize::BITS as usize],
    length: usize,
    sizes: usize,
}

impl CaveSystem {
    fn connections_to(&self, start: usize, end: usize, visited: usize) -> usize {
        if start == end {
            return 1;
        }

        self.neighbours(start)
            .filter_map(|neighbour| {
                if self.is_large_cave(neighbour) || visited & (1 << neighbour) == 0 {
                    let visited = visited | (1 << neighbour);
                    Some(self.connections_to(neighbour, end, visited))
                } else {
                    None
                }
            })
            .sum()
    }

    const fn is_connected(&self, pos: usize, other: usize) -> bool {
        self.connections[pos] & (1 << other) != 0
    }

    const fn is_large_cave(&self, pos: usize) -> bool {
        self.sizes & (1 << pos) != 0
    }

    fn neighbours(&self, pos: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.length).filter(move |neighbour| self.is_connected(pos, *neighbour))
    }
}

#[derive(Debug, PartialEq)]
struct ParseCaveSystemError;

impl FromStr for CaveSystem {
    type Err = ParseCaveSystemError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut connections = [0; usize::BITS as usize];
        let mut length = 2;
        let mut sizes = 0;

        let mut keys = BTreeMap::new();
        keys.insert("start", 0);
        keys.insert("end", 1);

        for line in input.lines() {
            let Some((first, second)) = line.split_once('-') else {
                return Err(ParseCaveSystemError);
            };

            let a = *keys.entry(first).or_insert_with(|| {
                let ix = length;
                length += 1;
                ix
            });
            let b = *keys.entry(second).or_insert_with(|| {
                let ix = length;
                length += 1;
                ix
            });

            if first
                .chars()
                .next()
                .as_ref()
                .is_some_and(char::is_ascii_uppercase)
            {
                sizes |= 1 << a;
            }
            if second
                .chars()
                .next()
                .as_ref()
                .is_some_and(char::is_ascii_uppercase)
            {
                sizes |= 1 << b;
            }

            connections[a] |= 1 << b;
            connections[b] |= 1 << a;
        }

        Ok(Self {
            connections,
            length,
            sizes,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    CaveSystem::from_str(input)
        .ok()
        .map(|system| system.connections_to(0, 1, 1))
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_cave_system() -> CaveSystem {
        let mut connections = [0; usize::BITS as usize];
        connections[0] = 304;
        connections[1] = 68;
        connections[2] = 58;
        connections[3] = 884;
        connections[4] = 45;
        connections[5] = 349;
        connections[6] = 426;
        connections[7] = 64;
        connections[8] = 105;
        connections[9] = 8;

        CaveSystem {
            length: 10,
            connections,
            sizes: 784,
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            CaveSystem::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_cave_system()),
        );
    }

    #[test]
    fn test_is_large_cave() {
        let system = example_cave_system();
        assert_eq!(system.is_large_cave(0), false);
        assert_eq!(system.is_large_cave(1), false);
        assert_eq!(system.is_large_cave(2), false);
        assert_eq!(system.is_large_cave(3), false);
        assert_eq!(system.is_large_cave(4), true);
        assert_eq!(system.is_large_cave(5), false);
        assert_eq!(system.is_large_cave(6), false);
        assert_eq!(system.is_large_cave(7), false);
        assert_eq!(system.is_large_cave(8), true);
        assert_eq!(system.is_large_cave(9), true);
    }

    #[test]
    fn test_neighbours() {
        let system = example_cave_system();

        let mut start = system.neighbours(0);
        assert_eq!(start.next(), Some(4));
        assert_eq!(start.next(), Some(5));
        assert_eq!(start.next(), Some(8));
        assert_eq!(start.next(), None);

        let mut he = system.neighbours(3);
        assert_eq!(he.next(), Some(2));
        assert_eq!(he.next(), Some(4));
        assert_eq!(he.next(), Some(5));
        assert_eq!(he.next(), Some(6));
        assert_eq!(he.next(), Some(8));
        assert_eq!(he.next(), Some(9));
        assert_eq!(he.next(), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(226));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
