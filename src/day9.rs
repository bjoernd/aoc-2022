use crate::{DaySolution, FromInput};
use std::{collections::HashSet, hash::Hash};

// TODO: Model the problem into this struct

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Coord {
    fn mov(&mut self, d: &Direction) {
        match d {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }

    /*
      Following is harder than I thought. I didn't get it until I read
      https://github.com/cs-cordero/advent-of-code/blob/master/rs/2022/day09/src/main.rs
    */
    fn follow(&mut self, other: &Coord) {
        let dist_x: i32 = other.x - self.x;
        let dist_y: i32 = other.y - self.y;
        //println!("{} {} dx={} dy={}", other.x, other.y, dist_x, dist_y);

        if dist_x != 0 && dist_y != 0 && (dist_x.abs() + dist_y.abs() > 2) {
            self.x += dist_x.signum();
            self.y += dist_y.signum();
        } else if dist_x.abs() > 1 {
            self.x += dist_x.signum();
        } else if dist_y.abs() > 1 {
            self.y += dist_y.signum();
        }
    }
}

struct Cmd {
    dir: Direction,
    count: usize,
}

pub struct Day9 {
    commands: Vec<Cmd>,
}

impl FromInput for Day9 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d9 = Day9 { commands: vec![] };

        for l in lines {
            let mut raw = l.split(" ");
            let dir = raw.next().unwrap();
            let count = raw.next().unwrap();

            match dir {
                "L" => d9.commands.push(Cmd {
                    dir: Direction::Left,
                    count: usize::from_str_radix(count, 10).unwrap(),
                }),
                "R" => d9.commands.push(Cmd {
                    dir: Direction::Right,
                    count: usize::from_str_radix(count, 10).unwrap(),
                }),
                "U" => d9.commands.push(Cmd {
                    dir: Direction::Up,
                    count: usize::from_str_radix(count, 10).unwrap(),
                }),
                "D" => d9.commands.push(Cmd {
                    dir: Direction::Down,
                    count: usize::from_str_radix(count, 10).unwrap(),
                }),
                _ => panic!("Unknown direction {}!", dir),
            }
        }

        d9
    }
}

impl DaySolution for Day9 {
    fn part_one(&self) -> String {
        let mut head = Coord { x: 0, y: 0 };
        let mut tail = Coord { x: 0, y: 0 };
        let mut coords = HashSet::<Coord>::new();

        for c in &self.commands {
            for _ in 0..c.count {
                head.mov(&c.dir);
                tail.follow(&head);
                coords.insert(tail);
            }
        }

        coords.len().to_string()
    }

    fn part_two(&self) -> String {
        let mut snake = vec![];
        let mut coords = HashSet::<Coord>::new();
        for _ in 0..10 {
            snake.push(Coord { x: 0, y: 0 });
        }

        for c in &self.commands {
            for _ in 0..c.count {
                snake[0].mov(&c.dir);
                for i in 1..10 {
                    let prev = snake[i - 1];
                    snake[i].follow(&prev);
                }
                coords.insert(snake[9]);
            }
        }

        coords.len().to_string()
    }
}
