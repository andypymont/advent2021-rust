use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(13);

type Point = (usize, usize);

fn bottom_right(first: Point, second: Point) -> Point {
    (first.0.max(second.0), first.1.max(second.1))
}

#[derive(Debug, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    const fn move_coordinate(coord: usize, fold_line: usize) -> Option<usize> {
        if coord > fold_line {
            let dist = coord.abs_diff(fold_line);
            fold_line.checked_sub(dist)
        } else {
            Some(coord)
        }
    }

    fn move_dot(&self, dot: &Point) -> Option<Point> {
        let x = match self {
            Self::X(fold_line) => Self::move_coordinate(dot.0, *fold_line),
            Self::Y(_) => Some(dot.0),
        }?;
        let y = match self {
            Self::X(_) => Some(dot.1),
            Self::Y(fold_line) => Self::move_coordinate(dot.1, *fold_line),
        }?;
        Some((x, y))
    }
}

#[derive(Debug, PartialEq)]
struct Paper {
    dots: BTreeSet<Point>,
    folds: Vec<Fold>,
}

impl Paper {
    fn dots_after(dots: &BTreeSet<Point>, fold: &Fold) -> BTreeSet<Point> {
        let mut after = BTreeSet::new();
        after.extend(dots.iter().filter_map(|dot| fold.move_dot(dot)));
        after
    }

    fn fold_once(&mut self) {
        if let Some(fold) = self.folds.pop() {
            self.dots = Self::dots_after(&self.dots, &fold);
        }
    }

    fn output(dots: &BTreeSet<Point>) -> String {
        let (max_x, max_y) = dots.iter().fold((0, 0), |acc, pt| bottom_right(acc, *pt));

        let mut output = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let ch = if dots.contains(&(x, y)) { '█' } else { ' ' };
                output.push(ch);
            }
            output.push('\n');
        }

        output
    }

    fn fold_and_output(mut self) -> String {
        let mut dots = self.dots;

        while let Some(fold) = self.folds.pop() {
            dots = Self::dots_after(&dots, &fold);
        }

        Self::output(&dots)
    }
}

#[derive(Debug, PartialEq)]
struct ParsePaperError;

impl FromStr for Fold {
    type Err = ParsePaperError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let Some(text) = text.strip_prefix("fold along ") else {
            return Err(ParsePaperError);
        };
        let Some((dimension, coord)) = text.split_once('=') else {
            return Err(ParsePaperError);
        };
        let coord = coord.parse().map_err(|_| ParsePaperError)?;
        match dimension {
            "x" => Ok(Self::X(coord)),
            "y" => Ok(Self::Y(coord)),
            _ => Err(ParsePaperError),
        }
    }
}

impl FromStr for Paper {
    type Err = ParsePaperError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let Some((dots_str, folds_str)) = input.split_once("\n\n") else {
            return Err(ParsePaperError);
        };

        let mut dots = BTreeSet::new();
        for line in dots_str.lines() {
            let Some((x, y)) = line.split_once(',') else {
                return Err(ParsePaperError);
            };
            let x = x.parse().map_err(|_| ParsePaperError)?;
            let y = y.parse().map_err(|_| ParsePaperError)?;
            dots.insert((x, y));
        }

        let mut folds = Vec::new();
        for line in folds_str.lines().rev() {
            let fold = Fold::from_str(line)?;
            folds.push(fold);
        }

        Ok(Self { dots, folds })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    Paper::from_str(input).ok().map(|mut paper| {
        paper.fold_once();
        paper.dots.len()
    })
}

#[must_use]
pub fn part_two(input: &str) -> Option<String> {
    Paper::from_str(input).ok().map(Paper::fold_and_output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_paper() -> Paper {
        let mut dots = BTreeSet::new();
        dots.insert((6, 10));
        dots.insert((0, 14));
        dots.insert((9, 10));
        dots.insert((0, 3));
        dots.insert((10, 4));
        dots.insert((4, 11));
        dots.insert((6, 0));
        dots.insert((6, 12));
        dots.insert((4, 1));
        dots.insert((0, 13));
        dots.insert((10, 12));
        dots.insert((3, 4));
        dots.insert((3, 0));
        dots.insert((8, 4));
        dots.insert((1, 10));
        dots.insert((2, 14));
        dots.insert((8, 10));
        dots.insert((9, 0));

        Paper {
            dots,
            folds: vec![Fold::X(5), Fold::Y(7)],
        }
    }

    fn example_paper_after_fold() -> Paper {
        let mut dots = BTreeSet::new();

        dots.insert((0, 0));
        dots.insert((2, 0));
        dots.insert((3, 0));
        dots.insert((6, 0));
        dots.insert((9, 0));
        dots.insert((0, 1));
        dots.insert((4, 1));
        dots.insert((6, 2));
        dots.insert((10, 2));
        dots.insert((0, 3));
        dots.insert((4, 3));
        dots.insert((1, 4));
        dots.insert((3, 4));
        dots.insert((6, 4));
        dots.insert((8, 4));
        dots.insert((9, 4));
        dots.insert((10, 4));

        Paper {
            dots,
            folds: vec![Fold::X(5)],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Paper::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_paper()),
        );
    }

    #[test]
    fn test_paper_fold_once() {
        let mut paper = example_paper();
        paper.fold_once();
        assert_eq!(paper, example_paper_after_fold());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    fn example_output() -> String {
        let mut output = String::new();
        output.push_str("█████\n");
        output.push_str("█   █\n");
        output.push_str("█   █\n");
        output.push_str("█   █\n");
        output.push_str("█████\n");
        output
    }

    #[test]
    fn test_fold_and_output() {
        assert_eq!(example_paper().fold_and_output(), example_output(),);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(example_output()));
    }
}
