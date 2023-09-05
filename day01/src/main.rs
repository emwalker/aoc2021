use color_eyre::Result;
use itertools::Itertools;
use std::io::{self, Read};

struct Task {
    readings: Vec<u32>,
}

impl Task {
    fn part1(&self) -> usize {
        self.readings
            .iter()
            .tuple_windows()
            .map(|(a, b)| (b > a) as usize)
            .sum()
    }

    fn part2(&self) -> usize {
        self.readings
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .map(|(a, b)| (b > a) as usize)
            .sum()
    }
}

fn parse(s: &str) -> Result<Task> {
    let readings = s
        .trim()
        .lines()
        .map(|l| l.trim().parse::<u32>().unwrap())
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

    const INPUT: &str = "
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    fn example() -> Task {
        parse(INPUT).unwrap()
    }

    #[test]
    fn parsing() {
        let task = example();
        assert_eq!(task.readings.len(), 10);
    }

    #[test]
    fn part1() {
        let task = example();
        assert_eq!(task.part1(), 7);
    }

    #[test]
    fn part2() {
        let task = example();
        assert_eq!(task.part2(), 5);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 1752);
        assert_eq!(task.part2(), 1781);
    }
}
