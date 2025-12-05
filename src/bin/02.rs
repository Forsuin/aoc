advent_of_code::solution!(2);

struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}

fn get_divisors(n: u32) -> Vec<u32> {
    let mut divisors = Vec::with_capacity(10);

    for x in 1..=(n / 2) {
        if n % x == 0 {
            divisors.push(x);
        }
    }

    divisors
}

fn get_num_digits(n: u64) -> u32 {
    if n == 0 { 1 } else { n.ilog10() + 1 }
}

fn group_num_by_digits(num: u64, n: usize) -> Vec<u64> {
    let s = num.to_string();
    let len = s.len();
    let mut result = vec![];

    let mut end = len;
    while end > 0 {
        let start = if end >= n { end - n } else { 0 };
        // Get the slice for the current group
        let slice = &s[start..end];
        // Convert the slice back to a number
        if let Ok(group_num) = slice.parse::<u64>() {
            result.push(group_num);
        }
        end = start;
    }

    result
}

fn is_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut id_sum = 0;

    input
        .split(",")
        .filter_map(|range| {
            let (start, end) = range.split_once("-").expect("Couldn't split range");

            if start.len() % 2 != 0 && end.len() % 2 != 0 {
                return None;
            }

            Some(Range::new(
                start.parse::<u64>().unwrap(),
                end.parse::<u64>().unwrap(),
            ))
        })
        .for_each(|range| {
            for id in range.start..=range.end {
                let num_digits = get_num_digits(id);

                if num_digits % 2 != 0 {
                    continue;
                }

                let chunks = group_num_by_digits(id, (num_digits / 2) as usize);

                if is_all_same(chunks.as_slice()) {
                    id_sum += id;
                }
            }
        });

    Some(id_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut id_sum = 0;

    input
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").expect("Couldn't split range");

            Range::new(start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .for_each(|range| {
            for id in range.start..=range.end {
                let num_digits = get_num_digits(id);

                let divisors = get_divisors(num_digits);

                for divisor in divisors {
                    let chunks = group_num_by_digits(id, divisor as usize);


                    if is_all_same(chunks.as_slice()) {
                        id_sum += id;
                        break;
                    }
                }
            }
        });
    Some(id_sum)
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
