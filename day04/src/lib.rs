use std::collections::HashSet;

#[derive(Debug)]
pub struct Card {
    winning_nums: HashSet<u32>,
    card_nums: HashSet<u32>,
}

impl Card {
    fn count_matches(&self) -> u32 {
        self.winning_nums.intersection(&self.card_nums).count() as u32
    }
}

pub fn process_cards(lines: Vec<&str>) -> Vec<Card> {
    lines
        .iter()
        .map(|&card| {
            let c = card.split(":").collect::<Vec<&str>>();
            let winning_nums = c[1].split("|").collect::<Vec<&str>>()[0]
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let card_nums = c[1].split("|").collect::<Vec<&str>>()[1]
                .split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|&s| s.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            Card {
                winning_nums,
                card_nums,
            }
        })
        .collect::<Vec<Card>>()
}

pub fn part_one(input: &str) -> u32 {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }

    process_cards(lines)
        .iter()
        .map(|c| {
            let count = c.count_matches();
            if count == 0 {
                return 0;
            }
            2_u32.pow(count - 1)
        })
        .sum::<u32>()
}

pub fn part_two(input: &str) -> u32 {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }

    let cards = process_cards(lines);
    // Keep track of the games and the number of times we have played them
    let mut games = vec![0; cards.len()];

    // Go through each card games once to get play count for it
    // and adjust count of games that need to be played again
    for i in 0..cards.len() {
        games[i] += 1;
        let win_count = cards[i].count_matches();
        // For each win we want to go through the next x cards where x is the
        // number of wins
        for win in 1..win_count+1{
            // Need to add i + win to get the correct game index
            let index = i + usize::try_from(win).unwrap();
            // Update the game with the count of the current game since it will
            // be played that many times
            games[index] += games[i];
        }
    }

    games.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_two(input), 30);
    }
}
