advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;
    let safely_increasing: Vec<_> = (1..4).collect();
    let safely_decreasing: Vec<_>  = (-3..0).collect();
    for line in input.lines() {
        let sequence: Vec<_> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
