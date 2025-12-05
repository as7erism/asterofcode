const DIAL_START: i32 = 50;
const DIAL_LIMIT: i32 = 100;

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = i32;

    fn part_one(input: &str) -> Self::OutputOne {
        let mut sum = DIAL_START;
        let mut password = 0;

        for line in input.lines() {
            let val = line[1..].parse::<i32>().unwrap();

            if line.starts_with('L') {
                sum -= val;
            } else {
                sum += val;
            }
            sum %= DIAL_LIMIT;

            if sum == 0 {
                password += 1;
            }
        }

        password
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        let mut sum = DIAL_START;
        let mut password = 0;

        for line in input.lines() {
            let val = line[1..].parse::<i32>().unwrap();

            if line.starts_with('L') {
                // integer div won't detect the first overflow, so handle it here
                // also, check if we already hit 0 last time around
                if sum - val <= 0 && sum != 0 {
                    password += 1;
                }
                sum -= val;
            } else {
                sum += val;
            }

            password += (sum / DIAL_LIMIT).abs();
            sum = sum.rem_euclid(DIAL_LIMIT);
        }

        password
    }
}
