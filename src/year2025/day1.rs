const DIAL_START: i32 = 50;
const DIAL_LIMIT: i32 = 100;

pub fn run(part: Option<u8>, input: &str) {
    match part {
        Some(p) => {
            if p == 1 {
                part_one(input);
            } else {
                part_two(input);
            }
        }
        None => {
            part_one(input);
            part_two(input);
        }
    }
}

fn part_one(input: &str) {
    let mut sum = DIAL_START;
    let mut password = 0;

    for line in input.lines() {
        let val = line[1..].parse::<i32>().unwrap();

        if line.chars().next().unwrap() == 'L' {
            sum -= val;
        } else {
            sum += val;
        }
        sum %= DIAL_LIMIT;

        if sum == 0 {
            password += 1;
        }
    }

    println!("{password}");
}

fn part_two(input: &str) {
    let mut sum = DIAL_START;
    let mut password = 0;

    for line in input.lines() {
        let val = line[1..].parse::<i32>().unwrap();

        if line.chars().next().unwrap() == 'L' {
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

    println!("{password}");
}
