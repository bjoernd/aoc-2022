use crate::{DaySolution, FromInput};
use std::vec::Vec;

// TODO: Model the problem into this struct
pub struct Day1 {
    weights: Vec<usize>,
}

impl FromInput for Day1 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut cur_weight: usize = 0;
        let mut self_struct = Day1 {
            weights: Vec::new(),
        };

        for l in lines {
            match l.len() {
                0 => {
                    self_struct.weights.push(cur_weight);
                    cur_weight = 0;
                }
                _ => {
                    cur_weight += l.parse::<usize>().unwrap();
                }
            }
        }
        self_struct.weights.push(cur_weight);

        self_struct.weights.sort();

        self_struct
    }
}

impl DaySolution for Day1 {
    fn part_one(&self) -> String {
        self.weights.last().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        self.weights.iter().rev().take(3).sum::<usize>().to_string()
    }
}
