#![feature(iter_array_chunks, iter_intersperse)]

use std::ops::Rem;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut cpu = CPU1::init();
    cpu.execute_many(&input());
    println!("{:?}", cpu.signals.iter().sum::<isize>());
}

fn part2() {
    let mut cpu = CPU2::init();
    cpu.execute_many(&input());
    println!(
        "{}",
        cpu.screen
            .chars()
            .array_chunks::<40>()
            .map(|x| x.iter().collect::<String>())
            .intersperse("\n".to_owned())
            .collect::<String>()
    );
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx(isize),
}

impl Op {
    fn cycle_time(&self) -> isize {
        match self {
            Op::Noop => 1,
            Op::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct CPU1 {
    x: isize,
    cycle: isize,
    signals: Vec<isize>,
    next_signal_cycle: isize,
}

impl CPU1 {
    fn init() -> Self {
        CPU1 {
            x: 1,
            cycle: 1,
            signals: vec![],
            next_signal_cycle: 20,
        }
    }

    fn execute_one(&mut self, op: &Op) {
        let curr = self.cycle;
        let next = self.cycle + op.cycle_time();
        self.cycle = next;
        if curr <= self.next_signal_cycle && next > self.next_signal_cycle {
            self.signals.push(self.x * self.next_signal_cycle);
            self.next_signal_cycle += 40;
        }
        if let Op::Addx(d) = op {
            self.x += d
        }
    }

    fn execute_many(&mut self, ops: &Vec<Op>) {
        ops.iter().for_each(|op| {
            self.execute_one(op);
        });
    }
}

#[derive(Debug)]
struct CPU2 {
    x: isize,
    cycle: isize,
    screen: String,
}

impl CPU2 {
    fn init() -> Self {
        CPU2 {
            x: 1,
            cycle: 0,
            screen: "".to_owned(),
        }
    }

    fn execute_one(&mut self, op: &Op) {
        let curr = self.cycle;
        let next = self.cycle + op.cycle_time();
        (curr..next).into_iter().for_each(|_| {
            let rowpos = self.cycle.rem(40);
            self.screen.push(if (rowpos - self.x).abs() <= 1 {
                '#'
            } else {
                '.'
            });
            self.cycle += 1;
        });
        if let Op::Addx(d) = op {
            self.x += d
        }
    }

    fn execute_many(&mut self, ops: &Vec<Op>) {
        ops.iter().for_each(|op| {
            self.execute_one(op);
        });
    }
}

fn input() -> Vec<Op> {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        // include_str!("../test2.txt")
        .lines()
        .map(|s| {
            if s == "noop" {
                Op::Noop
            } else {
                let (_, n) = s.split_once(" ").unwrap();
                let d = n.parse().unwrap();
                Op::Addx(d)
            }
        })
        .collect()
}
