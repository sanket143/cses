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
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let n = scan.next::<num>();

    let power = n.ilog(5);
    let mut d_power = power;
    let mut ans = n / 5;

    while d_power > 1 {
        ans += 5_u128.pow(d_power - 2);

        if d_power > 1 {
            ans += (n - 5_u128.pow(power)) / 5_u128.pow(d_power);
        }

        d_power -= 1;
    }

    writeln!(out, "{ans}").ok();
}
