use aoc_2023::input::Input;

fn p1(input: &[String]) -> u32 {
    let mut sum = 0u32;
    for line in input {
        let mut dig = line.chars().filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c as u32 - '0' as u32)
            } else {
                None
            }
        });
        let mut num = 0;
        if let Some(d) = dig.next() {
            num = d * 10;
        }
        if let Some(d) = dig.last() {
            num += d;
        } else {
            num = num + (num / 10);
        }
        sum += num;
    }
    sum
}

fn p2(input: &[String]) -> u32 {
    let mut sum = 0;
    for line in input {
        let mut v: Vec<u8> = p2_line_to_vec(line);
        let mut n = 0u32;
        if let Some(d) = v.first() {
            n = (*d as u32) * 10;
        }
        if let Some(d) = v.last() {
            n += *d as u32;
        }
        sum += n;
    }
    sum
}

fn p2_line_to_vec(line: &str) -> Vec<u8> {
    let mut v = vec![];
    for i in 0..line.len() {
        let l = &line[i..];
        if let Some(c) = l.chars().next() {
            if c.is_ascii_digit() {
                v.push(c as u8 - b'0');
            } else if l.starts_with("one") {
                v.push(1);
            } else if l.starts_with("two") {
                v.push(2);
            } else if l.starts_with("three") {
                v.push(3);
            } else if l.starts_with("four") {
                v.push(4);
            } else if l.starts_with("five") {
                v.push(5);
            } else if l.starts_with("six") {
                v.push(6);
            } else if l.starts_with("seven") {
                v.push(7);
            } else if l.starts_with("eight") {
                v.push(8);
            } else if l.starts_with("nine") {
                v.push(9);
            }
        }
    }
    v
}

fn main() {
    let input = Input::default().lines();
    println!("{}", p1(&input));
    println!("{}", p2(&input));
}

#[test]
fn test_p1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    let v: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let out = p1(&v);
    assert_eq!(out, 142);
}

#[test]
fn test_p2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    let v: Vec<String> = input.lines().map(|s| s.to_owned()).collect();

    for l in &v {
        let v8 = p2_line_to_vec(l);
        println!(
            "{}",
            v8.iter().map(|n| (*n + b'0') as char).collect::<String>()
        )
    }

    let out = p2(&v);
    assert_eq!(out, 281);
}
