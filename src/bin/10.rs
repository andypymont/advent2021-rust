use std::collections::VecDeque;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq)]
enum Line {
    Corrupted(u64),
    Incomplete(u64),
}

fn check_line(line: &str) -> Line {
    let mut groups = VecDeque::new();

    for ch in line.chars() {
        if let Some(closing) = match ch {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None,
        } {
            groups.push_front(closing);
        } else if groups.pop_front() != Some(ch) {
            let score = match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            };
            return Line::Corrupted(score);
        }
    }

    let score = groups.iter().fold(0, |total, ch| {
        (total * 5)
            + match ch {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            }
    });
    Line::Incomplete(score)
}

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(check_line)
            .map(|line| match line {
                Line::Corrupted(x) => x,
                Line::Incomplete(_) => 0,
            })
            .sum(),
    )
}

#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let mut scores = Vec::new();
    for line in input.lines() {
        if let Line::Incomplete(score) = check_line(line) {
            scores.push(score);
        }
    }
    scores.sort_unstable();
    scores.get(scores.len() / 2).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_line() {
        assert_eq!(
            check_line("{([(<{}[<>[]}>{[]{[(<()>"),
            Line::Corrupted(1197)
        );
        assert_eq!(check_line("[[<[([]))<([[{}[[()]]]"), Line::Corrupted(3));
        assert_eq!(check_line("[{[{({}]{}}([{[{{{}}([]"), Line::Corrupted(57));
        assert_eq!(check_line("[<(<(<(<{}))><([]([]()"), Line::Corrupted(3));
        assert_eq!(
            check_line("<{([([[(<>()){}]>(<<{{"),
            Line::Corrupted(25_137)
        );
        assert_eq!(
            check_line("[({(<(())[]>[[{[]{<()<>>"),
            Line::Incomplete(288_957)
        );
        assert_eq!(
            check_line("[(()[<>])]({[<{<<[]>>("),
            Line::Incomplete(5_566)
        );
        assert_eq!(
            check_line("(((({<>}<{<{<>}{[]{[]{}"),
            Line::Incomplete(1_480_781)
        );
        assert_eq!(
            check_line("{<[[]]>}<{[{[{[]{()[[[]"),
            Line::Incomplete(995_444)
        );
        assert_eq!(
            check_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            Line::Incomplete(294)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(26_397));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288_957));
    }
}
