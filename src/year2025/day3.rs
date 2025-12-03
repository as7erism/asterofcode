const JOLTAGE_BATTERIES: usize = 12;

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
                let battery = choose_battery(&bank[..(bank.len() - 1)]);

                bank[battery].to_digit(10).unwrap() * 10
                    + bank[(battery + 1)..]
                        .iter()
                        .max()
                        .unwrap()
                        .to_digit(10)
                        .unwrap()
            })
            .sum::<u32>()
    );
}

fn part_two(input: &str) {
    println!(
        "{}",
        input
            .lines()
            .map(|line| {
                let bank = line.chars().collect::<Vec<char>>();
                let mut joltage = String::with_capacity(JOLTAGE_BATTERIES);
                let mut i = 0;

                // while we still need to pick more batteries from our bank and we have more
                // batteries to pick from than battery slots left,
                while JOLTAGE_BATTERIES > joltage.len()
                    && bank[i..].len() > JOLTAGE_BATTERIES - joltage.len()
                {
                    // pick a battery from a range such that even if we pick the last battery in
                    // the range, there will be enough batteries after to fill all our slots
                    let battery = choose_battery(
                        &bank[i..=(bank.len() - (JOLTAGE_BATTERIES - joltage.len()))],
                    );

                    joltage.push(bank[i + battery]);
                    // only search for batteries past this point
                    i += battery + 1;
                }

                // if we short-circuited because we couldn't make any more choices, add the rest of
                // the batteries to the joltage here
                if JOLTAGE_BATTERIES > joltage.len() {
                    joltage.push_str(&String::from_iter(&bank[i..]));
                }

                joltage.parse::<u64>().unwrap()
            })
            .sum::<u64>()
    );
}

fn choose_battery(slice: &[char]) -> usize {
    slice
        .iter()
        .enumerate()
        // we want the max, but `.max()` and its ilk return the last element that matches, which
        // means our second partition would be unnecessarily small. `.min()` and company return the
        // first, so use that and reverse the comparison order
        .min_by(|(_, x), (_, y)| y.cmp(x))
        .map(|(i, _)| i)
        .unwrap()
}
