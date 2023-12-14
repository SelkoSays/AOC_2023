use std::collections::HashMap;
use std::fmt::Display;
//use std::ffi::OsString;
use std::str::FromStr;

use aoc_2023::input::Input;
use aoc_2023::matrix::Matrix;

const C: usize = 1_000_000_000;

fn p1(d: &Data) -> usize {
    let mut d = d.clone();
    d.tilt_north();
    d.total_load()
}

fn p2(d: &Data) -> usize {
    let mut d = d.clone();
    let mut hm: HashMap<CompactData, usize> = HashMap::new();
    let mut idx = 0;
    for i in 0..C {
        d.cycle();
        if hm.insert(d.compact(), d.total_load()).is_some() {
            idx = i;
            break;
        }
    }
    let mut cv: Vec<CompactData> = Vec::new();
    cv.push(d.compact());
    for _ in idx..C {
        d.cycle();
        let c = d.compact();
        if c == cv[0] {
            break;
        }
        cv.push(c);
    }
    *hm.get(&cv[(C - idx - 1) % cv.len()]).unwrap()
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy)]
enum Stone {
    NoStone,
    RoundStone,
    SquareStone,
}

impl From<char> for Stone {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::NoStone,
            'O' => Self::RoundStone,
            '#' => Self::SquareStone,
            _ => panic!("wrong input -> {value}"),
        }
    }
}

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stone::NoStone => write!(f, "."),
            Stone::RoundStone => write!(f, "O"),
            Stone::SquareStone => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
struct Data(Matrix<Stone>);

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct CompactData(Vec<u8>);

impl Data {
    pub fn compact(&self) -> CompactData {
        let mut b: Vec<u8> = Vec::new();
        let mut byte = 0u8;
        let mut pos = 0usize;
        for el in self.0.iter() {
            match el {
                Stone::SquareStone => {}
                e => {
                    if pos > 0 && (pos & 7) == 0 {
                        b.push(byte);
                        byte = 0;
                    }
                    byte <<= 1;
                    byte |= *e as u8;
                    pos += 1;
                }
            }
        }
        b.push(byte);
        //unsafe { OsString::from_encoded_bytes_unchecked(b) }
        CompactData(b)
    }

    pub fn look(&self, x: usize, y: usize, dir: Dir) -> usize {
        match dir {
            Dir::North => {
                for i in (0..y).rev() {
                    if let Some(Stone::SquareStone | Stone::RoundStone) = self.0.get(x, i) {
                        return i + 1;
                    }
                }
                0
            }
            Dir::South => {
                for i in y + 1..self.0.leny() {
                    if let Some(Stone::SquareStone | Stone::RoundStone) = self.0.get(x, i) {
                        return i - 1;
                    }
                }
                self.0.leny() - 1
            }
            Dir::East => {
                for i in x + 1..self.0.lenx() {
                    if let Some(Stone::SquareStone | Stone::RoundStone) = self.0.get(i, y) {
                        return i - 1;
                    }
                }
                self.0.lenx() - 1
            }
            Dir::West => {
                for i in (0..x).rev() {
                    if let Some(Stone::SquareStone | Stone::RoundStone) = self.0.get(i, y) {
                        return i + 1;
                    }
                }
                0
            }
        }
    }

    pub fn tilt_north(&mut self) {
        for y in 0..self.0.leny() {
            for x in 0..self.0.lenx() {
                if let Some(Stone::RoundStone) = self.0.get(x, y) {
                    let i = self.look(x, y, Dir::North);
                    let e = self.0.get(x, i);
                    if e.is_some() {
                        *self.0.get_mut(x, y).unwrap() = Stone::NoStone;
                        *self.0.get_mut(x, i).unwrap() = Stone::RoundStone;
                    }
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for y in (0..self.0.leny()).rev() {
            for x in 0..self.0.lenx() {
                if let Some(Stone::RoundStone) = self.0.get(x, y) {
                    let i = self.look(x, y, Dir::South);
                    let e = self.0.get(x, i);
                    if e.is_some() {
                        *self.0.get_mut(x, y).unwrap() = Stone::NoStone;
                        *self.0.get_mut(x, i).unwrap() = Stone::RoundStone;
                    }
                }
            }
        }
    }
    pub fn tilt_east(&mut self) {
        for y in 0..self.0.leny() {
            for x in (0..self.0.lenx()).rev() {
                if let Some(Stone::RoundStone) = self.0.get(x, y) {
                    let i = self.look(x, y, Dir::East);
                    let e = self.0.get(i, y);
                    if e.is_some() {
                        *self.0.get_mut(x, y).unwrap() = Stone::NoStone;
                        *self.0.get_mut(i, y).unwrap() = Stone::RoundStone;
                    }
                }
            }
        }
    }
    pub fn tilt_west(&mut self) {
        for y in 0..self.0.leny() {
            for x in 0..self.0.lenx() {
                if let Some(Stone::RoundStone) = self.0.get(x, y) {
                    let i = self.look(x, y, Dir::West);
                    let e = self.0.get(i, y);
                    if e.is_some() {
                        *self.0.get_mut(x, y).unwrap() = Stone::NoStone;
                        *self.0.get_mut(i, y).unwrap() = Stone::RoundStone;
                    }
                }
            }
        }
    }

    pub fn cycle(&mut self) {
        // ^ < v >
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn total_load(&self) -> usize {
        self.0
             .0
            .iter()
            .rev()
            .enumerate()
            .flat_map(|(y, v)| v.iter().map(move |s| (y, s)))
            .filter(|(_, s)| matches!(s, &&Stone::RoundStone))
            .map(|(y, _)| y + 1)
            .sum()
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vv: Vec<Vec<Stone>> = Vec::new();
        for l in s.lines() {
            vv.push(l.chars().map(Stone::from).collect());
        }
        Ok(Self(Matrix(vv)))
    }
}

fn main() {
    let input: Data = Input::default().read().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let mut input: Data = Input::inline(
        "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
    )
    .read()
    .unwrap();
    //input.tilt_north();
    //input.tilt_south();
    //println!("{}", input.0);
    assert_eq!(p1(&input), 136);
}

#[test]
fn test_p2() {
    let input: Data = Input::inline(
        "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
    )
    .read()
    .unwrap();

    {
        let mut data2 = input.clone();
        data2.tilt_north();
        assert_eq!(
            data2.to_string(),
            "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....\n"
        );
        data2.tilt_west();
        assert_eq!(
            data2.to_string(),
            "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....\n"
        );
        data2.tilt_south();
        assert_eq!(
            data2.to_string(),
            ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....\n"
        );
        data2.tilt_east();
        assert_eq!(
            data2.to_string(),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....\n"
        );
        data2.cycle();
        assert_eq!(
            data2.to_string(),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O\n"
        );
        data2.cycle();
        assert_eq!(
            data2.to_string(),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O\n"
        );
    }

    //println!("{data}");
    assert_eq!(p2(&input), 64);
}

#[test]
fn final_test() {
    let input: Data = Input::file("./data/day14.txt").unwrap().read().unwrap();
    assert_eq!(p1(&input), 110090);
    assert_eq!(p2(&input), 95254);
}

#[test]
fn test_stringify() {
    let d: Data = Input::inline(
        "\
.O.#..
.#..OO
",
    )
    .read()
    .unwrap();
    let s = d.compact().0;
    assert_eq!(2, s.len());
    assert_eq!(0b01000000, s[0]);
    assert_eq!(0b11, s[1]);
}
