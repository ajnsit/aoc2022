use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        input()
            .into_iter()
            .map(|s| {
                let (a, b) = s.split_at(s.len() / 2);
                let sa = a.chars().collect::<HashSet<char>>();
                let sb = b.chars().collect::<HashSet<char>>();
                sa.intersection(&sb)
                    .map(|x| priority(*x))
                    // Assume there's only one element so `sum` does the right thing
                    .sum::<u32>()
            })
            .sum::<u32>()
    );
}

fn part2() {
    println!(
        "{:?}",
        input()
            .chunks(3)
            .into_iter()
            .map(|s| {
                let [s1, s2, s3] = s else {
                panic!("");
            };
                let h1 = s1.chars().collect::<HashSet<char>>();
                let h2 = s2.chars().collect::<HashSet<char>>();
                let h3 = s3.chars().collect::<HashSet<char>>();
                h1.intersection(&h2)
                    .map(|x| *x)
                    .collect::<HashSet<char>>()
                    .intersection(&h3)
                    .map(|x| priority(*x))
                    .sum::<u32>()
            })
            .sum::<u32>()
    );
}

fn input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect::<Vec<&str>>()
}

fn priority(c: char) -> u32 {
    let i = c as u32;
    let zsmall = 'z' as u32;
    let zbig = 'Z' as u32;
    if zsmall < zbig {
        if i <= zsmall {
            1 + i - 'a' as u32
        } else {
            27 + i - 'A' as u32
        }
    } else {
        if i <= zbig {
            27 + i - 'A' as u32
        } else {
            1 + i - 'a' as u32
        }
    }
}
