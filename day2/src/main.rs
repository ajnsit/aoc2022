fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{:?}",
        input()
            .into_iter()
            .map(|(a, b)| (char_to_rps(a), char_to_rps(b)))
            .map(|(a, b)| score(a, b))
            .sum::<i32>()
    );
}

fn part2() {
    println!(
        "{:?}",
        input()
            .into_iter()
            .map(|(a, b)| (
                char_to_rps(a),
                choose_next(char_to_rps(a), char_to_outcome(b))
            ))
            .map(|(a, b)| score(a, b))
            .sum::<i32>()
    );
}

fn input() -> Vec<(&'static str, &'static str)> {
    include_str!("../input.txt")
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .collect::<Vec<(&str, &str)>>()
}

fn char_to_rps(s: &str) -> RPS {
    match s {
        "A" => RPS::R,
        "B" => RPS::P,
        "C" => RPS::S,
        "X" => RPS::R,
        "Y" => RPS::P,
        "Z" => RPS::S,
        _ => RPS::R,
    }
}

fn char_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::L,
        "Y" => Outcome::D,
        "Z" => Outcome::W,
        _ => Outcome::D,
    }
}

fn score(opponent: RPS, me: RPS) -> i32 {
    match (opponent, me) {
        (RPS::R, RPS::P) => 6 + 2,
        (RPS::P, RPS::S) => 6 + 3,
        (RPS::S, RPS::R) => 6 + 1,
        (RPS::P, RPS::R) => 0 + 1,
        (RPS::R, RPS::S) => 0 + 3,
        (RPS::S, RPS::P) => 0 + 2,
        (RPS::R, RPS::R) => 3 + 1,
        (RPS::P, RPS::P) => 3 + 2,
        (RPS::S, RPS::S) => 3 + 3,
    }
}

fn choose_next(opponent: RPS, outcome: Outcome) -> RPS {
    match (opponent, outcome) {
        (x, Outcome::D) => x,
        (RPS::R, Outcome::W) => RPS::P,
        (RPS::R, Outcome::L) => RPS::S,
        (RPS::P, Outcome::W) => RPS::S,
        (RPS::P, Outcome::L) => RPS::R,
        (RPS::S, Outcome::W) => RPS::R,
        (RPS::S, Outcome::L) => RPS::P,
    }
}

#[derive(Debug)]
enum RPS {
    R,
    P,
    S,
}

#[derive(Debug)]
enum Outcome {
    W,
    L,
    D,
}
