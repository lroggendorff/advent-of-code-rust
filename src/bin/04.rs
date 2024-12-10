advent_of_code::solution!(4);


pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let m: Vec<Vec<char>> = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut found = 0;
    for (x, _) in m.iter().enumerate() {
        for (y, _) in m[x].iter().enumerate() {
            if m[x][y] != 'X' { continue; }

            // look right →
            if y < m[x].len().checked_sub(3).unwrap() {
                if m[x][y + 1] == 'M' && m[x][y + 2] == 'A' && m[x][y + 3] == 'S' {
                    found += 1;
                }
            }

            // ← look left
            if y > 2 {
                if m[x][y - 1] == 'M' && m[x][y - 2] == 'A' && m[x][y - 3] == 'S' {
                    found += 1;
                }
            }

            // ↓ look down
            if x < m.len().checked_sub(3).unwrap() {
                if m[x + 1][y] == 'M' && m[x + 2][y] == 'A' && m[x + 3][y] == 'S' {
                    found += 1;
                }
            }

            // ↑ look up
            if x > 2 {
                if m[x - 1][y] == 'M' && m[x - 2][y] == 'A' && m[x - 3][y] == 'S' {
                    found += 1;
                }
            }

            // ↖︎ look diagonally up and left
            if x > 2 && y > 2 {
                if m[x - 1][y - 1] == 'M' && m[x - 2][y - 2] == 'A' && m[x - 3][y - 3] == 'S' {
                    found += 1;
                }
            }

            // ↗︎ look diagonally up and right
            if x > 2 &&  y < m[x].len().checked_sub(3).unwrap() {
                if m[x - 1][y + 1] == 'M' && m[x - 2][y + 2] == 'A' && m[x - 3][y + 3] == 'S' {
                    found += 1;
                }
            }

            // ↘︎ look diagonally down and right
            if x < m.len().checked_sub(3).unwrap() &&  y < m[x].len().checked_sub(3).unwrap() {
                if m[x + 1][y + 1] == 'M' && m[x + 2][y + 2] == 'A' && m[x + 3][y + 3] == 'S' {
                    found += 1;
                }
            }

            // ↙︎ look diagonally down and left
            if x < m.len().checked_sub(3).unwrap() &&  y > 2 {
                if m[x + 1][y - 1] == 'M' && m[x + 2][y - 2] == 'A' && m[x + 3][y - 3] == 'S' {
                    found += 1;
                }
            }
        }
    }
    Some(found)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let m: Vec<Vec<char>> = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut found = 0;
    for (x, _) in m.iter().enumerate() {
        for (y, _) in m[x].iter().enumerate() {
            if m[x][y] != 'A' { continue; }

            // make sure we're at least one row or column away from the edge
            if x < 1 || y < 1 || x >= m.len().checked_sub(1).unwrap() || y >= m.len().checked_sub(1).unwrap() {
                continue;
            }

            // ↖︎ ↗︎ ↘︎ ↙︎ check all the arms
            if (m[x - 1][y - 1] == 'M'
                && m[x - 1][y + 1] == 'M'
                && m[x + 1][y - 1] == 'S'
                && m[x + 1][y + 1] == 'S')
                || (m[x - 1][y - 1] == 'S'
                && m[x - 1][y + 1] == 'S'
                && m[x + 1][y - 1] == 'M'
                && m[x + 1][y + 1] == 'M')
                || (m[x - 1][y - 1] == 'M'
                && m[x - 1][y + 1] == 'S'
                && m[x + 1][y - 1] == 'M'
                && m[x + 1][y + 1] == 'S')
                || (m[x - 1][y - 1] == 'S'
                && m[x - 1][y + 1] == 'M'
                && m[x + 1][y - 1] == 'S'
                && m[x + 1][y + 1] == 'M')
            {
                found += 1;
            }
        }
    }
    Some(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
