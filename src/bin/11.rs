use std::collections::VecDeque;
use std::str::FromStr;

advent_of_code::solution!(11);

const GRID_SIZE: usize = 10;
const GRID_SIZE_TOTAL: usize = GRID_SIZE * GRID_SIZE;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl Direction {
    fn step_from(&self, pos: usize) -> Option<usize> {
        let row = pos / GRID_SIZE;
        let col = pos % GRID_SIZE;

        let row = match self {
            Self::North | Self::Northwest | Self::Northeast => row.checked_sub(1),
            Self::South | Self::Southwest | Self::Southeast => {
                let south = row + 1;
                if south >= GRID_SIZE {
                    None
                } else {
                    Some(south)
                }
            }
            Self::East | Self::West => Some(row),
        };
        let row = row?;

        let col = match self {
            Self::Northwest | Self::West | Self::Southwest => col.checked_sub(1),
            Self::Northeast | Self::East | Self::Southeast => {
                let east = col + 1;
                if east >= GRID_SIZE {
                    None
                } else {
                    Some(east)
                }
            }
            Self::North | Self::South => Some(col),
        };
        col.map(|c| (row * GRID_SIZE) + c)
    }
}

const COMPASS: [Direction; 8] = [
    Direction::North,
    Direction::Northeast,
    Direction::East,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
];

#[derive(Debug, PartialEq)]
struct OctopusGrid {
    spaces: [u32; GRID_SIZE_TOTAL],
    latest: usize,
    flashes: usize,
}

impl OctopusGrid {
    fn cycle_when_all_flash(mut self) -> usize {
        for ix in 0.. {
            if self.latest == GRID_SIZE_TOTAL {
                return ix;
            }
            self.progress();
        }

        0
    }

    fn flashes_after(mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.progress();
        }
        self.flashes
    }

    fn neighbours(pos: usize) -> impl Iterator<Item = usize> {
        COMPASS.iter().filter_map(move |dir| dir.step_from(pos))
    }

    fn progress(&mut self) {
        self.latest = 0;
        let mut flashed = [false; GRID_SIZE_TOTAL];
        let mut queue = VecDeque::new();
        queue.extend(0..GRID_SIZE_TOTAL);

        while let Some(pos) = queue.pop_front() {
            if flashed[pos] {
                continue;
            }

            if self.spaces[pos] == 9 {
                self.spaces[pos] = 0;

                for neighbour in Self::neighbours(pos) {
                    queue.push_back(neighbour);
                }
                flashed[pos] = true;
                self.latest += 1;
                continue;
            }

            self.spaces[pos] += 1;
        }

        self.flashes += self.latest;
    }
}

#[derive(Debug, PartialEq)]
struct ParseOctopusGridError;

impl FromStr for OctopusGrid {
    type Err = ParseOctopusGridError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut spaces = [0; GRID_SIZE_TOTAL];

        for (row, line) in input.lines().enumerate() {
            for (col, space) in line.chars().enumerate() {
                let value = space.to_digit(10).ok_or(ParseOctopusGridError)?;
                spaces[(row * GRID_SIZE) + col] = value;
            }
        }

        Ok(Self {
            spaces,
            latest: 0,
            flashes: 0,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    OctopusGrid::from_str(input)
        .ok()
        .map(|og| og.flashes_after(100))
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(input: &str) -> Option<usize> {
    OctopusGrid::from_str(input)
        .ok()
        .map(OctopusGrid::cycle_when_all_flash)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_grid() -> OctopusGrid {
        OctopusGrid {
            spaces: [
                5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1,
                7, 3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2,
                4, 6, 4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6,
                8, 4, 8, 5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6,
            ],
            latest: 0,
            flashes: 0,
        }
    }

    fn example_grid_progressed() -> OctopusGrid {
        OctopusGrid {
            spaces: [
                6, 5, 9, 4, 2, 5, 4, 3, 3, 4, 3, 8, 5, 6, 9, 6, 5, 8, 2, 2, 6, 3, 7, 5, 6, 6, 7, 2,
                8, 4, 7, 2, 5, 2, 4, 4, 7, 2, 5, 7, 7, 4, 6, 8, 4, 9, 6, 5, 8, 9, 5, 2, 7, 8, 6, 3,
                5, 7, 5, 6, 3, 2, 8, 7, 9, 5, 2, 8, 3, 2, 7, 9, 9, 3, 9, 9, 2, 2, 4, 5, 5, 9, 5, 7,
                9, 5, 9, 6, 6, 5, 6, 3, 9, 4, 8, 6, 2, 6, 3, 7,
            ],
            latest: 0,
            flashes: 0,
        }
    }

    fn example_grid_progressed_twice() -> OctopusGrid {
        OctopusGrid {
            spaces: [
                8, 8, 0, 7, 4, 7, 6, 5, 5, 5, 5, 0, 8, 9, 0, 8, 7, 0, 5, 4, 8, 5, 9, 7, 8, 8, 9, 6,
                0, 8, 8, 4, 8, 5, 7, 6, 9, 6, 0, 0, 8, 7, 0, 0, 9, 0, 8, 8, 0, 0, 6, 6, 0, 0, 0, 8,
                8, 9, 8, 9, 6, 8, 0, 0, 0, 0, 5, 9, 4, 3, 0, 0, 0, 0, 0, 0, 7, 4, 5, 6, 9, 0, 0, 0,
                0, 0, 0, 8, 7, 6, 8, 7, 0, 0, 0, 0, 6, 8, 4, 8,
            ],
            latest: 35,
            flashes: 35,
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            OctopusGrid::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_grid()),
        );
    }

    #[test]
    fn test_progress() {
        let mut grid = example_grid();
        grid.progress();
        assert_eq!(grid, example_grid_progressed(),);
        grid.progress();
        assert_eq!(grid, example_grid_progressed_twice(),);
    }

    #[test]
    fn test_flashes_after() {
        assert_eq!(example_grid().flashes_after(10), 204,);
        assert_eq!(example_grid().flashes_after(100), 1656,);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1656));
    }

    #[test]
    fn test_cycle_when_all_flash() {
        assert_eq!(example_grid().cycle_when_all_flash(), 195,);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(195));
    }
}
