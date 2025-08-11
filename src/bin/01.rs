advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<u32>().unwrap());
        right.push(items.next().unwrap().parse::<u32>().unwrap());
    }

    let mut sum: u32 = 0;
    for left_number in left.iter() {
        let right_occurrences = right.iter().filter(|i| *i == left_number).count();
        sum += left_number * right_occurrences as u32;
    }

    Some(sum)
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
