advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut loc = 50;
    let mut res = 0;
    input.lines().for_each(|line| {
        let mut alpha = 1;
        if line.contains("L") {
            alpha = -1;
        }
        let delta = line
            .trim_start_matches(|c| c == 'L' || c == 'R')
            .parse::<i64>()
            .unwrap()
            * alpha;

        loc = (loc + delta) % 100;

        if loc < 0 {
            loc += 100
        }
        if loc == 0 {
            res += 1;
        }
    });
    return Some(res);
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut loc = 50;
    let mut res = 0;
    let mut prev_zero = false;
    input.lines().for_each(|line| {
        let mut alpha = 1;
        if line.contains("L") {
            alpha = -1;
        }
        let delta = line
            .trim_start_matches(|c| c == 'L' || c == 'R')
            .parse::<i64>()
            .unwrap()
            * alpha;

        // Rotate
        loc += delta;

        // Positive rotation
        if loc > 0 {
            res += loc / 100;
        } else if loc <= 0 {
            res += -(loc / 100) + 1
        }

        // Make the loc back in [0,99]
        loc %= 100;
        if loc < 0 {
            loc += 100
        }

        if prev_zero && delta < 0 {
            res -= 1
        }

        if loc == 0 {
            prev_zero = true
        } else {
            prev_zero = false
        }
    });
    return Some(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
