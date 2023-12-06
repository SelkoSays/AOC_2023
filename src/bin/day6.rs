use aoc_2023::input::Input;

fn p1(inp: &[Race]) -> u64 {
    inp.iter().map(|r| r.win()).product()
}

fn p2(r: &Race) -> u64 {
    r.win()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn win(&self) -> u64 {
        self.upper_bound() - self.lower_bound() + 1
    }

    fn lower_bound(&self) -> u64 {
        let mut lo = 0u64;
        let mut hi = self.time;
        while lo < hi {
            let t = (lo + hi) / 2;
            let d = self.time - t;
            let td = t * d;
            match td.cmp(&self.dist) {
                std::cmp::Ordering::Less => lo = t + 1,
                std::cmp::Ordering::Equal => lo += 1,
                std::cmp::Ordering::Greater => hi = t,
            }
        }
        if lo == hi {
            lo
        } else {
            panic!("Not found")
        }
    }

    fn upper_bound(&self) -> u64 {
        let mut lo = 0u64;
        let mut hi = self.time;
        while lo < hi {
            let t = (lo + hi) / 2;
            let d = self.time - t;
            let td = t * d;
            match td.cmp(&self.dist) {
                std::cmp::Ordering::Less => hi = t,
                std::cmp::Ordering::Equal => hi -= 1,
                std::cmp::Ordering::Greater => lo = t + 1,
            }
        }
        if lo == hi {
            lo - 1
        } else {
            panic!("Not found")
        }
    }
}

fn input_to_race_trasforminator(v: &[String]) -> Vec<Race> {
    let mut td = v.iter().map(|l| {
        l.split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u64>>()
    });
    let t = td.next().unwrap();
    let d = td.next().unwrap();
    t.into_iter()
        .zip(d)
        .map(|n| Race {
            time: n.0,
            dist: n.1,
        })
        .collect()
}

fn input_to_one_race(v: &[String]) -> Race {
    let mut td = v.iter().map(|l| {
        l.split_once(':')
            .unwrap()
            .1
            .split_ascii_whitespace()
            .fold("".to_owned(), |mut acc, s| {
                acc.push_str(s);
                acc
            })
            .parse::<u64>()
            .unwrap()
    });
    let t = td.next().unwrap();
    let d = td.next().unwrap();
    Race { time: t, dist: d }
}

fn main() {
    let input = Input::default().lines();
    let races = input_to_race_trasforminator(&input);
    let r = input_to_one_race(&input);
    println!("P1: {}", p1(&races));
    println!("P2: {}", p2(&r));
}

#[test]
fn test_p1() {
    let input = Input::inline(
        "\
Time:      7  15   30
Distance:  9  40  200
",
    )
    .lines();
    let rs = input_to_race_trasforminator(&input);
    let exp = vec![
        Race { time: 7, dist: 9 },
        Race { time: 15, dist: 40 },
        Race {
            time: 30,
            dist: 200,
        },
    ];
    assert_eq!(rs, exp);
    let exp_b = vec![(2u64, 5u64), (4, 11), (11, 19)];
    let bs: Vec<_> = exp
        .iter()
        .map(|r| (r.lower_bound(), r.upper_bound()))
        .collect();
    assert_eq!(exp_b, bs);
    assert_eq!(p1(&rs), 288);
}

#[test]
fn test_p2() {
    let input = Input::inline(
        "\
Time:      7  15   30
Distance:  9  40  200
",
    )
    .lines();
    let r = input_to_one_race(&input);
    assert_eq!(
        r,
        Race {
            time: 71530,
            dist: 940200
        }
    );
    assert_eq!(p2(&r), 71503);
}

#[test]
fn final_test() {
    let input = Input::file("./data/day6.txt").unwrap().lines();
    let races = input_to_race_trasforminator(&input);
    let r = input_to_one_race(&input);
    assert_eq!(625968, p1(&races));
    assert_eq!(43663323, p2(&r));
}
