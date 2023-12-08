use std::{collections::HashMap, str::FromStr};

use aoc_2023::input::Input;

fn p1(ins: &Instrctions, map: &Map) -> usize {
    let mut it = 0;
    let mut pos = "AAA".to_string();
    while pos != "ZZZ" {
        for i in &ins.0 {
            if let Some(p) = map.map.get(&pos) {
                pos = p[*i as usize].clone();
            }
        }
        it += ins.0.len();
    }
    it
}

fn p2(ins: &Instrctions, map: &Map) -> usize {
    let pos: Vec<String> = map
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect();
    let cy: Vec<usize> = pos
        .iter()
        .map(|s| map.calc_cycle(s, ins))
        .map(|(x, _)| x) // x in _ sta enaka
        .collect();
    cy.iter().fold(1, |acc, v| lcm(acc, *v))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    L,
    D,
}

impl Dir {
    fn from_char(c: &char) -> Result<Self, ()> {
        //println!("char: {}", *c as u8);
        match c {
            'L' => Ok(Self::L),
            'R' => Ok(Self::D),
            _ => Err(()),
        }
    }
}
#[derive(Debug)]
struct Instrctions(Vec<Dir>);

impl FromStr for Instrctions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .chars()
                .map(|c| Dir::from_char(&c).unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug)]
struct Map {
    map: HashMap<String, [String; 2]>,
}

impl Map {
    pub fn from_vec_n(v: Vec<N>) -> Self {
        Self {
            map: v.into_iter().map(|n| (n.0, n.1)).collect(),
        }
    }

    pub fn calc_cycle(&self, start: &str, ins: &Instrctions) -> (usize, usize) {
        let mut it = 0;
        let mut pos = start;
        let mut it_to_z = 0;
        let mut passed_z = 1;
        for i in ins.0.iter().cycle() {
            it += 1 ^ passed_z;
            it_to_z += 1 & passed_z;
            if let Some(p) = self.map.get(pos) {
                pos = &p[*i as usize];
            }
            if pos.ends_with('Z') {
                if passed_z == 0 {
                    break;
                }
                passed_z = 0;
            }
        }
        (it, it_to_z)
    }
}

struct N(String, [String; 2]);
impl FromStr for N {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (k, r) = s.split_once('=').ok_or(())?;
        let (l, d) = r.split_once(',').ok_or(())?;
        Ok(N(
            k.trim().to_owned(),
            [l.trim().replace('(', ""), d.trim().replace(')', "")],
        ))
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (ins, v): (Instrctions, Vec<N>) = Input::default().read_headers_n_data().unwrap();
    let data = Map::from_vec_n(v);
    //p2(&ins, &data);
    println!("P1: {}", p1(&ins, &data));
    println!("P2: {}", p2(&ins, &data));
}

#[test]
fn test_p1() {
    let (ins, v): (Instrctions, Vec<N>) = Input::inline(
        "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
    )
    .read_headers_n_data()
    .unwrap();
    let data = Map::from_vec_n(v);
    assert_eq!(ins.0, vec![Dir::D, Dir::L]);
    let hm = HashMap::<String, [String; 2]>::from([
        ("AAA".into(), ["BBB".into(), "CCC".into()]),
        ("BBB".into(), ["DDD".into(), "EEE".into()]),
        ("CCC".into(), ["ZZZ".into(), "GGG".into()]),
        ("DDD".into(), ["DDD".into(), "DDD".into()]),
        ("EEE".into(), ["EEE".into(), "EEE".into()]),
        ("GGG".into(), ["GGG".into(), "GGG".into()]),
        ("ZZZ".into(), ["ZZZ".into(), "ZZZ".into()]),
    ]);
    assert_eq!(hm, data.map);
    assert_eq!(2, p1(&ins, &data));
    let (ins, v): (Instrctions, Vec<N>) = Input::inline(
        "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
    )
    .read_headers_n_data()
    .unwrap();
    let data = Map::from_vec_n(v);
    assert_eq!(6, p1(&ins, &data));
}

#[test]
fn test_p2() {
    let (ins, v): (Instrctions, Vec<N>) = Input::inline(
        "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
    )
    .read_headers_n_data()
    .unwrap();
    let data = Map::from_vec_n(v);
    assert_eq!(data.calc_cycle("11A", &ins), (2, 2));
    assert_eq!(data.calc_cycle("22A", &ins), (3, 3));
    assert_eq!(6, p2(&ins, &data));
}

#[test]
fn final_test() {
    let (ins, v): (Instrctions, Vec<N>) = Input::file("./data/day8.txt")
        .unwrap()
        .read_headers_n_data()
        .unwrap();
    let data = Map::from_vec_n(v);
    //p2(&ins, &data);
    assert_eq!(20777, p1(&ins, &data));
    assert_eq!(13289612809129, p2(&ins, &data));
}
