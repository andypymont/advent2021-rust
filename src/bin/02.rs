advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct SimpleSubmarinePosition {
    depth: u32,
    horiz: u32,
}

#[derive(Debug, PartialEq)]
struct ComplicatedSubmarinePosition {
    aim: u32,
    depth: u32,
    horiz: u32,
}

trait SubmarinePosition: Sized {
    fn new() -> Self;
    fn forward(&self, dist: u32) -> Self;
    fn up(&self, dist: u32) -> Self;
    fn down(&self, dist: u32) -> Self;

    fn from_instructions<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut position = Self::new();
        for line in lines {
            let Some((command, dist_str)) = line.split_once(' ') else {
                continue;
            };
            let Ok(dist) = dist_str.parse() else {
                continue;
            };
            if command == "forward" {
                position = position.forward(dist);
            } else if command == "up" {
                position = position.up(dist);
            } else if command == "down" {
                position = position.down(dist);
            }
        }
        position
    }
}

impl SubmarinePosition for SimpleSubmarinePosition {
    fn new() -> Self {
        Self { depth: 0, horiz: 0 }
    }

    fn down(&self, dist: u32) -> Self {
        Self {
            depth: self.depth + dist,
            horiz: self.horiz,
        }
    }

    fn forward(&self, dist: u32) -> Self {
        Self {
            depth: self.depth,
            horiz: self.horiz + dist,
        }
    }

    fn up(&self, dist: u32) -> Self {
        Self {
            depth: self.depth.saturating_sub(dist),
            horiz: self.horiz,
        }
    }
}

impl SubmarinePosition for ComplicatedSubmarinePosition {
    fn new() -> Self {
        Self {
            aim: 0,
            depth: 0,
            horiz: 0,
        }
    }

    fn forward(&self, dist: u32) -> Self {
        Self {
            aim: self.aim,
            depth: self.depth + (self.aim * dist),
            horiz: self.horiz + dist,
        }
    }

    fn up(&self, dist: u32) -> Self {
        Self {
            aim: self.aim.saturating_sub(dist),
            depth: self.depth,
            horiz: self.horiz,
        }
    }

    fn down(&self, dist: u32) -> Self {
        Self {
            aim: self.aim + dist,
            depth: self.depth,
            horiz: self.horiz,
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let position = SimpleSubmarinePosition::from_instructions(input.lines());
    Some(position.depth * position.horiz)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let position = ComplicatedSubmarinePosition::from_instructions(input.lines());
    Some(position.depth * position.horiz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horiz() {
        let before = SimpleSubmarinePosition { depth: 0, horiz: 0 };
        let after = SimpleSubmarinePosition { depth: 0, horiz: 3 };
        assert_eq!(before.forward(3), after);

        let before = SimpleSubmarinePosition { depth: 0, horiz: 1 };
        assert_eq!(before.forward(2), after);

        let before = SimpleSubmarinePosition {
            depth: 4,
            horiz: 13,
        };
        let after = SimpleSubmarinePosition {
            depth: 4,
            horiz: 27,
        };
        assert_eq!(before.forward(14), after);
    }

    #[test]
    fn test_depth() {
        let before = SimpleSubmarinePosition { depth: 2, horiz: 4 };
        let after = SimpleSubmarinePosition { depth: 5, horiz: 4 };
        assert_eq!(before.down(3), after);

        let before = SimpleSubmarinePosition {
            depth: 10,
            horiz: 4,
        };
        assert_eq!(before.up(5), after);
    }

    #[test]
    fn test_complicated_position() {
        let a = ComplicatedSubmarinePosition {
            aim: 0,
            depth: 0,
            horiz: 0,
        };
        let b = ComplicatedSubmarinePosition {
            aim: 0,
            depth: 0,
            horiz: 5,
        };
        assert_eq!(a.forward(5), b);
        let c = ComplicatedSubmarinePosition {
            aim: 5,
            depth: 0,
            horiz: 5,
        };
        assert_eq!(b.down(5), c);
        let d = ComplicatedSubmarinePosition {
            aim: 5,
            depth: 40,
            horiz: 13,
        };
        assert_eq!(c.forward(8), d);
        let e = ComplicatedSubmarinePosition {
            aim: 2,
            depth: 40,
            horiz: 13,
        };
        assert_eq!(d.up(3), e);
        let f = ComplicatedSubmarinePosition {
            aim: 10,
            depth: 40,
            horiz: 13,
        };
        assert_eq!(e.down(8), f);
        let g = ComplicatedSubmarinePosition {
            aim: 10,
            depth: 60,
            horiz: 15,
        };
        assert_eq!(f.forward(2), g);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
