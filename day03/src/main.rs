// https://adventofcode.com/2021/day/3
//
// Reddit solution thread: https://www.reddit.com/r/adventofcode/comments/r7r0ff/2021_day_3_solutions/
//
// Research:
//  - https://github.com/Crazytieguy/advent-of-code/blob/master/2021/src/bin/day3/main.rs
//    Revisit.  Concise bit arithmetic.  take_while.  Deriving epsilon from gamma.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1axfc/
//    Revisit.  Very concise bit arithmetic.  Makes assumptions about input.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn5bc2i/
//    Revisit. Use of fold((0, 0), ...)
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1l7ud/
//    Revisit. split_whitespace, .map(Result::unwrap). let [value] = *values.
//    partition helper.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn29nvf/
//    Revisit.  Uses one-line combinators for part 2.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4bsaw/
//    slice::sort_unstable.  Binary search.  Derived epsilon.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn19xng/
//    iter::nth.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn43gfp/
//    Concise.  iter::max_by_key.  iter::fold.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4dv8s/
//    vec::retain.  Does not convert to ints until late.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4kh9z/
//    iter::partition.  Derived epsilon.  vec::retain.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4s5nf/
//    iter::fold.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1mtw7/
//    Uses several closures.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1ri7y/
//    flat_map(... result.ok()).  it.peek().  vec::retain.  Derived epsilon.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2qss5/
//    Very concise solution for part 2.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2tf3y/
//    enum { MostCommon, LeastCommon }. iter::take.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2vlnj/
//    Uses combinators for everything.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1oae9/
//    Concise.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1l4s9/
//    vec::retain.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1ke7f/
//    gamma.iter().fold(0, |acc, b| acc * 2 + b).
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn5fm9q/
//    Fairly complex bit arithmetic.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4b4be/
//    Uses a "digit bias".
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ud0u/
//    Does not parse input into ints.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3a30c/
//    SIMD for part 1.  slice::advance.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn344bb/
//    char::to_digit(2).
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn21o5u/
//    Goes back and forth between string and binary representations.
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn343ke/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn34v6g/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3887e/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ko6i/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3o1lp/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ohvh/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3s24g/
//
// TODO:
//  - Derive epsilon
//  - fold((0, 0), ...)
//  - split_whitepsace
//  - let [value] = *values
//  - partition helper
//  - vec::retain?
//
use color_eyre::{Report, Result};
use itertools::Itertools;
use std::{
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
    num_bits: usize,
}

// https://github.com/Crazytieguy/advent-of-code/blob/master/2021/src/bin/day3/main.rs
impl Task {
    fn part1(&self) -> usize {
        debug_assert!(self.readings.len() % 2 == 0);
        let n = self.readings.len() / 2;

        let gamma = (0..self.num_bits)
            .map(|i| 1 << i)
            .filter(|mask| {
                self.readings
                    .iter()
                    .filter(|&Reading(v)| v & mask != 0)
                    .count()
                    > n
            })
            .fold(0, |gamma, mask| gamma | mask) as usize;

        let mask = (1 << self.num_bits) - 1;
        gamma * (!gamma & mask)
    }

    fn part2(&self) -> usize {
        self.rating(true) * self.rating(false)
    }

    fn rating(&self, bit_criterion: bool) -> usize {
        let mut remaining = self.readings.iter().map(|&Reading(v)| v).collect_vec();

        for i in (0..self.num_bits).rev() {
            let mask = 1 << i;
            let mut groups = remaining.into_iter().into_group_map_by(|v| v & mask);
            remaining = if bit_criterion == (groups[&mask].len() >= groups[&0].len()) {
                groups.remove(&mask).unwrap()
            } else {
                groups.remove(&0).unwrap()
            };
            if remaining.len() == 1 {
                return remaining[0] as usize;
            }
        }

        unreachable!()
    }
}

fn parse(s: &str) -> Result<Task> {
    let readings = s
        .trim()
        .lines()
        .map(|l| l.trim().parse::<Reading>().expect("not a binary number"))
        .collect::<Vec<_>>();

    let num_bits = readings
        .iter()
        .map(|&Reading(value)| (u32::BITS - value.leading_zeros()) as usize)
        .max()
        .expect("a bit length");

    Ok(Task { readings, num_bits })
}

fn main() -> Result<()> {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s)?;
    let task = parse(&s)?;

    println!("part 1: {}", task.part1());
    println!("part 2: {}", task.part2());

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
    fn part2() {
        let task = example();
        assert_eq!(task.part2(), 230);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 3374136);
        assert_eq!(task.part2(), 4432698);
    }
}
