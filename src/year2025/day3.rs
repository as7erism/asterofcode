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
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let bank = line.chars().collect::<Vec<char>>();
                let (i, v1) = bank[..(bank.len() - 1)]
                    .iter()
                    .enumerate()
                    // we want the max, but `.max()` and its ilk return the last element that
                    // matches, which means our second partition would be unnecessarily small.
                    // `.min()` and company return the first, so use that and reverse the
                    // comparison order
                    .min_by(|(_, x), (_, y)| y.cmp(x))
                    .unwrap();

                v1.to_digit(10).unwrap() * 10
                    + bank[(i + 1)..].iter().max().unwrap().to_digit(10).unwrap()
            })
            .sum::<u32>()
    );
}

fn part_two(_: &str) {
    todo!();
}
