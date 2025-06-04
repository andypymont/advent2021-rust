use std::str::FromStr;

advent_of_code::solution!(9);

const GRID_SIZE: usize = if cfg!(test) { 10 } else { 100 };

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const COMPASS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Direction {
    fn move_from(&self, origin: usize) -> Option<usize> {
        let row = origin / GRID_SIZE;
        let col = origin % GRID_SIZE;

        let row = match self {
            Self::North => row.checked_sub(1),
            Self::South => {
                let row = row + 1;
                if row < GRID_SIZE {
                    Some(row)
                } else {
                    None
                }
            },
            Self::East | Self::West => Some(row),
        }?;
        let col = match self {
            Self::East => {
                let col = col + 1;
                if col < GRID_SIZE {
                    Some(col)
                } else {
                    None
                }
            },
            Self::West => col.checked_sub(1),
            Self::North | Self::South => Some(col),
        }?;

        Some((row * GRID_SIZE) + col)
    }
}

#[derive(Debug, PartialEq)]
struct CaveMap {
    heights: Vec<u32>,
}

impl CaveMap {
    fn neighbours(origin: usize) -> impl Iterator<Item = usize> {
        COMPASS.iter().filter_map(move |dir| dir.move_from(origin))
    }

    fn find_low_points(&self) -> impl Iterator<Item = u32> + '_ {
        self.heights.iter().enumerate().filter_map(|(point, height)| {
            if Self::neighbours(point)
                   .filter_map(|pt| self.heights.get(pt))
                   .all(|neighbour| neighbour > height) {
                Some(height).copied()
            } else {
                None
            }
        })
    }

    fn total_low_point_risk(&self) -> u32 {
        self.find_low_points().map(|lp| lp + 1).sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseCaveMapError;

impl FromStr for CaveMap {
    type Err = ParseCaveMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();

        for line in input.lines() {
            for height in line.chars() {
                let height = height.to_digit(10).ok_or(ParseCaveMapError)?;
                heights.push(height);
            }
        }

        Ok(Self { heights })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    CaveMap::from_str(input).ok().map(|cave| cave.total_low_point_risk())
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_cave_map() -> CaveMap {
        CaveMap {
            heights: vec![
                2, 1, 9, 9, 9, 4, 3, 2, 1, 0,
                3, 9, 8, 7, 8, 9, 4, 9, 2, 1,
                9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
                8, 7, 6, 7, 8, 9, 6, 7, 8, 9,
                9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            CaveMap::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_cave_map()),
        );
    }

    fn position(row: usize, col: usize) -> usize {
        (GRID_SIZE * row) + col
    }

    #[test]
    fn test_neighbours() {
        let mut neighbours = CaveMap::neighbours(position(1, 1));
        assert_eq!(neighbours.next(), Some(position(0, 1)));
        assert_eq!(neighbours.next(), Some(position(1, 2)));
        assert_eq!(neighbours.next(), Some(position(2, 1)));
        assert_eq!(neighbours.next(), Some(position(1, 0)));
        assert_eq!(neighbours.next(), None);

        let mut neighbours = CaveMap::neighbours(position(0, 1));
        assert_eq!(neighbours.next(), Some(position(0, 2)));
        assert_eq!(neighbours.next(), Some(position(1, 1)));
        assert_eq!(neighbours.next(), Some(position(0, 0)));
    }

    #[test]
    fn test_find_low_points() {
        let map = example_cave_map();
        let mut low = map.find_low_points();
        assert_eq!(low.next(), Some(1));
        assert_eq!(low.next(), Some(0));
        assert_eq!(low.next(), Some(5));
        assert_eq!(low.next(), Some(5));
        assert_eq!(low.next(), None);
    }

    #[test]
    fn test_find_total_low_point_risk() {
        assert_eq!(example_cave_map().total_low_point_risk(), 15);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
