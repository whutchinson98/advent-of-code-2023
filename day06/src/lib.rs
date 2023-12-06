#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn get_possible_times(&self) -> Vec<u64> {
        let mut results: Vec<u64> = Vec::new();
        for i in 0..self.time+1 {
            let possible_distance = i * (self.time - i);
            if possible_distance > self.distance {
                results.push(possible_distance);
            }
        }
        results
    }
}

pub fn process_input(lines: Vec<&str>) -> Vec<Race> {
    let times: Vec<u64> = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .map(|time| time.parse::<u64>().unwrap())
        .collect();

    times
        .iter()
        .zip(distances)
        .map(|(&time, distance)| Race { time, distance })
        .collect()
}

pub fn process_input_part_two(lines: Vec<&str>) -> Race {
    let time: u64 = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>().join("").parse::<u64>().unwrap();

    let distance: u64 = lines[1].split(":").collect::<Vec<&str>>()[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>().join("").parse::<u64>().unwrap();

    Race{
        time,
        distance
    }
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let races = process_input(lines);
    races.iter().map(|r| {
        r.get_possible_times().iter().count()
    }).product()
}

pub fn part_two(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    process_input_part_two(lines).get_possible_times().iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_one(input), 288);
    }
    #[test]
    fn test_part_two() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_two(input), 71503);
    }

}
