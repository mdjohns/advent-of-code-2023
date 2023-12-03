use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    cube_count: HashMap<Color, u32>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split(':');

        let id = iter
            .next()
            .expect("should exist")
            .trim_start_matches("Game ")
            .parse::<u32>()
            .expect("Game ID should be valid");

        let game = iter.next().expect("Game should come after ID").trim();
        let cube_count: HashMap<Color, u32> =
            game.split(';')
                .flat_map(|set| set.split(','))
                .fold(HashMap::new(), |mut acc, e| {
                    let mut iter = e.split_whitespace();
                    let count = iter
                        .next()
                        .expect("count should exist")
                        .parse::<u32>()
                        .expect("count should parse");

                    let color =
                        Color::from_str(iter.next().expect("color should come after count"))
                            .expect("color should parse");

                    if let Some(existing_count) = acc.get(&color) {
                        if &count > existing_count {
                            acc.insert(color, count);
                        }
                    } else {
                        acc.insert(color, count);
                    }

                    acc
                });

        Self { id, cube_count }
    }

    fn cube_power(&self) -> u32 {
        self.cube_count.values().product()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let max_red_count = 12;
    let max_green_count = 13;
    let max_blue_count = 14;

    Some(
        input
            .lines()
            .map(|line| Game::from_line(line))
            .filter_map(|game| {
                let actual_red_count = game.cube_count.get(&Color::Red).unwrap_or(&0);
                let actual_green_count = game.cube_count.get(&Color::Green).unwrap_or(&0);
                let actual_blue_count = game.cube_count.get(&Color::Blue).unwrap_or(&0);

                if &max_red_count >= actual_red_count
                    && &max_green_count >= actual_green_count
                    && &max_blue_count >= actual_blue_count
                {
                    return Some(game.id);
                }
                None
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| Game::from_line(line).cube_power())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut cube_count = HashMap::new();
        cube_count.insert(Color::Red, 4);
        cube_count.insert(Color::Green, 2);
        cube_count.insert(Color::Blue, 6);

        let expected = Game { id: 1, cube_count };

        let result = Game::from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
