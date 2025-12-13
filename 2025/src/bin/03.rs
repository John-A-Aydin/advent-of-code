advent_of_code::solution!(3);

pub trait MaxJoltage<const DIGITS: usize> {
    fn max_joltage(self) -> u32;
}

impl<I, const DIGITS: usize> MaxJoltage<DIGITS> for I
where
    I: Iterator<Item = char>,
{
    fn max_joltage(self) -> u32 {
        // whatever you want to compute
        self.filter_map(|c| c.to_digit(DIGITS as u32))
            .max()
            .unwrap_or(0)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let idx_1 = line
                    .chars()
                    .enumerate()
                    .take(line.len().saturating_sub(1)) // Ignore last element
                    .fold(0usize, |max_idx, (idx, c)| {
                        // This is just a max that returns the first instance rather than the last
                        if c > line.chars().nth(max_idx).unwrap() {
                            idx
                        } else {
                            max_idx
                        }
                    });
                let idx_2 = line
                    .chars()
                    .enumerate()
                    .skip(idx_1 + 1) //
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
    let n = 12usize;
    Some(
        // This is very intentionally written as spagetti code
        input
            .lines()
            .map(|line| {
                let mut skip_amt: usize = 0;
                (0..n)
                    .map(|digit_index| {
                        // Rust does some tomfoolery that wont let me chain mulitple reverses >:(
                        let take_amt = line.len() - (n - (digit_index + 1));
                        let (idx, c) = line
                            .chars()
                            .enumerate()
                            .take(take_amt)
                            .skip(skip_amt)
                            .reduce(
                                |acc @ (_, max_c), (idx, c)| {
                                    if c > max_c { (idx, c) } else { acc }
                                },
                            )
                            .unwrap();
                        skip_amt = idx + 1;
                        c
                    })
                    .enumerate()
                    .fold(0u64, |acc, (idx, c)| {
                        acc + (c.to_digit(10).unwrap() as u64) * 10u64.pow((n - (idx + 1)) as u32)
                    })
            })
            .sum(),
    )
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
        assert_eq!(result, Some(3121910778619));
    }
}
