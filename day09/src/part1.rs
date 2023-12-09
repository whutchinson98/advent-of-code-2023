#[derive(Debug, Clone)]
pub struct Sensor {
    readings: Vec<i64>,
}

impl Sensor {
    fn new(readings: Vec<i64>) -> Self {
        Sensor { readings }
    }

    fn are_differences_stable(&self, readings: &Vec<i64>) -> bool {
        let current = readings[0];
        for i in 0..readings.len() {
            if readings[i] != current {
                return false;
            }
        }
        true
    }

    fn get_differences(&self, readings: &Vec<i64>) -> Vec<i64> {
        readings.windows(2).map(|r| r[1] - r[0]).collect()
    }

    fn predict_next(&self) -> i64 {
        let mut stages: Vec<Vec<i64>> = Vec::new();
        let mut readings = self.readings.clone();
        // We need to see if all the differences are the same
        while !self.are_differences_stable(&readings) {
            // Update stages with the current reading
            stages.push(readings.clone());
            // Update readings
            readings = self.get_differences(&readings);
        }
        stages.push(readings);
        stages.reverse();
        let mut prediction: i64 = 0;
        stages.iter().for_each(|stage| {
            prediction += stage.last().unwrap();
        });
        prediction
    }
}

pub fn process_input(lines: Vec<&str>) -> Vec<Sensor> {
    lines
        .iter()
        .map(|&sensor| {
            Sensor::new(
                sensor
                    .split(" ")
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> i64 {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let sensors = process_input(lines);
    sensors.iter().map(|s| s.predict_next()).sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input_one = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(part_one(input_one), 114);
    }
}
