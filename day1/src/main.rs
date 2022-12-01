use std::cmp::Reverse;

fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        input()
            .split(|x| *x == 0)
            .map(|v| v.iter().sum::<u32>())
            .max(),
    );
}

fn part2() {
    let mut sums = input()
        .split(|x| *x == 0)
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    sums.sort_by_key(|k| Reverse(*k));
    println!("{:?}", sums.iter().take(3).sum::<u32>());
}

fn input() -> Vec<u32> {
    include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap_or(0))
        .collect::<Vec<u32>>()
}
