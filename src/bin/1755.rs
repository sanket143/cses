#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Stdout, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let mut alphabets = vec![0; 26];
    let input = scan.next::<String>().into_bytes();
    let mut odd_count_support = input.len() % 2;
    let mut pivot_character = 0;

    input.iter().for_each(|x| {
        let idx = (90 - x) as usize;
        alphabets[idx] += 1;
    });

    for i in 0..alphabets.len() {
        let x = alphabets[i];
        if x % 2 == 1 {
            if odd_count_support > 0 {
                pivot_character = 90 - i;
                odd_count_support -= 1;
                alphabets[i] -= 1;
                continue;
            } else {
                writeln!(out, "NO SOLUTION").ok();
                return;
            }
        }
    }

    for i in 0..alphabets.len() {
        for _ in 0..alphabets[i] / 2 {
            write!(out, "{}", std::str::from_utf8(&[90 - i as u8]).unwrap()).ok();
        }
    }

    if pivot_character > 0 {
        write!(
            out,
            "{}",
            std::str::from_utf8(&[pivot_character as u8]).unwrap()
        )
        .ok();
    }

    for i in 0..alphabets.len() {
        let i = alphabets.len() - i - 1;
        for _ in 0..alphabets[i] / 2 {
            write!(out, "{}", std::str::from_utf8(&[90 - i as u8]).unwrap()).ok();
        }
    }
}
