advent_of_code::solution!(1);

fn count_increases(mut values: impl Iterator<Item = u32>) -> Option<u32> {
    let mut prev = values.next()?;
    let mut increases = 0;
    for value in values {
        if value > prev {
            increases += 1;
        }
        prev = value;
    }
    Some(increases)
}

fn count_three_value_window_increases(mut values: impl Iterator<Item = u32>) -> Option<u32> {
    let mut a = values.next()?;
    let mut b = values.next()?;
    let mut c = values.next()?;
    let mut increases = 0;
    for d in values {
        if d > a {
            increases += 1;
        }
        a = b;
        b = c;
        c = d;
    }
    Some(increases)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    count_increases(input.lines().filter_map(|line| line.parse().ok()))
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    count_three_value_window_increases(input.lines().filter_map(|line| line.parse().ok()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increases() {
        let values: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(values.into_iter()), Some(7));
    }

    #[test]
    fn test_three_value_windows() {
        let values: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(
            count_three_value_window_increases(values.into_iter()),
            Some(5)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
