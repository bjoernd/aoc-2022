use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

#[derive(Clone, Copy)]
struct Move {
    count: usize,
    src: usize,
    dest: usize,
}

#[derive(Clone)]
pub struct Day5 {
    columns: Vec<Vec<char>>,
    moves: Vec<Move>,
}

impl Day5 {
    #[allow(dead_code)]
    fn print(&self) {
        let mut i = 1;
        for c in &self.columns {
            print!("{:02} ", i);
            for ch in c {
                print!("{} ", ch);
            }
            println!();
            i += 1;
        }
    }
}

impl FromInput for Day5 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        // before the first empty line, we are reading initial state, afterwards we are reading moves
        let mut reading_moves = false;

        let mut c = Vec::new();

        let mut i = 1;
        while i < 10 {
            c.push(Vec::<char>::new());
            i += 1;
        }

        let mut res = Day5 {
            columns: c,
            moves: Vec::new(),
        };

        for l in lines {
            if l == "" {
                reading_moves = true;
                continue;
            }

            //println!("{}", l);
            if reading_moves {
                // move 2 from 3 to 7
                let items = l.split(" ").collect::<Vec<&str>>();
                let cnt = usize::from_str_radix(items[1], 10).unwrap();
                let src = usize::from_str_radix(items[3], 10).unwrap();
                let dst = usize::from_str_radix(items[5], 10).unwrap();
                res.moves.push(Move {
                    count: cnt,
                    src: src,
                    dest: dst,
                });
            } else {
                /* reading init state */
                let mut i: usize = 1;
                while i < 10 {
                    let ch = l.chars().nth((i - 1) * 4 + 1).unwrap();

                    if ch.is_numeric() {
                        break;
                    };
                    //println!("'{}'", ch);

                    if ch != ' ' {
                        res.columns[i - 1].push(ch);
                    }
                    i += 1;
                }
            }
        }

        for i in 1..res.columns.len() + 1 {
            res.columns[i - 1].reverse();
        }

        //res.print();

        res
    }
}

impl Day5 {
    fn top_row(columns :&Vec<Vec<char>>) -> String {
        let mut res = String::from("");

        for col in columns {
            res.push(*col.last().unwrap());
        }

        res
    }
}

impl DaySolution for Day5 {
    fn part_one(&self) -> String {
        let mut cp = self.clone();

        for m in &cp.moves {
            for _ in 0..m.count {
                let ch = cp.columns[m.src-1].pop().unwrap();
                cp.columns[m.dest-1].push(ch);
            }
        }
        //cp.print();
        Day5::top_row(&cp.columns)
    }

    fn part_two(&self) -> String {
        let mut cp = self.clone();

        for m in &cp.moves {
            let mut tmp = Vec::<char>::new();

            for _ in 0..m.count {
                tmp.push(cp.columns[m.src-1].pop().unwrap());
            }
            tmp.reverse();
            for c in tmp {
                cp.columns[m.dest-1].push(c);
            }
        }
        //cp.print();
        Day5::top_row(&cp.columns)
    }
}
