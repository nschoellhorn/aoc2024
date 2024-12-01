advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.split("\n").into_iter()
        .flat_map(|line| line.split_whitespace())
        .enumerate()
        .map(|(index, element)| (index, str::parse::<u32>(element).expect("Failed to convert to number")))
        .for_each(|(index, element)| {
            if index % 2 == 0 {
                left_list.push(element);
            } else {
                right_list.push(element);
            }
        });

    left_list.sort();
    right_list.sort();

    let result = left_list.into_iter().zip(right_list)
        .map(|(left, right)| u32::abs_diff(left, right))
        .reduce(|acc, e| acc + e);

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left_list = vec![];
    let mut right_list = vec![];

    input.split("\n").into_iter()
        .flat_map(|line| line.split_whitespace())
        .enumerate()
        .map(|(index, element)| (index, str::parse::<u32>(element).expect("Failed to convert to number")))
        .for_each(|(index, element)| {
            if index % 2 == 0 {
                left_list.push(element);
            } else {
                right_list.push(element);
            }
        });

    let test = left_list.into_iter()
        .map(|element| (element, right_list.iter().filter(|&re| *re == element).count() as u32))
        .map(|(num, count)| num * count);

    let result = test.reduce(|acc, e| acc + e);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
