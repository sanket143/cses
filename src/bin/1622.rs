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

fn solve(input: Vec<num>, remains: Vec<num>, result: &mut HashMap<Vec<num>, bool>) {
    if remains.len() == 0 {
        result.insert(input, true);
        return;
    }

    for i in 0..remains.len() {
        let mut input = input.clone();
        let mut remains = remains.clone();

        input.push(remains.remove(i));

        solve(input, remains, result);
    }
}

fn main() {
    let out = &mut BufWriter::new(stdout());
    let mut scan = Scanner::default();
    let input = scan.next::<String>();
    let mut result_map = HashMap::new();
    let mut result = vec![];
    let int_to_char_vec = (0..26)
        .map(|x| (x + 97 as u8) as char)
        .collect::<Vec<char>>();
    let mut vec: Vec<num> = input
        .clone()
        .into_bytes()
        .iter()
        .map(|ch| (ch - 97) as num)
        .collect();

    vec.sort_by(|a, b| a.cmp(b));

    solve(vec![], vec.clone(), &mut result_map);

    writeln!(out, "{}", result_map.len()).ok();

    result_map.into_keys().for_each(|arr| {
        let combination = arr.iter().map(|x| int_to_char_vec[*x]).collect::<String>();
        result.push(combination);
    });

    result.sort_unstable();
    result.iter().for_each(|x| {
        writeln!(out, "{}", x).ok();
    });
}
