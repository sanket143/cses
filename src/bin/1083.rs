#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

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
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let n = scan.next::<usize>();
    let mut arr = vec![false; n];

    for _ in 0..n - 1 {
        let x = scan.next::<usize>();
        arr[x - 1] = true;
    }

    for i in 0..n {
        if !arr[i] {
            writeln!(out, "{}", i + 1).ok();
            break;
        }
    }
}
