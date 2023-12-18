use std::str::FromStr;

use aoc_2023::input::Input;
use aoc_2023::matrix::Matrix;

fn p1(data: &[Dig]) -> usize {
    let mut points: Vec<(isize, isize)> = Vec::new();
    let mut x: isize = 0;
    let mut y: isize = 0;
    for d in data {
        points.push((x, y));
        match d.dir {
            Dir::Up => y -= d.amm as isize,
            Dir::Down => y += d.amm as isize,
            Dir::Left => x -= d.amm as isize,
            Dir::Right => x += d.amm as isize,
        }
    }
    let (vx, vy): (Vec<isize>, Vec<isize>) = points.iter().cloned().unzip();
    let mx = vx.iter().min().unwrap();
    let my = vy.iter().min().unwrap();
    let lx = vx.iter().max().unwrap() - mx;
    let ly = vy.iter().max().unwrap() - my;
    let mut table = Matrix(vec![vec![Ign::Empty; lx as usize]; ly as usize]);
    let x_off = mx.abs();
    let y_off = my.abs();
    for (i, p) in points.iter().enumerate() {
        let n = (i + 1) % points.len();
        if let Some(e) = table.get_mut((p.0 + x_off) as usize, (p.1 + y_off) as usize) {
            *e = Ign::Pending;
            if points[n].0 != p.0 {
                let y = (p.1 + y_off) as usize;
                for x in p.0.min(points[n].0)..p.0.max(points[n].0) {
                    if let Some(e) = table.get_mut((x + x_off) as usize, y) {
                        *e = Ign::Pending;
                    }
                }
            } else if points[n].1 != p.1 {
                let x = (p.0 + x_off) as usize;
                for y in p.0.min(points[n].1)..p.0.max(points[n].1) {
                    if let Some(e) = table.get_mut(x, (y + y_off) as usize) {
                        *e = Ign::Pending;
                    }
                }
            }
        }
    }
    for y in 0..ly as usize {
        for x in 0..lx as usize {
            Ign::facilitate(&mut table, (x as isize, y as isize));
        }
    }
    for y in 0..ly as usize {
        let mut y1 = y;
        let mut in_area = false;
        for x in 0..y {
            if let Some(e) = table.get_mut(x, y1) {
                match e {
                    Ign::Empty => *e = if in_area { Ign::Inner } else { Ign::Outer },
                    Ign::NotIgnore => in_area = !in_area,
                    _ => {}
                }
            }
            y1 -= 1;
        }
    }
    println!("{table}");
    table
        .iter()
        .filter(|&e| matches!(*e, Ign::Inner | Ign::NotIgnore | Ign::Ignore))
        .count()
}

fn p2() -> u32 {
    0
}

#[derive(Debug, Clone)]
enum Ign {
    Empty,
    Ignore,
    NotIgnore,
    Inner,
    Outer,
    Pending,
}

impl std::fmt::Display for Ign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Ign::Empty => ".",
            Ign::Ignore => "#",
            Ign::NotIgnore => "#",
            Ign::Inner => "I",
            Ign::Outer => "O",
            Ign::Pending => "P",
        };
        write!(f, "{s}")
    }
}

type Point = (isize, isize);
impl Ign {
    // pub fn make(pp: Point, cp: Point, np: Point) -> Self {
    //     if (pp.0 == cp.0 && cp.0 == np.0) || (pp.1 == cp.1 && cp.1 == np.1) {
    //         // pp x == cp x == np x
    //         return Self::NotIgnore;
    //     }
    //     if (pp.0 < cp.0 && cp.1 < np.1) || (cp.0 == np.0 && pp.1 < cp.1) {
    //         // -+  ali |
    //         //  |      +-
    //         return Self::NotIgnore;
    //     }
    //     Self::Ignore
    // }
    pub fn facilitate(table: &mut Matrix<Self>, p: (isize, isize)) {
        let up = table.get(p.0 as usize, (p.1 - 1) as usize).cloned();
        let left = table.get((p.0 - 1) as usize, p.1 as usize).cloned();
        let down = table.get(p.0 as usize, (p.1 + 1) as usize).cloned();
        let right = table.get((p.0 - 1) as usize, p.1 as usize).cloned();
        if let Some(e) = table.get_mut(p.0 as usize, p.1 as usize) {
            if matches!(e, Self::Pending) {
                if (up.is_some() && left.is_some()) || (down.is_some() && right.is_some()) {
                    *e = Self::Ignore;
                } else {
                    *e = Self::NotIgnore;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
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
    amm: u8,
    clr: String,
}

impl FromStr for Dig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, r) = s.split_once(' ').unwrap();
        let dir: Dir = d.into();
        let (a, c) = r.split_once(' ').unwrap();
        let amm = a.parse().map_err(|_| ())?;
        let clr = c.replace(['(', ')'], "");
        Ok(Self { dir, amm, clr })
    }
}

fn main() {
    let input: Vec<Dig> = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2());
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
            clr: "#70c710".into()
        }
    );
    assert_eq!(p1(&input), 62);
}

#[test]
fn test_p2() {
    todo!();
}

#[test]
fn final_test() {
    let input = Input::file("./data/day18.txt").unwrap().lines();
}
