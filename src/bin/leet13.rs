use std::collections::HashMap;

struct Solution;

// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
//
// MCMXCIV
// 1000
// 1000 - 100 = 900
// 100 - 10 = 90
// 4
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut ans = 0;
        let roman_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut prev: Option<char> = None;

        s.chars().for_each(|ch| {
            if let Some(prev) = prev {
                if roman_map[&prev] < roman_map[&ch] {
                    ans += roman_map[&ch] - (roman_map[&prev] * 2);
                } else {
                    ans += roman_map[&ch];
                }
            } else {
                ans += roman_map[&ch];
            }

            prev = Some(ch);
        });

        ans
    }
}

fn main() {
    println!("{:?}", Solution::roman_to_int("III".into())); // 3
    println!("{:?}", Solution::roman_to_int("LVIII".into())); // 58
    println!("{:?}", Solution::roman_to_int("MCMXCIV".into())); // 1994
}

