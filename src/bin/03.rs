advent_of_code::solution!(3);

struct PowerConsumptionDigitCounter {
    one: u32,
    zero: u32,
}

impl PowerConsumptionDigitCounter {
    const fn new() -> Self {
        Self { one: 0, zero: 0 }
    }

    const fn insert(&mut self, digit: char) {
        match digit {
            '1' => self.one += 1,
            '0' => self.zero += 1,
            _ => (),
        }
    }
}

struct PowerConsumptionCounter {
    digits: Vec<PowerConsumptionDigitCounter>,
}

impl PowerConsumptionCounter {
    const fn new() -> Self {
        Self { digits: Vec::new() }
    }

    fn insert(&mut self, reading: &str) {
        for (pos, digit) in reading.chars().rev().enumerate() {
            while self.digits.len() <= pos {
                self.digits.push(PowerConsumptionDigitCounter::new());
            }
            self.digits[pos].insert(digit);
        }
    }

    fn get_values(&self) -> (u32, u32) {
        let mut gamma = 0;
        let mut epsilon = 0;

        for (pos, counts) in self.digits.iter().enumerate() {
            let digit = 1 << pos;
            if counts.one > counts.zero {
                gamma |= digit;
            } else {
                epsilon |= digit;
            }
        }

        (gamma, epsilon)
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut counter = PowerConsumptionCounter::new();
    for line in input.lines() {
        counter.insert(line);
    }
    let (gamma, epsilon) = counter.get_values();
    Some(gamma * epsilon)
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
    fn test_power_consumption_counter() {
        let mut counter = PowerConsumptionCounter::new();

        counter.insert("00100");
        counter.insert("11110");
        counter.insert("10110");
        counter.insert("10111");
        counter.insert("10101");
        counter.insert("01111");
        counter.insert("00111");
        counter.insert("11100");
        counter.insert("10000");
        counter.insert("11001");
        counter.insert("00010");
        counter.insert("01010");

        assert_eq!(counter.get_values(), (22, 9));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
