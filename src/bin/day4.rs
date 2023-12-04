use std::str::FromStr;

use aoc_2023::input::Input;
fn p1(inp: &[Card]) -> u32 {
    inp.iter().map(|c| c.score()).sum()
}

fn p2(inp: &[Card]) -> u32 {
    let mut v: Vec<u32> = vec![1; inp.len() + 1];
    for c in inp {
        let s = c.matching();
        let cs = v[c.id as usize];
        for i in c.id + 1..=c.id + s {
            if let Some(cc) = v.get_mut(i as usize) {
                *cc += cs;
            }
        }
    }
    //dbg!(&v);
    v.iter().skip(1).sum()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    having: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let mut score = 0;
        for w in &self.winning {
            if self.having.contains(w) {
                if score == 0 {
                    score = 1;
                } else {
                    score <<= 1;
                }
            }
        }
        score
    }
    fn matching(&self) -> u32 {
        let mut score = 0;
        for w in &self.winning {
            if self.having.contains(w) {
                score += 1;
            }
        }
        score
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, r) = s.split_once(':').ok_or(())?;
        let id = c
            .split_ascii_whitespace()
            .last()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?;
        let (w, h) = r.split_once('|').ok_or(())?;
        let winning = w
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let having = h
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Ok(Card {
            id,
            winning,
            having,
        })
    }
}

fn main() {
    let input: Vec<Card> = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}
#[test]
fn test_p1() {
    let input: Vec<Card> = Input::inline(
        "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
    )
    .read_data()
    .unwrap();
    //println!("{:?}", input[0]);
    assert_eq!(p1(&input), 13);
}
#[test]
fn test_p2() {
    let input: Vec<Card> = Input::inline(
        "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
    )
    .read_data()
    .unwrap();
    //println!("{:?}", input[0]);
    assert_eq!(p2(&input), 30);
}
#[test]
fn final_test() {
    let input: Vec<Card> = Input::file("./data/day4.txt").unwrap().read_data().unwrap();
    assert_eq!(19855, p1(&input));
    assert_eq!(10378710, p2(&input));
}
