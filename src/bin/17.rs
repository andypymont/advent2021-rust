use std::str::FromStr;

advent_of_code::solution!(17);

#[derive(Debug, PartialEq)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl TargetArea {
    const fn contains(&self, x: i32, y: i32) -> bool {
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }

    fn shot(&self, mut vel_x: i32, mut vel_y: i32) -> Option<i32> {
        let mut x = 0;
        let mut y = 0;
        let mut highpoint = 0;

        while x <= self.max_x && y >= self.min_y {
            highpoint = highpoint.max(y);
            if self.contains(x, y) {
                return Some(highpoint);
            }
            x += vel_x;
            y += vel_y;
            vel_x = if vel_x == 0 { 0 } else { vel_x - 1 };
            vel_y -= 1;
        }

        None
    }

    const fn x_velocity_hits(&self, mut velx: i32) -> bool {
        let mut x = 0;
        while velx >= 0 {
            if self.min_x <= x && x <= self.max_x {
                return true;
            }
            x += velx;
            velx -= 1;
        }
        false
    }

    fn x_velocities_hitting(&self) -> impl Iterator<Item = i32> + '_ {
        (0..=self.max_x).filter(|x| self.x_velocity_hits(*x))
    }

    fn hitting_shots(&self) -> impl Iterator<Item = i32> + '_ {
        self.x_velocities_hitting()
            .flat_map(move |velx| (self.min_y..=100).filter_map(move |vely| self.shot(velx, vely)))
    }
}

#[derive(Debug, PartialEq)]
struct ParseTargetAreaError;

impl FromStr for TargetArea {
    type Err = ParseTargetAreaError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let details = input
            .trim()
            .strip_prefix("target area: ")
            .ok_or(ParseTargetAreaError)?;
        let (x, y) = details.split_once(", ").ok_or(ParseTargetAreaError)?;
        let x = x.strip_prefix("x=").ok_or(ParseTargetAreaError)?;
        let y = y.strip_prefix("y=").ok_or(ParseTargetAreaError)?;

        let (min_x, max_x) = x.split_once("..").ok_or(ParseTargetAreaError)?;
        let (min_y, max_y) = y.split_once("..").ok_or(ParseTargetAreaError)?;

        let min_x = min_x.parse().map_err(|_| ParseTargetAreaError)?;
        let max_x = max_x.parse().map_err(|_| ParseTargetAreaError)?;
        let min_y = min_y.parse().map_err(|_| ParseTargetAreaError)?;
        let max_y = max_y.parse().map_err(|_| ParseTargetAreaError)?;

        Ok(Self {
            min_x,
            max_x,
            min_y,
            max_y,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<i32> {
    let area = TargetArea::from_str(input).ok()?;
    area.hitting_shots().max()
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let area = TargetArea::from_str(input).ok()?;
    Some(area.hitting_shots().map(|_| 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_target_area() -> TargetArea {
        TargetArea {
            min_x: 20,
            max_x: 30,
            min_y: -10,
            max_y: -5,
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            TargetArea::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_target_area()),
        );
    }

    #[test]
    fn test_x_velocities_hitting() {
        let hits: Vec<i32> = example_target_area().x_velocities_hitting().collect();
        assert_eq!(
            hits,
            vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30],
        );
    }

    #[test]
    fn test_hitting_shots() {
        let shots: Vec<i32> = example_target_area().hitting_shots().collect();
        assert_eq!(shots.len(), 112);
        assert_eq!(shots.iter().max(), Some(&45));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(112));
    }
}
