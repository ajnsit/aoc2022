use regex::Regex;

fn main() {
    part1();
    part2();
}

fn part1() {
    // let target = 10;
    let target = 2000000;
    println!(
        "{:?}",
        Ranges::mk(
            input()
                .iter()
                .filter_map(|s| s.range_covered_y(target))
                .collect()
        )
        .count()
    );
}

fn part2() {
    // let grid_size = 20;
    let grid_size = 4000000;
    let square = Range {
        min: 0,
        max: grid_size,
    };
    let sensors = input();
    for y in 0..=grid_size {
        let cov = Ranges::mk(
            sensors
                .iter()
                .filter_map(|s| s.range_covered_y(y).and_then(|r| r.clamp(&square)))
                .collect(),
        );
        let n = cov.count();
        // println!("CHECK: {}: {}", y, n);
        if n < grid_size as usize {
            println!("Found Y: {}", y);
            for x in 0..=grid_size {
                if !cov.contains(x) {
                    println!("Distress beacon at! {}:{}", x, y);
                    println!("Tuning freq: {}", x * 4000000 + y);
                }
            }
            break;
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn manhattan(&self, other: &Coord) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug)]
struct Range {
    min: isize,
    max: isize,
}

impl Range {
    fn count(&self) -> usize {
        self.max.abs_diff(self.min)
    }
    // Merge if the ranges overlap
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.max < other.min || self.min > other.max {
            None
        } else {
            Some(Range {
                min: self.min.min(other.min),
                max: self.max.max(other.max),
            })
        }
    }

    fn clamp(&self, range: &Self) -> Option<Self> {
        if self.min <= range.min && self.max >= range.max {
            None
        } else {
            Some(Range {
                min: self.min.max(range.min),
                max: self.max.min(range.max),
            })
        }
    }

    fn contains(&self, target: isize) -> bool {
        self.min <= target && self.max >= target
    }
}

#[derive(Debug)]
struct Ranges {
    // Non overlapping ranges
    ranges: Vec<Range>,
}

impl Ranges {
    fn merge(mut self, range: Range) -> Self {
        let mut ranges = vec![];
        let rnew = self.ranges.drain(..).fold(range, |candidate, existing| {
            match existing.merge(&candidate) {
                Some(merged) => merged,
                None => {
                    ranges.push(existing);
                    candidate
                }
            }
        });
        ranges.push(rnew);
        Ranges { ranges }
    }

    // Make a new Ranges structure, merging any overlapping ranges
    fn mk(rs: Vec<Range>) -> Self {
        rs.into_iter()
            .fold(Ranges { ranges: vec![] }, |ranges, r| ranges.merge(r))
    }

    fn clamp(&mut self, range: &Range) {
        let res = self
            .ranges
            .drain(..)
            .filter_map(|r| r.clamp(range))
            .collect();
        self.ranges = res;
    }

    fn count(&self) -> usize {
        self.ranges.iter().map(|x| x.count()).sum()
    }

    fn contains(&self, target: isize) -> bool {
        for x in self.ranges.iter() {
            if x.contains(target) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest: Coord,
}

impl Sensor {
    // Range of cells covered by this sensor at the specified y row
    fn range_covered_y(&self, y: isize) -> Option<Range> {
        let range = self.pos.manhattan(&self.closest);
        let perpendicular_distance = self.pos.y.abs_diff(y);
        if range < perpendicular_distance {
            None
        } else {
            let x_width = (range - perpendicular_distance) as isize;
            Some(Range {
                min: self.pos.x - x_width,
                max: self.pos.x + x_width,
            })
        }
    }
}

fn input() -> Vec<Sensor> {
    include_str!("../input.txt")
        // include_str!("../test.txt")
        .lines()
        .map(|s| {
            let captures = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            )
            .unwrap()
            .captures(s)
            .expect(&format!("Unable to parse string: {:?}", s));
            Sensor {
                pos: Coord {
                    x: captures[1].parse::<isize>().unwrap(),
                    y: captures[2].parse::<isize>().unwrap(),
                },
                closest: Coord {
                    x: captures[3].parse::<isize>().unwrap(),
                    y: captures[4].parse::<isize>().unwrap(),
                },
            }
        })
        .collect()
}
