use std::collections::HashMap;

use aoc_2023::input::Input;

fn p1(inp: Vec<Hand>) -> usize {
    let mut d = inp;
    d.sort();
    d.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum()
}

fn p2(inp: Vec<Hand>) -> usize {
    p1(inp)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    pub fn new(cards: &[u8; 5]) -> Self {
        let mut hm: HashMap<u8, u8> = HashMap::new();
        for c in cards {
            if let Some(v) = hm.get_mut(c) {
                *v += 1;
            } else {
                hm.insert(*c, 1);
            }
        }
        if let Some(j) = hm.remove(&1) {
            // Joker = 1
            if !hm.is_empty() {
                let k = *hm
                    .iter()
                    .filter(|(k, _)| **k != 1)
                    .max_by_key(|e| e.1)
                    .unwrap()
                    .0;
                if let Some(v) = hm.get_mut(&k) {
                    *v += j;
                }
            }
        }
        match hm.len() {
            1 | 0 => Self::FiveOfKind,
            2 => match hm.get(hm.keys().next().unwrap()) {
                Some(v) if *v == 1 || *v == 4 => Self::FourOfKind,
                Some(_) => Self::FullHouse,
                None => unreachable!("Key doesn't exist"),
            },
            3 => cards
                .iter()
                .filter_map(|c| {
                    if let Some(v) = hm.get(c) {
                        if *v == 3 {
                            return Some(Self::ThreeOfKind);
                        }
                    }
                    None
                })
                .next()
                .unwrap_or(Self::TwoPair),
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!("There is more than five different cards"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    ty: HandType,
    cards: [u8; 5],
    bid: usize,
}

impl Hand {
    pub fn new(cards: [u8; 5], bid: usize) -> Self {
        Self {
            ty: HandType::new(&cards),
            cards,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.ty.cmp(&other.ty) {
            core::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

fn hand_transforminator(inp: &[String], hm: &HashMap<char, u8>) -> Vec<Hand> {
    inp.iter()
        .map(|l| {
            let (c, b) = l.split_once(' ').unwrap();
            Hand::new(
                c.chars()
                    .map(|ch| *hm.get(&ch).unwrap())
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap(),
                b.parse().unwrap(),
            )
        })
        .collect()
}

fn main() {
    let mut hm: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let input = Input::default().lines();
    let data = hand_transforminator(&input, &hm);
    println!("P1: {}", p1(data));
    if let Some(v) = hm.get_mut(&'J') {
        *v = 1
    }
    let data = hand_transforminator(&input, &hm);
    println!("P2: {}", p2(data));
}

#[test]
fn test_p1() {
    let input = Input::inline(
        "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
    )
    .lines();
    let hm: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let h = hand_transforminator(&input, &hm);
    let mut exp = [
        Hand {
            ty: HandType::OnePair,
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        },
        Hand {
            ty: HandType::ThreeOfKind,
            cards: [10, 5, 5, 11, 5],
            bid: 684,
        },
        Hand {
            ty: HandType::TwoPair,
            cards: [13, 13, 6, 7, 7],
            bid: 28,
        },
        Hand {
            ty: HandType::TwoPair,
            cards: [13, 10, 11, 11, 10],
            bid: 220,
        },
        Hand {
            ty: HandType::ThreeOfKind,
            cards: [12, 12, 12, 11, 14],
            bid: 483,
        },
    ];
    assert_eq!(h, exp);
    let exp_s = [
        Hand::new([3, 2, 10, 3, 13], 765),
        Hand::new([13, 10, 11, 11, 10], 220),
        Hand::new([13, 13, 6, 7, 7], 28),
        Hand::new([10, 5, 5, 11, 5], 684),
        Hand::new([12, 12, 12, 11, 14], 483),
    ];
    exp.sort();
    assert_eq!(exp_s, exp);
    assert_eq!(p1(h), 6440);
}

#[test]
fn test_p2() {
    let input = Input::inline(
        "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
    )
    .lines();
    let hm: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 1),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let h = hand_transforminator(&input, &hm);
    let mut exp = [
        Hand {
            ty: HandType::OnePair,
            cards: [3, 2, 10, 3, 13],
            bid: 765,
        },
        Hand {
            ty: HandType::FourOfKind,
            cards: [10, 5, 5, 1, 5],
            bid: 684,
        },
        Hand {
            ty: HandType::TwoPair,
            cards: [13, 13, 6, 7, 7],
            bid: 28,
        },
        Hand {
            ty: HandType::FourOfKind,
            cards: [13, 10, 1, 1, 10],
            bid: 220,
        },
        Hand {
            ty: HandType::FourOfKind,
            cards: [12, 12, 12, 1, 14],
            bid: 483,
        },
    ];
    assert_eq!(h, exp);
    let exp_s = [
        Hand::new([3, 2, 10, 3, 13], 765),
        Hand::new([13, 13, 6, 7, 7], 28),
        Hand::new([10, 5, 5, 1, 5], 684),
        Hand::new([12, 12, 12, 1, 14], 483),
        Hand::new([13, 10, 1, 1, 10], 220),
    ];
    exp.sort();
    assert_eq!(exp_s, exp);
    //assert_eq!(p2(h), 6440);
}

#[test]
fn final_test() {
    let mut hm: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);
    let input = Input::file("./data/day7.txt").unwrap().lines();
    let data = hand_transforminator(&input, &hm);
    assert_eq!(247823654, p1(data));
    if let Some(v) = hm.get_mut(&'J') {
        *v = 1
    }
    let data = hand_transforminator(&input, &hm);
    assert_eq!(245461700, p2(data));
}
