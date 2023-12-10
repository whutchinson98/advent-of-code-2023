use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

pub fn process_input(lines: Vec<&str>) -> HashMap<(i64, i64), TileType> {
    let mut map: HashMap<(i64, i64), TileType> = HashMap::new();
    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(column, c)| {
            let tile = match c {
                '|' => TileType::Vertical,
                '-' => TileType::Horizontal,
                'L' => TileType::NorthEast,
                'J' => TileType::NorthWest,
                '7' => TileType::SouthWest,
                'F' => TileType::SouthEast,
                '.' => TileType::Ground,
                'S' => TileType::Start,
                _ => panic!("Invalid character in input"),
            };
            if tile != TileType::Ground {
                map.insert((column as i64, row as i64), tile);
            }
        });
    });
    map
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let grid = process_input(lines);
    let start_position = grid
        .iter()
        .find_map(|(key, value)| (value == &TileType::Start).then_some(key))
        .expect("start exists");

    // Up
    let north: (i64, i64) = (start_position.0, start_position.1 - 1);
    let north_position = grid
        .get(&north)
        .is_some_and(|pipe_type| match pipe_type {
            TileType::Vertical | TileType::SouthWest | TileType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::South, north));
    // Right to go east
    let east = (start_position.0 + 1, start_position.1);
    let east_position = grid
        .get(&east)
        .is_some_and(|pipe_type| match pipe_type {
            TileType::Horizontal | TileType::NorthWest | TileType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::West, east));
    // Down to go south
    let south = (start_position.0, start_position.1 + 1);
    let south_position = grid
        .get(&south)
        .is_some_and(|pipe_type| match pipe_type {
            TileType::Vertical | TileType::NorthWest | TileType::NorthEast => true,
            _ => false,
        })
        .then_some((Direction::North, south));
    // Left to go west
    let west = (start_position.0 - 1, start_position.1);
    let west_position = grid
        .get(&west)
        .is_some_and(|pipe_type| match pipe_type {
            TileType::Horizontal | TileType::NorthEast | TileType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::East, west));
    let mut iters = vec![north_position, south_position, east_position, west_position]
        .into_iter()
        .flatten()
        .map(|tuple| {
            std::iter::successors(Some(tuple), |(from_direction, current_position)| {
                let pipe_type = grid
                    .get(current_position)
                    .expect("should not be asking for a grid position that doesn't exist");

                let direction_to_go = match (from_direction, pipe_type) {
                    (Direction::North, TileType::Vertical) => Direction::South,
                    (Direction::North, TileType::NorthEast) => Direction::East,
                    (Direction::North, TileType::NorthWest) => Direction::West,
                    (Direction::South, TileType::Vertical) => Direction::North,
                    (Direction::South, TileType::SouthEast) => Direction::East,
                    (Direction::South, TileType::SouthWest) => Direction::West,
                    (Direction::East, TileType::Horizontal) => Direction::West,
                    (Direction::East, TileType::NorthEast) => Direction::North,
                    (Direction::East, TileType::SouthEast) => Direction::South,
                    (Direction::West, TileType::Horizontal) => Direction::East,
                    (Direction::West, TileType::NorthWest) => Direction::North,
                    (Direction::West, TileType::SouthWest) => Direction::South,
                    value => {
                        panic!("uncaught case: {:?}", value);
                    }
                };
                Some(match direction_to_go {
                    Direction::North => (
                        Direction::South,
                        (current_position.0, current_position.1 - 1),
                    ),
                    Direction::South => (
                        Direction::North,
                        (current_position.0, current_position.1 + 1),
                    ),
                    Direction::East => (
                        Direction::West,
                        (current_position.0 + 1, current_position.1),
                    ),
                    Direction::West => (
                        Direction::East,
                        (current_position.0 - 1, current_position.1),
                    ),
                })
            })
        });

    // There should always be 2 paths as per the AOC problem
    if iters.clone().count() != 2 {
        panic!("should have exactly 2 paths");
    }

    let path_a = iters.next().expect("path a should_exist");
    let path_b = iters.next().expect("path b should exist");
    let final_position = path_a
        // Combines to iterators into a single iterator of pairs
        .zip(path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("should meet in the middle");

    // + 1 for the initial starting position
    final_position + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input_one = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, part_one(input_one));
        let input_two = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, part_one(input_two));
    }
}
