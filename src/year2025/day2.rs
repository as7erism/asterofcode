use aoc::NumDigits;

pub struct Solution;

impl crate::Solution for Solution {
    fn part_one(input: &str) {
        println!(
            "{}",
            input
                .split(',')
                .map(|range| {
                    let bounds = range
                        .split('-')
                        .map(|id| id.trim().parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (bounds[0]..(bounds[1] + 1)).filter(|id| !is_valid(*id))
                })
                .flatten()
                .sum::<u64>()
        );
    }

    fn part_two(input: &str) {
        println!(
            "{}",
            input
                .split(',')
                .map(|range| {
                    let bounds = range
                        .split('-')
                        .map(|id| id.trim().parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (bounds[0]..=bounds[1]).filter(|id| !is_valid_2(*id))
                })
                .flatten()
                .sum::<u64>()
        );
    }
}

fn is_valid(id: u64) -> bool {
    !(id.num_digits().is_multiple_of(2)
        && id / 10_u64.pow((id.num_digits() / 2) as u32)
            == id % 10_u64.pow((id.num_digits() / 2) as u32))
}

fn is_valid_2(id: u64) -> bool {
    // for each factor of this id, chop off that number of digits from the end of a running total
    // and see if each chopped off collection of digits match. if they all do, this is invalid!
    (0..=(id.num_digits() / 2))
        .filter(|factor| id.num_digits().is_multiple_of(*factor))
        .all(|factor| {
            let rightmost = id % 10_u64.pow(factor as u32);
            let mut state = id / 10_u64.pow(factor as u32);

            while state != 0 {
                if state % 10_u64.pow(factor as u32) != rightmost {
                    return true;
                }
                state /= 10_u64.pow(factor as u32);
            }

            false
        })
}
