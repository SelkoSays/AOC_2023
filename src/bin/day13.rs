use std::str::FromStr;

use aoc_2023::input::Input;

fn p1(inp: &[Map]) -> usize {
    let mut rc = 0;
    let mut cc = 0;
    for (r, c) in inp.iter().map(|m| (&m.rows, &m.cols)) {
        let it = c.windows(2).enumerate().filter_map(
            |(i, w)| {
                if w[0] == w[1] {
                    Some(i + 1)
                } else {
                    None
                }
            },
        );
        'outer: for i in it {
            for j in 1..=i {
                if let Some(n) = c.get(i + j - 1) {
                    if c[i - j] != *n {
                        continue 'outer;
                    }
                } else {
                    break;
                }
            }
            cc += i;
        }
        let it = r.windows(2).enumerate().filter_map(
            |(i, w)| {
                if w[0] == w[1] {
                    Some(i + 1)
                } else {
                    None
                }
            },
        );

        'outer: for i in it {
            for j in 1..=i {
                if let Some(n) = r.get(i + j - 1) {
                    if r[i - j] != *n {
                        continue 'outer;
                    }
                } else {
                    break;
                }
            }
            rc += i;
        }
    }
    cc + 100 * rc
}

fn p2(inp: &[Map]) -> usize {
    let mut rc = 0;
    let mut cc = 0;
    for (r, c) in inp.iter().map(|m| (&m.rows, &m.cols)) {
        let it = c.windows(2).enumerate().filter_map(|(i, w)| {
            if (w[0] & !w[1]).count_ones() <= 1 {
                Some(i + 1)
            } else {
                None
            }
        });
        let mut f = false;
        'outer: for i in it {
            f = false;
            for j in 1..=i {
                if let Some(n) = c.get(i + j - 1) {
                    match (c[i - j] & (!*n)).count_ones() {
                        1 => {
                            if f {
                                continue 'outer;
                            } else {
                                f = true;
                            }
                        }
                        2.. => {
                            continue 'outer;
                        }
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            cc += i;
        }
        let it = r.windows(2).enumerate().filter_map(|(i, w)| {
            if (w[0] & !w[1]).count_ones() <= 1 {
                Some(i + 1)
            } else {
                None
            }
        });

        'outer: for i in it {
            f = false;
            for j in 1..=i {
                if let Some(n) = r.get(i + j - 1) {
                    match (r[i - j] & (!*n)).count_ones() {
                        1 => {
                            if f {
                                continue 'outer;
                            } else {
                                f = true;
                            }
                        }
                        2.. => {
                            continue 'outer;
                        }
                        _ => {}
                    }
                } else {
                    break;
                }
            }
            rc += i;
        }
    }
    cc + 100 * rc
}

struct Map {
    rows: Vec<u32>,
    cols: Vec<u32>,
    lenx: usize,
    leny: usize,
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
            lenx,
            leny,
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
    assert_eq!(9, input[0].lenx);
    assert_eq!(7, input[0].leny);
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
}
