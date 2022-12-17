use crate::{DaySolution, FromInput};

#[derive(Clone, Copy)]
enum Operations {
    ADD,
    MULTIPLY,
    SQUARE,
}

impl Operations {
    fn to_string(&self) -> &str {
        match self {
            Self::ADD => "+",
            Self::MULTIPLY => "*",
            Self::SQUARE => "sq",
        }
    }
}

#[derive(Clone, Copy)]
struct Operation {
    op: Operations,
    param: usize,
}

impl Operation {
    fn to_string(&self) -> String {
        format!("{} {}", &self.op.to_string(), &self.param)
    }

    fn apply(&self, src: usize) -> usize {
        match self.op {
            Operations::ADD => src + self.param,
            Operations::MULTIPLY => src * self.param,
            Operations::SQUARE => src * src,
        }
    }

    fn apply64(&self, src: i64) -> i64 {
        match self.op {
            Operations::ADD => src + self.param as i64,
            Operations::MULTIPLY => src * self.param as i64,
            Operations::SQUARE => src * src as i64,
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Operation,
    test: usize,
    test_true: usize,
    test_false: usize,
    inspected: usize,
}

impl Monkey {
    fn print(&self) {
        print!("Monkey [");
        for i in &self.items {
            print!("{i} ");
        }
        println!(
            "] op ({}) test ({} {} {})",
            &self.op.to_string(),
            &self.test,
            &self.test_true,
            &self.test_false
        );
    }
}

// TODO: Model the problem into this struct
pub struct Day11 {
    monkeys: Vec<Monkey>,
}

impl FromInput for Day11 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut monkeys: Vec<Monkey> = vec![];
        let mut items = vec![];
        let mut operation = Operation {
            op: Operations::ADD,
            param: 1,
        };
        let mut test_var = 1;
        let mut test_false = 1;
        let mut test_true = 1;

        for l in lines {
            if l.starts_with("Monkey ") {
                continue;
            } else if l == "" {
                let m = Monkey {
                    items: items,
                    op: operation,
                    test: test_var,
                    test_false: test_false,
                    test_true: test_true,
                    inspected: 0,
                };
                //m.print();
                monkeys.push(m);
                items = vec![];
                operation = Operation {
                    op: Operations::ADD,
                    param: 1,
                };
                test_var = 1;
                test_false = 1;
                test_true = 1;
            } else if l.starts_with("  Starting items: ") {
                let mut data = l.trim().split(" ");
                let mut item = data.nth(2);
                while item.is_some() {
                    items.push(
                        i64::from_str_radix(item.unwrap().replace(",", "").as_str(), 10).unwrap(),
                    );
                    item = data.next();
                }
            } else if l.starts_with("  Operation:") {
                let mut data = l.trim().split(" ");
                let op = data.nth(4).unwrap();
                let param = data.next().unwrap();

                if param == "old" {
                    operation.op = Operations::SQUARE;
                } else {
                    match op {
                        "+" => operation.op = Operations::ADD,
                        "*" => operation.op = Operations::MULTIPLY,
                        _ => panic!("Unknown op {op}"),
                    }
                    operation.param = usize::from_str_radix(param, 10).unwrap();
                }
            } else if l.starts_with("  Test: divisible by") {
                let mut data = l.trim().split(" ");
                let p = data.nth(3).unwrap();
                test_var = usize::from_str_radix(p, 10).unwrap();
            } else if l.starts_with("    If true:") {
                let mut data = l.trim().split(" ");
                let p = data.nth(5).unwrap();
                test_true = usize::from_str_radix(p, 10).unwrap();
            } else if l.starts_with("    If false:") {
                let mut data = l.trim().split(" ");
                let p = data.nth(5).unwrap();
                test_false = usize::from_str_radix(p, 10).unwrap();
            } else {
                panic!("Unknown pattern {l}");
            }
        }

        let m = Monkey {
            items,
            op: operation,
            test: test_var,
            test_false,
            test_true,
            inspected: 0,
        };
        //m.print();
        monkeys.push(m);

        Day11 { monkeys: monkeys }
    }
}

impl DaySolution for Day11 {
    fn part_one(&self) -> String {
        let mut monkeys_ = self.monkeys.clone();

        for _ in 0..20 {
            for monkey_idx in 0..monkeys_.len() {
                //println!("Monkey {monkey_idx}:");

                for item_idx in 0..monkeys_[monkey_idx].items.len() {
                    let mut i = monkeys_[monkey_idx].items[item_idx];

                    //println!(" Monkey inspects an item with a worry level of {i}");

                    i = monkeys_[monkey_idx].op.apply64(i);
                    //println!("  Worry level increases to {i}");
                    i /= 3;
                    //println!("  Monkey gets bored with item. Worry level is divided by 3 to {i}.");

                    monkeys_[monkey_idx].inspected += 1;

                    let target = if i % monkeys_[monkey_idx].test as i64 == 0 {
                        //println!("  Current worry level IS divisible by {}", monkeys_[monkey_idx].test);
                        monkeys_[monkey_idx].test_true
                    } else {
                        //println!("  Current worry level is not divisible by {}", monkeys_[monkey_idx].test);
                        monkeys_[monkey_idx].test_false
                    };
                    //println!("  Item with worry level {i} is thrown to monkey {target}");
                    monkeys_[target].items.push(i);
                }
                monkeys_[monkey_idx].items.clear();
            }
            /*
            for idx in 0..monkeys_.len() {
                monkeys_[idx].print();
            }
            */
        }

        let mut max1 = 0;
        let mut max2 = 0;
        for idx in 0..monkeys_.len() {
            //println!("{idx} {}", monkeys_[idx].inspected);
            if monkeys_[idx].inspected > max1 {
                max2 = max1;
                max1 = monkeys_[idx].inspected;
            } else if monkeys_[idx].inspected > max2 {
                max2 = monkeys_[idx].inspected;
            }
        }

        String::from(format!("{}", max1 * max2))
    }

    fn part_two(&self) -> String {
        let mut monkeys_ = self.monkeys.clone();

        for _ in 0..20 {
            for monkey_idx in 0..monkeys_.len() {
                //println!("Monkey {monkey_idx}:");

                for item_idx in 0..monkeys_[monkey_idx].items.len() {
                    let mut i = monkeys_[monkey_idx].items[item_idx];

                    //println!(" Monkey inspects an item with a worry level of {i}");

                    i = monkeys_[monkey_idx].op.apply64(i);
                    //println!("  Worry level increases to {i}");
                    i /= 3;
                    //println!("  Monkey gets bored with item. Worry level is divided by 3 to {i}.");

                    monkeys_[monkey_idx].inspected += 1;

                    let target = if i % monkeys_[monkey_idx].test as i64 == 0 {
                        //println!("  Current worry level IS divisible by {}", monkeys_[monkey_idx].test);
                        monkeys_[monkey_idx].test_true
                    } else {
                        //println!("  Current worry level is not divisible by {}", monkeys_[monkey_idx].test);
                        monkeys_[monkey_idx].test_false
                    };
                    //println!("  Item with worry level {i} is thrown to monkey {target}");
                    monkeys_[target].items.push(i);
                }
                monkeys_[monkey_idx].items.clear();
            }
            /*
            for idx in 0..monkeys_.len() {
                monkeys_[idx].print();
            }
            */
        }

        let mut max1 = 0;
        let mut max2 = 0;
        for idx in 0..monkeys_.len() {
            //println!("{idx} {}", monkeys_[idx].inspected);
            if monkeys_[idx].inspected > max1 {
                max2 = max1;
                max1 = monkeys_[idx].inspected;
            } else if monkeys_[idx].inspected > max2 {
                max2 = monkeys_[idx].inspected;
            }
        }

        String::from(format!("{}", max1 * max2))
    }
}
