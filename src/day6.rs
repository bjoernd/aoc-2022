use crate::{DaySolution, FromInput};
use std::collections::HashSet;

// TODO: Model the problem into this struct
pub struct Day6 {
    input: String,
}

impl FromInput for Day6 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut x = String::from("");
        for l in lines {
            x = l.clone();
        }
        Day6 { input: x }
    }
}

impl Day6 {
    fn is_marker(s: &str) -> bool {
        let mut uniq_chars = HashSet::<char>::new();
        //println!("{}", s);
        for c in s.chars() {
            uniq_chars.insert(c);
        }
        uniq_chars.len() == s.len()
    }

    fn solution(&self, slicelen: usize) -> String {
        let mut b = slicelen; /* first candidate is the slice length itself */
        let mut s = &self.input[b - slicelen..b];
        while !Day6::is_marker(s) && b < self.input.len() {
            b += 1;
            s = &self.input[b - slicelen..b];
        }

        b.to_string()
    }
}

impl DaySolution for Day6 {
    fn part_one(&self) -> String {
        self.solution(4)
    }

    fn part_two(&self) -> String {
        self.solution(14)
    }
}
