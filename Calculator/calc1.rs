// Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.

// Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

 

// Example 1:

// Input: s = "1 + 1"
// Output: 2
// Example 2:

// Input: s = " 2-1 + 2 "
// Output: 3
// Example 3:

// Input: s = "(1+(4+5+2)-3)+(6+8)"
// Output: 23


impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;      //  running total
        let mut sign = 1;        //  adding (+1) or subtracting (-1)?
        let mut stack = vec![];   // memory box for parentheses
        let mut i = 0;
        let s = s.as_bytes();
        
        while i < s.len() {
            match s[i] {
                b' ' => i += 1,  // skip invisible spaces
                
                b'+' => {
                    sign = 1;     //  adding the next number
                    i += 1;
                }
                
                b'-' => {
                    sign = -1;    //  subtracting the next number
                    i += 1;
                }
                
                b'(' => {
                    // save work in the memory box
                    stack.push(result);
                    stack.push(sign);
                    // start fresh inside the parentheses
                    result = 0;
                    sign = 1;
                    i += 1;
                }
                
                b')' => {
                    // take work out of the memory box
                    let prev_sign = stack.pop().unwrap();
                    let prev_result = stack.pop().unwrap();
                    // combine with what's inside parentheses
                    result = prev_result + prev_sign * result;
                    i += 1;
                }
                
                b'0'..=b'9' => {
                    // build number (like 1, 2, 3 becomes 123)
                    let mut num = 0;
                    while i < s.len() && s[i].is_ascii_digit() {
                        num = num * 10 + (s[i] - b'0') as i32;
                        i += 1;
                    }
                    // Add/subtract this number to my running total
                    result = result + sign * num;
                }
                
                _ => i += 1,
            }
        }
        
        result  // Ta-daaaaa!
    }
}