use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
pub enum Status {
    In,
    Out,
}

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
            map.insert((column as i64, row as i64), tile);
        });
    });
    map
}

pub fn part_two(input: &str) -> usize {
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
    let mut zip_it = path_a.zip(path_b);
    let mut pipe_locations: HashSet<(i64,i64)> = HashSet::from([*start_position]);
    while let Some((path_a_node, path_b_node)) = zip_it.next() {
        pipe_locations.insert(path_a_node.1);
        pipe_locations.insert(path_b_node.1);

        if path_a_node.1 == path_b_node.1 {
            break;
        }
    }

    let result = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut status = Status::Out;

            line.chars()
                .enumerate()
                .filter(|(x, _)| {
                    let position = (*x as i64, y as i64);
                    let pipe_type = grid.get(&position).expect("should be a valid tile");
                    if pipe_locations.contains(&position) {
                        if [
                            TileType::Start,
                            TileType::Vertical,
                            TileType::SouthWest,
                            TileType::SouthEast,
                        ]
                        .contains(pipe_type)
                        {
                            status = match status {
                                Status::In => Status::Out,
                                Status::Out => Status::In,
                            };
                        };
                        false
                    } else {
                        match status {
                            Status::In => true,
                            Status::Out => false,
                        }
                    }
                })
                .count()
        })
        .sum::<usize>();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input_one = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4, part_two(input_one));
        let input_two = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8, part_two(input_two));
   let input_three = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10, part_two(input_three));
    }
}
