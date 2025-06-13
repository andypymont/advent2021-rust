use std::collections::BTreeSet;
use std::str::FromStr;

advent_of_code::solution!(13);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point(usize, usize);

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
        Some(Point(x, y))
    }
}

#[derive(Debug, PartialEq)]
struct Paper {
    dots: BTreeSet<Point>,
    folds: Vec<Fold>,
}

impl Paper {
    fn dots_after_fold(&self, fold: &Fold) -> BTreeSet<Point> {
        let mut after = BTreeSet::new();
        after.extend(self.dots.iter().filter_map(|dot| fold.move_dot(dot)));
        after
    }
}

#[derive(Debug, PartialEq)]
struct ParsePaperError;

impl FromStr for Point {
    type Err = ParsePaperError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let Some((x, y)) = line.split_once(',') else {
            return Err(ParsePaperError);
        };

        let x = x.parse().map_err(|_| ParsePaperError)?;
        let y = y.parse().map_err(|_| ParsePaperError)?;

        Ok(Self(x, y))
    }
}

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
            let point = Point::from_str(line)?;
            dots.insert(point);
        }

        let mut folds = Vec::new();
        for line in folds_str.lines() {
            let fold = Fold::from_str(line)?;
            folds.push(fold);
        }

        Ok(Self { dots, folds })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let paper = Paper::from_str(input).ok()?;
    paper
        .folds
        .first()
        .map(|fold| paper.dots_after_fold(fold).len())
}

#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_paper() -> Paper {
        let mut dots = BTreeSet::new();
        dots.insert(Point(6, 10));
        dots.insert(Point(0, 14));
        dots.insert(Point(9, 10));
        dots.insert(Point(0, 3));
        dots.insert(Point(10, 4));
        dots.insert(Point(4, 11));
        dots.insert(Point(6, 0));
        dots.insert(Point(6, 12));
        dots.insert(Point(4, 1));
        dots.insert(Point(0, 13));
        dots.insert(Point(10, 12));
        dots.insert(Point(3, 4));
        dots.insert(Point(3, 0));
        dots.insert(Point(8, 4));
        dots.insert(Point(1, 10));
        dots.insert(Point(2, 14));
        dots.insert(Point(8, 10));
        dots.insert(Point(9, 0));

        Paper {
            dots,
            folds: vec![Fold::Y(7), Fold::X(5)],
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
    fn test_dots_after_fold() {
        let fold = Fold::Y(7);
        let mut expected = BTreeSet::new();
        expected.insert(Point(0, 0));
        expected.insert(Point(2, 0));
        expected.insert(Point(3, 0));
        expected.insert(Point(6, 0));
        expected.insert(Point(9, 0));
        expected.insert(Point(0, 1));
        expected.insert(Point(4, 1));
        expected.insert(Point(6, 2));
        expected.insert(Point(10, 2));
        expected.insert(Point(0, 3));
        expected.insert(Point(4, 3));
        expected.insert(Point(1, 4));
        expected.insert(Point(3, 4));
        expected.insert(Point(6, 4));
        expected.insert(Point(8, 4));
        expected.insert(Point(9, 4));
        expected.insert(Point(10, 4));

        assert_eq!(example_paper().dots_after_fold(&fold), expected,);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
