use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug, PartialEq)]
struct LanternFishPopulation {
    counts: [u64; 9],
}

impl LanternFishPopulation {
    const fn progress(self) -> Self {
        Self {
            counts: [
                self.counts[1],
                self.counts[2],
                self.counts[3],
                self.counts[4],
                self.counts[5],
                self.counts[6],
                self.counts[7] + self.counts[0],
                self.counts[8],
                self.counts[0],
            ],
        }
    }

    fn after_days(self, days: usize) -> Self {
        let mut state = self;
        for _ in 0..days {
            state = state.progress();
        }
        state
    }

    fn total(self) -> u64 {
        self.counts.iter().sum()
    }
}

#[derive(Debug, PartialEq)]
struct ParseLanternFishPopulationError;

impl FromStr for LanternFishPopulation {
    type Err = ParseLanternFishPopulationError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut counts = [0; 9];
        for fish in input.trim().split(',') {
            let age = usize::from_str(fish).map_err(|_| ParseLanternFishPopulationError)?;
            counts[age] += 1;
        }
        Ok(Self { counts })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    LanternFishPopulation::from_str(input).map_or(None, |pop| Some(pop.after_days(80).total()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    LanternFishPopulation::from_str(input).map_or(None, |pop| Some(pop.after_days(256).total()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_lanternfish() -> LanternFishPopulation {
        LanternFishPopulation {
            counts: [0, 1, 1, 2, 1, 0, 0, 0, 0],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            LanternFishPopulation::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_lanternfish()),
        );
    }

    #[test]
    fn test_progress_population() {
        let initial = example_lanternfish();

        let one = LanternFishPopulation {
            counts: [1, 1, 2, 1, 0, 0, 0, 0, 0],
        };
        assert_eq!(initial.progress(), one);

        let two = LanternFishPopulation {
            counts: [1, 2, 1, 0, 0, 0, 1, 0, 1],
        };
        assert_eq!(one.progress(), two);
    }

    #[test]
    fn test_population_after() {
        let initial = example_lanternfish();

        let ten = LanternFishPopulation {
            counts: [3, 2, 2, 1, 0, 1, 1, 1, 1],
        };
        assert_eq!(initial.after_days(10), ten);

        let eighteen = LanternFishPopulation {
            counts: [3, 5, 3, 2, 2, 1, 5, 1, 4],
        };
        assert_eq!(initial.after_days(18), eighteen);
    }

    #[test]
    fn test_population_total() {
        let initial = example_lanternfish();
        assert_eq!(initial.total(), 5);

        let eighteen = LanternFishPopulation {
            counts: [3, 5, 3, 2, 2, 1, 5, 1, 4],
        };
        assert_eq!(eighteen.total(), 26);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5934));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26_984_457_539));
    }
}
