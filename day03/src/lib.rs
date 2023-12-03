#[derive(Debug, Clone)]
pub struct PartNumber {
    // Value of the part number
    value: usize,
    // Positions in the matrix that the number is found (row, col)
    positions: Vec<(usize, usize)>,
}

pub fn process_symbols(symbols: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut symbols_matches: Vec<(usize, usize)> = Vec::new();
    for i in 0..symbols.len() {
        for j in 0..symbols[i].len() {
            let v = symbols[i][j];
            if v != '.' && !v.is_digit(10) {
                symbols_matches.push((i, j));
            }
        }
    }
    symbols_matches
}

pub fn generate_symbols_matches(
    part_position: (usize, usize),
    symbol_position: (usize, usize),
) -> bool {
    // LEFT
    if part_position.0 > 0
        && (part_position.0 - 1 == symbol_position.0 && part_position.1 == symbol_position.1)
    {
        return true;
    }
    // RIGHT
    if part_position.0 + 1 == symbol_position.0 && part_position.1 == symbol_position.1 {
        return true;
    }
    // UP
    if part_position.1 + 1 == symbol_position.1 && part_position.0 == symbol_position.0 {
        return true;
    }
    // DOWN
    if part_position.1 > 0
        && (part_position.1 - 1 == symbol_position.1 && part_position.0 == symbol_position.0)
    {
        return true;
    }
    // LEFT-UP
    if part_position.0 > 0
        && (part_position.1 + 1 == symbol_position.1 && part_position.0 - 1 == symbol_position.0)
    {
        return true;
    }
    // RIGHT-UP
    if part_position.1 + 1 == symbol_position.1 && part_position.0 + 1 == symbol_position.0 {
        return true;
    }
    // RIGHT-DOWN
    if part_position.1 > 0
        && (part_position.1 - 1 == symbol_position.1 && part_position.0 + 1 == symbol_position.0)
    {
        return true;
    }
    // LEFT-DOWN
    if part_position.0 > 0
        && part_position.1 > 0
        && (part_position.1 - 1 == symbol_position.1 && part_position.0 - 1 == symbol_position.0)
    {
        return true;
    }
    false
}

pub fn validate_part_number(part_number: &PartNumber, symbol_matches: Vec<(usize, usize)>) -> bool {
    part_number
        .positions
        .iter()
        .map(|&p| {
            symbol_matches
                .iter()
                .filter(|&s| generate_symbols_matches(p, *s))
                .count()
        })
        .sum::<usize>()
        > 0
}

pub fn generate_part_number_positions(
    row: usize,
    col: usize,
    part_number: &str,
) -> Vec<(usize, usize)> {
    // 467 -> [(0,0),(0,1),(0,2)]
    let mut col_position = col;
    part_number
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|&_c| {
            let curr_position = col_position;
            col_position += 1;
            (row, curr_position)
        })
        .collect()
}

pub fn get_part_numbers(lines: Vec<&str>) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    // Iterate over the rows
    for i in 0..lines.len() {
        let row = lines[i].chars().collect::<Vec<char>>();
        let mut rolling_part_number = String::from("");
        let mut starting_position = 0;
        for j in 0..row.len() {
            let value = row[j];
            if value.is_digit(10) {
                if rolling_part_number.is_empty() {
                    starting_position = j;
                }
                rolling_part_number.push(value);
            }
            if !value.is_digit(10) {
                if !rolling_part_number.is_empty() {
                    part_numbers.push(PartNumber {
                        value: rolling_part_number.parse::<usize>().unwrap(),
                        positions: generate_part_number_positions(
                            i,
                            starting_position,
                            rolling_part_number.as_str(),
                        ),
                    });
                    rolling_part_number.clear();
                }
            }
        }
        if !rolling_part_number.is_empty() {
            part_numbers.push(PartNumber {
                value: rolling_part_number.parse::<usize>().unwrap(),
                positions: generate_part_number_positions(
                    i,
                    starting_position,
                    rolling_part_number.as_str(),
                ),
            });
            rolling_part_number.clear();
        }
    }
    part_numbers
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let symbols: Vec<(usize, usize)> =
        process_symbols(lines.iter().map(|&l| l.chars().collect()).collect());
    let part_numbers = get_part_numbers(lines);
    part_numbers
        .iter()
        .filter(|&p| return validate_part_number(p, symbols.clone()))
        .map(|p| p.value)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_part_number_positions() {
        assert_eq!(
            generate_part_number_positions(0, 0, "123"),
            vec![(0, 0), (0, 1), (0, 2)]
        );
    }

    #[test]
    fn test_process_input() {
        let result = get_part_numbers("123...617*...#...*123...*10*".split("\n").collect());
        assert_eq!(
            result.iter().map(|p| { p.value }).collect::<Vec<usize>>(),
            vec![123, 617, 123, 10]
        )
    }

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_one(input), 4361);
    }
}
