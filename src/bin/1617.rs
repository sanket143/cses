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
    let n = scan.next::<num>();
    let out = &mut BufWriter::new(stdout());
    let mod_limit = (10 as num).pow(9) + 7;
    let max_n = 63;
    let times = n / max_n;
    let reminders = n % max_n;
    let mut result = (2 as num).pow(reminders as u32) % mod_limit;

    for _ in 0..times {
        result = (result * (2 as num).pow(max_n as u32) % mod_limit) % mod_limit;
    }

    writeln!(out, "{}", result % mod_limit).ok();
}
