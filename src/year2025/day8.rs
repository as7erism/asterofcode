use std::{cmp::Reverse, collections::BinaryHeap};

use aoc::DisjointSets;

const CONNECTIONS: usize = 1000;

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = usize;
    type OutputTwo = u64;

    fn part_one(input: &str) -> Self::OutputOne {
        let points = input
            .lines()
            .map(|l| {
                let mut components = l.split(',');
                let x = components.next().unwrap().parse::<u64>().unwrap();
                let y = components.next().unwrap().parse::<u64>().unwrap();
                let z = components.next().unwrap().parse::<u64>().unwrap();
                (x, y, z)
            })
            .collect::<Vec<_>>();
        let mut distances = BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);

        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                distances.push(Reverse((OrderedF64(sld(points[i], points[j])), i, j)));
            }
        }

        let mut sets = DisjointSets::new(points.len());
        for _ in 0..CONNECTIONS {
            let (_, box1, box2) = distances.pop().unwrap().0;
            sets.union(box1, box2);
        }

        let mut set_lengths = sets
            .collect()
            .into_iter()
            .map(|set| Reverse(set.len()))
            .collect::<Vec<_>>();
        set_lengths.sort();
        set_lengths[0].0 * set_lengths[1].0 * set_lengths[2].0
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        let points = input
            .lines()
            .map(|l| {
                let mut components = l.split(',');
                let x = components.next().unwrap().parse::<u64>().unwrap();
                let y = components.next().unwrap().parse::<u64>().unwrap();
                let z = components.next().unwrap().parse::<u64>().unwrap();
                (x, y, z)
            })
            .collect::<Vec<_>>();
        let mut distances = BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);

        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                distances.push(Reverse((OrderedF64(sld(points[i], points[j])), i, j)));
            }
        }

        let mut num_sets = points.len();
        let mut sets = DisjointSets::new(num_sets);

        loop {
            let (_, box1, box2) = distances.pop().unwrap().0;
            if sets.union(box1, box2) {
                num_sets -= 1;
                if num_sets <= 1 {
                    return points[box1].0 * points[box2].0;
                }
            }
        }
    }
}

fn sld(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    ((b.0.abs_diff(a.0).pow(2) + b.1.abs_diff(a.1).pow(2) + b.2.abs_diff(a.2).pow(2)) as f64).sqrt()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct OrderedF64(pub f64);

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for OrderedF64 {}
