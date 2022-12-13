use std::ffi::c_uchar;

use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct
pub struct Day8 {
    trees: Vec<Vec<u8>>,
    dim_x: usize,
    dim_y: usize,
}

impl FromInput for Day8 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut trees = vec![];

        let mut x = 0;
        let mut y = 0;

        for l in lines {
            let mut heights = vec![];
            x = 0;
            y += 1;

            for c in l.as_bytes() {
                let num = c - '0' as c_uchar;
                heights.push(num);
                x += 1;
            }

            trees.push(heights);
        }

        Day8 {
            trees: trees,
            dim_x: x,
            dim_y: y,
        }
    }
}

impl DaySolution for Day8 {
    fn part_one(&self) -> String {
        let mut reachable = vec![];
        for _ in 0..self.dim_y {
            let mut line = vec![];
            for _ in 0..self.dim_x {
                line.push(false);
            }
            reachable.push(line);
        }

        /* mark boundaries as reachable */
        for i in 0..self.dim_x {
            reachable[0][i] = true;
            reachable[i][0] = true;
            reachable[self.dim_y - 1][i] = true;
            reachable[i][self.dim_x - 1] = true;
        }

        for x in 1..self.dim_x - 1 {
            for y in 1..self.dim_y - 1 {
                /* left */
                let val = self.trees[x][y];
                let mut vis1 = true;
                let mut vis2 = true;
                let mut vis3 = true;
                let mut vis4 = true;
                for x2 in (0..x).rev() {
                    if self.trees[x2][y] >= val {
                        vis1 = false;
                        break;
                    }
                }
                /* right */
                for x2 in x + 1..self.dim_x {
                    if self.trees[x2][y] >= val {
                        vis2 = false;
                        break;
                    }
                }
                /* up */
                for y2 in (0..y).rev() {
                    if self.trees[x][y2] >= val {
                        vis3 = false;
                        break;
                    }
                }
                /* down */
                for y2 in y + 1..self.dim_y {
                    if self.trees[x][y2] >= val {
                        vis4 = false;
                        break;
                    }
                }

                //println!("x{} y{} val {} -> {} {} {} {}", x, y, val, vis1, vis2, vis3, vis4);
                reachable[x][y] = vis1 | vis2 | vis3 | vis4;
            }
        }

        let mut count = 0;
        for line in reachable {
            for b in line {
                if b {
                    count += 1;
                }
            }
        }

        count.to_string()
    }

    fn part_two(&self) -> String {
        let mut scores = vec![];
        let mut max = 0;
        for x in 0..self.dim_x {
            let mut line = vec![];
            for y in 0..self.dim_y {
                let height = self.trees[x][y];

                if x == 0 || y == 0 || x == self.dim_x - 1 || y == self.dim_y - 1 {
                    line.push(0);
                    continue;
                }

                //println!("{x} {y} {height}");

                let mut x2 = x;
                while x2 > 0 {
                    x2 -= 1;
                    if self.trees[x2][y] >= height {
                        break;
                    }
                }
                let count_l = x - x2;
                //println!("  {x2}..{x} {y} {height} {}", count_l);

                let mut x2 = x;
                while x2 < self.dim_x - 1 {
                    x2 += 1;
                    if self.trees[x2][y] >= height {
                        break;
                    }
                }
                let count_r = x2 - x;
                //println!("  {x}..{x2} {y} {height} {}", count_r);

                let mut y2 = y;
                while y2 > 0 {
                    y2 -= 1;
                    if self.trees[x][y2] >= height {
                        break;
                    }
                }
                let count_d = y - y2;
                //println!("  {x} {y2}..{y} {height} {}", count_d);

                let mut y2 = y;
                while y2 < self.dim_y - 1 {
                    y2 += 1;
                    if self.trees[x][y2] >= height {
                        break;
                    }
                }
                let count_u = y2 - y;
                //println!("  {x} {y2}..{y} {height} {}", count_u);

                //println!("  => x{x} y{y} h{height} {count_l} {count_r} {count_u} {count_d} {}", count_l*count_u*count_r*count_d);
                let res = count_l * count_r * count_u * count_d;
                if max < res {
                    max = res;
                }
                line.push(res);
            }
            scores.push(line);
        }

        max.to_string()
    }
}
