advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
struct SubmarinePosition {
    depth: u32,
    horiz: u32,
}

impl SubmarinePosition {
    const fn new() -> Self {
        Self { depth: 0, horiz: 0 }
    }

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

    const fn down(&self, dist: u32) -> Self {
        Self {
            depth: self.depth + dist,
            horiz: self.horiz,
        }
    }

    const fn forward(&self, dist: u32) -> Self {
        Self {
            depth: self.depth,
            horiz: self.horiz + dist,
        }
    }

    const fn up(&self, dist: u32) -> Self {
        Self {
            depth: self.depth.saturating_sub(dist),
            horiz: self.horiz,
        }
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let position = SubmarinePosition::from_instructions(input.lines());
    Some(position.depth * position.horiz)
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_position() {
        let expected = SubmarinePosition { depth: 0, horiz: 0 };
        assert_eq!(SubmarinePosition::new(), expected);
    }

    #[test]
    fn test_horiz() {
        let before = SubmarinePosition { depth: 0, horiz: 0 };
        let after = SubmarinePosition { depth: 0, horiz: 3 };
        assert_eq!(before.forward(3), after);

        let before = SubmarinePosition { depth: 0, horiz: 1 };
        assert_eq!(before.forward(2), after);

        let before = SubmarinePosition {
            depth: 4,
            horiz: 13,
        };
        let after = SubmarinePosition {
            depth: 4,
            horiz: 27,
        };
        assert_eq!(before.forward(14), after);
    }

    #[test]
    fn test_depth() {
        let before = SubmarinePosition { depth: 2, horiz: 4 };
        let after = SubmarinePosition { depth: 5, horiz: 4 };
        assert_eq!(before.down(3), after);

        let before = SubmarinePosition {
            depth: 10,
            horiz: 4,
        };
        assert_eq!(before.up(5), after);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
