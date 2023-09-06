use color_eyre::{eyre::eyre, Result};
use std::io::{self, Read};

type Int = i32;

enum Instruction {
    Forward(Int),
    Down(Int),
    Up(Int),
}

use Instruction::*;

struct Task {
    instructions: Vec<Instruction>,
}

impl Task {
    fn part1(&self) -> Int {
        let (mut i, mut j) = (0, 0);

        for ins in &self.instructions {
            match ins {
                Forward(v) => j += v,
                Down(v) => i += v,
                Up(v) => i -= v,
            }
        }

        i * j
    }

    fn part2(&self) -> Int {
        let (mut i, mut j, mut aim) = (0, 0, 0);

        for ins in &self.instructions {
            match ins {
                Forward(v) => {
                    j += v;
                    i += aim * v;
                }

                Down(v) => aim += v,
                Up(v) => aim -= v,
            }
        }

        i * j
    }
}

fn parse(s: &str) -> Result<Task> {
    let mut instructions = vec![];

    for line in s.trim().lines() {
        let (ins, v) = line.trim().split_once(' ').expect("a space");
        let v = v.parse::<Int>()?;

        let ins = match ins {
            "forward" => Forward(v),
            "down" => Down(v),
            "up" => Up(v),
            _ => return Err(eyre!("unknown instrution: {}", line)),
        };

        instructions.push(ins);
    }

    Ok(Task { instructions })
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
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    fn example() -> Task {
        parse(INPUT).unwrap()
    }

    #[test]
    fn parsing() {
        let task = example();
        assert_eq!(task.instructions.len(), 6);
    }

    #[test]
    fn part1() {
        let task = example();
        assert_eq!(task.part1(), 150);
    }

    #[test]
    fn part2() {
        let task = example();
        assert_eq!(task.part2(), 900);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 2027977);
        assert_eq!(task.part2(), 1_903_644_897);
    }
}
