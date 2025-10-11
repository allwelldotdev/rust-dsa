//! # Valid Parentheses algorithm (solution example)
//!
//! 1. Use a stack to push opening brackets.
//! 2. For closing, pop and check if it matches the expected opener (via hashmap).
//! 3. If mismatch or leftover stack, invalid.
//!
//! NOTES: `chars()` iterator; `unwrap()`` safe here due to checks.
//!
//! Time: O(n), Space: O(n).
//! Edge: Empty string (true), odd length (false).
//! Alternative: Match statements for brackets instead of map.

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // Using a hashmap to lookup closing parentheses
        let mut stack = Vec::new();
        let mut map = std::collections::HashMap::new();
        map.insert(')', '(');
        map.insert(']', '[');
        map.insert('}', '{');

        for ch in s.chars() {
            if map.contains_key(&ch) {
                if stack.is_empty() || stack.pop().unwrap() != *map.get(&ch).unwrap() {
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }
        stack.is_empty()

        // ---------

        // // Using match statements
        // let mut stack = Vec::new();

        // for ch in s.chars() {
        //     match ch {
        //         '(' | '[' | '{' => stack.push(ch),
        //         ')' => {
        //             if stack.pop() != Some('(') {
        //                 return false;
        //             }
        //         }
        //         ']' => {
        //             if stack.pop() != Some('[') {
        //                 return false;
        //             }
        //         }
        //         '}' => {
        //             if stack.pop() != Some('{') {
        //                 return false;
        //             }
        //         }
        //         _ => return false,
        //     }
        // }
        // stack.is_empty()
    }
}
