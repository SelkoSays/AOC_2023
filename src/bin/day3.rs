use aoc_2023::input::Input;
use std::ops::Range;

fn p1(inp: &[Vec<R>]) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for (i, l) in inp.iter().enumerate() {
        for r in l {
            if let Re::Sym(_) = r.v {
                continue;
            }
            if i != 0 && check_adj_inator(r, &inp[i - 1], &mut v) {
                continue;
            }
            if check_adj_inator(r, &inp[i], &mut v) {
                continue;
            }
            if i < inp.len() - 1 {
                check_adj_inator(r, &inp[i + 1], &mut v);
            }
        }
    }
    v.iter().sum()
}

fn p2() -> u32 {
    todo!();
}

fn check_adj_inator(r: &R, v: &[R], vv: &mut Vec<u32>) -> bool {
    let rv = match r.v {
        Re::Num(n) => n,
        _ => unreachable!(),
    };
    for rr in v {
        match rr.v {
            Re::Num(_) => {}
            Re::Sym(_) => {
                if r.adj(rr) {
                    vv.push(rv);
                    return true;
                }
            }
        }
    }
    false
}

fn transforminator(inp: &[String]) -> Vec<Vec<R>> {
    let mut vv: Vec<Vec<R>> = Vec::with_capacity(inp.len());
    for (j, l) in inp.iter().enumerate() {
        let mut cur: Vec<R> = Vec::with_capacity(l.len());
        let mut n = String::new();
        let mut num = 0u32;
        let mut pd = false; // previous digit
        let mut s_idx = 0;
        for (i, c) in l.chars().enumerate() {
            if !c.is_ascii_digit() {
                if pd {
                    pd = false;
                    num = l[s_idx..i].parse().unwrap();
                    cur.push(R::new(Re::Num(num), j, s_idx..(i - 1)));
                    n.clear();
                }
                if c != '.' {
                    cur.push(R::new(Re::Sym(c), j, i..i));
                }
            } else if !pd {
                pd = true;
                s_idx = i;
                n.push(c);
            } else {
                n.push(c);
            }
        }
        vv.push(cur);
    }
    vv
}

#[derive(Debug, PartialEq, Eq)]
enum Re {
    Sym(char),
    Num(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct R {
    v: Re,           // value of smth
    l: usize,        // line
    r: Range<usize>, // range it's taking
}

impl PartialOrd for R {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.r.end.partial_cmp(&(other.r.start - 1))
    }
}

impl R {
    pub fn new(v: Re, l: usize, r: Range<usize>) -> Self {
        Self { v, l, r }
    }

    pub fn adj(&self, o: &Self) -> bool {
        use std::cmp::{max, min};
        (max(self.l, o.l) - min(self.l, o.l) < 2)
            && (max(self.r.end, o.r.end) - min(self.r.start, o.r.start)
                <= (self.r.len() + o.r.len() + 1))
    }
}

// fn arange_inator(inp: &mut [Vec<u32>]) {}

fn main() {
    // let input: Vec<String> = Input::file("./data/day3.txt").unwrap().lines();
    let input: Vec<String> = Input::default().lines();
    let input = transforminator(&input);
    println!("P1: {}", p1(&input));
}
#[test]
fn test_p1() {
    let input = Input::inline(
        "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
    )
    .lines();
    let data = transforminator(&input);
    let expected: Vec<Vec<R>> = vec![
        vec![R::new(Re::Num(467), 0, 0..2), R::new(Re::Num(114), 0, 5..7)],
        vec![R::new(Re::Sym('*'), 1, 3..3)],
        vec![R::new(Re::Num(35), 2, 2..3), R::new(Re::Num(633), 2, 6..8)],
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(e, &data[i]);
    }
    assert_eq!(p1(&data), 4361);
}
#[test]
fn test_p2() {
    todo!();
}
#[test]
fn final_p1() {}
#[test]
fn final_p2() {}
