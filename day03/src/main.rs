// https://adventofcode.com/2021/day/3
//
// Reddit solution thread: https://www.reddit.com/r/adventofcode/comments/r7r0ff/2021_day_3_solutions/
//
// Research:
//  - https://github.com/Crazytieguy/advent-of-code/blob/master/2021/src/bin/day3/main.rs
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn21o5u/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1axfc/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn344bb/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3a30c/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4bsaw/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn19xng/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ud0u/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn43gfp/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4b4be/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4dv8s/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4kh9z/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn4s5nf/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn52cck/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn5bc2i/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn5fm9q/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1ke7f/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1l4s9/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1l7ud/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1mtw7/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1oae9/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn1ri7y/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn29nvf/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2qss5/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2tf3y/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn2vlnj/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn343ke/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn34v6g/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3887e/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ko6i/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3o1lp/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3ohvh/
//  - https://www.reddit.com/r/adventofcode/comments/r7r0ff/comment/hn3s24g/
//
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
        let mut counts = Counter::<usize>::new();

        for &Reading(v) in &self.readings {
            let bits = (Int::BITS - v.leading_zeros()) as usize;
            for i in 0..bits {
                counts[&i] += (1 & (v >> i)) as usize;
            }
        }

        debug_assert!(self.readings.len() % 2 == 0);
        let n = self.readings.len() / 2;
        let (mut gamma, mut epsilon) = (0, 0);

        for (i, count) in counts {
            match count.cmp(&n) {
                Ordering::Less => epsilon |= 1 << i,
                Ordering::Equal => {}
                Ordering::Greater => gamma |= 1 << i,
            }
        }

        epsilon * gamma
    }

    fn part2(&self) -> u32 {
        let initial = self
            .readings
            .iter()
            .map(|&Reading(v)| v)
            .collect::<Vec<_>>();
        let bits = initial
            .iter()
            .map(|&value| u32::BITS - value.leading_zeros())
            .max()
            .expect("a bit length");

        let (mut ogr, mut csr) = (initial.clone(), initial);

        for i in (0..bits).rev() {
            if ogr.len() > 1 {
                let (ones, zeros): (Vec<_>, Vec<_>) =
                    ogr.iter().partition(|&&value| (1 & (value >> i)) == 1);

                if ones.len() >= zeros.len() {
                    ogr = ones;
                } else {
                    ogr = zeros;
                }
            }

            if csr.len() > 1 {
                let (ones, zeros): (Vec<_>, Vec<_>) =
                    csr.iter().partition(|&&value| (1 & (value >> i)) == 1);

                if zeros.len() <= ones.len() {
                    csr = zeros;
                } else {
                    csr = ones;
                }
            }
        }

        debug_assert_eq!(ogr.len(), 1);
        debug_assert_eq!(csr.len(), 1);

        ogr[0] * csr[0]
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
