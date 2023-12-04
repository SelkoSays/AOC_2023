use aoc_2023::input::Input;
use std::ops::Range;

fn p1(inp: &[Vec<R>]) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for (i, l) in inp.iter().enumerate() {
        for r in l {
            if let Re::Sym(_) = r.v {
                continue;
            }
            if i != 0 && check_adj_inator(r, &inp[i - 1]) {
                v.push(r.v.unwrap_num());
                continue;
            }
            if check_adj_inator(r, &inp[i]) {
                v.push(r.v.unwrap_num());
                continue;
            }
            if (i < inp.len() - 1) && check_adj_inator(r, &inp[i + 1]) {
                v.push(r.v.unwrap_num());
            }
        }
        //println!("{:?}", &v[idx..v.len()]);
    }
    v.iter().sum()
}

fn p2(inp: &[Vec<R>]) -> u32 {
    let mut v: Vec<u32> = Vec::new();
    for (i, l) in inp.iter().enumerate() {
        for r in l {
            match r.v {
                Re::Sym(s) if s != '*' => continue,
                Re::Num(_) => continue,
                _ => {}
            }
            let mut read = 0;
            let mut vv: Vec<Vec<u32>> = vec![];
            if i != 0 {
                let a = check_adj_inator_for_2(r, &inp[i - 1]);
                read += a.len();
                vv.push(a);
            }
            let a = check_adj_inator_for_2(r, &inp[i]);
            read += a.len();
            vv.push(a);
            if i < inp.len() - 1 {
                let a = check_adj_inator_for_2(r, &inp[i + 1]);
                read += a.len();
                vv.push(a);
            }
            if read != 2 {
                continue;
            }
            v.push(vv.into_iter().flatten().product::<u32>());
        }
        //println!("{:?}", &v[idx..v.len()]);
    }
    v.iter().sum()
}

fn check_adj_inator(r: &R, v: &[R]) -> bool {
    for rr in v {
        match rr.v {
            Re::Num(_) => {}
            Re::Sym(_) => {
                if r.adj(rr) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_adj_inator_for_2(r: &R, v: &[R]) -> Vec<u32> {
    let mut adj = vec![];
    for rr in v {
        if let Re::Num(n) = rr.v {
            if r.adj(rr) {
                adj.push(n);
            }
        }
    }
    adj
}

fn transforminator(inp: &[String]) -> Vec<Vec<R>> {
    let mut vv: Vec<Vec<R>> = Vec::with_capacity(inp.len());
    for (j, l) in inp.iter().enumerate() {
        let mut cur: Vec<R> = Vec::with_capacity(l.len());
        let mut n = String::new();
        let mut pd = false; // previous digit
        let mut s_idx = 0;
        for (i, c) in l.chars().enumerate() {
            if !c.is_ascii_digit() {
                if pd {
                    pd = false;
                    cur.push(R::new(
                        Re::Num(l[s_idx..i].parse().unwrap()),
                        j,
                        s_idx..(i - 1),
                    ));
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
        if !n.is_empty() {
            cur.push(R::new(
                Re::Num(l[s_idx..l.len()].parse().unwrap()),
                j,
                s_idx..(l.len() - 1),
            ));
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

impl Re {
    pub fn unwrap_num(&self) -> u32 {
        if let Self::Num(n) = self {
            *n
        } else {
            panic!("Re is a symbol: {:?}", self);
        }
    }
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

fn main() {
    let input: Vec<String> = Input::default().lines();
    let input = transforminator(&input);
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
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
        vec![R::new(Re::Sym('#'), 3, 6..6)],
        vec![R::new(Re::Num(617), 4, 0..2), R::new(Re::Sym('*'), 4, 3..3)],
    ];
    for (i, e) in expected.iter().enumerate() {
        assert_eq!(e, &data[i]);
    }
    assert_eq!(p1(&data), 4361);
}
#[test]
fn test_p2() {
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
    assert_eq!(p2(&data), 467835);
}

#[test]
fn test_adj() {
    let input = Input::inline(
        "\
.**
.*4
.#*
",
    )
    .lines();
    let data = transforminator(&input);
    //dbg!(&data);
    for d in &data {
        for r in d {
            if let Re::Sym(_) = r.v {
                assert!(r.adj(&data[1][1]));
            }
        }
    }
    let input = Input::inline(
        "\
*****
*...*
*.4.*
*...*
*****
",
    )
    .lines();
    let data = transforminator(&input);
    assert_eq!(data[2][1].v.unwrap_num(), 4);
    for d in &data {
        for r in d {
            if let Re::Sym(_) = r.v {
                assert!(!r.adj(&data[2][1]));
            }
        }
    }
}

#[test]
fn final_test() {
    let input: Vec<String> = Input::file("./data/day3.txt").unwrap().lines();
    let input = transforminator(&input);
    assert_eq!(533784, p1(&input));
    assert_eq!(78826761, p2(&input));
}
