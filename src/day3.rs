use std::{vec};

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

#[derive(Clone)]
struct Compartments {
    first : vec::Vec<u8>,
    second : vec::Vec<u8>
}

pub struct Day3 {
    rucksacks : vec::Vec<Compartments>
}

impl Compartments {
    fn from_string(s : &str) -> Compartments {
        let pivot = s.len()/2;
        Compartments{ first : s[0..pivot].as_bytes().to_vec(),
                      second: s[pivot..s.len()].as_bytes().to_vec() } 
    }

    fn find_overlapping(&self) -> u8 {
        **self.first.iter().filter(|&x| self.second.contains(x)).collect::<Vec<_>>().first().unwrap()
    }

    /*
     * Haha, for the first task we split up things into compartments 1 and 2, now we need
     * a single view of this again...
     */
    fn all(&self) -> Vec<u8> {
        let mut r = Vec::new();
        for c in self.first.clone() {
            r.push(c);
        }
        for c in self.second.clone() {
            r.push(c)
        }
        r
    }

    fn score(c : &u8) -> u8 {
        match c {
            b'a'..=b'z' => *c-b'a'+1,
            b'A'..=b'Z' => *c-b'A'+27,
            _ => panic!("Unexpected character {}", c)
        }
    }
}

impl Day3 {
    fn score_1(&self) -> u32 {
        let mut score: u32 = 0;
        for r in &self.rucksacks {
            //println!("overlap: {} score {}", *&r.find_overlapping() as char, Compartments::score(&r.find_overlapping()));
            score += Compartments::score(&r.find_overlapping()) as u32;
        }
        score
    }

    fn score_2(&self) -> u32 {
        let group_count = self.rucksacks.len() / 3;
        let mut score = 0;
        for n in 0..group_count {
            let r1 = self.rucksacks[3*n].all();
            let r2 = self.rucksacks[3*n+1].all();
            let r3 = self.rucksacks[3*n+2].all();

            for c in r1 {
                if r2.contains(&c) && r3.contains(&c) {
                    //println!("{} {}", n, c as char);
                    score += Compartments::score(&c) as u32;
                    break;
                }
            }
        }
        score
    }
}

impl FromInput for Day3 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut rucksack = Day3{ rucksacks: Vec::new() };
        for l in lines {
            rucksack.rucksacks.push(Compartments::from_string(&l));
        }
        rucksack
    }
}

impl DaySolution for Day3 {
    fn part_one(&self) -> String {
        self.score_1().to_string()
    }

    fn part_two(&self) -> String {
        self.score_2().to_string()
    }
}
