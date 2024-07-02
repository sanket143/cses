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

    let sequence = scan.next::<String>();
    let mut last_character = 'X';
    let mut max_count = 0;
    let mut curr_count = 0;

    for i in sequence.chars() {
        if last_character == 'X' || last_character == i {
            curr_count += 1;

            if max_count < curr_count {
                max_count = curr_count;
            }
        } else {
            curr_count = 1;
        }

        last_character = i;
    }

    writeln!(out, "{}", max_count).ok();
}
