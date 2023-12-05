use rayon::prelude::*;

pub fn get_seeds(seed_line: &str) -> Vec<usize> {
    seed_line.split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

pub fn get_seeds_part_two(seed_line: &str) -> Vec<(i64, i64)> {
    seed_line.split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|s| (s[0].parse::<i64>().unwrap(), s[1].parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>()
}

pub fn process_map_ranges(lines: Vec<&str>) -> Vec<Vec<(usize, usize, usize)>> {
    let mut seed_to_soil_range = (0, 0);
    let mut soil_to_fertilizer_range = (0, 0);
    let mut fertilizer_to_water_range = (0, 0);
    let mut water_to_light_range = (0, 0);
    let mut light_to_temperature_range = (0, 0);
    let mut temperature_to_humidity_range = (0, 0);
    let mut humidity_to_location_range = (0, lines.len());

    for i in 0..lines.len() {
        // First line is the seeds line
        if i == 0 {
            continue;
        }
        match lines[i] {
            "seed-to-soil map:" => seed_to_soil_range.0 = i + 1,
            "soil-to-fertilizer map:" => {
                seed_to_soil_range.1 = i;
                soil_to_fertilizer_range.0 = i + 1
            }
            "fertilizer-to-water map:" => {
                soil_to_fertilizer_range.1 = i;
                fertilizer_to_water_range.0 = i + 1
            }
            "water-to-light map:" => {
                fertilizer_to_water_range.1 = i;
                water_to_light_range.0 = i + 1
            }
            "light-to-temperature map:" => {
                water_to_light_range.1 = i;
                light_to_temperature_range.0 = i + 1
            }
            "temperature-to-humidity map:" => {
                light_to_temperature_range.1 = i;
                temperature_to_humidity_range.0 = i + 1
            }
            "humidity-to-location map:" => {
                temperature_to_humidity_range.1 = i;
                humidity_to_location_range.0 = i + 1
            }
            _ => {}
        }
    }

    let mut seed_to_soil_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in seed_to_soil_range.0..seed_to_soil_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        seed_to_soil_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut soil_to_fertilizer_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in soil_to_fertilizer_range.0..soil_to_fertilizer_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        soil_to_fertilizer_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut fertilizer_to_water_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in fertilizer_to_water_range.0..fertilizer_to_water_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        fertilizer_to_water_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut water_to_light_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in water_to_light_range.0..water_to_light_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        water_to_light_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut light_to_temperature_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in light_to_temperature_range.0..light_to_temperature_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        light_to_temperature_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut temperature_to_humidity_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in temperature_to_humidity_range.0..temperature_to_humidity_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        temperature_to_humidity_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }
    let mut humidity_to_location_map_lines: Vec<(usize, usize, usize)> = Vec::new();

    for i in humidity_to_location_range.0..humidity_to_location_range.1 {
        let parse = lines[i].split(" ").collect::<Vec<&str>>();
        humidity_to_location_map_lines.push((
            parse[0].parse::<usize>().unwrap(),
            parse[1].parse::<usize>().unwrap(),
            parse[2].parse::<usize>().unwrap(),
        ))
    }

    vec![
        seed_to_soil_map_lines,
        soil_to_fertilizer_map_lines,
        fertilizer_to_water_map_lines,
        water_to_light_map_lines,
        light_to_temperature_map_lines,
        temperature_to_humidity_map_lines,
        humidity_to_location_map_lines,
    ]
}

// Converts a seed number to its corresponding location number.
fn convert_to_location(seed: usize, map_lines: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let soil = map_category(seed, &map_lines[0]);
    let fertilizer = map_category(soil, &map_lines[1]);
    let water = map_category(fertilizer, &map_lines[2]);
    let light = map_category(water, &map_lines[3]);
    let temperature = map_category(light, &map_lines[4]);
    let humidity = map_category(temperature, &map_lines[5]);
    map_category(humidity, &map_lines[6])
}

fn map_category(num: usize, mappings: &Vec<(usize, usize, usize)>) -> usize {
    for &(dest_start, src_start, length) in mappings {
        if num >= src_start && num < src_start + length {
            return dest_start + (num - src_start);
        }
    }
    // Return num by default
    num
}

pub fn part_one(input: &str) -> usize {
    // Remove the white lines separating the sections
    let lines: Vec<&str> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| !s.is_empty())
        .map(|&s| s)
        .collect();

    let seeds: Vec<usize> = get_seeds(lines[0]);
    let map_lines = process_map_ranges(lines);

    let locations = seeds
        .iter()
        .map(|&seed| convert_to_location(seed, &map_lines))
        .collect::<Vec<usize>>();
    *locations.iter().min().unwrap()
}

pub fn part_two(input: &str) -> i64 {
    // Remove the white lines separating the sections
    let lines: Vec<&str> = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .filter(|&s| !s.is_empty())
        .map(|&s| s)
        .collect();

    let seed_ranges: Vec<(i64, i64)> = get_seeds_part_two(lines[0]);
    let map_lines = process_map_ranges(lines);
    let lowest_location = std::sync::atomic::AtomicI64::new(i64::MAX);

    seed_ranges.par_iter().for_each(|seed_range| {
        let start = seed_range.0;
        let length = seed_range.1;

        // Iterate through all the seed values
        for i in start..=start + length {
            let next = transform_seed_value(i, &map_lines);
            let current_lowest = lowest_location.load(std::sync::atomic::Ordering::Relaxed);
            if next < current_lowest {
                lowest_location.store(next, std::sync::atomic::Ordering::Relaxed);
            }
        }
    });

    lowest_location.load(std::sync::atomic::Ordering::Relaxed)
}

fn transform_seed_value(mut value: i64, map_lines: &Vec<Vec<(usize, usize, usize)>>) -> i64 {
    for mapping in map_lines {
        for &map in mapping {
            if let Some(new_value) = get_potential_new_value((map.0 as i64, map.1 as i64, map.2 as i64), value) {
                value = new_value;
                break;
            }
        }
    }
    value
}

fn get_potential_new_value(map: (i64, i64, i64), from: i64) -> Option<i64> {
    if from >= map.1 && from < map.1 + map.2 {
        Some(from + (map.0 - map.1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_seeds() {
        let seed_line = "seeds: 79 14 55 13";
        assert_eq!(get_seeds(seed_line), vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_part_one() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_one(input), 35);
    }

    #[test]
    fn test_part_two() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(part_two(input), 46);
    }
}
