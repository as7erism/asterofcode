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

fn part_one(_: &str) {
    todo!();
}

fn part_two(_: &str) {
    todo!();
}
