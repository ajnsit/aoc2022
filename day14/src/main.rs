use std::collections::HashSet;

use itertools::Either;

fn main() {
    part1();
    part2();
}

fn part1() {
    let walls = input();
    let res = settle_and_count_grains(input(), false);
    let sands = res.0;
    let count = res.1;
    println!("{}", display_in_a_grid(walls, sands));
    println!("Settled after {} grains had fallen", count);
}

fn part2() {
    let walls = input();
    let res = settle_and_count_grains(input(), true);
    let sands = res.0;
    let count = res.1;
    println!("{}", display_in_a_grid(walls, sands));
    println!("Settled after {} grains had fallen", count + 1);
}

type Coord = (usize, usize);
type Grid = HashSet<Coord>;

const SPOUT: usize = 500;

// For some reason there is no iterate which consumes the state, or uses a mutable ref to state
// Also need this to use a loop instead of iteration to avoid stack overflow
fn iterate_maybe<A, R, F>(mut st: A, step: F) -> R
where
    F: Fn(A) -> Either<A, R>,
{
    loop {
        match step(st) {
            Either::Left(st1) => st = st1,
            Either::Right(r) => break r,
        }
    }
}

fn settle_and_count_grains(ginit: Grid, has_floor: bool) -> (Grid, usize) {
    // iterate1(Some(ginit), |eg| eg.and_then(|g| drop_grain(g))).count()
    let b = bounds(&ginit);
    iterate_maybe((ginit, 0), |(g, i)| match drop_grain(g, &b, has_floor) {
        Either::Left(g1) => Either::Left((g1, i + 1)),
        Either::Right(g1) => Either::Right((g1, i)),
    })
}

// Need to use Either, instead of Option, because we always return the Grid back
fn drop_grain(mut g: Grid, b: &Bounds, has_floor: bool) -> Either<Grid, Grid> {
    let f = if has_floor { settle_2 } else { settle };
    match f((SPOUT, 0), &g, b) {
        Some(p) => {
            g.insert(p);
            Either::Left(g)
        }
        None => Either::Right(g),
    }
}

fn settle((x, y): Coord, g: &Grid, b: &Bounds) -> Option<Coord> {
    iterate_maybe((x, y), |(x, y)| {
        if !within_bounds((x, y), b) {
            println!("ABYSS!");
            Either::Right(None)
        } else {
            grain_move(x, y, g)
        }
    })
}
fn settle_2((x, y): Coord, g: &Grid, b: &Bounds) -> Option<Coord> {
    iterate_maybe((x, y), |(x, y)| {
        if y >= b.1 .1 + 1 {
            Either::Right(Some((x, y)))
        } else {
            let l = grain_move(x, y, g);
            match l {
                Either::Right(Some(_)) => {
                    if x == SPOUT && y == 0 {
                        println!("STOP!");
                        Either::Right(None)
                    } else {
                        l
                    }
                }
                l => l,
            }
        }
    })
}

fn grain_move(x: usize, y: usize, g: &Grid) -> Either<Coord, Option<Coord>> {
    if !g.contains(&(x, y + 1)) {
        Either::Left((x, y + 1))
    } else if !g.contains(&(x - 1, y + 1)) {
        Either::Left((x - 1, y + 1))
    } else if !g.contains(&(x + 1, y + 1)) {
        Either::Left((x + 1, y + 1))
    } else {
        Either::Right(Some((x, y)))
    }
}

fn input() -> Grid {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        .lines()
        .fold(HashSet::new(), |mut h, x| {
            x.split(" -> ")
                .map(|y| {
                    let (u, v) = y.split_once(",").unwrap();
                    (u.parse::<usize>().unwrap(), v.parse::<usize>().unwrap())
                })
                .fold(None, |mprev, next| {
                    if let Some(prev) = mprev {
                        line(prev, next).iter().for_each(|p| {
                            h.insert(*p);
                        });
                    }
                    Some(next)
                });
            h
        })
}

type Bounds = (Coord, Coord);

fn line(prev: Coord, next: Coord) -> Vec<Coord> {
    let diffx = prev.0.abs_diff(next.0);
    let diffy = prev.1.abs_diff(next.1);
    // Walk along the longer axis. Assumes lines are vertical or horizontal.
    if diffx < diffy {
        range(prev.1, next.1).map(|y| (prev.0, y)).collect()
    } else {
        range(prev.0, next.0).map(|x| (x, prev.1)).collect()
    }
}

fn range(x: usize, y: usize) -> impl Iterator<Item = usize> {
    if x < y {
        x..=y
    } else {
        y..=x
    }
}

fn bounds(g: &Grid) -> Bounds {
    let init = ((usize::MAX, usize::MAX), (usize::MIN, usize::MIN));
    g.iter().fold(init, |(prev_min, prev_max), (x, y)| {
        (
            (prev_min.0.min(*x), prev_min.1.min(*y)),
            (prev_max.0.max(*x), prev_max.1.max(*y)),
        )
    })
}

fn within_bounds((x, y): Coord, ((xmin, _ymin), (xmax, ymax)): &Bounds) -> bool {
    // Sand going above (y <= ymin) the highest wall is OK
    x >= *xmin && x <= *xmax /* && y >= *ymin */ && y <= *ymax
}

fn display_in_a_grid(walls: HashSet<Coord>, sands: HashSet<Coord>) -> String {
    // Sands include walls, so no need to iterate over walls
    let (lx, ly, hx, hy) = sands.iter().fold(
        (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
        |(lx, ly, hx, hy), (x, y)| {
            (
                if *x < lx { *x } else { lx },
                if *y < ly { *y } else { ly },
                if *x > hx { *x } else { hx },
                if *y > hy { *y } else { hy },
            )
        },
    );
    (ly..=hy).fold("".to_owned(), |mut s1, y| {
        let srow = (lx..=hx).fold("".to_owned(), |mut s2, x| {
            // Sands includes walls, so need to check walls first
            s2.push(if walls.contains(&(x, y)) {
                '#'
            } else if sands.contains(&(x, y)) {
                'o'
            } else {
                '.'
            });
            s2
        });
        s1.push_str("\n");
        s1.push_str(&srow);
        s1
    })
}
