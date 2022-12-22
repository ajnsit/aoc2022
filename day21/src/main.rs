use std::collections::HashMap;

use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut i = input();
    dbg!(&i);
    dbg!(yell("root", &mut i));
}

fn part2() {}

#[derive(Debug, Clone)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
}

impl Op {
    fn apply(&self, x: isize, y: isize) -> isize {
        match self {
            Op::Plus => x + y,
            Op::Minus => x - y,
            Op::Mult => x * y,
            Op::Div => x / y,
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Num(isize),
    Op(Op, String, String),
    Var(isize), // The num is only used in part 1
}

fn yell(name: &str, monkeys: &mut HashMap<String, Monkey>) -> isize {
    match monkeys.get(name).unwrap().clone() {
        Monkey::Num(x) => x,
        Monkey::Var(x) => x,
        Monkey::Op(op, m1, m2) => {
            let v1 = yell(&m1, monkeys);
            monkeys.insert(m1, Monkey::Num(v1));
            let v2 = yell(&m2, monkeys);
            monkeys.insert(m2, Monkey::Num(v2));
            op.apply(v1, v2)
        }
    }
}

fn input() -> HashMap<String, Monkey> {
    let reg1 = Regex::new(r"(\w+): (.+)").unwrap();
    let reg_op = Regex::new(r"(\w+) (.) (\w+)").unwrap();
    let reg_num = Regex::new(r"(\d+)").unwrap();
    include_str!("../input.txt")
        // include_str!("../test.txt")
        .lines()
        .map(|s| {
            let captures1 = reg1
                .captures(s)
                .expect(&format!("Unable to parse string: {:?}", s));
            let name = captures1[1].to_owned();
            let val = captures1[2].to_owned();
            (
                name.clone(),
                reg_op
                    .captures(&val)
                    .map(|c| Monkey::Op(op(&c[2]), c[1].to_owned(), c[3].to_owned()))
                    .or(reg_num.captures(&val).map(|c| {
                        let v = c[1].parse::<isize>().unwrap();
                        if name == "humn" {
                            Monkey::Var(v)
                        } else {
                            Monkey::Num(v)
                        }
                    }))
                    .unwrap(),
            )
        })
        .collect()
}

fn op(s: &str) -> Op {
    match s {
        "+" => Op::Plus,
        "-" => Op::Minus,
        "*" => Op::Mult,
        "/" => Op::Div,
        _ => panic!("UNKNOWN OP {}", s),
    }
}
