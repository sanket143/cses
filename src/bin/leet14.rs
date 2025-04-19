use std::cmp;

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut answer = String::new();

        if strs.len() == 1 {
            return strs[0].clone();
        }

        let smallest_string_length = strs
            .iter()
            .fold(strs[0].len(), |acc, x| cmp::min(acc, x.len()));

        for i in 0..smallest_string_length {
            let f_ch = strs[0].get(i..i + 1).unwrap();

            for string in &strs {
                let ch = string.get(i..i + 1).unwrap();

                if f_ch != ch {
                    return answer;
                }
            }

            answer.push_str(f_ch);
        }

        answer
    }
}

fn main() {
    [
        vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()],
        vec![
            "dog".to_owned(),
            "racecar".to_owned(),
            "car".to_owned(),
            "".to_owned(),
        ],
        vec!["a".to_owned()],
        vec!["a".to_owned(), "ab".to_owned()],
    ]
    .iter()
    .for_each(|x| {
        println!("{:?}", Solution::longest_common_prefix(x.to_owned()));
    });
}
