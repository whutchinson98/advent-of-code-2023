#[derive(Debug)]
pub struct Island {
    rows: Vec<String>,
    cols: Vec<String>,
}

#[derive(Debug)]
pub enum MirrorType {
    Row,
    Col,
}

impl Island {
    fn new(rows: Vec<String>) -> Island {
        // Determine the number of columns in the first row
        let num_columns = rows[0].chars().count();

        // Initialize a vector for each column
        let mut cols = vec![String::new(); num_columns];

        // Iterate over each row
        for row in rows.clone() {
            // Split the row into columns
            for (i, value) in row.chars().enumerate() {
                // Append the value to the corresponding column
                if i < cols.len() {
                    cols[i].push_str(&value.to_string());
                }
            }
        }
        Island { rows, cols }
    }
}

pub fn check_fold(strings: &Vec<String>, left: usize, right: usize) -> bool {
    let mut i = left;
    let mut j = right;
    loop {
        let l = &strings[i];
        let r = &strings[j];
        if !l.eq(r) {
            return false;
        }
        if i == 0 || j == strings.len() - 1 {
            return true;
        }
        i -= 1;
        j += 1;
    }
}

pub fn check_for_mirrors(strings: &Vec<String>) -> Option<usize> {
    let len = strings.len();
    let mut right = 1;

    for left in 0..len {
        let l = &strings[left];
        let r = &strings[right];

        if l.eq(r) {
            if check_fold(strings, left, right) {
                return Some(left + 1);
            }
        }
        right += 1;

        if right >= len {
            return None;
        }
    }
    None
}

impl Island {
    fn find_mirror(&self, mirror_type: MirrorType) -> Option<usize> {
        match mirror_type {
            MirrorType::Row => check_for_mirrors(&self.rows),
            MirrorType::Col => check_for_mirrors(&self.cols),
        }
    }

    fn calculate_mirror_result(&self) -> usize {
        let row_mirror = self.find_mirror(MirrorType::Row);
        if row_mirror.is_none() {
            return self.find_mirror(MirrorType::Col).unwrap();
        }
        return row_mirror.unwrap() * 100;
    }
}

pub fn process_input(lines: Vec<&str>) -> Vec<Island> {
    lines
        .iter()
        .map(|line| {
            let island: Vec<String> = line
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|l| l.to_string())
                .collect();
            let rows: Vec<String> = island.clone();
            let result = Island::new(rows);
            result
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    process_input(lines)
        .iter()
        .map(|island| island.calculate_mirror_result()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
        #[test]
        fn test_part_one() {
            let input_one = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
            assert_eq!(405, part_one(input_one));
        }
}
