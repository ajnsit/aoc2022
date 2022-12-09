use std::{cmp::max, collections::HashSet, fmt::format};

type Forest = Vec<Vec<usize>>;

fn transpose(inp: &mut Forest) {
    let mut res: Forest = Vec::new();
    inp.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(i, c)| match res.get_mut(i) {
                Some(v) => v.push(*c),
                None => {
                    res.push(vec![*c]);
                }
            });
    });
    *inp = res;
}

type Coord = (usize, usize);

fn visible_from_left(inp: &mut Forest) -> HashSet<Coord> {
    let len = inp.first().unwrap().len();
    // let mut res = HashSet::new();
    let res = inp
        .iter()
        .enumerate()
        .map(|(i, x)| {
            x.iter()
                .enumerate()
                .fold((HashSet::new(), 0), |(mut h, prevmax), (j, y)| {
                    if *y > prevmax {
                        h.insert((i, j));
                        (h, *y)
                    } else {
                        (h, prevmax)
                    }
                })
                .0
        })
        .fold(HashSet::new(), |h, h1| {
            h.union(&h1).copied().collect::<HashSet<(usize, usize)>>()
        });
    res
}

fn visible_from_right(inp: &mut Forest) -> HashSet<Coord> {
    let len = inp.first().unwrap().len();
    flip(inp);
    let res = visible_from_left(inp);
    flip(inp);
    res.into_iter()
        .map(|c| coord_from_other_side(c, len))
        .collect()
}

fn visible_from_top(inp: &mut Forest) -> HashSet<Coord> {
    transpose(inp);
    let res = visible_from_left(inp);
    transpose(inp);
    res.into_iter().map(|c| coord_from_transposed(c)).collect()
}

fn visible_from_bottom(inp: &mut Forest) -> HashSet<Coord> {
    let len = inp.first().unwrap().len();
    transpose(inp);
    flip(inp);
    let res = visible_from_left(inp);
    flip(inp);
    transpose(inp);
    res.into_iter()
        .map(|c| coord_from_transposed(coord_from_other_side(c, len)))
        .collect()
}

fn visible_from_anywhere(inp: &mut Forest) -> HashSet<Coord> {
    let mut res = HashSet::new();
    res = res.union(&visible_from_left(inp)).copied().collect();
    res = res.union(&visible_from_right(inp)).copied().collect();
    res = res.union(&visible_from_top(inp)).copied().collect();
    res = res.union(&visible_from_bottom(inp)).copied().collect();
    res
}

fn coord_from_other_side((row, col): Coord, len: usize) -> Coord {
    (row, len - col - 1)
}

fn coord_from_transposed((row, col): Coord) -> Coord {
    (col, row)
}

fn flip(inp: &mut Forest) {
    inp.iter_mut().for_each(|r| {
        r.reverse();
    });
}

fn main() {
    part1();
    part2();
}

fn part1() {
    println!("{:?}", input());
    println!("{}", visible_from_anywhere(&mut input()).len());

    // println!("{}\n--", display_in_a_grid(visible_from_left(&mut input())));
    // println!("{}", display_in_a_grid(visible_from_right(&mut input())));
    println!("{}\n--", display_in_a_grid(visible_from_top(&mut input())));
    // println!(
    //     "{}\n--",
    //     display_in_a_grid(visible_from_bottom(&mut input()))
    // );
    // println!("{}", display_in_a_grid(visible_from_anywhere(&mut input())));
}

fn scenic_scores_from_left(inp: &Forest) -> Vec<Vec<usize>> {
    let mut inp1 = inp.clone();
    inp1.iter_mut()
        .map(|x| {
            let mut v = Vec::new();
            while let Some(l) = x.pop() {
                let mut extra = 0;
                let score = if x.len() == 0 {
                    0
                } else {
                    x.iter()
                        .rev()
                        .take_while(|y| {
                            if **y < l {
                                true
                            } else {
                                extra = 1;
                                false
                            }
                        })
                        .count()
                        + extra
                };
                v.push(score);
            }
            v.reverse();
            v
        })
        .collect()
}

fn scenic_scores_from_right(inp: &Forest) -> Vec<Vec<usize>> {
    let mut inp1 = inp.clone();
    flip(&mut inp1);
    let mut res = scenic_scores_from_left(&inp1);
    flip(&mut res);
    res
}

fn scenic_scores_from_top(inp: &Forest) -> Vec<Vec<usize>> {
    let mut inp1 = inp.clone();
    transpose(&mut inp1);
    let mut res = scenic_scores_from_left(&inp1);
    transpose(&mut res);
    res
}

fn scenic_scores_from_bottom(inp: &Forest) -> Vec<Vec<usize>> {
    let mut inp1 = inp.clone();
    transpose(&mut inp1);
    flip(&mut inp1);
    let mut res = scenic_scores_from_left(&inp1);
    flip(&mut res);
    transpose(&mut res);
    res
}

fn scenic_scores_from_everywhere(inp: &Forest) -> Vec<Vec<usize>> {
    // Manually do a 4 way zip_with, sheesh!
    scenic_scores_from_left(inp)
        .iter()
        .zip(
            scenic_scores_from_right(inp)
                .iter()
                .zip(
                    scenic_scores_from_top(inp)
                        .iter()
                        .zip(scenic_scores_from_bottom(inp))
                        .map(|(x, y)| x.iter().zip(y).map(|(u, v)| u * v).collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                )
                .map(|(x, y)| x.iter().zip(y).map(|(u, v)| u * v).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
        .map(|(x, y)| x.iter().zip(y).map(|(u, v)| u * v).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part2() {
    println!(
        "{:?}",
        scenic_scores_from_everywhere(&input())
            .iter()
            .fold(0, |x, y| max(x, y.iter().fold(0, |u, v| max(u, *v))))
    );
}

fn display_in_a_grid(inp: HashSet<Coord>) -> String {
    // let (rowsize, colsize) = inp.iter().fold((0, 0), |(n, m), (x, y)| {
    //     (if *x > n { *x } else { n }, if *y > m { *y } else { m })
    // });
    let (rowsize, colsize) = (5, 5);
    (0..rowsize).fold("".to_owned(), |mut s1, i| {
        let srow = (0..colsize).fold("".to_owned(), |mut s2, j| {
            let scell = if inp.contains(&(i, j)) { "X" } else { "." };
            s2.push_str(scell);
            s2
        });
        s1.push_str("\n");
        s1.push_str(&srow);
        s1
    })
}

fn input() -> Forest {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<_>>()
}
