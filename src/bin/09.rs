use std::collections::VecDeque;
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
            }
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
            }
            Self::West => col.checked_sub(1),
            Self::North | Self::South => Some(col),
        }?;

        Some((row * GRID_SIZE) + col)
    }
}

#[derive(Debug, PartialEq)]
struct LowPoint {
    position: usize,
    height: u32,
}

#[derive(Debug, PartialEq)]
struct CaveMap {
    heights: Vec<u32>,
}

impl CaveMap {
    fn basin_size(&self, low_point: LowPoint) -> u32 {
        let mut visited = [false; GRID_SIZE * GRID_SIZE];
        let mut queue = VecDeque::new();
        queue.push_back(low_point);

        while let Some(point) = queue.pop_front() {
            visited[point.position] = true;

            for neighbour in Self::neighbours(point.position) {
                let Some(height) = self.heights.get(neighbour) else {
                    continue;
                };
                if !visited[neighbour] && *height < 9 && *height > point.height {
                    queue.push_back(LowPoint {
                        position: neighbour,
                        height: *height,
                    });
                }
            }
        }

        visited.iter().map(|v| u32::from(*v)).sum()
    }

    fn neighbours(origin: usize) -> impl Iterator<Item = usize> {
        COMPASS.iter().filter_map(move |dir| dir.move_from(origin))
    }

    fn find_low_points(&self) -> impl Iterator<Item = LowPoint> + '_ {
        self.heights
            .iter()
            .enumerate()
            .filter_map(|(point, height)| {
                if Self::neighbours(point)
                    .filter_map(|pt| self.heights.get(pt))
                    .all(|neighbour| neighbour > height)
                {
                    Some(LowPoint {
                        position: point,
                        height: *height,
                    })
                } else {
                    None
                }
            })
    }

    fn three_largest_basins(&self) -> (u32, u32, u32) {
        self.find_low_points()
            .map(|pt| self.basin_size(pt))
            .fold((0, 0, 0), |(a, b, c), d| {
                if d >= a {
                    return (d, a, b);
                }
                if d >= b {
                    return (a, d, b);
                }
                if d >= c {
                    return (a, b, d);
                }
                (a, b, c)
            })
    }

    fn total_low_point_risk(&self) -> u32 {
        self.find_low_points().map(|lp| lp.height + 1).sum()
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
    CaveMap::from_str(input)
        .ok()
        .map(|cave| cave.total_low_point_risk())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    CaveMap::from_str(input).ok().map(|cave| {
        let (a, b, c) = cave.three_largest_basins();
        a * b * c
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_cave_map() -> CaveMap {
        CaveMap {
            heights: vec![
                2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8,
                9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
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
        assert_eq!(
            low.next(),
            Some(LowPoint {
                position: position(0, 1),
                height: 1
            })
        );
        assert_eq!(
            low.next(),
            Some(LowPoint {
                position: position(0, 9),
                height: 0
            })
        );
        assert_eq!(
            low.next(),
            Some(LowPoint {
                position: position(2, 2),
                height: 5
            })
        );
        assert_eq!(
            low.next(),
            Some(LowPoint {
                position: position(4, 6),
                height: 5
            })
        );
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
    fn test_basin_size() {
        let map = example_cave_map();
        assert_eq!(
            map.basin_size(LowPoint {
                position: position(0, 1),
                height: 1
            }),
            3
        );
        assert_eq!(
            map.basin_size(LowPoint {
                position: position(0, 9),
                height: 0
            }),
            9
        );
        assert_eq!(
            map.basin_size(LowPoint {
                position: position(2, 2),
                height: 5
            }),
            14
        );
        assert_eq!(
            map.basin_size(LowPoint {
                position: position(4, 6),
                height: 5
            }),
            9
        );
    }

    #[test]
    fn test_three_largest_basins() {
        let map = example_cave_map();
        assert_eq!(map.three_largest_basins(), (14, 9, 9));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1134));
    }
}
