use crate::{DaySolution, FromInput};

// TODO: Model the problem into this struct

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize, /* inclusive */
}

pub struct Day4 {
    ranges: Vec<Range>,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start <= self.start && other.end >= self.start)
            || (other.start <= self.end && other.end >= self.end)
    }
}

impl FromInput for Day4 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut d = Day4 { ranges: Vec::new() };
        for l in lines {
            for r in l.split(",") {
                let mut sp = r.split("-");
                let first = usize::from_str_radix(sp.next().unwrap(), 10).unwrap();
                let second = usize::from_str_radix(sp.next().unwrap(), 10).unwrap();
                d.ranges.push(Range {
                    start: first,
                    end: second,
                });
                //println!("{},{}", first, second);
            }
        }
        d
    }
}

macro_rules! solver {
    ($obj:expr, $detector:expr) => {{
        let max = $obj.ranges.len() / 2;
        let mut count = 0;
        for i in 0..max {
            let r1 = &$obj.ranges[i * 2];
            let r2 = &$obj.ranges[i * 2 + 1];

            if $detector(r1, r2) || $detector(r2, r1) {
                count += 1;
            }
        }
        count.to_string()
    }};
}

impl DaySolution for Day4 {
    fn part_one(&self) -> String {
        solver!(self, Range::contains)
    }

    fn part_two(&self) -> String {
        solver!(self, Range::overlaps)
    }
}
