use std::cmp::Ordering;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.split("\n")
        .filter(|line| line.trim() != "")
        .map(|elem| count_unsafe_levels(elem))
        .filter(|&unsafe_levels| unsafe_levels == 0)
        .count() as u32;

    Some(result)
}

fn count_unsafe_levels(line: &str) -> u32 {
    let mut messages = vec![];
    let fold = line.split_whitespace()
        .map(|num| str::parse::<i32>(num).expect("Failed to parse as number"))
        .enumerate()
        .fold((0, -1, Direction::Neutral), |(unsafe_count, prev, prev_dir), (index, num)| {
            let diff = prev - num;
            let dir = if prev_dir == Direction::Neutral && prev == -1 {
                Direction::Neutral
            } else {
                match prev.cmp(&num) {
                    Ordering::Greater => Direction::Desc,
                    Ordering::Less => Direction::Asc,
                    Ordering::Equal => Direction::Neutral,
                }
            };

            let result = if index == 0 {
                messages.push(format!("index 0"));
                (unsafe_count, num, Direction::Neutral)
            } else if diff.abs() > 3 {
                messages.push(format!("False because diff too high"));
                (unsafe_count + 1, num, dir)
            } else if index != 1 && prev_dir != dir {
                messages.push(format!("False because direction change"));

                (unsafe_count + 1, num, dir)
            } else if prev_dir != Direction::Neutral && dir == Direction::Neutral {
                messages.push(format!("False because no increase or decrease"));

                (unsafe_count + 1, num, dir)
            } else {
                (unsafe_count, num, dir)
            };

            result
        });

    fold.0
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Asc,
    Desc,
    Neutral,
}

pub fn part_two(input: &str) -> Option<u32> {
   let result = input.split("\n")
        .filter(|line| line.trim() != "")
        .map(|elem| count_unsafe_levels(elem))
        .filter(|&unsafe_levels| unsafe_levels <= 1)
        .count() as u32;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
