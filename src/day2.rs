use crate::{DaySolution, FromInput};
use std::vec::Vec;

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Copy, Clone)]
struct Move {
    me: Choice,
    opponent: Choice,
    goal: Outcome,
}

// TODO: Model the problem into this struct
pub struct Day2 {
    moves: Vec<Move>,
}

impl Choice {
    fn from_string(x: &str) -> Choice {
        match x {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => {
                panic!("Invalid Choice input: {}", x)
            }
        }
    }
}

impl Outcome {
    fn from_string(x: &str) -> Outcome {
        match x {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => {
                panic!("Invalid Outcome input: {}", x)
            }
        }
    }
}

impl Move {
    fn evaluate(&self) -> Outcome {
        match self.opponent {
            Choice::Rock => match self.me {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Win,
                Choice::Scissors => Outcome::Loss,
            },
            Choice::Paper => match self.me {
                Choice::Rock => Outcome::Loss,
                Choice::Paper => Outcome::Draw,
                Choice::Scissors => Outcome::Win,
            },
            Choice::Scissors => match self.me {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Loss,
                Choice::Scissors => Outcome::Draw,
            },
        }
    }

    fn solve(c: &Choice, res: &Outcome) -> Choice {
        match res {
            Outcome::Draw => match c {
                Choice::Rock => Choice::Rock,
                Choice::Paper => Choice::Paper,
                Choice::Scissors => Choice::Scissors,
            },
            Outcome::Win => match c {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Loss => match c {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
        }
    }

    fn score(&self) -> usize {
        let mut score = match self.me {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };

        match self.evaluate() {
            Outcome::Draw => score += 3,
            Outcome::Win => score += 6,
            Outcome::Loss => {}
        }

        score
    }
}

impl FromInput for Day2 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut m = Day2 {
            moves: std::vec::Vec::new(),
        };

        for l in lines {
            let mut items = l.split_whitespace();
            let opp = items.next().unwrap();
            let me = items.next().unwrap();
            m.moves.push(Move {
                me: Choice::from_string(me),
                opponent: Choice::from_string(opp),
                goal: Outcome::from_string(me),
            });
        }
        m
    }
}

impl DaySolution for Day2 {
    fn part_one(&self) -> String {
        let mut total = 0;
        for m in &self.moves {
            total += m.score()
        }
        total.to_string()
    }

    fn part_two(&self) -> String {
        let mut total = 0;
        for m in &self.moves {
            let mut move2 = m.clone();
            move2.me = Move::solve(&move2.opponent, &move2.goal);
            total += move2.score();
        }
        total.to_string()
    }
}
