advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut maybe_mul = "".to_string();
    let mut comma_seen = false;
    let mut left_operand = "".to_string();
    let mut right_operand = "".to_string();
    let mut sum = 0;
    for c in input.chars() {
        if c == ')' && left_operand != "" && right_operand != "" {
            let left = left_operand.parse::<u32>().unwrap();
            let right = right_operand.parse::<u32>().unwrap();

            sum += left * right;

            maybe_mul = "".to_string();
            left_operand = "".to_string();
            right_operand = "".to_string();
            comma_seen = false;
            continue;
        }

        if (maybe_mul == "" && c == 'm')
            || (maybe_mul == "m" && c == 'u')
            || (maybe_mul == "mu" && c == 'l')
            || (maybe_mul == "mul" && c == '(')
        {
            maybe_mul.push(c);
            continue;
        }

        if (maybe_mul == "mul" && c != '(') || (maybe_mul == "mul(" && c != ',' && !c.is_numeric())
        {
            maybe_mul = "".to_string();
            left_operand = "".to_string();
            right_operand = "".to_string();
            comma_seen = false;
            continue;
        }

        if maybe_mul == "mul(" && c == ',' {
            comma_seen = true;
            continue;
        }

        if maybe_mul == "mul(" && c.is_numeric() {
            if comma_seen {
                right_operand.push(c);
            } else {
                left_operand.push(c);
            }
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_one_found() {
        let result = part_one("mul(3,3)");
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_one_found_tricky() {
        let result = part_one("(mul(3,3)mul)");
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_one_found_super_tricky() {
        let result = part_one("mu(3,3]mul_+mul(3)mul(2,5)[&]");
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_found_more_tricky() {
        let result = part_one("#[mul(215,783)from()?(^mul(25,319)##&?,mul(922@when())/)%why()mul(544,417)what()(^{mul(137,343)}>:why()");
        assert_eq!(result, Some(450159));
    }

    #[test]
    fn test_part_one_found_another_tricky() {
        let result = part_one("4) !?;mul>:/from(945,813)/mul(996,850)!");
        assert_eq!(result, Some(846600));
    }

    #[test]
    fn test_part_one_found_ridiculous() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(170807108));
    }

    #[test]
    fn test_part_one_nope() {
        let result = part_one("mul(3,3]");
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
