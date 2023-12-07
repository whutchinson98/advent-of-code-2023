use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Score {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

// score, hand
fn score_hand(hand: &str) -> (Score, Vec<usize>) {
    // 2,3,4,5,6,7,8,9,T,J,Q,K,A
    let hand_chars = hand.chars().collect::<Vec<char>>();
    let mut count: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    hand_chars.iter().for_each(|&char| match char {
        'T' => count[8] += 1,
        'J' => count[9] += 1,
        'Q' => count[10] += 1,
        'K' => count[11] += 1,
        'A' => count[12] += 1,
        _ => {
            let digit = char.to_digit(10).unwrap() - 2;
            count[digit as usize] += 1;
        }
    });

    count.sort();

    let hand_types: String = count
        .iter()
        .map(|&s| s.to_string())
        .filter(|s| s != "0")
        .collect::<Vec<String>>()
        .join("");

    let score: Score = match hand_types.as_str() {
        "5" => Score::FiveOfAKind,
        "14" => Score::FourOfAKind,
        "23" => Score::FullHouse,
        "113" => Score::ThreeOfAKind,
        "122" => Score::TwoPair,
        "1112" => Score::OnePair,
        "11111" => Score::HighCard,
        _ => panic!("should never happen {:?}", hand_types),
    };

    let mut hand_score: Vec<usize> = vec![0, 0, 0, 0, 0];
    for i in 0..hand_chars.len() {
        match hand_chars[i] {
            'A' => hand_score[i] = 14,
            'K' => hand_score[i] = 13,
            'Q' => hand_score[i] = 12,
            'J' => hand_score[i] = 11,
            'T' => hand_score[i] = 10,
            _ => hand_score[i] = hand_chars[i].to_digit(10).unwrap() as usize,
        }
    }
    (score, hand_score)
}

pub fn process_input(lines: Vec<&str>) -> Vec<((Score, Vec<usize>), usize)> {
    let mut sorted_hands: Vec<((Score, Vec<usize>), usize)> = lines
        .iter()
        .map(|&line| {
            let (hand, bet) = line.split_once(" ").unwrap();
            let bet = bet.parse::<usize>().unwrap();
            let score = score_hand(hand);
            (score, bet)
        })
        .collect::<Vec<((Score, Vec<usize>), usize)>>();

    sorted_hands.sort_by(|a, b| {
        let a_score = a.0 .0 as u8;
        let b_score = b.0 .0 as u8;
        if a_score == b_score {
            for i in 0..5 {
                if a.0 .1[i] > b.0 .1[i] {
                    return Ordering::Greater;
                } else if a.0 .1[i] < b.0 .1[i] {
                    return Ordering::Less;
                }
            }
            return Ordering::Equal;
        } else {
            a_score.cmp(&b_score)
        }
    });
    sorted_hands
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let sorted_hands = process_input(lines);
    sorted_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let result = hand.1 * (index + 1);
            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_one(input), 6440);
    }
}
