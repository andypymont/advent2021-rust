use std::cmp::Ordering;
use std::iter::{empty, successors};
use std::str::FromStr;

advent_of_code::solution!(5);

const GRID_SIZE: usize = 1000;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    fn get_slope(&self) -> (Ordering, Ordering) {
        (
            self.finish.x.cmp(&self.start.x),
            self.finish.y.cmp(&self.start.y),
        )
    }

    fn get_points(&self, allow_diagonal: bool) -> Box<dyn Iterator<Item = Point> + '_> {
        let (slope_x, slope_y) = self.get_slope();
        let parallel_to_grid = slope_x == Ordering::Equal || slope_y == Ordering::Equal;

        if allow_diagonal || parallel_to_grid {
            Box::new(successors(Some(self.start), move |prev| {
                let x = match slope_x {
                    Ordering::Greater => {
                        let next_x = prev.x + 1;
                        if next_x > self.finish.x {
                            None
                        } else {
                            Some(next_x)
                        }
                    }
                    Ordering::Less => {
                        let next_x = prev.x.checked_sub(1)?;
                        if next_x < self.finish.x {
                            None
                        } else {
                            Some(next_x)
                        }
                    }
                    Ordering::Equal => Some(prev.x),
                }?;
                let y = match slope_y {
                    Ordering::Greater => {
                        let next_y = prev.y + 1;
                        if next_y > self.finish.y {
                            None
                        } else {
                            Some(next_y)
                        }
                    }
                    Ordering::Less => {
                        let next_y = prev.y.checked_sub(1)?;
                        if next_y < self.finish.y {
                            None
                        } else {
                            Some(next_y)
                        }
                    }
                    Ordering::Equal => Some(prev.y),
                }?;
                Some(Point { x, y })
            }))
        } else {
            Box::new(empty())
        }
    }
}

#[derive(Debug, PartialEq)]
struct VentSystem {
    vents: Vec<Vent>,
}

impl VentSystem {
    fn count_overlapping_points(&self, allow_diagonal: bool) -> usize {
        let mut visited: Vec<u8> = vec![0; GRID_SIZE * GRID_SIZE];

        for vent in &self.vents {
            for point in vent.get_points(allow_diagonal) {
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
    VentSystem::from_str(input).map_or(None, |system| Some(system.count_overlapping_points(false)))
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    VentSystem::from_str(input).map_or(None, |system| Some(system.count_overlapping_points(true)))
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
    fn test_points_diagonal_not_allowed() {
        let diagonal = Vent {
            start: Point { x: 0, y: 0 },
            finish: Point { x: 8, y: 8 },
        };
        let mut points = diagonal.get_points(false);
        assert_eq!(points.next(), None);
    }

    #[test]
    fn test_points_horizontal() {
        let horizontal = Vent {
            start: Point { x: 2, y: 4 },
            finish: Point { x: 5, y: 4 },
        };
        let mut points = horizontal.get_points(false);
        assert_eq!(points.next(), Some(Point { x: 2, y: 4 }));
        assert_eq!(points.next(), Some(Point { x: 3, y: 4 }));
        assert_eq!(points.next(), Some(Point { x: 4, y: 4 }));
        assert_eq!(points.next(), Some(Point { x: 5, y: 4 }));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn test_points_vertical() {
        let vertical = Vent {
            start: Point { x: 7, y: 0 },
            finish: Point { x: 7, y: 4 },
        };
        let mut points = vertical.get_points(false);
        assert_eq!(points.next(), Some(Point { x: 7, y: 0 }));
        assert_eq!(points.next(), Some(Point { x: 7, y: 1 }));
        assert_eq!(points.next(), Some(Point { x: 7, y: 2 }));
        assert_eq!(points.next(), Some(Point { x: 7, y: 3 }));
        assert_eq!(points.next(), Some(Point { x: 7, y: 4 }));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn test_count_overlapping_points() {
        let system = example_vent_system();
        assert_eq!(system.count_overlapping_points(false), 5);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_vent_slope() {
        let vertical = Vent {
            start: Point { x: 7, y: 0 },
            finish: Point { x: 7, y: 4 },
        };
        assert_eq!(vertical.get_slope(), (Ordering::Equal, Ordering::Greater));

        let horizontal = Vent {
            start: Point { x: 2, y: 3 },
            finish: Point { x: 6, y: 3 },
        };
        assert_eq!(horizontal.get_slope(), (Ordering::Greater, Ordering::Equal));

        let diagonal = Vent {
            start: Point { x: 4, y: 4 },
            finish: Point { x: 6, y: 2 },
        };
        assert_eq!(diagonal.get_slope(), (Ordering::Greater, Ordering::Less));
    }

    #[test]
    fn test_points_diagonal() {
        let diagonal = Vent {
            start: Point { x: 0, y: 0 },
            finish: Point { x: 8, y: 8 },
        };
        let mut points = diagonal.get_points(true);
        assert_eq!(points.next(), Some(Point { x: 0, y: 0 }));
        assert_eq!(points.next(), Some(Point { x: 1, y: 1 }));
        assert_eq!(points.next(), Some(Point { x: 2, y: 2 }));
        assert_eq!(points.next(), Some(Point { x: 3, y: 3 }));
        assert_eq!(points.next(), Some(Point { x: 4, y: 4 }));
        assert_eq!(points.next(), Some(Point { x: 5, y: 5 }));
        assert_eq!(points.next(), Some(Point { x: 6, y: 6 }));
        assert_eq!(points.next(), Some(Point { x: 7, y: 7 }));
        assert_eq!(points.next(), Some(Point { x: 8, y: 8 }));
        assert_eq!(points.next(), None);
    }

    #[test]
    fn test_count_overlapping_points_with_diagonals() {
        let system = example_vent_system();
        assert_eq!(system.count_overlapping_points(true), 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
