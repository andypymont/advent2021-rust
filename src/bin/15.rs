use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(15);

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Grid {
    size: usize,
    spaces: Vec<usize>,
}

impl Grid {
    const fn step(&self, position: usize, direction: Direction) -> Option<usize> {
        let row = position / self.size;
        let col = position % self.size;

        match direction {
            Direction::North => position.checked_sub(self.size),
            Direction::South => {
                if (row + 1) >= self.size {
                    None
                } else {
                    Some(position + self.size)
                }
            }
            Direction::East => {
                if (col + 1) >= self.size {
                    None
                } else {
                    Some(position + 1)
                }
            }
            Direction::West => {
                if col == 0 {
                    None
                } else {
                    Some(position - 1)
                }
            }
        }
    }

    fn neighbours(&self, position: usize) -> impl Iterator<Item = usize> + '_ {
        COMPASS
            .iter()
            .filter_map(move |dir| self.step(position, *dir))
    }

    fn lowest_risk_path(&self) -> usize {
        let mut best = vec![usize::MAX; self.size * self.size];

        let mut queue = BTreeSet::new();
        queue.insert((0, 0));

        while let Some((risk, position)) = queue.pop_first() {
            if risk >= best[position] {
                continue;
            }

            best[position] = risk;

            for neighbour in self.neighbours(position) {
                let risk = risk + self.spaces[neighbour];
                if risk < best[neighbour] {
                    queue.insert((risk, neighbour));
                }
            }
        }

        *best.last().unwrap_or(&0)
    }

    fn expand(&self) -> Self {
        let size = self.size * 5;
        let mut spaces = vec![0; size * size];

        for (position, risk) in self.spaces.iter().enumerate() {
            let x = position % self.size;
            let y = position / self.size;

            for segment_x in 0..5 {
                for segment_y in 0..5 {
                    let risk = {
                        let candidate = risk + segment_x + segment_y;
                        if candidate > 9 {
                            candidate - 9
                        } else {
                            candidate
                        }
                    };

                    let x = (segment_x * self.size) + x;
                    let y = (segment_y * self.size) + y;
                    spaces[(y * size) + x] = risk;
                }
            }
        }

        Self { size, spaces }
    }
}

#[derive(Debug, PartialEq)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut spaces = Vec::new();
        let mut size = 0;

        for line in input.lines() {
            for (x, ch) in line.chars().enumerate() {
                let risk = ch
                    .to_digit(10)
                    .ok_or(ParseGridError)
                    .and_then(|x| usize::try_from(x).map_err(|_| ParseGridError))?;
                size = size.max(x + 1);
                spaces.push(risk);
            }
        }

        Ok(Self { size, spaces })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Grid::from_str(input)
        .ok()
        .as_ref()
        .map(Grid::lowest_risk_path)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    Grid::from_str(input)
        .ok()
        .as_ref()
        .map(Grid::expand)
        .as_ref()
        .map(Grid::lowest_risk_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> Grid {
        Grid {
            size: 10,
            spaces: vec![
                1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1, 1, 3,
                2, 8, 3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3, 1, 9, 1, 2,
                8, 1, 3, 7, 1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 1, 2, 9, 3,
                1, 3, 8, 5, 2, 1, 2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
            ],
        }
    }

    #[test]
    fn test_parse_grid() {
        assert_eq!(
            Grid::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_grid()),
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_expand() {
        let grid = Grid {
            size: 2,
            spaces: vec![1, 2, 3, 4],
        };
        let expanded = Grid {
            size: 10,
            spaces: vec![
                1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 2, 3, 3, 4, 4, 5, 5, 6,
                6, 7, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 5, 6, 6, 7, 7, 8,
                8, 9, 9, 1, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 6, 7, 7, 8, 8, 9, 9, 1, 1, 2, 5, 6, 6, 7,
                7, 8, 8, 9, 9, 1, 7, 8, 8, 9, 9, 1, 1, 2, 2, 3,
            ],
        };
        assert_eq!(grid.expand(), expanded);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(315));
    }
}
