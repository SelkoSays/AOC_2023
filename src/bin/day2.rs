use std::str::FromStr;

use aoc_2023::input::Input;

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;

fn p1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|&g| {
            g.showings
                .iter()
                .all(|s| s.0 <= MAX_R && s.1 <= MAX_G && s.2 <= MAX_B)
        })
        .fold(0, |l, r| l + r.id as u32)
}

fn p2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let r = g.showings.iter().map(|v| v.0).max().unwrap_or_default();
            let g_ = g.showings.iter().map(|v| v.1).max().unwrap_or_default();
            let b = g.showings.iter().map(|v| v.2).max().unwrap_or_default();
            r * g_ * b
        })
        .sum::<u32>()
}

#[derive(Debug, Clone)]
struct Cubes(u32, u32, u32);

impl Cubes {
    pub fn new() -> Self {
        Self(0, 0, 0)
    }

    fn add(&mut self, clr: &str, num: u32) {
        match clr.to_lowercase().as_str() {
            "red" => self.0 = num,
            "green" => self.1 = num,
            "blue" => self.2 = num,
            c => panic!("Unknown color {c}"),
        }
    }
    fn clear(&mut self) {
        self.0 = 0;
        self.1 = 0;
        self.2 = 0;
    }
}

#[derive(Debug, Clone)]
struct Game {
    pub id: u16,
    pub showings: Vec<Cubes>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars().skip("Game ".len());
        let id = it
            .by_ref()
            .take_while(|c| *c != ':')
            .collect::<String>()
            .parse()
            .map_err(|_| ())?;
        //println!("Parsed id");
        //let mut r: Peekable<_> = it.filter(|c| !c.is_ascii_whitespace()).peekable();
        let mut showings = vec![];
        let mut cur = Cubes::new();
        let mut n = String::new();
        let mut clr = String::new();
        let mut num = 0u32;
        let mut next = it.by_ref().nth(1);
        while next.is_some() {
            let c = next.unwrap();
            match c {
                ' ' => {
                    //println!("Preparse num '{n}'");
                    num = n.parse().map_err(|_| ())?;
                    //println!("Post num {num}");
                    n.clear();
                }
                ',' => {
                    cur.add(&clr, num);
                    clr.clear();
                    it.next();
                }
                ';' => {
                    cur.add(&clr, num);
                    clr.clear();
                    showings.push(cur.clone());
                    cur.clear();
                    it.next();
                }
                ch if ch.is_ascii_digit() => n.push(ch),
                ch if ch.is_ascii_alphabetic() => clr.push(ch),
                ch => {
                    println!("Wrong character: {ch}");
                    return Err(());
                }
            }
            next = it.next();
        }
        cur.add(&clr, num);
        showings.push(cur.clone());

        Ok(Self { id, showings })
    }
}

fn main() {
    let input: Vec<Game> = Input::default().read_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input = Input::inline(
        "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    );
    let data: Vec<Game> = input.read_data().unwrap();
    // for g in &data {
    //     println!("{g:?}");
    // }
    assert_eq!(p1(&data), 8);
}

#[test]
fn test_p2() {
    let input = Input::inline(
        "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
    );
    let data: Vec<Game> = input.read_data().unwrap();
    assert_eq!(p2(&data), 2286);
}

#[test]
fn final_p1() {
    let input = Input::file("./data/day2.txt").unwrap();
    let data: Vec<Game> = input.read_data().unwrap();
    assert_eq!(p1(&data), 2913);
}

#[test]
fn final_p2() {}
