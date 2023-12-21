use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(data: &[Dig]) -> usize {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut x: isize = 0;
    let mut y: isize = 0;
    for d in data {
        points.push((x, y));
        match d.dir {
            Dir::Up => y += d.amm as isize,
            Dir::Down => y -= d.amm as isize,
            Dir::Left => x -= d.amm as isize,
            Dir::Right => x += d.amm as isize,
        }
    }
    //println!(" Points: {points:?}");
    trapezoid_formula(&points) + calc_edge(&points)
}

fn p2(data: &[Dig]) -> usize {
    let data: Vec<Dig> = data.iter().map(|d| d.transfonm()).collect();
    p1(&data)
}

fn trapezoid_formula(points: &[(isize, isize)]) -> usize {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let p = points[i];
        sum += (p.1 + points[i + 1].1) * (p.0 - points[i + 1].0);
    }
    sum.unsigned_abs() / 2
}

fn distance(p1: (isize, isize), p2: (isize, isize)) -> usize {
    (p1.0 - p2.0).unsigned_abs() + (p1.1 - p2.1).unsigned_abs()
}

fn calc_edge(points: &[(isize, isize)]) -> usize {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        sum += distance(points[i], points[i + 1]);
    }
    sum += distance(points[0], *points.last().unwrap());
    sum / 2 + 1
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for Dir {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("Wrong value: {value}"),
        }
    }
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown conversion: {value}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Dig {
    dir: Dir,
    amm: usize,
    clr: String,
}

impl Dig {
    pub fn transfonm(&self) -> Self {
        let n = usize::from_str_radix(&self.clr, 16).unwrap();
        let dir = (n & 0b1111).into(); // mod 16
        let amm = n >> 4; // div 16
        Self {
            dir,
            amm,
            clr: self.clr.clone(),
        }
    }
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, r) = s.split_once(' ').unwrap();
        let dir: Dir = d.into();
        let (a, c) = r.split_once(' ').unwrap();
        let amm = a.parse().map_err(|_| ())?;
        let clr = c.replace(['(', ')', '#'], "");
        Ok(Self { dir, amm, clr })
    }
}

fn main() {
    let input: Vec<Dig> = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input: Vec<Dig> = Input::inline(
        "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    )
    .read_data()
    .unwrap();
    assert_eq!(input.len(), 14);
    assert_eq!(
        input[0],
        Dig {
            dir: Dir::Right,
            amm: 6,
            clr: "70c710".into()
        }
    );
    assert_eq!(p1(&input), 62);
}

#[test]
fn test_p2() {
    //todo!();
}

#[test]
fn final_test() {
    let input: Vec<Dig> = Input::file("./data/day18.txt")
        .unwrap()
        .read_data()
        .unwrap();
    assert_eq!(p1(&input), 40714);
    assert_eq!(p2(&input), 129849166997110);
}
