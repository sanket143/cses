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
type num = u128;

fn main() {
    let mut scan = Scanner::default();
    let n = scan.next::<usize>();
    let mut arr = (0..n).map(|_| scan.next::<num>()).collect::<Vec<num>>();
    arr.sort();

    let out = &mut BufWriter::new(stdout());
    let mut idx: (usize, usize) = (0, n - 1);

    let mut acc: (num, num) = (0, 0);

    while idx.0 <= idx.1 {
        if acc.0 < acc.1 {
            writeln!(out, "[0] {}", arr[idx.0]).ok();
            acc.0 += arr[idx.0];
            idx.0 += 1;
        } else {
            writeln!(out, "[1] {}", arr[idx.1]).ok();
            acc.1 += arr[idx.1];
            idx.1 -= 1;
        }
    }

    writeln!(out, "{:?} {:?}", arr, acc).ok();
}
