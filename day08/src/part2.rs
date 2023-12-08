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

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_two(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let (directions, nodes) = process_input(lines);
let starting_nodes: Vec<&str> = nodes
        .keys()
        .filter(|key| key.ends_with("A"))
        .cloned()
        .collect();

    let results = starting_nodes
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;
            // cycle is magically "the first Z",
            // and other assorted conditions due
            // to how the input is constructed.
            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options =
                        nodes.get(current_node).expect(
                            "always exist at a valid node",
                        );
                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };
                    if next_node.ends_with("Z") {
                        Some(index + 1)
                    } else {
                        visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();

    let min_cycle = lcm(&results);

    min_cycle
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_two() {
        let input_one = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_two(input_one), 6);
    }
}
