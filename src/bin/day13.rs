use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(inp: &[Map]) -> usize {
    let (r, c): (Vec<_>, Vec<_>) = inp.iter().map(|m| (&m.rows, &m.cols)).unzip();
    let rc: usize = mirrors(&r).iter().sum();
    let cc: usize = mirrors(&c).iter().sum();
    cc + 100 * rc
}

fn p2(inp: &[Map]) -> usize {
    let (r, c): (Vec<_>, Vec<_>) = inp.iter().map(|m| (&m.rows, &m.cols)).unzip();
    let rc: usize = mirrors2(&r).iter().sum();
    let cc: usize = mirrors2(&c).iter().sum();
    cc + 100 * rc
}

fn mirrors(v: &[&Vec<u32>]) -> Vec<usize> {
    let mut vi = Vec::new();
    for vv in v {
        'outer: for i in 0..vv.len() {
            for j in 1..=i {
                if let Some(n) = vv.get(i + j - 1) {
                    if vv[i - j] != *n {
                        continue 'outer;
                    }
                } else {
                    break;
                }
            }
            vi.push(i);
        }
    }
    vi
}

fn mirrors2(v: &[&Vec<u32>]) -> Vec<usize> {
    let mut vi = Vec::new();
    for vv in v {
        //let vv = v[0];
        'outer: for i in 0..vv.len() {
            let mut f = false;
            for j in 1..=i {
                if let Some(n) = vv.get(i + j - 1) {
                    match (vv[i - j] ^ *n).count_ones().cmp(&1) {
                        std::cmp::Ordering::Equal => {
                            if f {
                                continue 'outer;
                            } else {
                                f = true;
                            }
                        }
                        std::cmp::Ordering::Greater => continue 'outer,
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            if f {
                vi.push(i);
            }
        }
    }
    vi
}

struct Map {
    rows: Vec<u32>,
    cols: Vec<u32>,
    _lenx: usize,
    _leny: usize,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lenx = s.split_once('\n').ok_or(())?.0.len();
        let rows: Vec<u32> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => 0u32,
                        '#' => 1,
                        _ => panic!("Invalid, character"),
                    })
                    .fold(0u32, |acc, v| (acc << 1) | v)
            })
            .collect();
        let leny = rows.len();
        let mut cols = vec![0u32; lenx];
        let mut x = lenx;
        for col in cols.iter_mut() {
            for row in rows.iter() {
                *col <<= 1;
                let mask = 1u32 << (x - 1);
                *col |= (row & mask) >> (x - 1);
            }
            x -= 1;
        }
        Ok(Self {
            rows,
            cols,
            _lenx: lenx,
            _leny: leny,
        })
    }
}

fn main() {
    let input: Vec<Map> = Input::default().read_spaced_data().unwrap();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input: Vec<Map> = Input::inline(
        "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    )
    .read_spaced_data()
    .unwrap();
    assert_eq!(9, input[0]._lenx);
    assert_eq!(7, input[0]._leny);
    let exp_rows = vec![
        0b101100110u32,
        0b001011010,
        0b110000001,
        0b110000001,
        0b001011010,
        0b001100110,
        0b101011010,
    ];
    assert_eq!(exp_rows, input[0].rows);
    let exp_cols = vec![
        0b1011001u32,
        0b0011000,
        0b1100111,
        0b1000010,
        0b0100101,
        0b0100101,
        0b1000010,
        0b1100111,
        0b0011000,
    ];
    assert_eq!(exp_cols, input[0].cols);
    assert_eq!(405, p1(&input));
}

#[test]
fn test_p2() {
    let input: Vec<Map> = Input::inline(
        "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    )
    .read_spaced_data()
    .unwrap();
    assert_eq!(400, p2(&input));
}

#[test]
fn final_test() {
    let input: Vec<Map> = Input::file("./data/day13.txt")
        .unwrap()
        .read_spaced_data()
        .unwrap();
    assert_eq!(37975, p1(&input));
    assert_eq!(32497, p2(&input));
}
