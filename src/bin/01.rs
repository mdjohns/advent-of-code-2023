advent_of_code::solution!(1);

fn digits_from_line(line: &str) -> u32 {
    let chars = line.chars().collect::<Vec<char>>();

    let mut left_ptr = 0;
    let mut right_ptr = chars.len() - 1;
    let mut result = 0;

    while left_ptr <= right_ptr {
        let left = chars[left_ptr].to_digit(10);
        let right = chars[right_ptr].to_digit(10);

        if let Some(left_num) = left {
            if let Some(right_num) = right {
                result = format!("{}{}", left_num, right_num)
                    .parse::<u32>()
                    .expect("concatenating two digits should successfully parse");
                break;
            } else {
                right_ptr -= 1;
            }
        } else if let Some(_) = right {
            left_ptr += 1;
        } else {
            left_ptr += 1;
            right_ptr -= 1;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|line| digits_from_line(line)).sum())
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
