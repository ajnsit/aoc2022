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
                let sa = a.chars().collect::<HashSet<_>>();
                let sb = b.chars().collect::<HashSet<_>>();
                sa.intersection(&sb)
                    .map(|x| priority(*x))
                    // Assume there's only one element so `sum` does the right thing
                    .sum::<usize>()
            })
            .sum::<usize>()
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
                  panic!("Invalid number of elements in the chunk");
                };
                let h1 = s1.chars().collect::<HashSet<_>>();
                let h2 = s2.chars().collect::<HashSet<_>>();
                let h3 = s3.chars().collect::<HashSet<_>>();
                h1.intersection(&h2)
                    .copied()
                    .collect::<HashSet<_>>()
                    .intersection(&h3)
                    .map(|x| priority(*x))
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
}

fn input() -> Vec<&'static str> {
    include_str!("../input.txt").lines().collect::<Vec<_>>()
}

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => 1 + (c as u8 - b'a') as usize,
        'A'..='Z' => 27 + (c as u8 - b'A') as usize,
        _ => panic!("Invalid char!"),
    }
}
