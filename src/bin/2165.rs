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

/*
a = 1 2
b = 1 3
c = 2 3

2 = abc
3 = abc a CBA c abc
4 = (abc a CBA c abc) a (CBA C abc A CBA) c (abc a CBA c abc)

2 = abc
3 = baC b Acb
4 = abc b ACb a cbA c bac
5 = baC b Acb a CBA C baC b (Acb c aBc A bca b cAb)
*/

fn solve_order(n: num, config: [u8; 3], result_count: &mut num, out: &mut BufWriter<Stdout>) {
    if n == 1 {
        writeln!(out, "{} {}", config[0], config[2]).ok();
        return;
    }

    *result_count += 1;

    solve_order(n - 1, [config[0], config[2], config[1]], result_count, out);
    solve_order(1, config, result_count, out);
    solve_order(n - 1, [config[1], config[0], config[2]], result_count, out);
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let n = scan.next::<num>();
    let mut result_count: num = 0;

    for _ in 0..n {
        result_count += result_count + 1;
    }

    writeln!(out, "{result_count}").ok();
    solve_order(n, [1, 2, 3], &mut result_count, out);
}
