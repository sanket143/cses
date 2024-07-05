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
type num = i128;

fn compute(a: num, b: num) -> num {
    (2 * b - a) / 3
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let t = scan.next::<num>();

    for _ in 0..t {
        let a = scan.next::<num>();
        let b = scan.next::<num>();

        if (2 * b - a) * (2 * a - b) % 3 == 0 && (compute(a, b) >= 0 && compute(b, a) >= 0) {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
        }
    }
}
