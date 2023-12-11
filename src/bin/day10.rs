#![allow(dead_code)]

use aoc_2023::input::Input;

fn p1(m: &Matrix<Tile>) -> usize {
    let (mut x, mut y) = m.find(&Tile::Start).unwrap();
    let mut t = m.get(x, y).unwrap();
    assert_eq!(t, &Tile::Start);
    let mut prev = 5usize;
    let d = [Dir::North, Dir::West, Dir::South, Dir::East];
    let c: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut len = 0usize;
    loop {
        for (i, ot) in m.get_adj(x, y).into_iter().enumerate() {
            if i == prev {
                continue;
            }
            if let Some(nt) = ot {
                if t.compatible(nt, d[i]) {
                    t = nt;
                    x = (c[i].0 + x as isize) as usize;
                    y = (c[i].1 + y as isize) as usize;
                    prev = (i + 2) & 3;
                    len += 1;
                    break;
                }
            }
        }
        if t == &Tile::Start {
            break;
        }
    }
    len / 2
}

fn p2(m: &Matrix<Tile>) -> usize {
    let mut m2 = mark_points(m);
    let mut area = 0usize;
    println!("{m2}");
    for v in m2.0.iter_mut() {
        let mut in_area = false;
        for e in v.iter_mut() {
            match e {
                Mark::Empty => *e = if in_area { Mark::Inner } else { Mark::Outer },
                //Mark::Start => todo!(),
                Mark::Pipe(Dir::North, Dir::South) => in_area = !in_area,
                Mark::Pipe(_, Dir::East) => in_area = true,
                Mark::Pipe(Dir::North | Dir::South, Dir::West) => in_area = !in_area,
                Mark::Pipe(_, _) => {}
                _ => unreachable!(),
            }
            // if e == &1 {
            //     in_area = !in_area;
            // } else if e == &2 {
            // } else if in_area {
            //     area += 1;
            //     *e = 3;
            // }
        }
    }
    println!("{m2}");
    area
}

#[derive(Debug, Clone, Copy)]
enum Mark {
    Empty,
    Inner,
    Outer,
    Pipe(Dir, Dir),
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(Dir::North, Dir::South) => write!(f, "│"),
            Self::Pipe(Dir::East, Dir::West) => write!(f, "─"),
            Self::Pipe(Dir::North, Dir::East) => write!(f, "└"),
            Self::Pipe(Dir::North, Dir::West) => write!(f, "┘"),
            Self::Pipe(Dir::South, Dir::West) => write!(f, "┐"),
            Self::Pipe(Dir::South, Dir::East) => write!(f, "┌"),
            //Self::Start => write!(f, "S"),
            Self::Empty => write!(f, "."),
            Self::Inner => write!(f, "I"),
            Self::Outer => write!(f, "O"),
            _ => panic!("Wrong pipe {self:?}"),
        }
    }
}

fn mark_points(m: &Matrix<Tile>) -> Matrix<Mark> {
    let (mut x, mut y) = m.find(&Tile::Start).unwrap();
    let mut t = m.get(x, y).unwrap();
    assert_eq!(t, &Tile::Start);
    let mut prev = 5usize;
    let d = [Dir::North, Dir::West, Dir::South, Dir::East];
    let c: [(isize, isize); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];
    let mut v = vec![vec![Mark::Empty; m.0[0].len()]; m.0.len()];
    loop {
        for (i, ot) in m.get_adj(x, y).into_iter().enumerate() {
            if i == prev {
                continue;
            }
            if let Some(nt) = ot {
                if t.compatible(nt, d[i]) {
                    match t {
                        Tile::Start => {
                            let a: Vec<bool> = m
                                .get_adj(x, y)
                                .iter()
                                .enumerate()
                                .map(|(k, c)| c.is_some() && t.compatible(c.unwrap(), d[k]))
                                .collect();
                            v[y][x] = start_pipe(&a);
                        }
                        Tile::Pipe(a, b) => v[y][x] = Mark::Pipe(*a, *b),
                        _ => {}
                    }
                    t = nt;
                    x = (c[i].0 + x as isize) as usize;
                    y = (c[i].1 + y as isize) as usize;
                    prev = (i + 2) & 3;
                    break;
                }
            }
        }
        if t == &Tile::Start {
            break;
        }
    }
    Matrix(v)
}

fn start_pipe(a: &[bool]) -> Mark {
    let mut b = [Dir::North, Dir::South];
    let mut k = 0;
    if a[0] {
        b[k] = Dir::North;
        k += 1;
    }
    if a[2] {
        b[k] = Dir::South;
        k += 1;
    }
    if a[3] {
        b[k] = Dir::East;
        k += 1;
    }
    if a[1] {
        b[k] = Dir::West;
        k += 1;
    }
    Mark::Pipe(b[0], b[1])
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Ground,
    Pipe(Dir, Dir),
}

impl Tile {
    pub fn compatible(&self, o: &Self, to: Dir) -> bool {
        if matches!(o, Self::Ground) {
            return false;
        }
        let start = matches!(o, Self::Start);
        let n = matches!(o, Self::Pipe(Dir::North, _)) || start;
        let s = matches!(o, Self::Pipe(Dir::South, _) | Self::Pipe(_, Dir::South)) || start;
        let e = matches!(o, Self::Pipe(_, Dir::East) | Self::Pipe(Dir::East, _)) || start;
        let w = matches!(o, Self::Pipe(_, Dir::West)) || start;
        match self {
            Self::Start => match to {
                Dir::North => s,
                Dir::South => n,
                Dir::West => e,
                Dir::East => w,
            },
            Self::Ground => false,
            Self::Pipe(Dir::North, Dir::South) => match to {
                Dir::North => s,
                Dir::South => n,
                _ => false,
            },
            Self::Pipe(Dir::East, Dir::West) => match to {
                Dir::East => w,
                Dir::West => e,
                _ => false,
            },
            Self::Pipe(Dir::North, Dir::East) => match to {
                Dir::North => s,
                Dir::East => w,
                _ => false,
            },
            Self::Pipe(Dir::North, Dir::West) => match to {
                Dir::North => s,
                Dir::West => e,
                _ => false,
            },
            Self::Pipe(Dir::South, Dir::West) => match to {
                Dir::South => n,
                Dir::West => e,
                _ => false,
            },
            Self::Pipe(Dir::South, Dir::East) => match to {
                Dir::South => n,
                Dir::East => w,
                _ => false,
            },
            _ => panic!("Unknown tile"),
        }
    }
}

impl Tile {
    pub fn from_slice(v: &[String]) -> Matrix<Tile> {
        Matrix(
            v.iter()
                .map(|l| l.chars().map(Self::from).collect())
                .collect(),
        )
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(Dir::North, Dir::South) => write!(f, "│"),
            Self::Pipe(Dir::East, Dir::West) => write!(f, "─"),
            Self::Pipe(Dir::North, Dir::East) => write!(f, "└"),
            Self::Pipe(Dir::North, Dir::West) => write!(f, "┘"),
            Self::Pipe(Dir::South, Dir::West) => write!(f, "┐"),
            Self::Pipe(Dir::South, Dir::East) => write!(f, "┌"),
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
            _ => panic!("Pipes are wrong"),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '.' => Self::Ground,
            'L' => Self::Pipe(Dir::North, Dir::East),
            'J' => Self::Pipe(Dir::North, Dir::West),
            'F' => Self::Pipe(Dir::South, Dir::East),
            '7' => Self::Pipe(Dir::South, Dir::West),
            '|' => Self::Pipe(Dir::North, Dir::South),
            '-' => Self::Pipe(Dir::East, Dir::West),
            _ => panic!("Unknown conversion from {value} to tile"),
        }
    }
}

#[derive(Debug)]
struct Matrix<T>(Vec<Vec<T>>);

impl<T> Matrix<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).map(|v| v.get(x)).unwrap_or(None)
    }

    /// returns optional adjacent elemnts in order: Up, Left, Down, Right
    /// [^, <, v, >]
    pub fn get_adj(&self, x: usize, y: usize) -> [Option<&T>; 4] {
        [
            self.get(x, (y as isize - 1) as usize),
            self.get((x as isize - 1) as usize, y),
            self.get(x, (y as isize + 1) as usize),
            self.get((x as isize + 1) as usize, y),
        ]
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn find(&self, el: &T) -> Option<(usize, usize)> {
        for (y, v) in self.0.iter().enumerate() {
            for (x, e) in v.iter().enumerate() {
                if e == el {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.0 {
            for t in v {
                match write!(f, "{t}") {
                    Ok(_) => {}
                    err => return err,
                }
            }
            match writeln!(f) {
                Ok(_) => {}
                err => return err,
            }
        }
        Ok(())
    }
}

fn main() {
    let input = Tile::from_slice(&Input::default().lines());
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input = Input::inline(
        "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
",
    )
    .lines();
    let m = Tile::from_slice(&input);
    //println!("{m}");
    //println!("{:?}", m.find(&Tile::Start));
    assert_eq!(8, p1(&m));
}

#[test]
fn test_p2() {
    let input = Input::inline(
        "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........        
",
    )
    .lines();
    let m = Tile::from_slice(&input);
    println!("{m}");
    println!("{}", p2(&m));
    let input = Input::inline(
            "\
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...
        ",
        )
        .lines();
        let m = Tile::from_slice(&input);
        println!("{m}");
        println!("{}", p2(&m));
}

#[test]
fn final_test() {
    let input = Input::file("./data/day10.txt").unwrap().lines();
}
