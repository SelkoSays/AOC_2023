use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use aoc_2023::input::Input;

fn p1(wf: &HashMap<String, Workflow>, rs: &[Rating]) -> u32 {
    rs.iter()
        .filter(|&r| Workflow::accept("in", wf, r))
        .map(|r| r.0.values().sum::<u32>())
        .sum()
}

fn p2(wf: &HashMap<String, Workflow>) -> u32 {
    let r = RatingRange::new(1..=4000, 1..=4000, 1..=4000, 1..=4000);

    0
}

#[derive(Debug)]
enum Rule {
    True,
    Lt(String, u32),
    Gt(String, u32),
}

impl Rule {
    pub fn apply(&self, r: &Rating) -> bool {
        match self {
            Rule::True => true,
            Rule::Lt(s, n) => {
                if let Some(p) = r.0.get(s) {
                    p < n
                } else {
                    panic!("Unknown part: {s}")
                }
            }
            Rule::Gt(s, n) => {
                if let Some(p) = r.0.get(s) {
                    p > n
                } else {
                    panic!("Unknown part: {s}")
                }
            }
        }
    }

    pub fn apply_range(&self, r: RatingRange) -> Option<RatingRange> {
        match self {
            Rule::True => Some(r),
            Rule::Lt(s, n) => {
                if let Some(p) = r.0.get(s) {
                    if p.end() < &(*n as usize) {
                        Some(r)
                    } else if p.start() < &(*n as usize) {
                        Some(r.replace(s, *p.start()..=(*n - 1) as usize))
                    } else {
                        None
                    }
                } else {
                    panic!("Unknown part: {s}")
                }
            }
            Rule::Gt(s, n) => {
                if let Some(p) = r.0.get(s) {
                    if p.start() > &(*n as usize) {
                        Some(r)
                    } else if p.end() > &(*n as usize) {
                        Some(r.replace(s, *n as usize..=*p.end()))
                    } else {
                        None
                    }
                } else {
                    panic!("Unknown part: {s}")
                }
            }
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((l, r)) = s.split_once('<') {
            Ok(Self::Lt(l.into(), r.parse().map_err(|_| ())?))
        } else if let Some((l, r)) = s.split_once('>') {
            Ok(Self::Gt(l.into(), r.parse().map_err(|_| ())?))
        } else {
            Ok(Self::True)
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<(Rule, String)>,
}

impl Workflow {
    pub fn walk_through(&self, r: &Rating) -> String {
        for (rule, wf) in &self.rules {
            if rule.apply(r) {
                return wf.clone();
            }
        }
        panic!("Couldn't find a workflow");
    }

    pub fn accept(start: &str, wfs: &HashMap<String, Workflow>, r: &Rating) -> bool {
        let mut wf_name = start.to_owned();
        let mut cur_wf = wfs.get(&wf_name);
        loop {
            match cur_wf {
                Some(wf) => {
                    wf_name = wf.walk_through(r);
                    cur_wf = wfs.get(&wf_name);
                }
                None => match wf_name.as_str() {
                    "R" => return false,
                    "A" => return true,
                    _ => panic!("Should be A|R, got: {wf_name}"),
                },
            }
        }
    }

    pub fn walk_through_range(&self, r: &mut RatingRange) -> Vec<String> {
        todo!()
    }

    pub fn accept_range(
        start: Vec<String>,
        wfs: &HashMap<String, Workflow>,
        r: &mut RatingRange,
    ) -> bool {
        let mut wf_name = start.to_owned();
        todo!();
        /*let mut cur_wf = wfs.get(&wf_name);
        loop {
            match cur_wf {
                Some(wf) => {
                    wf_name = wf.walk_through_range(r);
                    cur_wf = wfs.get(&wf_name);
                }
                None => match wf_name.as_str() {
                    "R" => return false,
                    "A" => return true,
                    _ => panic!("Should be A|R, got: {wf_name}"),
                },
            }
        }*/
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, r) = s.split_once('{').ok_or(())?;
        let r = r.replace('}', "");
        let rules = r
            .split(',')
            .map(|rl| match rl.split_once(':') {
                Some((r, w)) => (r.parse().unwrap(), w.to_owned()),
                None => (Rule::True, rl.to_string()),
            })
            .collect();

        Ok(Self {
            name: name.into(),
            rules,
        })
    }
}

#[derive(Debug)]
struct RatingRange(HashMap<String, RangeInclusive<usize>>);

impl RatingRange {
    pub fn new(
        x: RangeInclusive<usize>,
        m: RangeInclusive<usize>,
        a: RangeInclusive<usize>,
        s: RangeInclusive<usize>,
    ) -> Self {
        Self(HashMap::from([
            ("x".into(), x),
            ("m".into(), m),
            ("a".into(), a),
            ("s".into(), s),
        ]))
    }

    pub fn replace(&self, field: impl Into<String>, v: RangeInclusive<usize>) -> Self {
        match field.into().as_str() {
            "x" => Self::new(
                v,
                self.0.get("m").unwrap().clone(),
                self.0.get("a").unwrap().clone(),
                self.0.get("s").unwrap().clone(),
            ),
            "m" => Self::new(
                self.0.get("x").unwrap().clone(),
                v,
                self.0.get("a").unwrap().clone(),
                self.0.get("s").unwrap().clone(),
            ),
            "a" => Self::new(
                self.0.get("x").unwrap().clone(),
                self.0.get("m").unwrap().clone(),
                v,
                self.0.get("s").unwrap().clone(),
            ),
            "s" => Self::new(
                self.0.get("x").unwrap().clone(),
                self.0.get("m").unwrap().clone(),
                self.0.get("a").unwrap().clone(),
                v,
            ),
            s => panic!("Unknown field: {s}"),
        }
    }
}

#[derive(Debug)]
struct Rating(HashMap<String, u32>);

impl FromStr for Rating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(['{', '}'], "");
        let sit = s.split(',');
        let mut hm: HashMap<String, u32> = HashMap::new();
        for s in sit {
            let (n, v) = s.split_once('=').ok_or(())?;
            hm.insert(n.into(), v.parse().map_err(|_| ())?);
        }
        Ok(Self(hm))
    }
}

fn main() {
    let (workflows, ratings): (Vec<Workflow>, Vec<Rating>) =
        Input::default().read_n_split_once_on_empty_line().unwrap();
    let wfs: HashMap<String, Workflow> =
        workflows.into_iter().map(|w| (w.name.clone(), w)).collect();
    println!("P1: {}", p1(&wfs, &ratings));
    println!("P2: {}", p2(&wfs));
}

#[test]
fn test_p1() {
    let (wfs, rs): (Vec<Workflow>, Vec<Rating>) = Input::inline(
        "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
    )
    .read_n_split_once_on_empty_line()
    .unwrap();
    let wfs: HashMap<String, Workflow> = wfs.into_iter().map(|w| (w.name.clone(), w)).collect();
    assert_eq!(p1(&wfs, &rs), 19114);
}

#[test]
fn test_p2() {
    todo!();
}

#[test]
fn final_test() {
    let input = Input::file("./data/day19.txt").unwrap().lines();
}
