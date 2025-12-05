use std::{collections::BTreeMap, ops::Bound};

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = usize;
    type OutputTwo = u64;

    fn part_one(input: &str) -> Self::OutputOne {
        let mut ranges = BTreeMap::new();

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut range = line.split('-').map(|id| id.parse::<i64>().unwrap());

            insert_range(&mut ranges, range.next().unwrap(), range.next().unwrap());
        }

        lines
            .map(|id| id.parse::<i64>().unwrap())
            .filter(|&id| is_in_range(&mut ranges, id))
            .count()

        // quadratic solution:
        //let mut ranges = Vec::new();
        //
        //let mut lines = input.lines();
        //while let Some(line) = lines.next() {
        //    if line.is_empty() {
        //        break;
        //    }
        //
        //    let mut range_iter = line.split('-').map(|id| id.parse::<u64>().unwrap());
        //    ranges.push((range_iter.next().unwrap(), range_iter.next().unwrap()));
        //}
        //
        //lines
        //    .map(|id| id.parse::<u64>().unwrap())
        //    .filter(|id| ranges.iter().any(|(begin, end)| id >= begin && id <= end))
        //    .count()
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        //let mut range_set = BTreeMap::new();
        //
        //let mut lines = input.lines();
        //while let Some(line) = lines.next() {
        //    if line.is_empty() {
        //        break;
        //    }
        //
        //    let mut range = line.split('-').map(|id| id.parse::<i64>().unwrap());
        //
        //    insert_range(&mut range_set, range.next().unwrap(), range.next().unwrap());
        //}
        //
        //range_set
        //    .iter()
        //    .map(|(val, marker)| match marker {
        //        RangeMarker::End => *val,
        //        RangeMarker::Begin => -val,
        //    })
        //    .sum::<i64>() as u64

        // quadratic solution:
        let mut ranges = Vec::new();

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut range_iter = line.split('-').map(|id| id.parse::<u64>().unwrap());
            ranges.push((range_iter.next().unwrap(), range_iter.next().unwrap()));
        }

        let num_ranges = ranges.len();
        let mut skips = vec![false; num_ranges];
        for i in 0..num_ranges {
            for j in i + 1..num_ranges {
                if ranges_overlap(ranges[i], ranges[j]) {
                    ranges[j] = (
                        std::cmp::min(ranges[i].0, ranges[j].0),
                        std::cmp::max(ranges[i].1, ranges[j].1),
                    );
                    skips[i] = true;
                }
            }
        }

        ranges
            .iter()
            .enumerate()
            .filter(|&(i, _)| !skips[i])
            .map(|(_, (b, e))| (e - b))
            .sum::<u64>()
    }
}

// used for quadratic solution
#[allow(dead_code)]
fn ranges_overlap(r1: (u64, u64), r2: (u64, u64)) -> bool {
    (r2.0 >= r1.0 && r2.0 <= r1.1)
        || (r2.1 >= r1.0 && r2.1 <= r1.1)
        || (r1.0 >= r2.0 && r1.0 <= r2.1)
        || (r1.1 >= r2.0 && r1.1 <= r2.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RangeMarker {
    End = 0,
    Begin = 1,
}

fn insert_range(range_map: &mut BTreeMap<i64, RangeMarker>, begin: i64, end: i64) {
    let mut cursor = range_map.lower_bound_mut(Bound::Included(&begin));
    while cursor
        .peek_next()
        .is_some_and(|(&val, marker)| val < end + 1 + (*marker) as i64)
    {
        cursor.remove_next();
    }

    if !matches!(cursor.peek_next(), Some((_, RangeMarker::End))) {
        let _ = cursor.insert_after(end + 1, RangeMarker::End);
    }

    if !matches!(cursor.peek_prev(), Some((_, RangeMarker::Begin))) {
        let _ = cursor.insert_before(begin, RangeMarker::Begin);
    }
}

fn is_in_range(range_map: &mut BTreeMap<i64, RangeMarker>, val: i64) -> bool {
    let cursor = range_map.upper_bound(Bound::Excluded(&val));
    matches!(cursor.peek_next(), Some((_, RangeMarker::End)))
}
