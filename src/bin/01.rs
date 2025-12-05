advent_of_code::solution!(1);

struct Combination {
    pub current_num: i32,
    pub num_zeros: u32,
    pub pass_zeros: u32,
}

impl Combination {
    pub fn new() -> Self {
        Self {
            current_num: 50,
            num_zeros: 0,
            pass_zeros: 0,
        }
    }

    pub fn rotate_left(&mut self, amount: i32) {
        let old_num = self.current_num;

        self.current_num = (self.current_num - amount).rem_euclid(100);

        if self.current_num == 0 {
            self.num_zeros += 1;
        }

        if old_num < self.current_num && self.current_num != 0 && old_num != 0 {
            self.pass_zeros += 1;
        }
    }

    pub fn rotate_right(&mut self, amount: i32) {
        let old_num = self.current_num;

        self.current_num = (self.current_num + amount).rem_euclid(100);

        if self.current_num == 0 {
            self.num_zeros += 1;
        }

        if old_num > self.current_num && self.current_num != 0 && old_num != 0 {
            self.pass_zeros += 1;
        }
    }

    pub fn add_full_rotations(&mut self, num_rotations: u32) {
        self.pass_zeros += num_rotations;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut combo = Combination::new();

    input.lines().into_iter()
        .for_each(|l| {
           match l.chars().nth(0).unwrap() {
               'L' => {
                  let amount = &l[1..].parse::<i32>().expect("Parsing fucked up");
                   
                   combo.rotate_left(*amount);
               }
               'R' => {
                   let amount = &l[1..].parse::<i32>().expect("Parsing fucked up");
                   combo.rotate_right(*amount);
               }
               bad @ _ => panic!("Unexpected character at beginning of line: {bad}")
           }
        });

    Some(combo.num_zeros as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut combo = Combination::new();

    input.lines().into_iter()
        .for_each(|l| {
            match l.chars().nth(0).unwrap() {
                'L' => {
                    let amount = &l[1..].parse::<i32>().expect("Parsing fucked up");

                    let num_complete_rotations = amount / 100;
                    let extra_rot = amount % 100;

                    if num_complete_rotations > 0 {
                        combo.add_full_rotations(num_complete_rotations as u32);
                    }
                    combo.rotate_left(extra_rot);
                }
                'R' => {
                    let amount = &l[1..].parse::<i32>().expect("Parsing fucked up");

                    let num_complete_rotations = amount / 100;
                    let extra_rot = amount % 100;

                    if num_complete_rotations > 0 {
                        combo.add_full_rotations(num_complete_rotations as u32);
                    }
                    combo.rotate_right(extra_rot);
                }
                bad @ _ => panic!("Unexpected character at beginning of line: {bad}")
            }
        });

    // println!("Passed zeros: {}, Num zeroes: {}", combo.pass_zeros, combo.num_zeros);
    Some(combo.pass_zeros as u64 + combo.num_zeros as u64)
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
