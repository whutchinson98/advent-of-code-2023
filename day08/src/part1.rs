use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub fn process_input(lines: Vec<&str>) -> (Vec<Direction>, BTreeMap<&str, (&str, &str)>) {
    let mut nodes: BTreeMap<&str, (&str, &str)> = BTreeMap::new();

    let directions: Vec<Direction> = lines[0]
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction {}", c),
        })
        .collect::<Vec<Direction>>();
    lines.iter().skip(2).for_each(|line| {
        let (node, children) = line.split_once(" = ").unwrap();
        let children = children
            .split(", ")
            .map(|child| child.trim_matches(|c| c == '(' || c == ')'))
            .collect::<Vec<&str>>();
        nodes.insert(node, (children[0], children[1]));
    });
    (directions, nodes)
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let (directions, nodes) = process_input(lines);

    let mut current_node = "AAA";
    let count: Option<usize> = directions
        .iter()
        .cycle()
        .enumerate()
        .find_map(|(index, dir)| {
            if current_node == "ZZZ" {
                return Some(index);
            }
            let node = nodes.get(current_node).expect("node is valid");
            match dir {
                Direction::Left => {
                    current_node = node.0;
                },
                Direction::Right => {
                    current_node = node.1;
                }
            }
            None
        });

    if count.is_none() {
        panic!("count is none");
    }

    count.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input_one = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input_two = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input_one), 2);
        assert_eq!(part_one(input_two), 6);
    }
}
