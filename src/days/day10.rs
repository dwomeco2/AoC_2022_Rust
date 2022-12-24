use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../../input/day10");

pub fn part1() {
    let mut cpu = build_cpu();
    let mut sl = StrengthLogger::new(vec![20, 60, 100, 140, 180, 220]);

    while !cpu.finished() {
        sl.log(cpu.cycle, cpu.get_register("X"));
        cpu.tick();
    }

    let result = sl.logged_strength.iter().map(|&(_, s)| s).sum::<isize>();
    println!("{result}");
}

pub fn part2() {
    let mut cpu = build_cpu();
    let mut crt = Crt::new(40, 6);

    while !cpu.finished() {
        crt.draw(cpu.get_register("X"));
        cpu.tick();
    }

    for l in crt.pixels.chunks(40) {
        println!("{:?}", l.iter().collect::<String>());
    }
}

fn build_cpu<'a>() -> Cpu<'a> {
    let mut registers = HashMap::new();
    registers.insert("X", 1);

    let ins_prog = parse().map(|i| (i, i.needed_cycle()));
    Cpu::new(registers, ins_prog)
}

fn parse() -> impl Iterator<Item = Instruction> {
    INPUT.lines().map(|l| match l {
        "noop" => Instruction::Noop,
        l if l.starts_with("addx") => {
            let mut it = l.split_whitespace();
            let val = it.nth(1).unwrap().parse::<isize>().unwrap();
            Instruction::Addx(val)
        }
        _ => unreachable!(),
    })
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    fn needed_cycle(&self) -> usize {
        match *self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

type InstructionProgress = (Instruction, usize);

struct Cpu<'a> {
    cycle: usize,
    registers: HashMap<&'a str, isize>,
    instructions: VecDeque<InstructionProgress>,
}

impl<'a> Cpu<'a> {
    pub fn new<I: Iterator<Item = InstructionProgress> + 'a>(
        registers: HashMap<&'a str, isize>,
        it: I,
    ) -> Self {
        Self {
            cycle: 1,
            registers,
            instructions: it.collect(),
        }
    }

    fn get_register(&self, name: &str) -> isize {
        *self.registers.get(name).unwrap()
    }

    fn tick(&mut self) {
        //During cycle
        let ins = self.instructions.pop_front();
        if let Some(ins) = ins {
            if ins.1 > 1 {
                self.instructions.push_front((ins.0, ins.1 - 1));
            } else {
                match ins.0 {
                    Instruction::Noop => (),
                    Instruction::Addx(val) => {
                        *self.registers.entry("X").or_default() += val;
                    }
                }
            }
        }
        //After cycle
        self.cycle += 1;
    }

    fn finished(&self) -> bool {
        self.instructions.is_empty()
    }
}

struct StrengthLogger {
    log_cycle: Vec<usize>,
    logged_strength: Vec<(usize, isize)>,
}

impl StrengthLogger {
    pub fn new(log_cycle: Vec<usize>) -> Self {
        Self {
            log_cycle,
            logged_strength: vec![],
        }
    }

    fn log(&mut self, cycle: usize, val: isize) {
        // println!("{cycle} {val} {}", cycle as isize * val);
        if self.log_cycle.contains(&cycle) {
            self.logged_strength.push((cycle, cycle as isize * val));
        }
    }
}

struct Crt {
    width: usize,
    #[allow(dead_code)]
    height: usize,
    pixels: Vec<char>,
}

impl Crt {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![],
        }
    }

    fn draw(&mut self, reg_x: isize) {
        let curr_pos = (self.pixels.len() % self.width) as isize;
        if curr_pos >= reg_x - 1 && curr_pos <= reg_x + 1 {
            self.pixels.push('#');
        } else {
            self.pixels.push('.');
        }
    }
}
