use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(inp: &[Cmd]) -> usize {
    inp.iter().map(|c| c.hash()).sum()
}

fn p2(inp: &[Cmd]) -> usize {
    let mut boxes = vec![vec![]; 256];
    for c in inp {
        c.execute(&mut boxes);
    }
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, l)| (i + 1) * (j + 1) * l.f_len as usize)
        })
        .sum()
}

#[derive(Debug)]
enum Op {
    Eq(u8), // ASCII value
    Dash,
}

#[derive(Debug)]
struct Cmd {
    label: String,
    op: Op,
}

impl Cmd {
    pub fn hash(&self) -> usize {
        let mut h = self.label_hash();
        match self.op {
            Op::Eq(c) => {
                h = ((h + '=' as usize) * 17) % 256;
                h = ((h + c as usize) * 17) % 256;
            }
            Op::Dash => h = ((h + '-' as usize) * 17) % 256,
        }
        h
    }

    pub fn label_hash(&self) -> usize {
        self.label
            .chars()
            .map(|c| c as u8)
            .fold(0usize, |acc, v| ((acc + v as usize) * 17) % 256)
    }

    pub fn execute(&self, boxes: &mut [Vec<Lens>]) {
        let h = self.label_hash();
        let lenses = &mut boxes[h];
        let p = lenses.iter().position(|e| e.label == self.label);
        match self.op {
            Op::Eq(ascii_repr) => {
                if let Some(p) = p {
                    (*lenses)[p].f_len = ascii_repr - b'0';
                } else {
                    (*lenses).push(Lens {
                        label: self.label.clone(),
                        f_len: ascii_repr - b'0',
                    });
                }
            }
            Op::Dash => {
                if let Some(p) = p {
                    (*lenses).remove(p);
                }
            }
        }
    }
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((l, r)) = s.split_once('=') {
            Ok(Self {
                label: l.into(),
                op: Op::Eq(r.chars().next().ok_or(())? as u8),
            })
        } else {
            Ok(Self {
                label: s.replace('-', ""),
                op: Op::Dash,
            })
        }
    }
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    f_len: u8,
}

fn main() {
    let input: Vec<Cmd> = Input::default().read_seperated(',').unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input: Vec<Cmd> = Input::inline("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        .read_seperated(',')
        .unwrap();
    assert_eq!(p1(&input), 1320);
}

#[test]
fn test_p2() {
    let input: Vec<Cmd> = Input::inline("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
        .read_seperated(',')
        .unwrap();
    assert_eq!(p2(&input), 145);
}

#[test]
fn final_test() {
    let input: Vec<Cmd> = Input::file("./data/day15.txt")
        .unwrap()
        .read_seperated(',')
        .unwrap();
    assert_eq!(516469, p1(&input));
    assert_eq!(221627, p2(&input));
}
