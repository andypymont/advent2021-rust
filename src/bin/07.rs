use std::str::FromStr;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct SubmarinePositions {
    max: u32,
    min: u32,
    positions: Vec<u32>,
}

impl SubmarinePositions {
    const fn fuel_consumption_for_distance(distance: u32) -> u32 {
        distance * (distance + 1) / 2
    }

    fn total_distance_to(&self, position: u32) -> u32 {
        self.positions
            .iter()
            .map(|sub| sub.abs_diff(position))
            .sum()
    }

    fn total_fuel_consumption_to(&self, position: u32) -> u32 {
        self.positions
            .iter()
            .map(|sub| Self::fuel_consumption_for_distance(sub.abs_diff(position)))
            .sum()
    }

    fn cheapest_aligned_position(&self, fuel_based: bool) -> Option<u32> {
        (self.min..=self.max).fold(None, |best, position| {
            let cost = if fuel_based {
                self.total_fuel_consumption_to(position)
            } else {
                self.total_distance_to(position)
            };
            Some(best.map_or(cost, |best| best.min(cost)))
        })
    }
}

#[derive(Debug, PartialEq)]
struct ParseSubmarinePositionsError;

impl FromStr for SubmarinePositions {
    type Err = ParseSubmarinePositionsError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut positions = Vec::new();
        let mut min: Option<u32> = None;
        let mut max: Option<u32> = None;

        for position in input.trim().split(',') {
            let position = u32::from_str(position).map_err(|_| ParseSubmarinePositionsError)?;
            min = Some(min.map_or(position, |best| best.min(position)));
            max = Some(max.map_or(position, |best| best.max(position)));
            positions.push(position);
        }

        let min = min.ok_or(ParseSubmarinePositionsError)?;
        let max = max.ok_or(ParseSubmarinePositionsError)?;

        Ok(Self {
            max,
            min,
            positions,
        })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    SubmarinePositions::from_str(input)
        .map_or(None, |positions| positions.cheapest_aligned_position(false))
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(input: &str) -> Option<u32> {
    SubmarinePositions::from_str(input)
        .map_or(None, |positions| positions.cheapest_aligned_position(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_positions() -> SubmarinePositions {
        let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        SubmarinePositions {
            max: 16,
            min: 0,
            positions,
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            SubmarinePositions::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_positions()),
        );
    }

    #[test]
    fn test_total_distance_to() {
        let positions = example_positions();
        assert_eq!(positions.total_distance_to(2), 37);
        assert_eq!(positions.total_distance_to(1), 41);
        assert_eq!(positions.total_distance_to(3), 39);
        assert_eq!(positions.total_distance_to(10), 71);
    }

    #[test]
    fn test_cheapest_aligned_position() {
        let positions = example_positions();
        assert_eq!(positions.cheapest_aligned_position(false), Some(37));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37));
    }

    #[test]
    fn test_total_fuel_consumption_to() {
        let positions = example_positions();
        assert_eq!(positions.total_fuel_consumption_to(2), 206);
        assert_eq!(positions.total_fuel_consumption_to(5), 168);
    }

    #[test]
    fn test_cheapest_aligned_position_fuel_based() {
        let positions = example_positions();
        assert_eq!(positions.cheapest_aligned_position(true), Some(168));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(168));
    }
}
