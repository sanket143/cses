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
    use std::cmp;
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = scan.next::<u32>();

    for _ in 1..t + 1 {
        let x = scan.next::<i128>();
        let y = scan.next::<i128>();
        let m = cmp::max(x, y);
        let res = match m % 2 {
            0 => (m * (m - 1) + 1) + (x - y),
            1 => (m * (m - 1) + 1) + (y - x),
            _ => 0,
        };

        writeln!(out, "{}", res).ok();
    }
}
