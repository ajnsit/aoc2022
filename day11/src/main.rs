use std::cmp::Reverse;
use std::fmt::{Debug, Result};
use std::{collections::HashMap, fmt::Formatter};

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("{:?}", score(iterate(20, round_part1, input())));
}

fn part2() {
    println!("{:?}", score(iterate(10000, round_part2, input())));
}

fn iterate<A>(times: usize, f: fn(A) -> A, initial: A) -> A {
    if times <= 0 {
        initial
    } else {
        iterate(times - 1, f, f(initial))
    }
}

fn score(monkeys: Vec<Monkey>) -> usize {
    let mut m1 = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    m1.sort_by_key(|i| Reverse(*i));
    m1.iter().take(2).product::<usize>()
}

fn input() -> Vec<Monkey> {
    let mut monkeys = include_str!("../input.txt")
        // let mut monkeys = include_str!("../test.txt")
        .split("\n\n")
        .map(|s| Monkey::parse(s))
        .collect::<Vec<_>>();
    monkeys.sort_by_key(|m| m.id);
    monkeys
}

struct Monkey {
    id: usize,
    items: Vec<usize>,
    step: Box<dyn Fn(usize) -> usize>,
    factor: usize,
    throw: Box<dyn Fn(usize) -> usize>,
    inspections: usize,
}

impl Monkey {
    fn turn(&mut self, rem: usize, part: usize) -> Vec<(usize, usize)> {
        let mut x = vec![];
        std::mem::swap(&mut x, &mut self.items);
        self.inspections += x.len();
        x.iter()
            .map(|worry| {
                let new_worry = if part == 1 {
                    (self.step)(*worry) / 3
                } else {
                    (self.step)(*worry) % rem
                };
                let giveto = (self.throw)(new_worry);
                (giveto, new_worry)
            })
            .collect::<Vec<(usize, usize)>>()
    }

    fn parse(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<&str>>();
        let id: usize = lines[0].split([' ', ':']).nth(1).unwrap().parse().unwrap();
        let items = lines[1]
            .split_once(':')
            .unwrap()
            .1
            .split(", ")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();
        let step = parse_function(&extract_val(lines[2])[" new = ".len()..]);
        let divisible_by: usize = extract_val(lines[3])
            .split_ascii_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        let factor = divisible_by;
        let on_true: usize = extract_val(lines[4])
            .split_ascii_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let on_false: usize = extract_val(lines[5])
            .split_ascii_whitespace()
            .nth(3)
            .unwrap()
            .parse()
            .unwrap();
        let throw = Box::new(move |x| {
            if x % divisible_by == 0 {
                on_true
            } else {
                on_false
            }
        });
        Monkey {
            id,
            items,
            step,
            factor,
            throw,
            inspections: 0,
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self.items)
    }
}

fn extract_val(s: &str) -> &str {
    s.split_once(':').unwrap().1
}

// There seems to be no way to return an unboxed function when I have a conditional inside the function
type IntFn = Box<dyn Fn(usize) -> usize>;

fn parse_arg(s: &str) -> IntFn {
    let is_var = s == "old";
    if is_var {
        Box::new(|x| x)
    } else {
        let y = s.parse().unwrap();
        Box::new(move |_| y)
    }
}

fn parse_function(s: &str) -> IntFn {
    let toks = s.split_ascii_whitespace().collect::<Vec<_>>();
    let arg1 = parse_arg(toks[0]);
    let arg2 = parse_arg(toks[2]);
    match toks[1] {
        "+" => Box::new(move |x| arg1(x) + arg2(x)),
        "*" => Box::new(move |x| arg1(x) * arg2(x)),
        _ => panic!("Unknown operator {}", toks[1]),
    }
}

fn round_part1(monkeys: Vec<Monkey>) -> Vec<Monkey> {
    round(monkeys, 1)
}
fn round_part2(monkeys: Vec<Monkey>) -> Vec<Monkey> {
    round(monkeys, 2)
}

fn round(monkeys: Vec<Monkey>, part: usize) -> Vec<Monkey> {
    let lcm = monkeys.iter().map(|m| m.factor).product::<usize>();
    let len = monkeys.len();
    let mut map = monkeys
        .into_iter()
        .map(|m| (m.id, m))
        .collect::<HashMap<usize, Monkey>>();
    (0..len).for_each(|i| {
        let m = map.get_mut(&i).unwrap();
        let changes = m.turn(lcm, part);
        changes.into_iter().for_each(|(m, j)| {
            map.get_mut(&m).unwrap().items.push(j);
        });
    });
    map.into_values().into_iter().collect::<Vec<_>>()
}
