use std::collections::HashMap;
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
3 = 2 a -2 c 2
3 = 3 a -3 c 3
*/

fn switch(text: String) -> String {
    text.chars()
        .into_iter()
        .map(|x| {
            if x.is_uppercase() {
                x.to_lowercase().to_string()
            } else {
                x.to_uppercase().to_string()
            }
        })
        .rfold(String::from(""), |mut res, x| {
            res.push_str(x.as_str());
            res
        })
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let n = scan.next::<num>();
    let mut hanoi_solutions = HashMap::from([(1, String::from("b")), (2, String::from("abc"))]);

    for i in 3..(n + 1) {
        let mut soln = hanoi_solutions[&(i - 1)].to_owned();
        soln.push_str("a");
        soln.push_str(switch(hanoi_solutions[&(i - 1)].to_owned()).as_str());
        soln.push_str("c");
        soln.push_str(hanoi_solutions[&(i - 1)].to_owned().as_str());
        hanoi_solutions.insert(i, soln);
        hanoi_solutions.remove(&(i - 1));
    }

    writeln!(out, "{}", hanoi_solutions[&n].len()).ok();
    for i in hanoi_solutions[&n].chars() {
        match i {
            'a' => writeln!(out, "1 2").ok(),
            'b' => writeln!(out, "1 3").ok(),
            'c' => writeln!(out, "2 3").ok(),
            'A' => writeln!(out, "2 1").ok(),
            'B' => writeln!(out, "3 1").ok(),
            'C' => writeln!(out, "3 2").ok(),
            _ => writeln!(out, "").ok(),
        };
    }
}
