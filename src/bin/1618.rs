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
    let mut correct_ans = 0;
    let mut temp = 0;
    let mut temp_pow = 1;
    let mut idx = 0;

    for i in (5..n + 1).step_by(5) {
        let mut n = i;
        temp += 1;

        while n % 5 == 0 {
            n = n / 5;
            correct_ans += 1;
        }

        if i == 5_u128.pow(temp_pow) {
            write!(out, "\n").ok();
            temp_pow += 1;
            idx = 1;
        }

        let power = i.ilog(5);
        let mut d_power = power;
        let mut ans = i / 5; // + ((i - 5_u128.pow(power)) / 25) + ((i - 5_u128.pow(power)) / 125); // ;

        while d_power > 1 {
            ans += 5_u128.pow(d_power - 2);

            if d_power > 1 {
                ans += (i - 5_u128.pow(power)) / 5_u128.pow(d_power);
            }

            d_power -= 1;
        }

        write!(out, "[{i}] {temp}:{} ", correct_ans).ok();

        write!(
            out,
            "{}:{} {} / {} + {} = {}",
            idx,
            i.ilog(5),
            i / 5,
            (5_u128.pow(i.ilog(5))),
            i / 5,
            ans
        )
        .ok();

        write!(out, "\n").ok();

        if ans != correct_ans {
            writeln!(out, "WRONG_ANSWER!").ok();
        }
        idx += 1;
    }
}

/*
0
1 = 5 ^ 0
6 = 5^1 + 5^0
31 = 5^2 + 5^1 + 5^0
*/
