// Learnt a new thing, array window functions! And partition_dedup!
#![feature(array_windows, slice_partition_dedup)]

fn main() {
    part1();
    part2();
}

fn part1() {
    solver::<4>();
}

fn part2() {
    solver::<14>();
}

fn solver<const N: usize>() {
    println!(
        "{:?}",
        input()
            .chars()
            .collect::<Vec<_>>()
            // Learnt a new thing - How do I use something like array_windows<const N: Int>. Const generics!
            .array_windows::<N>()
            .enumerate()
            .filter(|(_, s)| {
                let mut x = **s;
                x.sort();
                let (_, dups) = x.partition_dedup();
                dups.len() == 0
            })
            .map(|(i, s)| (i + N, s))
            .next()
            .unwrap()
    );
}

fn input() -> &'static str {
    include_str!("../input.txt")
    // include_str!("../test.txt")
}
