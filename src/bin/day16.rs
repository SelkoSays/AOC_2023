use std::collections::HashSet;
use std::str::FromStr;

use aoc_2023::input::Input;
use aoc_2023::matrix::Matrix;

fn p1(data: &Data, d: Option<&mut Matrix<char>>) -> usize {
    let mut hs: HashSet<Pos> = HashSet::new();
    data.trace(Pos::new(0, 0, Dir::Right), &mut hs);
    if let Some(d) = d {
        for p in hs.iter() {
            if let Some(c) = d.get_mut(p.x, p.y) {
                *c = '#';
            } else {
                print!("m: {}, {}", p.x, p.y);
            }
        }
        println!("\n{d}");
    }
    hs.iter()
        .map(|p| (p.x, p.y))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

fn p2(data: &Data) -> usize {
    let mut v = Vec::new();
    for y in 0..data.0.leny() {
        let mut hs: HashSet<Pos> = HashSet::new();
        data.trace(Pos::new(0, y, Dir::Right), &mut hs);
        v.push(
            hs.iter()
                .map(|p| (p.x, p.y))
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        );
        hs.clear();
        data.trace(Pos::new(data.0.lenx() - 1, y, Dir::Left), &mut hs);
        v.push(
            hs.iter()
                .map(|p| (p.x, p.y))
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        );
    }
    for x in 0..data.0.lenx() {
        let mut hs: HashSet<Pos> = HashSet::new();
        data.trace(Pos::new(x, 0, Dir::Down), &mut hs);
        v.push(
            hs.iter()
                .map(|p| (p.x, p.y))
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        );
        hs.clear();
        data.trace(Pos::new(x, data.0.leny() - 1, Dir::Up), &mut hs);
        v.push(
            hs.iter()
                .map(|p| (p.x, p.y))
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        );
    }
    *v.iter().max().unwrap()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
enum Refl {
    Empty,    // '.'
    VSplit,   // '|'
    HSplit,   // '-'
    LRMirror, // '/'
    RLMirror, // '\'
}

impl From<char> for Refl {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '|' => Self::VSplit,
            '-' => Self::HSplit,
            '/' => Self::LRMirror,
            '\\' => Self::RLMirror,
            _ => panic!("unknown character -> {value}"),
        }
    }
}

impl std::fmt::Display for Refl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Refl::Empty => '.',
            Refl::VSplit => '|',
            Refl::HSplit => '-',
            Refl::LRMirror => '/',
            Refl::RLMirror => '\\',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
    d: Dir,
}

impl Pos {
    pub fn new(x: usize, y: usize, d: Dir) -> Self {
        Self { x, y, d }
    }
}

#[derive(Debug)]
struct Data(Matrix<Refl>);

impl Data {
    pub fn trace(&self, cp: Pos, hs: &mut HashSet<Pos>) {
        if cp.x >= self.0.lenx() || cp.y >= self.0.leny() || hs.contains(&cp) {
            return;
        }
        hs.insert(cp);
        if let Some(el) = self.0.get(cp.x, cp.y) {
            match el {
                Refl::Empty => match cp.d {
                    Dir::Up => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, cp.d), hs)
                        }
                    }
                    Dir::Left => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, cp.d), hs)
                        }
                    }
                    Dir::Down => self.trace(Pos::new(cp.x, cp.y + 1, cp.d), hs),
                    Dir::Right => self.trace(Pos::new(cp.x + 1, cp.y, cp.d), hs),
                },
                Refl::VSplit => match cp.d {
                    Dir::Up => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, cp.d), hs)
                        }
                    }
                    Dir::Left => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, Dir::Up), hs);
                        }
                        self.trace(Pos::new(cp.x, cp.y + 1, Dir::Down), hs);
                    }
                    Dir::Down => self.trace(Pos::new(cp.x, cp.y + 1, cp.d), hs),
                    Dir::Right => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, Dir::Up), hs);
                        }
                        self.trace(Pos::new(cp.x, cp.y + 1, Dir::Down), hs);
                    }
                },
                Refl::HSplit => match cp.d {
                    Dir::Up => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, Dir::Left), hs);
                        }
                        self.trace(Pos::new(cp.x + 1, cp.y, Dir::Right), hs);
                    }
                    Dir::Left => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, cp.d), hs)
                        }
                    }
                    Dir::Down => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, Dir::Left), hs);
                        }
                        self.trace(Pos::new(cp.x + 1, cp.y, Dir::Right), hs);
                    }
                    Dir::Right => self.trace(Pos::new(cp.x + 1, cp.y, cp.d), hs),
                },
                // /
                Refl::LRMirror => match cp.d {
                    Dir::Up => self.trace(Pos::new(cp.x + 1, cp.y, Dir::Right), hs),
                    Dir::Right => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, Dir::Up), hs);
                        }
                    }
                    Dir::Down => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, Dir::Left), hs);
                        }
                    }
                    Dir::Left => self.trace(Pos::new(cp.x, cp.y + 1, Dir::Down), hs),
                },
                // \
                Refl::RLMirror => match cp.d {
                    Dir::Down => self.trace(Pos::new(cp.x + 1, cp.y, Dir::Right), hs),
                    Dir::Left => {
                        if cp.y > 0 {
                            self.trace(Pos::new(cp.x, cp.y - 1, Dir::Up), hs);
                        }
                    }
                    Dir::Up => {
                        if cp.x > 0 {
                            self.trace(Pos::new(cp.x - 1, cp.y, Dir::Left), hs);
                        }
                    }
                    Dir::Right => self.trace(Pos::new(cp.x, cp.y + 1, Dir::Down), hs),
                },
            }
        }
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Matrix(
            s.lines()
                .map(|l| l.chars().map(Refl::from).collect())
                .collect(),
        )))
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let input: Data = Input::default().read().unwrap();
    println!("P1: {}", p1(&input, None));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input: Data = Input::inline(
        "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
    ",
    )
    .read()
    .unwrap();
    let mut d: Matrix<char> = Matrix(
        input
            .0
             .0
            .iter()
            .map(|v| v.iter().map(|_| '.').collect())
            .collect(),
    );
    println!("{input}");
    assert_eq!(p1(&input, Some(&mut d)), 46);
}

#[test]
fn test_p2() {
    let input: Data = Input::inline(
        "\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
    ",
    )
    .read()
    .unwrap();
    assert_eq!(p2(&input), 51);
}

#[test]
fn final_test() {
    let input: Data = Input::file("./data/day16.txt").unwrap().read().unwrap();
    assert_eq!(8901, p1(&input, None));
    assert_eq!(9064, p2(&input));
}
