use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(v: &mut [SpringRow]) -> usize {
    v.iter_mut().map(|s| s.combinations(0)).sum()
}

fn p2(v: &mut [SpringRow]) -> usize {
    v.iter_mut()
        .map(|s| s.unfold(5))
        .map(|mut s| s.combinations(0))
        .sum()
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<u8>,
    fails: Vec<u8>,
}

impl SpringRow {
    pub fn combinations(&mut self, start: usize) -> usize {
        if let Some(i) = self.find(b'?', start) {
            self.springs[i] = b'.';
            let j = self.combinations(i + 1);
            self.springs[i] = b'#';
            let k = self.combinations(i + 1);
            self.springs[i] = b'?';
            j + k
        } else {
            self.validate()
        }
    }

    pub fn validate(&self) -> usize {
        let v: Vec<_> = self
            .springs
            .split(|c| c == &b'.')
            .filter(|s| !s.is_empty())
            .map(|s| s.len() as u8)
            .collect();
        if (v.len() == self.fails.len()) && self.fails.iter().enumerate().all(|(i, f)| f == &v[i]) {
            1
        } else {
            0
        }
    }

    pub fn find(&self, ch: u8, start: usize) -> Option<usize> {
        (start..self.springs.len()).find(|&i| self.springs[i] == ch)
    }

    pub fn unfold(&self, scale: usize) -> Self {
        let mut springs = Vec::new();
        let mut fails = Vec::new();
        for i in 0..scale {
            springs.extend(self.springs.clone());
            if i != scale - 1 {
                springs.push(b'?');
            }
            fails.extend(self.fails.clone());
        }
        Self { springs, fails }
    }

    pub fn like_unfold(&self) -> Self {
        let mut springs = vec![b'?'];
        springs.extend(self.springs.clone());
        Self {
            springs,
            fails: self.fails.clone(),
        }
    }
}

impl FromStr for SpringRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once(' ').ok_or(())?;
        let springs = l.as_bytes().to_vec();
        let fails = r.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(Self { springs, fails })
    }
}

fn main() {
    let mut input = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&mut input));
    println!("P2: {}", p2(&mut input));
}

#[test]
fn test_p1() {
    let mut input: Vec<SpringRow> = Input::inline(
        "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
    )
    .read_data()
    .unwrap();
    let exp = vec![1usize, 4, 1, 1, 4, 10];
    let v: Vec<_> = input.iter_mut().map(|s| s.combinations(0)).collect();
    assert_eq!(exp, v);
    assert_eq!(21, p1(&mut input));
}

#[test]
fn test_p2() {
    let mut input: Vec<SpringRow> = Input::inline(
        "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
    )
    .read_data()
    .unwrap();
    // let exp = vec![1usize, 4, 1, 1, 4, 10];
    // let v: Vec<_> = input.iter_mut().map(|s| s.combinations(0)).collect();
    // assert_eq!(exp, v);
}

#[test]
fn final_test() {
    let input = Input::file("./data/day12.txt").unwrap().lines();
}
