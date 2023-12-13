use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    graph: HashMap<(i64, i64), i64>,
    nodes: Vec<Vec<Node>>,
}

impl Grid {
    fn print_universe(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|l| {
                let mut s = String::from("");
                l.iter().for_each(|c| {
                    s.push(c.symbol);
                });
                s
            })
            .collect()
    }
    // vec of empty row
    fn find_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows: Vec<usize> = Vec::new();
        self.nodes.iter().enumerate().for_each(|(index, row)| {
            if row.iter().filter(|x| x.symbol == '#').count() == 0 {
                empty_rows.push(index);
            }
        });
        empty_rows
    }

    // vec of empty cols
    fn find_empty_cols(&self) -> Vec<usize> {
        // Represents the count of items that are not . in a given col
        let mut col_items: Vec<usize> = vec![0; self.nodes[0].len()];
        self.nodes.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(y, c)| match c.symbol {
                '#' => col_items[y] += 1,
                _ => {}
            });
        });

        col_items
            .iter()
            .enumerate()
            .filter_map(|(index, &v)| {
                if v == 0 {
                    return Some(index);
                }
                None
            })
            .collect()
    }

    fn expand_universe(&self) -> Grid {
        let mut expanded_universe = self.nodes.clone();
        let mut offset: usize = 0;

        // Expand rows
        for &row_index in &self.find_empty_rows() {
            let empty_row = vec![Node::new(); self.nodes[0].len()]; // Create an empty row with the same length as other rows
            expanded_universe.insert(row_index + offset, empty_row);
            offset += 1;
        }

        // Expand columns
        for &col_index in self.find_empty_cols().iter().rev() {
            for row in expanded_universe.iter_mut() {
                let node = Node::new();
                row.insert(col_index, node); // Insert a duplicate node in each row
            }
        }
        Grid {
            nodes: expanded_universe,
            graph: self.graph.clone(),
        }
    }

    // Method to find all galaxy pairs
    fn galaxy_pairs(&self) -> Vec<(Node, Node)> {
        // Generate all possible pairs
        todo!()
    }

    // Method to calculate the shortest path between two galaxies
    fn shortest_path(&self, start: Node, end: Node) -> usize {
        // Calculate shortest path length
        todo!()
    }

    // Method to sum the path lengths for all galaxy pairs
    fn sum_of_paths(&self) -> usize {
        // Iterate over galaxy pairs and sum their path lengths
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Node {
    id: Option<i64>,
    symbol: char,
}

impl Node {
    fn new() -> Self {
        Node {
            id: None,
            symbol: '.',
        }
    }
}

pub fn process(lines: Vec<&str>) -> Grid {
    let mut nodes = Vec::new();
    let mut graph = HashMap::new();
    let mut unique_id = 1;
    lines.iter().enumerate().for_each(|(y, line)| {
        let mut row = Vec::new();
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                graph.insert((x as i64, y as i64), unique_id);
                row.push(Node {
                    id: Some(unique_id),
                    symbol: c,
                });
                unique_id += 1;
            } else {
                row.push(Node {
                    id: None,
                    symbol: c,
                });
            }
        });
        nodes.push(row);
    });
    Grid { nodes, graph }
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let mut grid = process(lines);

    grid = grid.expand_universe();

    grid.nodes.iter().for_each(|l| {
        let mut s = String::from("");
        l.iter().for_each(|c| {
            s.push(c.symbol);
        });
        println!("{}", s);
    });

    todo!()
}

/**
 * galaxies!/2
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input_one = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, part_one(input_one));
    }

    #[test]
    fn test_expanded_universe() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let grid = process(input.split("\n").collect::<Vec<&str>>());
        assert_eq!(
            expected
                .split("\n")
                .map(|s| String::from(s))
                .collect::<Vec<String>>(),
            grid.expand_universe().print_universe(),
        );
    }
}
