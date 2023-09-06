// https://adventofcode.com/2021/day/3
use color_eyre::{Report, Result};
use counter::Counter;
use std::{
    cmp::Ordering,
    io::{self, Read},
    str::FromStr,
};

type Int = u32;

struct Reading(Int);

impl FromStr for Reading {
    type Err = Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(u32::from_str_radix(s, 2)?))
    }
}

struct Task {
    readings: Vec<Reading>,
}

impl Task {
    fn part1(&self) -> usize {
        let mut counter = Counter::<usize>::new();

        for &Reading(v) in &self.readings {
            let bits = (Int::BITS - v.leading_zeros()) as usize;
            for i in 0..bits {
                counter[&i] += (1 & (v >> i)) as usize;
            }
        }

        debug_assert!(self.readings.len() % 2 == 0);
        let n = self.readings.len() / 2;
        let (mut gamma, mut epsilon) = (0, 0);

        for (i, count) in counter {
            match count.cmp(&n) {
                Ordering::Less => epsilon |= 1 << i,
                Ordering::Equal => {}
                Ordering::Greater => gamma |= 1 << i,
            }
        }

        epsilon * gamma
    }
}

fn parse(s: &str) -> Result<Task> {
    let readings = s
        .trim()
        .lines()
        .map(|l| l.trim().parse::<Reading>().expect("not a binary number"))
        .collect::<Vec<_>>();
    Ok(Task { readings })
}

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let task = parse(&s)?;

    println!("part 1: {}", task.part1());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    fn example() -> Task {
        parse(EXAMPLE).unwrap()
    }

    #[test]
    fn parsing() {
        let task = example();
        assert_eq!(task.readings.len(), 12);
        assert_eq!(task.readings[0].0, 4);
        assert_eq!(task.readings.iter().last().unwrap().0, 10);
    }

    #[test]
    fn part1() {
        let task = example();
        assert_eq!(task.part1(), 198);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 3374136);
    }
}
