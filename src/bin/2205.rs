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

fn print_bits(arr: &Vec<num>, out: &mut BufWriter<Stdout>) {
    writeln!(
        out,
        "{}",
        arr.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    )
    .ok();
}

fn bitflip(arr: &mut Vec<num>, n: num) {
    arr[n] = (arr[n] + 1) % 2;
}

fn solve(arr: &mut Vec<num>, n: num, out: &mut BufWriter<Stdout>) {
    let len = arr.len();

    if n == 2 {
        bitflip(arr, len - 1);
        print_bits(arr, out);
        bitflip(arr, len - 2);
        print_bits(arr, out);
        bitflip(arr, len - 1);
        print_bits(arr, out);
        return;
    }

    solve(arr, n - 1, out);
    bitflip(arr, len - n);
    print_bits(arr, out);
    solve(arr, n - 1, out);
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let n = scan.next::<num>();
    let mut arr = vec![0; n];

    if n == 1 {
        write!(out, "0\n1").ok();
        return;
    }

    print_bits(&arr, out);
    solve(&mut arr, n, out);
}
