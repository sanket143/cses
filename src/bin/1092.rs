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
type num = usize;

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = scan.next::<num>();

    if n % 4 == 0 || (n + 1) % 4 == 0 {
        let mut current = n;
        let mut limit = n * (n + 1) / 4 - current;
        let mut arr = vec![false; n];

        arr[current - 1] = true;
        current = n - 1;

        while limit != 0 || current != 0 {
            if !arr[current - 1] && limit >= current {
                limit -= current;
                arr[current - 1] = true;
            }

            current = cmp::min(cmp::min(limit, n), current - 1);
        }

        let first: Vec<String> = arr
            .clone()
            .into_iter()
            .enumerate()
            .map(|(e, x)| {
                {
                    if x {
                        e + 1
                    } else {
                        0
                    }
                }
                .to_string()
            })
            .filter(|x| *x != "0")
            .collect();
        let second: Vec<String> = arr
            .clone()
            .into_iter()
            .enumerate()
            .map(|(e, x)| {
                {
                    if !x {
                        e + 1
                    } else {
                        0
                    }
                }
                .to_string()
            })
            .filter(|x| *x != "0")
            .collect();

        writeln!(out, "YES").ok();
        writeln!(out, "{}", first.len()).ok();
        writeln!(out, "{}", first.join(" ")).ok();
        writeln!(out, "{}", second.len()).ok();
        writeln!(out, "{}", second.join(" ")).ok();
    } else {
        writeln!(out, "NO").ok();
    }
}
