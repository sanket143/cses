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

fn solve(n: usize, out: &mut BufWriter<Stdout>) {
    let mut temp = 1;

    for i in 1..n + 1 {
        write!(out, "{}", temp).ok();
        temp += 2;

        if temp > n {
            temp = 2;
        }

        if i != n {
            write!(out, " ").ok();
        }
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<usize>();

    let res = match n {
        1 => writeln!(out, "1"),
        2 => writeln!(out, "NO SOLUTION"),
        3 => writeln!(out, "NO SOLUTION"),
        4 => writeln!(out, "3 1 4 2"),
        _ => {
            solve(n, out);
            writeln!(out, "")
        }
    };

    res.ok();
}
