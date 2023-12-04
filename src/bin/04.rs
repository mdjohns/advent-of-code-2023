use std::collections::HashMap;

advent_of_code::solution!(4);

fn score_game(line: &str) -> u32 {
    let (_, game) = line.split_once(":").expect("line should be in game format");
    let (winning, played) = line.split_once("|").expect("game should have winning and played cards");

    let winning_map = winning.split_whitespace()
    .fold(HashMap::new(), |mut map, card| {
        map.insert(card, 1);

        map
    });

    played.split_whitespace().fold(0, |acc, card| {
        if let Some(_) = winning_map.get(card) {
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        } else {
            acc
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| score_game(line)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
