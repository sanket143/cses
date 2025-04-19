struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        let opening = ['{', '[', '('];
        let closing = ['}', ']', ')'];

        for ch in s.chars() {
            let s_idx = opening.iter().position(|x| x == &ch);
            let c_idx = closing.iter().position(|x| x == &ch);

            if let Some(idx) = s_idx {
                stack.push(ch);
            } else if let Some(idx) = c_idx {
                if let Some(last) = stack.last() {
                    if last == &opening[idx] {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        stack.is_empty()
    }
}

fn main() {
    [
        String::from("()"),
        String::from("()[]{}"),
        String::from("(]"),
        String::from("([])"),
        String::from("("),
    ]
    .iter()
    .for_each(|x| {
        println!("{:?}", Solution::is_valid(x.to_owned()));
    });
}
