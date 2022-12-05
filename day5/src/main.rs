use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut blocks, moves) = parsed(input());
    for mov in moves {
        move_block(&mut blocks, mov, false);
    }
    println!("{:?}", blocks.into_iter().map(|x| x[0]).collect::<String>());
}

fn part2() {
    let (mut blocks, moves) = parsed(input());
    for mov in moves {
        move_block(&mut blocks, mov, true);
    }
    println!("{:?}", blocks.into_iter().map(|x| x[0]).collect::<String>());
}

fn input() -> &'static str {
    include_str!("../input.txt")
    // include_str!("../test.txt")
}

type Blocks = Vec<Vec<char>>;
type Move = (usize, usize, usize);

fn parsed(input: &str) -> (Blocks, Vec<Move>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let blocks = parts[0];
    let moves = parts[1];
    let mut parsed_blocks: Blocks = blocks
        .lines()
        .map(|s| -> Vec<char> {
            s.chars()
                .collect::<Vec<char>>()
                .chunks(4) // "[x] "
                .map(|block| block[1])
                .collect()
        })
        .collect();

    parsed_blocks.pop(); // Assume column names are sequential integers, and don't need to be parsed

    parsed_blocks = transpose(parsed_blocks);

    let parsed_moves: Vec<Move> = moves
        .lines()
        .map(|s| {
            let captures = Regex::new(r"move (\d+) from (\d+) to (\d+)")
                .unwrap()
                .captures(s)
                .unwrap();
            (
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap(),
                captures[3].parse::<usize>().unwrap(),
            )
        })
        .collect();

    (parsed_blocks, parsed_moves)
}

fn transpose(inp: Blocks) -> Blocks {
    let mut res: Blocks = Vec::new();
    inp.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(i, c)| match res.get_mut(i) {
                Some(v) => v.push(c),
                None => {
                    res.push(vec![c]);
                }
            });
    });
    // Take the opportunity to clean up blanks
    res.into_iter()
        .map(|x| x.into_iter().filter(|y| *y != ' ').collect())
        .collect()
}

fn move_block(blocks: &mut Blocks, (n, from, to): Move, machine_9001: bool) {
    let mut moving = blocks[from - 1].drain(..n).collect::<Vec<_>>();
    if machine_9001 {
        moving.reverse();
    }
    blocks[to - 1] = moving.into_iter().chain(blocks[to - 1].drain(..)).collect();
}
