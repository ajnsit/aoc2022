fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        input()
            .lines()
            .filter(|s| {
                let (r1, r2) = parse(s);
                contains(r1, r2) || contains(r2, r1)
            })
            .count()
    );
}

fn part2() {
    println!(
        "{:?}",
        input()
            .lines()
            .filter(|s| {
                let (r1, r2) = parse(s);
                overlaps(r1, r2) || overlaps(r2, r1)
            })
            .count()
    );
}

type Range = (usize, usize);

fn parse(s: &str) -> (Range, Range) {
    let (s1, s2) = s.split_once(",").unwrap();
    let (sx1, sy1) = s1.split_once("-").unwrap();
    let (sx2, sy2) = s2.split_once("-").unwrap();
    (
        (sx1.parse().unwrap(), sy1.parse().unwrap()),
        (sx2.parse().unwrap(), sy2.parse().unwrap()),
    )
}

fn contains((x1, y1): Range, (x2, y2): Range) -> bool {
    (x1 <= x2) && (y1 >= y2)
}

fn overlaps((x1, y1): Range, (x2, y2): Range) -> bool {
    (x1 >= x2 && x1 <= y2) || (y1 >= x2 && y1 <= y2)
}

fn input() -> &'static str {
    include_str!("../input.txt")
}
