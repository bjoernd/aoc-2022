use crate::{DaySolution, FromInput};

#[derive(Copy, Clone)]
enum Instruction {
    NOOP,
    ADDX,
}

#[derive(Copy, Clone)]
struct Command {
    instr: Instruction,
    cycles_completed: usize,
    parameter: i32,
}

struct CPU {
    reg_x: i32,
    cycle_counter: usize,
}

impl CPU {
    fn execute(&mut self, c: &mut Command) -> bool {
        self.cycle_counter += 1;
        match c.instr {
            Instruction::NOOP => true,
            Instruction::ADDX => {
                if c.cycles_completed == 0 {
                    c.cycles_completed += 1;
                    false
                } else {
                    self.reg_x += c.parameter;
                    c.cycles_completed += 1;
                    true
                }
            }
        }
    }

    fn draw(&self) {
        let pos = (self.cycle_counter - 1) % 40;

        let sprite_dist = self.reg_x - pos as i32;
        if sprite_dist.abs() < 2 {
            print!("#");
        } else {
            print!(" ");
        }

        if pos == 39 {
            println!();
        }
    }
}

// TODO: Model the problem into this struct
pub struct Day10 {
    instructions: Vec<Command>,
}

impl FromInput for Day10 {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut instr = vec![];
        for l in lines {
            let mut data = l.split(" ");
            let op = data.next().unwrap();
            match op {
                "noop" => instr.push(Command {
                    instr: Instruction::NOOP,
                    cycles_completed: 0,
                    parameter: 0,
                }),
                "addx" => {
                    let param = i32::from_str_radix(data.next().unwrap(), 10).unwrap();
                    instr.push(Command {
                        instr: Instruction::ADDX,
                        cycles_completed: 0,
                        parameter: param,
                    });
                }
                _ => panic!("Unknown op {op}"),
            }
        }

        Day10 {
            instructions: instr,
        }
    }
}

impl DaySolution for Day10 {
    fn part_one(&self) -> String {
        let mut cpu = CPU {
            reg_x: 1,
            cycle_counter: 1,
        };
        let mut target_inst = 20;
        let mut score = 0;

        for inst in &self.instructions {
            let mut tmp_i = inst.clone();
            let mut done = false;

            while !done {
                //println!("[{:04}] {}", cpu.cycle_counter, cpu.reg_x);
                done = cpu.execute(&mut tmp_i);
                if cpu.cycle_counter == target_inst {
                    target_inst += 40;
                    score += cpu.reg_x * cpu.cycle_counter as i32;
                }
            }
        }
        score.to_string()
    }

    fn part_two(&self) -> String {
        let mut cpu = CPU {
            reg_x: 1,
            cycle_counter: 1,
        };

        for inst in &self.instructions {
            let mut tmp_i = inst.clone();
            let mut done = false;

            while !done {
                //println!("[{:04}] {}", cpu.cycle_counter, cpu.reg_x);
                cpu.draw();
                done = cpu.execute(&mut tmp_i);
            }
        }
        String::from("OK")
    }
}
