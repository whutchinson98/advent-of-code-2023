use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    println!("Part 1 {}", part_one(&contents));
    println!("Part 2 {}", part_two(&contents));
}

fn part_one(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect();
    lines
        .iter()
        .map(|&val| {
            let number_chars: Vec<char> = val.chars().filter(|&c| c.is_digit(10)).collect();
            if number_chars.len() == 0 {
                return 0;
            }
            format!(
                "{}{}",
                number_chars[0],
                number_chars[number_chars.len() - 1]
            )
            .parse::<usize>()
            .unwrap()
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<_>()
}

fn part_two(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").collect();

    lines
        .iter()
        .map(|&val| {
            // We need to replace with wordNumberword so the spelling of other
            // numbers is kept correct. example: eightwo -> 82 but without
            // this the 't' for eight is consumed when we replace it with 2
            let number_chars: Vec<char> = val
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter(|c| c.is_digit(10))
                .collect();

            if number_chars.len() == 0 {
                return 0;
            }
            format!(
                "{}{}",
                number_chars[0],
                number_chars[number_chars.len() - 1]
            )
            .parse::<usize>()
            .unwrap()
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<_>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn part_one_test() {
        assert_eq!(part_one(INPUT), 142);
    }
    const INPUT_TWO: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_two_test() {
        assert_eq!(part_two(INPUT_TWO), 281);
    }
}
