advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;
    let safely_increasing: Vec<_> = (1..4).collect();
    let safely_decreasing: Vec<_> = (-3..0).collect();
    for line in input.lines() {
        let sequence: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut is_safely_increasing = true;
        let mut is_safely_decreasing = true;

        for (i, _) in sequence.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let difference = sequence[i] - sequence[i - 1];
            if !safely_increasing.iter().any(|&v| difference == v) {
                is_safely_increasing = false
            }
        }

        for (i, _) in sequence.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let difference = sequence[i] - sequence[i - 1];
            if !safely_decreasing.iter().any(|&v| difference == v) {
                is_safely_decreasing = false
            }
        }

        if is_safely_increasing || is_safely_decreasing {
            safe += 1;
        }
    }

    Some(safe)
}

fn is_safely_increasing_pair(pair: &[i32]) -> bool {
    let safely_increasing: Vec<i32> = (1..4).collect();
    let difference = pair[1] - pair[0];
    return safely_increasing.iter().any(|&v| difference == v);
}

fn is_safely_increasing_sequence(sequence: Vec<i32>) -> bool {
    return sequence.windows(2).all(is_safely_increasing_pair);
}

fn is_safely_decreasing_pair(pair: &[i32]) -> bool {
    let safely_decreasing: Vec<_> = (-3..0).collect();
    let difference = pair[1] - pair[0];
    return safely_decreasing.iter().any(|&v| difference == v);
}

fn is_safely_decreasing_sequence(sequence: Vec<i32>) -> bool {
    return sequence.windows(2).all(is_safely_decreasing_pair);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;
    for line in input.lines() {
        let sequence: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        for i in 0..sequence.len() {
            // for each item in sequence
            let skipper = sequence
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| i != *j) // skip the item
                .map(|(_, val)| val)
                .collect::<Vec<i32>>();

            if is_safely_increasing_sequence(skipper.clone())
                || is_safely_decreasing_sequence(skipper.clone())
            {
                safe += 1;
                break; // if the new sequence with a skipped value is safe, we're done
            }
        }
    }

    Some(safe)
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
    fn test_is_safely_increasing() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let sequence: Vec<i32> = input
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let result = is_safely_increasing_sequence(sequence);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_safely_decreasing() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let sequence: Vec<i32> = input
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let result = is_safely_decreasing_sequence(sequence);
        assert_eq!(result, true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
