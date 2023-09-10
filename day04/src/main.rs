// https://adventofcode.com/2021/day/4
//
// Research
// https://www.reddit.com/r/adventofcode/comments/r8i1lq/2021_day_4_solutions/
//
// Other solutions:
//  - https://github.com/Crazytieguy/advent-of-code/blob/master/2021/src/bin/day4/main.rs
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn7y21w/
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn8q7pt/
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn5zor9/
//  - https://www.reddit.com/r/adventofcode/comments/r8i1lq/comment/hn7k99s/
//
use color_eyre::{eyre::eyre, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated, tuple},
    Finish, IResult,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{self, Read},
};

type Int = u16;
type Pos = (usize, usize);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Move(Int);

#[derive(Clone, Debug)]
struct Row([Int; 5]);

impl Row {
    fn iter(&self) -> impl Iterator<Item = &Int> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
struct Board([Row; 5]);

impl Board {
    fn iter(&self) -> impl Iterator<Item = (Pos, &Int)> {
        self.0.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, value)| ((i, j), value))
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
            .map(|((i, j), &value)| (value, (i, j)))
            .collect::<HashMap<_, _>>();

        Self {
            map,
            squares: HashSet::new(),
        }
    }
}

impl BoardState {
    fn add(&mut self, mv: &Move) {
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
    moves: Vec<Move>,
}

impl Task {
    fn part1(&self) -> Int {
        let mut boards = self.boards.iter().map(BoardState::from).collect_vec();

        for m in &self.moves {
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

        for m in &self.moves {
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

fn parse_row(s: &str) -> IResult<&str, Row> {
    map(
        preceded(
            opt(tag(" ")),
            separated_list1(many1(tag(" ")), nom::character::complete::u16),
        ),
        |cols| Row(cols.try_into().expect("failed to cast to array")),
    )(s.trim())
}

fn parse_moves(s: &str) -> IResult<&str, Vec<Move>> {
    terminated(
        map(
            separated_list1(tag(","), nom::character::complete::u16),
            |nums| nums.into_iter().map(Move).collect_vec(),
        ),
        newline,
    )(s)
}

fn parse_boards(s: &str) -> IResult<&str, Vec<Board>> {
    map(separated_list1(newline, parse_row), |rows| {
        rows.into_iter()
            .chunks(5)
            .into_iter()
            .map(|rows| rows.collect::<Vec<_>>())
            .map(|rows| Board(rows.try_into().expect("failed to cast to array")))
            .collect_vec()
    })(s)
}

fn parse_input(s: &str) -> IResult<&str, Task> {
    map(
        tuple((parse_moves, newline, parse_boards)),
        |(moves, _, boards)| Task { moves, boards },
    )(s)
}

fn parse(s: &str) -> Result<Task> {
    let (_s, task) = all_consuming(parse_input)(s)
        .finish()
        .or(Err(eyre!("failed to parse input: {}", s)))?;
    Ok(task)
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
        assert_eq!(task.moves.len(), 27);
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
