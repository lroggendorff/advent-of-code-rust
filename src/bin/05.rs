use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    // build a map where each key contains a set of all the pages which must not come after it
    for rule in parts[0].lines() {
        let r: Vec<&str> = rule.split("|").collect();
        rules.entry(r[1]).or_insert(HashSet::new()).insert(r[0]);
    }
    println!("{:?}", rules);
    let updates = parts[1].lines();
    let middle_pages = 0;
    // for update in updates {
    //     let pages: Vec<&str> = update.split(",").collect();
    //     for page in pages {
    //         match rules.get(page) {
    //             Some(&rule) => println!("{}", rule),
    //             _ => println!("nope"),
    //         }
    //     }
    // }
    Some(0)
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
