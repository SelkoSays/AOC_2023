use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use aoc_2023::{
    input::Input,
    matrix::{Matrix, Point},
};

fn p1(data: &Data, num_steps: usize) -> usize {
    let mut d = data.clone();
    let start = d.0.find_first_p(&Tile::Checked).unwrap();
    if let Some(e) = d.0.pget_mut(&start) {
        *e = Tile::Plot;
    }
    mark_plots2(&start, &mut d, num_steps)
    // (
    //     d.0.iter().filter(|&t| matches!(t, Tile::Checked)).count(),
    //     d,
    // )
}

fn p2() -> u32 {
    0
}

fn mark_plots(start: &Point, d: &mut Data, num_steps: usize) {
    let mut options: VecDeque<(Point, usize)> = VecDeque::from([(start.clone(), 0usize)]);
    while !options.is_empty() {
        if options.front().unwrap().1 >= num_steps {
            break;
        }
        let opt = options.pop_front().unwrap();
        let opts = get_opts(&opt.0, d);
        if opt.1 < num_steps {
            options.extend(opts.into_iter().map(|p| (p, opt.1 + 1)));
        } else {
            break;
        }
    }
    for (p, n) in options {
        if n != num_steps {
            println!("Ooops");
            continue;
        }
        if let Some(e) = d.0.pget_mut(&p) {
            *e = Tile::Checked;
        }
    }
}

fn mark_plots2(start: &Point, d: &mut Data, num_steps: usize) -> usize {
    let mut options: VecDeque<(Point, usize)> = VecDeque::from([(start.clone(), 0usize)]);
    let mut points = HashSet::new();
    points.insert((start.clone(), 0usize));
    while !options.is_empty() {
        if options.front().unwrap().1 >= num_steps {
            break;
        }
        let opt = options.pop_front().unwrap();
        let opts = get_opts(&opt.0, d);
        for p in &opts {
            if let Some(pp) = d.0.pget_mut(p) {
                *pp = Tile::Checked;
            }
            points.insert((p.clone(), opt.1 + 1));
        }
        options.extend(opts.into_iter().map(|p| (p, opt.1 + 1)));
    }
    for (p, n) in &points {
        if let Some(t) = d.0.pget_mut(p) {
            *t = match t {
                Tile::Number(nn) if n < nn => Tile::Number(*n),
                Tile::Checked | Tile::Plot => Tile::Number(*n),
                _ => continue,
            }
        }
    }
    //println!("hs len: {}", points.len());
    //println!("{}", d.0);
    let l = points
        .into_iter()
        .filter(|(_, d)| (d & 1) == (num_steps & 1))
        .map(|(p, _)| p)
        .collect::<HashSet<Point>>()
        .len();
    //println!("l: {l}");
    l
}

fn mark_plots3(plot: &Point, cs: usize, d: &Data, num_steps: usize, points: &mut HashSet<Point>) {
    if cs >= num_steps {
        //println!("found");
        points.insert(plot.clone());
        return;
    }
    let opts = get_opts(plot, d);
    for p in &opts {
        mark_plots3(p, cs + 1, d, num_steps, points);
    }
}

fn get_opts(point: &Point, d: &Data) -> Vec<Point> {
    let mut v = Vec::new();
    // Look up
    if point.y > 0 {
        let p = Point::new(point.x, point.y - 1);
        if let Some(e) = d.0.pget(&p) {
            if matches!(e, Tile::Plot) {
                v.push(p);
            }
        }
    }
    // Look right
    let p = Point::new(point.x + 1, point.y);
    if let Some(e) = d.0.pget(&p) {
        if matches!(e, Tile::Plot) {
            v.push(p);
        }
    }
    // Look down
    let p = Point::new(point.x, point.y + 1);
    if let Some(e) = d.0.pget(&p) {
        if matches!(e, Tile::Plot) {
            v.push(p);
        }
    }
    if point.x > 0 {
        // Look left
        let p = Point::new(point.x - 1, point.y);
        if let Some(e) = d.0.pget(&p) {
            if matches!(e, Tile::Plot) {
                v.push(p);
            }
        }
    }
    v
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
    Checked,
    Number(usize),
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Plot => ".",
            Tile::Rock => "#",
            Tile::Checked => "O",
            Self::Number(n) => return write!(f, "{n}"),
        };
        write!(f, "{s}")
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' | 'O' => Self::Checked,
            _ => panic!("Unknown option: {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Data(Matrix<Tile>);

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Matrix(
            s.lines()
                .map(|l| l.trim().chars().map(Tile::from).collect())
                .collect(),
        )))
    }
}

fn main() {
    let input: Data = Input::default().read().unwrap();
    println!("P1: {}", p1(&input, 64));
    println!("P2: {}", p2());
}

#[test]
fn test_p1() {
    let input: Data = Input::inline(
        "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    )
    .read()
    .unwrap();
    //assert_eq!(p1(&input, 1), 2);
    assert_eq!(p1(&input, 2), 4);
    //assert_eq!(p1(&input, 3), 6);
    assert_eq!(p1(&input, 6), 16);
}

#[test]
fn test_print_neki() {
    let mut d: Data = Input::inline(
        "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    )
    .read()
    .unwrap();
    let start = d.0.find_first_p(&Tile::Checked).unwrap();
    if let Some(e) = d.0.pget_mut(&start) {
        *e = Tile::Plot;
    }
    mark_plots2(&start, &mut d, 6);
}

#[test]
fn test_p2() {
    todo!();
}

#[test]
fn final_test() {
    let input = Input::file("./data/day21.txt").unwrap().lines();
}
