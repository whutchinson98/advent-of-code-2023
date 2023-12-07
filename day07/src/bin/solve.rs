use std::fs;
use day07::part_one;

fn main() {
    let contents =
        fs::read_to_string("./input.txt").expect("Should have been able to read the file");

    println!("Part 1: {}", part_one(&contents));
}
