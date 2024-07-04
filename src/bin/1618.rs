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
    let mut ans = 0;
    let mut temp = 0;
    let mut pow = 1;
    let mut idx = 0;

    for i in (5..n + 1).step_by(5) {
        let mut n = i;
        temp += 1;

        while n % 5 == 0 {
            n = n / 5;
            ans += 1;
        }

        if i == 5_u128.pow(pow) {
            write!(out, "\n").ok();
            pow += 1;
            idx = 1;
        }

        write!(out, "[{i}] {temp}:{} ", ans).ok();

        write!(out, "{}:{} {}", idx, i.ilog(5), i / 5).ok();

        write!(out, "\n").ok();
        idx += 1;
    }
}

/*
missing numbers

5
11
17
23

29
30
36
42
48
54
*/
