use std::iter::empty;
use std::str::FromStr;

advent_of_code::solution!(5);

const GRID_SIZE: usize = 1000;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Vent {
    start: Point,
    finish: Point,
}

impl Vent {
    fn grid_aligned_segments(&self) -> Box<dyn Iterator<Item = Point> + '_> {
        if self.start.y == self.finish.y {
            let (x1, x2) = if self.start.x > self.finish.x {
                (self.finish.x, self.start.x)
            } else {
                (self.start.x, self.finish.x)
            };
            return Box::new((x1..=x2).map(move |x| Point { x, y: self.start.y }));
        }
        if self.start.x == self.finish.x {
            let (y1, y2) = if self.start.y > self.finish.y {
                (self.finish.y, self.start.y)
            } else {
                (self.start.y, self.finish.y)
            };
            return Box::new((y1..=y2).map(move |y| Point { x: self.start.x, y }));
        }

        Box::new(empty())
    }
}

#[derive(Debug, PartialEq)]
struct VentSystem {
    vents: Vec<Vent>,
}

impl VentSystem {
    fn count_overlapping_points(&self) -> usize {
        let mut visited: Vec<u8> = vec![0; GRID_SIZE * GRID_SIZE];

        for vent in &self.vents {
            for point in vent.grid_aligned_segments() {
                let ix = (point.y * GRID_SIZE) + point.x;
                visited[ix] = visited[ix].saturating_add(1);
            }
        }

        visited.into_iter().filter(|x| *x > 1).count()
    }
}

#[derive(Debug, PartialEq)]
struct ParseVentError;

impl FromStr for Point {
    type Err = ParseVentError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let Some((x, y)) = text.split_once(',') else {
            return Err(ParseVentError);
        };

        let x = usize::from_str(x).map_err(|_| ParseVentError)?;
        let y = usize::from_str(y).map_err(|_| ParseVentError)?;

        Ok(Self { x, y })
    }
}

impl FromStr for Vent {
    type Err = ParseVentError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((start, finish)) = line.split_once(" -> ") else {
            return Err(ParseVentError);
        };

        let start = Point::from_str(start)?;
        let finish = Point::from_str(finish)?;

        Ok(Self { start, finish })
    }
}

impl FromStr for VentSystem {
    type Err = ParseVentError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut vents = Vec::new();

        for line in input.lines() {
            let vent = Vent::from_str(line)?;
            vents.push(vent);
        }

        Ok(Self { vents })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    VentSystem::from_str(input).map_or(None, |system| Some(system.count_overlapping_points()))
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_vent_system() -> VentSystem {
        VentSystem {
            vents: vec![
                Vent {
                    start: Point { x: 0, y: 9 },
                    finish: Point { x: 5, y: 9 },
                },
                Vent {
                    start: Point { x: 8, y: 0 },
                    finish: Point { x: 0, y: 8 },
                },
                Vent {
                    start: Point { x: 9, y: 4 },
                    finish: Point { x: 3, y: 4 },
                },
                Vent {
                    start: Point { x: 2, y: 2 },
                    finish: Point { x: 2, y: 1 },
                },
                Vent {
                    start: Point { x: 7, y: 0 },
                    finish: Point { x: 7, y: 4 },
                },
                Vent {
                    start: Point { x: 6, y: 4 },
                    finish: Point { x: 2, y: 0 },
                },
                Vent {
                    start: Point { x: 0, y: 9 },
                    finish: Point { x: 2, y: 9 },
                },
                Vent {
                    start: Point { x: 3, y: 4 },
                    finish: Point { x: 1, y: 4 },
                },
                Vent {
                    start: Point { x: 0, y: 0 },
                    finish: Point { x: 8, y: 8 },
                },
                Vent {
                    start: Point { x: 5, y: 5 },
                    finish: Point { x: 8, y: 2 },
                },
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            VentSystem::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_vent_system())
        );
    }

    #[test]
    fn test_diagonal_has_no_grid_aligned_segments() {
        let diagonal = Vent {
            start: Point { x: 0, y: 0 },
            finish: Point { x: 8, y: 8 },
        };
        let mut segments = diagonal.grid_aligned_segments();
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn test_horizontal_grid_aligned_segments() {
        let horizontal = Vent {
            start: Point { x: 2, y: 4 },
            finish: Point { x: 5, y: 4 },
        };
        let mut segments = horizontal.grid_aligned_segments();
        assert_eq!(segments.next(), Some(Point { x: 2, y: 4 }));
        assert_eq!(segments.next(), Some(Point { x: 3, y: 4 }));
        assert_eq!(segments.next(), Some(Point { x: 4, y: 4 }));
        assert_eq!(segments.next(), Some(Point { x: 5, y: 4 }));
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn test_vertical_grid_aligned_segments() {
        let vertical = Vent {
            start: Point { x: 7, y: 0 },
            finish: Point { x: 7, y: 4 },
        };
        let mut segments = vertical.grid_aligned_segments();
        assert_eq!(segments.next(), Some(Point { x: 7, y: 0 }));
        assert_eq!(segments.next(), Some(Point { x: 7, y: 1 }));
        assert_eq!(segments.next(), Some(Point { x: 7, y: 2 }));
        assert_eq!(segments.next(), Some(Point { x: 7, y: 3 }));
        assert_eq!(segments.next(), Some(Point { x: 7, y: 4 }));
        assert_eq!(segments.next(), None);
    }

    #[test]
    fn test_count_overlapping_points() {
        let system = example_vent_system();
        assert_eq!(system.count_overlapping_points(), 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
