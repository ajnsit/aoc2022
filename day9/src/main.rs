use std::collections::HashSet;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut s = HashSet::new();
    println!(
        "{:?}",
        input().into_iter().fold(((0, 0), (0, 0)), |(h, t), m| {
            follow_track(h, t, &mut s, m)
        }),
    );
    println!("{}", s.len());
    println!("{}", display_in_a_grid(s));
}

fn part2() {
    let mut s = HashSet::new();
    // Can't find a "replicate" for vectors, so doing this weird thing
    let chain = (vec![(0, 0)].into_iter().cycle())
        .take(10)
        .collect::<Vec<Coord>>();
    println!(
        "{:?}",
        input()
            .into_iter()
            .fold(chain, |chain, m| { follow_chain_track(chain, &mut s, m) }),
    );
    println!("{}", s.len());
    println!("{}", display_in_a_grid(s));
}

#[derive(Debug)]
enum Move {
    Vert(isize),
    Horz(isize),
}

type Coord = (isize, isize);

// Relative position of H compared to T
// T is considered to be at 0,0, H can be at most 2 spaces away,
// i.e. in the grid with the diagonal -2,-2 -> 2,2
fn move_once_relative((x, y): Coord) -> Coord {
    if x.abs() <= 1 && y.abs() <= 1 {
        (0, 0)
    } else if x == 0 || y == 0 || x == y {
        (x / 2 as isize, y / 2 as isize)
    } else if x.abs() == 1 {
        (x, y / 2 as isize)
    } else {
        (x / 2 as isize, y)
    }
}

fn move_once(h: Coord, t: Coord) -> Coord {
    sum(t, move_once_relative(diff(h, t)))
}

fn sum(x: Coord, y: Coord) -> Coord {
    (x.0 + y.0, x.1 + y.1)
}

fn diff(x: Coord, y: Coord) -> Coord {
    (x.0 - y.0, x.1 - y.1)
}

type Chain = Vec<Coord>;

fn follow_chain(chain: Chain, m: Move) -> Chain {
    let (diff, steps) = match m {
        Move::Vert(d) => ((0, d.signum()), d.abs()),
        Move::Horz(d) => ((d.signum(), 0), d.abs()),
    };
    (0..steps).fold(chain, |mut chain, _| {
        let h = chain.get_mut(0).unwrap();
        *h = sum(*h, diff);
        chain
            .windows(2)
            .map(|x| match x {
                [h, t] => move_once(*h, *t),
                _ => panic!("Impossibru"),
            })
            .collect::<Chain>()
    })
}

fn follow_chain_track(chain: Chain, s: &mut HashSet<Coord>, m: Move) -> Chain {
    let (diff, steps) = match m {
        Move::Vert(d) => ((0, d.signum()), d.abs()),
        Move::Horz(d) => ((d.signum(), 0), d.abs()),
    };
    let chain = (0..steps).fold(chain, |mut chain, _| {
        let t = chain.last().unwrap();
        s.insert(*t);
        let h = chain.first_mut().unwrap();
        *h = sum(*h, diff);
        let mut res = vec![*h];
        res.append(
            &mut chain
                .windows(2)
                .map(|x| match x {
                    [h, t] => move_once(*h, *t),
                    _ => panic!("Impossibru"),
                })
                .collect::<Chain>(),
        );
        res
    });
    println!("{:?}", chain);
    let t = chain.last().unwrap();
    s.insert(*t);
    chain
}

fn follow(h: Coord, t: Coord, m: Move) -> (Coord, Coord) {
    let (diff, steps) = match m {
        Move::Vert(d) => ((0, d.signum()), d.abs()),
        Move::Horz(d) => ((d.signum(), 0), d.abs()),
    };
    (0..steps).fold((h, t), |(h, t), _| (sum(h, diff), move_once(h, t)))
}

fn follow_track(h: Coord, t: Coord, s: &mut HashSet<Coord>, m: Move) -> (Coord, Coord) {
    let (diff, steps) = match m {
        Move::Vert(d) => ((0, d.signum()), d.abs()),
        Move::Horz(d) => ((d.signum(), 0), d.abs()),
    };
    let (h, t) = (0..steps).fold((h, t), |(h, t), _| {
        s.insert(t);
        (sum(h, diff), move_once(h, t))
    });
    s.insert(t);
    (h, t)
}

fn input() -> Vec<Move> {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        // include_str!("../test2.txt")
        .lines()
        .map(|s| {
            let (c, n) = s.split_once(" ").unwrap();
            let d = n.parse().unwrap();
            match c {
                "U" => Move::Vert(d),
                "D" => Move::Vert(0 - d),
                "R" => Move::Horz(d),
                "L" => Move::Horz(0 - d),
                _ => panic!("Invalid movement"),
            }
        })
        .collect()
}

fn display_in_a_grid(inp: HashSet<Coord>) -> String {
    let (lx, ly, hx, hy) = inp.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(lx, ly, hx, hy), (x, y)| {
            (
                if *x < lx { *x } else { lx },
                if *y < ly { *y } else { ly },
                if *x > hx { *x } else { hx },
                if *y > hy { *y } else { hy },
            )
        },
    );
    // let (rowsize, colsize) = (60, 60);
    (lx - 3..hx + 3).fold("".to_owned(), |mut s1, i| {
        let srow = (ly - 3..hy + 3).fold("".to_owned(), |mut s2, j| {
            s2.push(if inp.contains(&(i, j)) { '#' } else { '.' });
            s2
        });
        s1.push_str("\n");
        s1.push_str(&srow);
        s1
    })
}
