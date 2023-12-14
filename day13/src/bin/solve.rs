use std::fs;
use day13::{part1::part_one, part2::part_two};

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    println!("Part 1: {}", part_one(&contents));
    println!("Part 2: {}", part_two(&contents));
}
