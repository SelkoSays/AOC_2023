use std::{ops::Range, str::FromStr};

use aoc_2023::input::Input;

fn p1(h: &Seeds, v: &[AtoBMap]) -> usize {
    h.0.iter()
        .map(|seed| v.iter().fold(*seed, |loc, ab| ab.remap(loc)))
        .min()
        .unwrap()
}

fn p2(h: Seeds, v: &[AtoBMap]) -> usize {
    let h = seed_to_range_transforminator(h);
    h.0.into_iter()
        .map(|r| vec![r])
        .map(|seed| {
            v.iter().fold(seed, |acc, ab| {
                acc.iter().flat_map(|r| ab.remap_range(r.clone())).collect()
            })
        })
        .map(|v| v.iter().map(|r| r.0.start).min().unwrap())
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Seeds(Vec<usize>);

impl FromStr for Seeds {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s.split_once(": ").ok_or(())?.1;
        Ok(Self(
            nums.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        ))
    }
}

#[derive(Debug)]
struct SeedsR(Vec<R>);

#[derive(Debug, Default)]
struct AtoBMap {
    _a: String,
    _b: String,
    dst: Vec<R>,
    src: Vec<R>,
}

impl AtoBMap {
    pub fn new(_a: String, _b: String, dst: Vec<R>, src: Vec<R>) -> Self {
        Self { _a, _b, dst, src }
    }

    pub fn remap(&self, seed: usize) -> usize {
        for (i, s) in self.src.iter().enumerate() {
            if s.contains(seed) {
                return self.dst[i].0.start + (seed - s.0.start);
            }
        }
        seed
    }

    pub fn remap_range(&self, seed: R) -> Vec<R> {
        let mut sp = vec![seed.clone()];
        for s in &self.src {
            if s.overlap(&seed) {
                sp = s.split(&seed);
                break;
            }
        }
        for r in &mut sp {
            for (i, s) in self.src.iter().enumerate() {
                if s.overlap(r) {
                    *r = R((self.dst[i].0.start as isize
                        + (r.0.start as isize - s.0.start as isize))
                        as usize
                        ..(self.dst[i].0.start as isize + (r.0.end as isize - s.0.start as isize))
                            as usize);
                    break;
                }
            }
        }
        sp
    }
}

impl FromStr for AtoBMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut h = lines.next().unwrap();
        h = h.split_once(' ').unwrap().0;
        let mut ab = h.split('-');
        let a = ab.next().unwrap();
        let b = ab.last().unwrap();
        let mut dst = vec![];
        let mut src = vec![];
        for l in lines {
            let nums = l
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>();
            dst.push(R(nums[0]..nums[0] + nums[2]));
            src.push(R(nums[1]..nums[1] + nums[2]));
        }
        Ok(Self::new(a.into(), b.into(), dst, src))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct R(Range<usize>);

impl R {
    pub fn contains(&self, item: usize) -> bool {
        self.0.contains(&item)
    }

    pub fn overlap(&self, o: &Self) -> bool {
        use std::cmp::{max, min};
        max(self.0.end, o.0.end) - min(self.0.start, o.0.start) <= (self.0.len() + o.0.len() - 1)
    }

    pub fn split(&self, seed: &Self) -> Vec<Self> {
        let mut v = vec![];
        if seed.0.start >= self.0.start && seed.0.end <= self.0.end {
            return vec![seed.clone()];
        }
        if self.0.start > seed.0.start {
            v.push(Self(seed.0.start..self.0.start));
        }
        if seed.0.end <= self.0.end {
            if self.0.start < seed.0.start {
                v.push(Self(seed.0.start..seed.0.end));
            } else {
                v.push(Self(self.0.start..seed.0.end));
            }
        } else {
            if self.0.start < seed.0.start {
                v.push(Self(seed.0.start..self.0.end));
            } else {
                v.push(self.clone());
            }
            //v.push(self.clone());
            v.push(Self(self.0.end..seed.0.end));
        }

        v
    }
}

fn seed_to_range_transforminator(seeds: Seeds) -> SeedsR {
    SeedsR(seeds.0.chunks(2).map(|c| R(c[0]..c[0] + c[1])).collect())
}

fn main() {
    let (h, v): (Seeds, Vec<AtoBMap>) = Input::default().read_headers_n_spaced_data().unwrap();
    println!("P1: {}", p1(&h, &v));
    println!("P2: {}", p2(h, &v));
}

#[test]
fn test_p1() {
    let input: (Seeds, Vec<AtoBMap>) = Input::inline(
        "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    )
    .read_headers_n_spaced_data()
    .unwrap();

    let expected_seeds = [79usize, 14, 55, 13];
    assert_eq!(input.0 .0.len(), expected_seeds.len());
    for (i, n) in input.0 .0.iter().enumerate() {
        assert_eq!(*n, expected_seeds[i]);
    }

    let exp_atob = [
        AtoBMap::new(
            "seed".into(),
            "soil".into(),
            vec![R(50..52), R(52..100)],
            vec![R(98..100), R(50..98)],
        ),
        AtoBMap::new(
            "soil".into(),
            "fertilizer".into(),
            vec![R(0..37), R(37..39), R(39..39 + 15)],
            vec![R(15..15 + 37), R(52..54), R(0..15)],
        ),
    ];
    assert_eq!(input.1.len(), 7);
    for (i, ab) in exp_atob.iter().enumerate() {
        assert_eq!(ab._a, input.1[i]._a);
        assert_eq!(ab._b, input.1[i]._b);
        assert_eq!(ab.dst, input.1[i].dst);
        assert_eq!(ab.src, input.1[i].src);
    }

    assert_eq!(exp_atob[0].remap(50), 52);
    assert_eq!(exp_atob[0].remap(100), 100);
    assert_eq!(p1(&input.0, &input.1), 35);
}

#[test]
fn test_p2() {
    let (h, v): (Seeds, Vec<AtoBMap>) = Input::inline(
        "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    )
    .read_headers_n_spaced_data()
    .unwrap();
    assert_eq!(p2(h, &v), 46);
}

#[test]
fn test_split() {
    let r = R(5..10);
    let s = [
        R(5..10),
        R(1..6),
        R(5..7),
        R(6..8),
        R(1..10),
        R(6..11),
        R(1..11),
    ];
    let exp = [
        vec![R(5..10)],
        vec![R(1..5), R(5..6)],
        vec![R(5..7)],
        vec![R(6..8)],
        vec![R(1..5), R(5..10)],
        vec![R(6..10), R(10..11)],
        vec![R(1..5), R(5..10), R(10..11)],
    ];
    for (rr, e) in s.iter().zip(exp) {
        let ss = r.split(rr);
        assert_eq!(ss, e);
    }
}

#[test]
fn test_remap_range() {
    let ab = AtoBMap::new(
        "seed".into(),
        "soil".into(),
        vec![R(50..52), R(52..100)],
        vec![R(98..100), R(50..98)],
    );

    assert_eq!(ab.remap_range(R(97..99)), [R(99..100), R(50..51)]);
}

#[test]
fn final_test() {
    let (h, v): (Seeds, Vec<AtoBMap>) = Input::file("./data/day5.txt")
        .unwrap()
        .read_headers_n_spaced_data()
        .unwrap();
    assert_eq!(227653707, p1(&h, &v));
    assert_eq!(78775051, p2(h, &v));
}
