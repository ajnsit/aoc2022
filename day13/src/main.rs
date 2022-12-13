use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    error::VerboseError,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        input()
            .iter()
            .enumerate()
            .filter_map(|(i, pair)| {
                if compare_lists(pair.get(0).unwrap(), pair.get(1).unwrap()) == Ordering::Less {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum::<usize>()
    );
}

fn part2() {
    let mut res = input().into_iter().concat();
    let extra_item1 = vec![Elem::List(vec![Elem::Num(2)])];
    res.push(extra_item1.clone());
    let extra_item2 = vec![Elem::List(vec![Elem::Num(6)])];
    res.push(extra_item2.clone());
    res.sort_by(|l1, l2| compare_lists(l1, l2));
    let (i1, _) = res
        .iter()
        .enumerate()
        .find(|(_, x)| compare_lists(x, &extra_item1) == Ordering::Equal)
        .unwrap();
    let (i2, _) = res
        .iter()
        .enumerate()
        .find_position(|(_, x)| compare_lists(x, &extra_item2) == Ordering::Equal)
        .unwrap();
    println!("{}", (i1 + 1) * (i2 + 1));
}

type List = Vec<Elem>;

#[derive(Debug, Clone)]
enum Elem {
    Num(usize),
    List(List),
}

fn compare_lists(l1: &List, l2: &List) -> Ordering {
    l1.iter()
        .zip_longest(l2)
        .fold(Ordering::Equal, |c, x| match c {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match x {
                itertools::EitherOrBoth::Both(x, y) => match (x, y) {
                    (Elem::Num(n1), Elem::Num(n2)) => {
                        if n1 < n2 {
                            Ordering::Less
                        } else if n1 > n2 {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                    (Elem::Num(n), Elem::List(l)) => compare_lists(&vec![Elem::Num(*n)], l),
                    (Elem::List(l), Elem::Num(n)) => compare_lists(l, &vec![Elem::Num(*n)]),
                    (Elem::List(ll1), Elem::List(ll2)) => compare_lists(ll1, ll2),
                },
                itertools::EitherOrBoth::Left(_) => Ordering::Greater,
                itertools::EitherOrBoth::Right(_) => Ordering::Less,
            },
        })
}

fn parse_elem<'a>(i: &'a str) -> IResult<&'a str, Elem, VerboseError<&'a str>> {
    alt((map(parse_num, Elem::Num), map(parse_list, Elem::List)))(i)
}

fn parse_list<'a>(i: &'a str) -> IResult<&'a str, List, VerboseError<&'a str>> {
    delimited(char('['), separated_list0(char(','), parse_elem), char(']'))(i)
}

fn parse_num<'a>(i: &'a str) -> IResult<&'a str, usize, VerboseError<&'a str>> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<usize>())(i)
}

fn input() -> Vec<Vec<List>> {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|y| parse_list(y).map_err(|e| format!("{:?}", e)).unwrap().1)
                .collect()
        })
        .collect()
}
