use color_eyre::{eyre::eyre, Result};
use std::io::{self, Read};

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

use Instruction::*;

struct Task {
    instructions: Vec<Instruction>,
}

impl Task {
    fn part1(&self) -> i32 {
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
}

fn parse(s: &str) -> Result<Task> {
    let mut instructions = vec![];

    for line in s.trim().lines() {
        let (ins, v) = line.trim().split_once(' ').unwrap();
        let v = v.parse::<i32>().unwrap();

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
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 2027977);
    }
}
