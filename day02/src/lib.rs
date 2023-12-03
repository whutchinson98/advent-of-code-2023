pub struct Game {
    id: usize,
    // Each round is a vector of cubes stored as RED, GREEN, BLUE
    rounds: Vec<Vec<Cube>>,
}

pub struct Cube {
    color: String,
    count: usize,
}

pub fn count_cubes(rounds: &Vec<Vec<Cube>>) -> (usize, usize, usize) {
    let mut max_reds = 0;
    let mut max_greens = 0;
    let mut max_blues = 0;

    rounds.iter().for_each(|r| {
        r.iter().for_each(|c| match c.color.as_str() {
            "red" => {
                if max_reds < c.count {
                    max_reds = c.count;
                }
            },
            "green" => {
                if max_greens < c.count {
                    max_greens = c.count;
                }
            },
            "blue" => {
                if max_blues < c.count {
                    max_blues = c.count;
                }
            },
            _ => unreachable!(),
        });
    });

    (max_reds, max_greens, max_blues)
}
pub fn create_cube(cube: &str) -> Cube {
    let c = cube.split(" ").collect::<Vec<&str>>();
    Cube {
        color: String::from(c[1]),
        count: c[0].parse::<usize>().unwrap(),
    }
}
// Takes a game and will create a vector of all the rounds of the game
// Example game: {"3 blue, 4 red","1 red, 2 green, 6 blue","2 green"}
pub fn process_game(game: Vec<&str>) -> Vec<Vec<Cube>> {
    game.iter()
        .map(|&round| {
            //[3 blue, 4 red|1 red, 2 green, 6 blue|2 green]
            round
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&r| create_cube(r))
                .collect()
        })
        .collect::<Vec<Vec<Cube>>>()
}

pub fn process_games(games: Vec<&str>) -> Vec<Game> {
    games
        .iter()
        .map(|&game| {
            let split: Vec<&str> = game.split(": ").collect();
            // Get game_id
            let game_id: usize = split[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            split[0].split(" ").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            Game {
                id: game_id,
                rounds: process_game(split[1].split("; ").collect()),
            }
        })
        .collect::<Vec<Game>>()
}

pub fn part_one(input: &str) -> usize {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let processed_games = process_games(lines);
    processed_games
        .iter()
        .filter(|&g| {
            let count = count_cubes(&g.rounds);
            // rounds is a vector of cubes
            count.0 <= 12 && count.1 <= 13 && count.2 <= 14
        })
        .map(|g| g.id)
        .sum::<usize>()
}

pub fn part_two(input: &str) -> usize {
    let mut lines: Vec<&str> = input.split("\n").collect();
    if lines[lines.len() - 1] == "" {
        lines.pop();
    }
    let processed_games = process_games(lines);
    processed_games
        .iter()
        .map(|g| {
            let count = count_cubes(&g.rounds);
            count.0 * count.1 * count.2
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_cubes_test() {
        let rounds: Vec<Vec<Cube>> = vec![
            vec![
                Cube {
                    color: String::from("blue"),
                    count: 7,
                },
                Cube {
                    color: String::from("red"),
                    count: 2,
                },
            ],
            vec![
                Cube {
                    color: String::from("green"),
                    count: 1,
                },
                Cube {
                    color: String::from("red"),
                    count: 5,
                },
            ],
            vec![Cube {
                color: String::from("blue"),
                count: 4,
            }],
        ];
        assert_eq!(count_cubes(&rounds), (5, 1, 7))
    }

    #[test]
    fn process_games_test() {
        let games = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let processed_games = process_games(games);
        assert_eq!(processed_games.len(), 5)
    }

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }
    #[test]
    fn part_two_test() {
        assert_eq!(
            part_two(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
