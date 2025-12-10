advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let idx_1 = line
                    .chars()
                    .take(line.len().saturating_sub(1))
                    .enumerate() // Ignore last element
                    .fold(0usize, |max_idx, (idx, c)| {
                        if c > line.chars().nth(max_idx).unwrap() {
                            return idx;
                        }
                        max_idx
                    });
                let idx_2 = line
                    .chars()
                    .enumerate()
                    .skip(idx_1 + 1) // Ignore everything up to the first number
                    .fold(idx_1 + 1, |max_idx, (idx, c)| {
                        if c > line.chars().nth(max_idx).unwrap() {
                            return idx;
                        }
                        max_idx
                    });
                format!(
                    "{}{}",
                    line.chars().nth(idx_1).unwrap(),
                    line.chars().nth(idx_2).unwrap()
                )
                .parse::<u64>()
                .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
