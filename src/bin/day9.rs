use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(s: &[Seq]) -> i32 {
    s.iter().map(|seq| Seq::next_in_seq(seq.gen_seq())).sum()
}

fn p2(s: &[Seq]) -> i32 {
    s.iter().map(|seq| Seq::prev_in_seq(seq.gen_seq())).sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Seq(Vec<i32>);

impl Seq {
    pub fn gen_seq(&self) -> Vec<Self> {
        let mut v = vec![self.clone()];
        let mut i = 0;
        while !v[i].0.iter().all(|n| *n == 0) {
            v.push(Self(v[i].0.windows(2).map(|a| a[1] - a[0]).collect()));
            i += 1;
        }
        v
    }

    pub fn next_in_seq(seq: Vec<Self>) -> i32 {
        let mut s = seq;
        let mut i = s.len() - 1;
        s[i].0.push(0);
        while i > 0 {
            let n = s[i].0.last().unwrap() + s[i - 1].0.last().unwrap();
            s[i - 1].0.push(n);
            i -= 1;
        }
        *s[0].0.last().unwrap()
    }

    pub fn prev_in_seq(seq: Vec<Self>) -> i32 {
        let mut s: Vec<Self> = seq
            .into_iter()
            .map(|ss| Seq(ss.0.into_iter().rev().collect()))
            .collect();
        let mut i = s.len() - 1;
        s[i].0.push(0);
        while i > 0 {
            let n = s[i - 1].0.last().unwrap() - s[i].0.last().unwrap();
            s[i - 1].0.push(n);
            i -= 1;
        }
        *s[0].0.last().unwrap()
    }
}

impl FromStr for Seq {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        ))
    }
}

fn main() {
    let input: Vec<Seq> = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input: Vec<Seq> = Input::inline(
        "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    )
    .read_data()
    .unwrap();
    let exp = [
        vec![
            Seq(vec![0, 3, 6, 9, 12, 15]),
            Seq(vec![3, 3, 3, 3, 3]),
            Seq(vec![0, 0, 0, 0]),
        ],
        vec![
            Seq(vec![1, 3, 6, 10, 15, 21]),
            Seq(vec![2, 3, 4, 5, 6]),
            Seq(vec![1, 1, 1, 1]),
            Seq(vec![0, 0, 0]),
        ],
        vec![
            Seq(vec![10, 13, 16, 21, 30, 45]),
            Seq(vec![3, 3, 5, 9, 15]),
            Seq(vec![0, 2, 4, 6]),
            Seq(vec![2, 2, 2]),
            Seq(vec![0, 0]),
        ],
    ];
    let exp_next = [18, 28, 68];
    for (i, s) in input.iter().enumerate() {
        let seq = s.gen_seq();
        assert_eq!(exp[i], seq);
        assert_eq!(exp_next[i], Seq::next_in_seq(seq));
    }
    assert_eq!(114, p1(&input));
}

#[test]
fn test_p2() {
    let input: Vec<Seq> = Input::inline(
        "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
    )
    .read_data()
    .unwrap();
    assert_eq!(2, p2(&input));
}

#[test]
fn final_test() {
    let input = Input::file("./data/day9.txt").unwrap().read_data().unwrap();
    assert_eq!(1834108701, p1(&input));
    assert_eq!(993, p2(&input));
}
