use aoc_2023::input::Input;

fn p1(inp: &[String]) -> usize {
    let mut m = Tile::to_mat(inp);
    m.mark_empty_rows();
    m.mark_empty_columns();
    let g = m.get_gal(1);
    g.iter()
        .flat_map(|g1| g.iter().map(move |g2| (g1, g2)).filter(|(p1, p2)| p1 < p2))
        .map(|(g1, g2)| distance(*g1, *g2))
        .sum()
}

fn p2(inp: &[String]) -> usize {
    let mut m = Tile::to_mat(inp);
    m.mark_empty_rows();
    m.mark_empty_columns();
    let g = m.get_gal(999_999);
    g.iter()
        .flat_map(|g1| g.iter().map(move |g2| (g1, g2)).filter(|(p1, p2)| p1 < p2))
        .map(|(g1, g2)| distance(*g1, *g2))
        .sum()
}

fn distance(g1: (usize, usize), g2: (usize, usize)) -> usize {
    (g1.0.max(g2.0) - g1.0.min(g2.0)) + (g1.1.max(g2.1) - g1.1.min(g2.1))
}

enum Tile {
    Empty,
    Galaxy,
    Space,
}

impl Tile {
    pub fn to_mat(v: &[String]) -> Matrix<Tile> {
        Matrix(
            v.iter()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Tile::Empty,
                            '#' => Tile::Galaxy,
                            _ => panic!("Wrong char provided {c}"),
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Matrix<T>(Vec<Vec<T>>);
impl<T> Matrix<T> {
    #[allow(dead_code)]
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.0.get(y).map(|v| v.get(x)).unwrap_or(None)
    }
}

impl Matrix<Tile> {
    pub fn mark_empty_rows(&mut self) {
        self.0
            .iter_mut()
            .filter(|vt| vt.iter().all(|t| matches!(*t, Tile::Empty)))
            .for_each(|vt| vt.iter_mut().for_each(|t| *t = Tile::Space));
    }

    pub fn mark_empty_columns(&mut self) {
        for x in 0..self.0[0].len() {
            let mut all_empty = true;
            for y in 0..self.0.len() {
                match self.0[y][x] {
                    Tile::Empty => {}
                    Tile::Galaxy => {
                        all_empty = false;
                        break;
                    }
                    _ => {}
                }
            }
            if all_empty {
                for y in 0..self.0.len() {
                    self.0[y][x] = Tile::Space
                }
            }
        }
    }

    pub fn get_gal(&self, space: usize) -> Vec<(usize, usize)> {
        let mut sy = 0;
        let mut v = Vec::new();
        for (j, vt) in self.0.iter().enumerate() {
            // v
            if matches!(vt[0], Tile::Space) {
                sy += space;
                continue;
            }
            let mut sx = 0;
            for (i, t) in vt.iter().enumerate() {
                // ---->
                match t {
                    Tile::Empty => {}
                    Tile::Galaxy => v.push((sx + i, sy + j)),
                    Tile::Space => {
                        sx += space;
                    }
                }
            }
        }
        v
    }
}

fn main() {
    let input = Input::default().lines();
    println!("P1: {}", p1(&input));
    println!("P2: {}", p2(&input));
}

#[test]
fn test_p1() {
    let input = Input::inline(
        "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
    )
    .lines();
    let input2 = Input::inline(
        "\
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
",
    )
    .lines();
    let mut m = Tile::to_mat(&input);
    m.mark_empty_rows();
    m.mark_empty_columns();
    let g = m.get_gal(1);
    let m = Tile::to_mat(&input2);
    let g1 = m.get_gal(0);
    assert_eq!(g, g1);
    assert_eq!(374, p1(&input));
}

#[test]
fn test_p2() {
    let input = Input::inline(
        "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
    )
    .lines();
    let mut m = Tile::to_mat(&input);
    m.mark_empty_rows();
    m.mark_empty_columns();
    let g = m.get_gal(9);
    let sum: usize = g
        .iter()
        .flat_map(|g1| g.iter().map(move |g2| (g1, g2)).filter(|(p1, p2)| p1 < p2))
        .map(|(g1, g2)| distance(*g1, *g2))
        .sum();
    assert_eq!(1030, sum);
    let g = m.get_gal(99);
    let sum: usize = g
        .iter()
        .flat_map(|g1| g.iter().map(move |g2| (g1, g2)).filter(|(p1, p2)| p1 < p2))
        .map(|(g1, g2)| distance(*g1, *g2))
        .sum();
    assert_eq!(8410, sum);
}

#[test]
fn final_test() {
    let input = Input::file("./data/day11.txt").unwrap().lines();
    assert_eq!(9693756, p1(&input));
    assert_eq!(717878258016, p2(&input));
}
