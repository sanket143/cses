use std::cmp;
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

#[allow(non_camel_case_types)]
type num = i64;

fn solve(n: num) -> num {
    n.pow(2) * (n.pow(2) - 1)
        - (2 * 4)
        - (3 * 8)
        - (4 * cmp::max(n - 4, 0) * 4)
        - (4 * 4)
        - (6 * cmp::max(n - 4, 0) * 4)
        - (8 * cmp::max(n - 4, 0) * cmp::max(n - 4, 0))
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<num>();

    for n in 1..t + 1 {
        match n {
            1 => writeln!(out, "0"),
            2 => writeln!(out, "6"),
            3 => writeln!(out, "28"),
            _ => writeln!(out, "{}", solve(n) / 2),
        }
        .ok();
    }
}
