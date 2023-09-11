// https://adventofcode.com/2021/day/4
//
// Research
// https://www.reddit.com/r/adventofcode/comments/r8i1lq/2021_day_4_solutions/
//
// Other solutions:
//  - https://github.com/Crazytieguy/advent-of-code/blob/master/2021/src/bin/day4/main.rs
//    Revisit.  Concise solution.  Procedural.
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn5zor9/
//    Concise.  Procedural.
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn7y21w/
//    Similar to this one.
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn8q7pt/
//    Check method mutates state.
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn7k99s/
//
use color_eyre::Result;
use itertools::Itertools;
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

type Int = u16;
type Pos = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Draw(Int);

#[derive(Clone, Debug)]
struct Board(ArrayBase<OwnedRepr<Int>, Dim<[usize; 2]>>);

impl Board {
    fn iter(&self) -> impl Iterator<Item = (Pos, Int)> + '_ {
        self.0
            .rows()
            .into_iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.into_iter()
                    .enumerate()
                    .map(move |(j, &value)| ((i, j), value))
            })
    }
}

struct BoardState {
    map: HashMap<Int, Pos>,
    squares: HashSet<Pos>,
}

impl From<&Board> for BoardState {
    fn from(board: &Board) -> Self {
        let map = board
            .iter()
            .map(|((i, j), value)| (value, (i, j)))
            .collect::<HashMap<_, _>>();

        Self {
            map,
            squares: HashSet::new(),
        }
    }
}

impl BoardState {
    fn add(&mut self, mv: &Draw) {
        if let Some(pos) = self.map.get(&mv.0) {
            self.squares.insert(*pos);
        }
    }

    fn row_complete(&self) -> bool {
        (0..5).any(|i| (0..5).all(|j| self.squares.contains(&(i, j))))
            || (0..5).any(|j| (0..5).all(|i| self.squares.contains(&(i, j))))
    }

    fn unmarked_numbers(&self) -> impl Iterator<Item = &Int> {
        self.map.iter().filter_map(|(value, pos)| {
            if self.squares.contains(pos) {
                None
            } else {
                Some(value)
            }
        })
    }
}

struct Task {
    boards: Vec<Board>,
    draws: Vec<Draw>,
}

impl Task {
    fn part1(&self) -> Int {
        let mut boards = self.boards.iter().map(BoardState::from).collect_vec();

        for m in &self.draws {
            for board in &mut boards {
                board.add(m);

                if board.row_complete() {
                    let unmarked = board.unmarked_numbers().sum::<Int>();
                    return m.0 * unmarked;
                }
            }
        }

        unreachable!()
    }

    fn part2(&self) -> Int {
        let mut boards = self
            .boards
            .iter()
            .map(BoardState::from)
            .collect::<VecDeque<_>>();

        for m in &self.draws {
            for _ in 0..boards.len() {
                let mut board = boards.pop_front().unwrap();
                board.add(m);

                if board.row_complete() {
                    if boards.is_empty() {
                        let unmarked = board.unmarked_numbers().sum::<Int>();
                        return m.0 * unmarked;
                    }
                } else {
                    boards.push_back(board);
                }
            }
        }

        unreachable!()
    }
}

fn parse(s: &str) -> Result<Task> {
    let mut split = s.trim().split("\n\n");

    let draws: Vec<Draw> = split
        .next()
        .unwrap()
        .split(',')
        .map(|n| Draw(n.parse().unwrap()))
        .collect();

    let boards = split
        .map(|board_str| {
            let board_vec = board_str
                .split_whitespace()
                .map(|n| n.parse::<Int>().unwrap())
                .collect();
            let array = Array2::from_shape_vec([5, 5], board_vec).unwrap();
            Board(array)
        })
        .collect();

    Ok(Task { draws, boards })
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

    fn example() -> Task {
        parse(include_str!("../data/example.txt")).unwrap()
    }

    #[test]
    fn parsing() {
        let task = example();
        assert_eq!(task.boards.len(), 3);
        assert_eq!(task.draws.len(), 27);
    }

    #[test]
    fn part1() {
        let task = example();
        assert_eq!(task.part1(), 4512);
    }

    #[test]
    fn part2() {
        let task = example();
        assert_eq!(task.part2(), 1924);
    }

    #[test]
    fn input() {
        let task = parse(include_str!("../data/input.txt")).unwrap();
        assert_eq!(task.part1(), 39902);
        assert_eq!(task.part2(), 26936);
    }
}
