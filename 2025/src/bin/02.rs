advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;
    input.split(',').for_each(|range| {
        // Extract the min and max from the range
        let (min, max) = {
            let mut iter = range.split('-').map(|x| x.parse::<u64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };

        for n in min..=max {
            let s = n.to_string();
            if s.len() % 2 != 0 {
                continue;
            }

            let (left, right) = s.split_at(s.len() / 2);

            if left == right {
                res += n
            }
        }
    });
    return Some(res);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut res = 0;
    input.split(',').for_each(|range| {
        // Extract the min and max from the range
        let (min, max) = {
            let mut iter = range.split('-').map(|x| x.parse::<u64>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };
        for n in min..=max {
            let s = n.to_string();
            for num_splits in 2..=s.len() {
                // Check each split from 2 to s.len()
                if s.len() % num_splits != 0 {
                    // Can't cleanly split string with this size
                    continue;
                }
                let mut counter = s.len() / num_splits; // Initialize the counter asuming the first repition is true
                let shamt: usize = s.len() / num_splits;
                for pos in 0..s.len() / num_splits {
                    let first = s.chars().nth(pos).unwrap();
                    for shift in 1..num_splits {
                        if s.chars().nth(pos + shift * shamt).unwrap() == first {
                            counter += 1; // For a true value, should increase by 
                        }
                    }
                }
                if counter == s.len() {
                    res += n;
                    break; // Avoid double counting the same number
                }
            }
        }
    });

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
