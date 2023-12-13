pub fn part_one(input: &str) -> usize {
    let mut lines = input.split("\n").collect::<Vec<&str>>();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input_one = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(21, part_one(input_one));
    }
}
