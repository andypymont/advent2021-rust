use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug, PartialEq)]
struct BitSet(u128);

impl BitSet {
    const fn contains(&self, number: u8) -> bool {
        self.0 & 1 << number != 0
    }

    const fn insert(&mut self, number: u8) {
        self.0 |= 1 << number;
    }

    const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    const fn remove(&mut self, number: u8) {
        if self.contains(number) {
            self.0 -= 1 << number;
        }
    }
}

#[derive(Debug, PartialEq)]
struct BingoCard {
    cols: [BitSet; 5],
    rows: [BitSet; 5],
}

impl BingoCard {
    const fn new() -> Self {
        Self {
            cols: [BitSet(0), BitSet(0), BitSet(0), BitSet(0), BitSet(0)],
            rows: [BitSet(0), BitSet(0), BitSet(0), BitSet(0), BitSet(0)],
        }
    }

    const fn add_number(&mut self, row: usize, col: usize, number: u8) {
        self.cols[col].insert(number);
        self.rows[row].insert(number);
    }

    fn call_number(&mut self, number: u8) {
        for ix in 0..5 {
            self.cols[ix].remove(number);
            self.rows[ix].remove(number);
        }
    }

    fn has_won(&self) -> bool {
        self.cols.iter().any(BitSet::is_empty) || self.rows.iter().any(BitSet::is_empty)
    }

    fn contains_number(&self, number: u8) -> bool {
        self.cols
            .iter()
            .chain(self.rows.iter())
            .any(|set| set.contains(number))
    }

    fn sum_of_unmarked_numbers(&self) -> u32 {
        (0..100)
            .filter_map(|x| {
                if self.contains_number(x) {
                    Some(u32::from(x))
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
struct BingoGame {
    numbers: Vec<u8>,
    cards: Vec<BingoCard>,
}

impl BingoGame {
    fn first_win(&mut self) -> Option<u32> {
        for number in &self.numbers {
            for card in &mut self.cards {
                card.call_number(*number);
                if card.has_won() {
                    return Some(card.sum_of_unmarked_numbers() * u32::from(*number));
                }
            }
        }

        None
    }

    fn all_wins(&mut self) -> Vec<u32> {
        let mut wins = Vec::new();

        for number in &self.numbers {
            for card in self.cards.iter_mut().filter(|card| !card.has_won()) {
                card.call_number(*number);
                if card.has_won() {
                    wins.push(card.sum_of_unmarked_numbers() * u32::from(*number));
                }
            }
        }

        wins
    }
}

#[derive(Debug, PartialEq)]
struct ParseBingoGameError;

impl FromStr for BingoCard {
    type Err = ParseBingoGameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut card = Self::new();

        for (row, line) in text.lines().enumerate() {
            for (col, number_str) in line.split_whitespace().enumerate() {
                let number = u8::from_str(number_str).map_err(|_| ParseBingoGameError)?;
                card.add_number(row, col, number);
            }
        }

        Ok(card)
    }
}

impl FromStr for BingoGame {
    type Err = ParseBingoGameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut cards = Vec::new();

        let mut sections = input.split("\n\n");

        let Some(numbers_str) = sections.next() else {
            return Err(ParseBingoGameError);
        };
        for number_str in numbers_str.split(',') {
            let number = u8::from_str(number_str).map_err(|_| ParseBingoGameError)?;
            numbers.push(number);
        }

        for section in sections {
            let card = BingoCard::from_str(section)?;
            cards.push(card);
        }

        Ok(Self { numbers, cards })
    }
}

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    BingoGame::from_str(input).map_or(None, |mut game| game.first_win())
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    BingoGame::from_str(input).map_or(None, |mut game| game.all_wins().last().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_bingo_game() -> BingoGame {
        BingoGame {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            cards: vec![
                BingoCard {
                    cols: [
                        BitSet(6291778),
                        BitSet(13828),
                        BitSet(9584648),
                        BitSet(362512),
                        BitSet(17301665),
                    ],
                    rows: [
                        BitSet(4335617),
                        BitSet(25166100),
                        BitSet(2179712),
                        BitSet(263272),
                        BitSet(1609730),
                    ],
                },
                BingoCard {
                    cols: [
                        BitSet(1589768),
                        BitSet(2394368),
                        BitSet(74881),
                        BitSet(50466820),
                        BitSet(12583024),
                    ],
                    rows: [
                        BitSet(4227085),
                        BitSet(401952),
                        BitSet(42467712),
                        BitSet(17828880),
                        BitSet(2183232),
                    ],
                },
                BingoCard {
                    cols: [
                        BitSet(4473860),
                        BitSet(2164993),
                        BitSet(8564736),
                        BitSet(83886664),
                        BitSet(1573040),
                    ],
                    rows: [
                        BitSet(19021840),
                        BitSet(624128),
                        BitSet(76808448),
                        BitSet(4204640),
                        BitSet(4237),
                    ],
                },
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            BingoGame::from_str(&advent_of_code::template::read_file("examples", DAY)),
            Ok(example_bingo_game()),
        );
    }

    #[test]
    fn test_bitset() {
        let mut set = BitSet(0);
        set.insert(4);
        set.insert(5);
        assert_eq!(set.0, 48);

        set.remove(5);
        assert_eq!(set.0, 16);

        set.insert(0);
        set.insert(1);
        assert_eq!(set.0, 19);
    }

    #[test]
    fn test_bingo_card() {
        let mut game = example_bingo_game();
        let card = &mut game.cards[2];

        for number in &game.numbers[..11] {
            card.call_number(*number);
            assert_eq!(card.has_won(), false);
        }

        card.call_number(24);
        assert_eq!(card.has_won(), true);
        assert_eq!(card.sum_of_unmarked_numbers(), 188);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4512));
    }

    #[test]
    fn test_bingo_game_all_wins() {
        let mut game = example_bingo_game();
        assert_eq!(game.all_wins(), vec![4512, 2192, 1924,],);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1924));
    }
}
