use std::collections::VecDeque;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut stack = Vec::with_capacity(16);  
        let mut result = 0;
        let mut sign = 1;
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'0'..=b'9' => {
                    let mut num = 0;
                    while i < bytes.len() && bytes[i].is_ascii_digit() {
                        num = num * 10 + (bytes[i] - b'0') as i32;
                        i += 1;
                    }
                    result += sign * num;
                    continue;
                }
                b'+' => sign = 1,
                b'-' => sign = -1,
                b'(' => {
                    stack.push((result, sign));
                    result = 0;
                    sign = 1;
                }
                b')' => {
                    let (prev_result, prev_sign) = stack.pop().unwrap();
                    result = prev_result + prev_sign * result;
                }
                _ => {}
            }
            i += 1;
        }
        result
    }
}